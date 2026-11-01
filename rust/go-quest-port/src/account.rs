//! Explicit, read-only account connectivity check.
//!
//! This command is intentionally separate from Quest inspection. It does one
//! HTTPS GET to a fixed endpoint and never stores or prints the supplied token.
//! Personal user-token API use is not an officially supported Discord bot flow.

use reqwest::{
    blocking::Client,
    header::{HeaderValue, AUTHORIZATION, ACCEPT},
    redirect::Policy,
};
use serde::Serialize;
use std::{env, io::Read, time::Duration};

const USER_PROFILE_ENDPOINT: &str = "https://discord.com/api/v10/users/@me";
const MAX_BODY_BYTES: u64 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckError {
    MissingToken,
    InvalidToken,
    ConnectionFailed,
    Unauthorized,
    Forbidden,
    RateLimited,
    UnexpectedStatus(u16),
    InvalidResponse,
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingToken => write!(out, "DISCORD_USER_TOKEN is missing"),
            Self::InvalidToken => write!(out, "DISCORD_USER_TOKEN is empty or has invalid header bytes"),
            Self::ConnectionFailed => write!(out, "HTTPS connection failed or timed out"),
            Self::Unauthorized => write!(out, "HTTP 401: authorization rejected"),
            Self::Forbidden => write!(out, "HTTP 403: request forbidden"),
            Self::RateLimited => write!(out, "HTTP 429: rate limited; no automatic retries"),
            Self::UnexpectedStatus(code) => write!(out, "Unexpected HTTP status {code}"),
            Self::InvalidResponse => write!(out, "API response was not a valid user profile"),
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AccountCheck {
    pub authenticated: bool,
    pub http_status: u16,
    pub operation: &'static str,
}

/// A single, opt-in request. The token is not echoed in errors or results.
/// The endpoint parameter exists only to permit a loopback mock in unit tests;
/// the CLI always invokes this with the fixed HTTPS Discord endpoint.
fn check_with_endpoint(endpoint: &str, token: &str) -> Result<AccountCheck, CheckError> {
    if token.is_empty() || token.trim().is_empty() {
        return Err(CheckError::InvalidToken);
    }
    let token_header = HeaderValue::from_str(token).map_err(|_| CheckError::InvalidToken)?;
    if token.as_bytes().iter().any(|byte| byte.is_ascii_control()) {
        return Err(CheckError::InvalidToken);
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(8))
        .connect_timeout(Duration::from_secs(4))
        .redirect(Policy::none())
        // Do not forward a personal credential through ambient proxy settings.
        .no_proxy()
        .user_agent(concat!("go-quest-port/", env!("CARGO_PKG_VERSION"), " read-only-check"))
        .build()
        .map_err(|_| CheckError::ConnectionFailed)?;

    let mut response = client
        .get(endpoint)
        .header(AUTHORIZATION, token_header)
        .header(ACCEPT, "application/json")
        .send()
        .map_err(|_| CheckError::ConnectionFailed)?;

    let code = response.status().as_u16();
    match code {
        200 => {}
        401 => return Err(CheckError::Unauthorized),
        403 => return Err(CheckError::Forbidden),
        429 => return Err(CheckError::RateLimited),
        _ => return Err(CheckError::UnexpectedStatus(code)),
    }

    // Read only a small body. Never serialize a profile into our output.
    let mut body = Vec::new();
    response
        .take(MAX_BODY_BYTES + 1)
        .read_to_end(&mut body)
        .map_err(|_| CheckError::InvalidResponse)?;
    if body.len() as u64 > MAX_BODY_BYTES {
        return Err(CheckError::InvalidResponse);
    }
    let profile: serde_json::Value =
        serde_json::from_slice(&body).map_err(|_| CheckError::InvalidResponse)?;
    if profile
        .get("id")
        .and_then(serde_json::Value::as_str)
        .is_none_or(str::is_empty)
    {
        return Err(CheckError::InvalidResponse);
    }

    Ok(AccountCheck {
        authenticated: true,
        http_status: 200,
        operation: "read_only_users_me",
    })
}

/// Calls exactly one fixed Discord REST endpoint using a locally supplied raw
/// Authorization header. Never takes a token as a command-line argument.
pub fn check_account_from_env() -> Result<AccountCheck, CheckError> {
    let token = env::var("DISCORD_USER_TOKEN").map_err(|_| CheckError::MissingToken)?;
    check_with_endpoint(USER_PROFILE_ENDPOINT, &token)
}

#[cfg(test)]
mod tests {
    use super::{check_with_endpoint, CheckError};
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };

    fn mock_once(status: &str, payload: &str, expected_header: &str) -> String {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let status = status.to_string();
        let body = payload.to_string();
        let header = expected_header.to_string();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            let mut bytes = [0_u8; 4096];
            let count = stream.read(&mut bytes).unwrap();
            let text = String::from_utf8_lossy(&bytes[..count]).to_lowercase();
            assert!(text.starts_with("get /api/v10/users/@me http/1.1"), "{text}");
            assert!(text.contains(&format!("authorization: {header}\r\n")));
            let reply = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(reply.as_bytes()).unwrap();
        });
        let url = format!("http://{address}/api/v10/users/@me");
        let result = check_with_endpoint(&url, expected_header);
        server.join().unwrap();
        match result {
            Ok(response) => format!("{} {}", response.http_status, response.operation),
            Err(error) => error.to_string(),
        }
    }

    #[test]
    fn sends_unprefixed_raw_authorization_and_limits_output() {
        let result = mock_once("200 OK", r#"{"id":"123","username":"DO_NOT_PRINT"}"#, "FAKE_TEST_TOKEN");
        assert_eq!(result, "200 read_only_users_me");
        assert!(!result.contains("123"));
        assert!(!result.contains("FAKE_TEST_TOKEN"));
    }

    #[test]
    fn unauthenticated_status_never_echoes_token_or_response() {
        let result = mock_once("401 Unauthorized", r#"{"message":"SENSITIVE_BODY"}"#, "FAKE_TEST_TOKEN");
        assert_eq!(result, "HTTP 401: authorization rejected");
    }

    #[test]
    fn throttling_does_not_retry_or_echo_body() {
        let result = mock_once("429 Too Many Requests", r#"{"message":"NO_ECHO"}"#, "FAKE_TEST_TOKEN");
        assert_eq!(result, "HTTP 429: rate limited; no automatic retries");
    }

    #[test]
    fn rejects_empty_or_control_byte_tokens_before_request() {
        for token in ["", " ", "fake\nheader", "fake\r\nX-Test: inject"] {
            assert_eq!(
                check_with_endpoint("http://127.0.0.1:1", token),
                Err(CheckError::InvalidToken)
            );
        }
    }

    #[test]
    fn successful_status_still_requires_valid_minimal_profile() {
        assert_eq!(
            mock_once("200 OK", r#"{"other":true}"#, "FAKE_TEST_TOKEN"),
            "API response was not a valid user profile"
        );
    }

    #[test]
    fn no_following_redirect_with_authorization() {
        let result = mock_once("302 Found", r#"{"location":"example"}"#, "FAKE_TEST_TOKEN");
        assert_eq!(result, "Unexpected HTTP status 302");
    }
}
