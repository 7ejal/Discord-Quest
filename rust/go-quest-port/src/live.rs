//! Opt-in real HTTPS reads using documented public Quest config and OAuth2 identity.
//! Raw personal-token HTTP is a separate explicit, read-only diagnostic; never simulates rewarded Quest activity.
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderValue, AUTHORIZATION},
    redirect::Policy,
    StatusCode,
};
use serde::Serialize;
use serde_json::Value;
use std::{
    io::{self, IsTerminal, Read},
    time::Duration,
};
use zeroize::Zeroizing;

const API: &str = "https://discord.com/api/v10";
const MAX_BODY_BYTES: usize = 1024 * 1024;
const MAX_CREDENTIAL_BYTES: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub enum LiveError {
    InvalidQuestId,
    InvalidCredentialInput,
    InvalidRawCredentialInput,
    AuthenticationRequired,
    PermissionDenied,
    RateLimited,
    NotFound,
    RemoteUnavailable,
    UnexpectedStatus,
    HttpStatus(u16),
    OversizeResponse,
    InvalidResponse,
}

impl std::fmt::Display for LiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::HttpStatus(status) => {
                return write!(f, "HTTP {status}; response body suppressed; no retry")
            }
            Self::InvalidQuestId => "Quest ID must contain 1-20 ASCII digits",
            Self::InvalidCredentialInput => "Expected one access token from non-terminal stdin",
            Self::InvalidRawCredentialInput => {
                "Raw Authorization requires a single bare token on stdin, without Bot/Bearer prefix"
            }
            Self::AuthenticationRequired => "HTTP 401: request unauthorized or token invalid",
            Self::PermissionDenied => "Read was denied (HTTP 403)",
            Self::RateLimited => "Rate limited (HTTP 429); no retry was attempted",
            Self::NotFound => "Quest not found or is not publicly available (HTTP 404)",
            Self::RemoteUnavailable => "HTTPS request failed or timed out",
            Self::UnexpectedStatus => "Unexpected HTTP response; no body printed",
            Self::OversizeResponse => "HTTP response exceeded the fixed 1 MiB limit",
            Self::InvalidResponse => "Unexpected JSON response shape",
        };
        f.write_str(label)
    }
}

fn make_client() -> Result<Client, LiveError> {
    Client::builder()
        .https_only(true)
        .no_proxy()
        .redirect(Policy::none())
        .retry(reqwest::retry::never())
        .connect_timeout(Duration::from_secs(4))
        .timeout(Duration::from_secs(10))
        .user_agent(concat!(
            "go-quest-port/",
            env!("CARGO_PKG_VERSION"),
            " (read-only)"
        ))
        .build()
        .map_err(|_| LiveError::RemoteUnavailable)
}

fn get_json(client: &Client, path: &str, oauth: Option<&str>) -> Result<Value, LiveError> {
    let mut request = client.get(format!("{API}{path}"));
    if let Some(token) = oauth {
        request = request.bearer_auth(token);
    }
    send_json(request)
}

fn send_json(request: RequestBuilder) -> Result<Value, LiveError> {
    let response = request.send().map_err(|_| LiveError::RemoteUnavailable)?;
    match response.status() {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => return Err(LiveError::AuthenticationRequired),
        StatusCode::FORBIDDEN => return Err(LiveError::PermissionDenied),
        StatusCode::NOT_FOUND => return Err(LiveError::NotFound),
        StatusCode::TOO_MANY_REQUESTS => return Err(LiveError::RateLimited),
        status if status.is_redirection() => return Err(LiveError::UnexpectedStatus),
        status => return Err(LiveError::HttpStatus(status.as_u16())),
    }
    if response
        .content_length()
        .is_some_and(|n| n > MAX_BODY_BYTES as u64)
    {
        return Err(LiveError::OversizeResponse);
    }
    let mut bytes = Zeroizing::new(Vec::new());
    response
        .take((MAX_BODY_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| LiveError::RemoteUnavailable)?;
    if bytes.len() > MAX_BODY_BYTES {
        return Err(LiveError::OversizeResponse);
    }
    serde_json::from_slice(&bytes).map_err(|_| LiveError::InvalidResponse)
}

pub fn valid_quest_id(id: &str) -> bool {
    (1..=20).contains(&id.len()) && id.bytes().all(|b| b.is_ascii_digit())
}

#[derive(Debug, Serialize)]
pub struct PublicQuestConfig {
    pub input_trust: &'static str,
    pub quest_id: String,
    pub title: Option<String>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
    pub task_types: Vec<String>,
    pub note: &'static str,
}

/// Fetches public Quest *configuration*, not a user's enrollment, progress or reward state.
/// The existence and response shape of this unauthenticated endpoint come from
/// an unofficial API reference. A 200 does not prove Quest availability for a user.
pub fn fetch_public_quest_config(id: &str) -> Result<PublicQuestConfig, LiveError> {
    if !valid_quest_id(id) {
        return Err(LiveError::InvalidQuestId);
    }
    let client = make_client()?;
    let value = get_json(&client, &format!("/quests/{id}"), None)?;
    parse_public_config(id, &value)
}

fn parse_public_config(id: &str, value: &Value) -> Result<PublicQuestConfig, LiveError> {
    let object = value.as_object().ok_or(LiveError::InvalidResponse)?;
    // Never treat a malformed HTTP 200 JSON object as a valid Quest config.
    if !object
        .get("id")
        .and_then(Value::as_str)
        .is_some_and(|v| v == id)
        || !object.get("expires_at").is_some_and(Value::is_string)
    {
        return Err(LiveError::InvalidResponse);
    }
    let title = value
        .pointer("/messages/quest_name")
        .and_then(Value::as_str)
        .map(str::to_owned);
    let mut task_types: Vec<String> = value
        .pointer("/task_config_v2/tasks")
        .and_then(Value::as_object)
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_default();
    task_types.sort();
    Ok(PublicQuestConfig {
        input_trust: "unverified_remote_public_config",
        quest_id: id.to_owned(),
        title,
        starts_at: object
            .get("starts_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        expires_at: object
            .get("expires_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        task_types,
        note: "Public configuration only; no account status or verified reward completion",
    })
}

#[derive(Debug, Serialize)]
pub struct OAuthCheck {
    pub authentication: &'static str,
    pub valid: bool,
    pub note: &'static str,
}

/// Identity verification on Discord's officially supported OAuth2 identify grant.
/// A personal Discord session token is not an OAuth2 Bearer token.
fn read_token_from_stdin(raw: bool) -> Result<Zeroizing<String>, LiveError> {
    if io::stdin().is_terminal() {
        return Err(LiveError::InvalidCredentialInput);
    }
    let mut bytes = Zeroizing::new(Vec::new());
    io::stdin()
        .take((MAX_CREDENTIAL_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| LiveError::InvalidCredentialInput)?;
    if bytes.len() > MAX_CREDENTIAL_BYTES {
        return Err(LiveError::InvalidCredentialInput);
    }
    if bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    let token = std::str::from_utf8(&bytes).map_err(|_| LiveError::InvalidCredentialInput)?;
    let token = if raw {
        token.strip_prefix("Authorization: ").unwrap_or(token)
    } else {
        token
    };
    let token = Zeroizing::new(token.to_owned());
    if token.is_empty()
        || token.len() > MAX_CREDENTIAL_BYTES
        || token
            .bytes()
            .any(|b| b.is_ascii_whitespace() || b.is_ascii_control() || !b.is_ascii())
    {
        return Err(LiveError::InvalidCredentialInput);
    }
    Ok(token)
}

pub fn check_oauth_identity_from_stdin() -> Result<OAuthCheck, LiveError> {
    let token = read_token_from_stdin(false)?;
    let client = make_client()?;
    let json = get_json(&client, "/users/@me", Some(token.as_str()))?;
    let id = json.get("id").and_then(Value::as_str);
    if !id.is_some_and(valid_quest_id) {
        return Err(LiveError::InvalidResponse);
    }
    Ok(OAuthCheck {
        authentication: "official_oauth2_identify",
        valid: true,
        note: "Identity only; OAuth2 identify does not grant personal Quest access",
    })
}

/// One explicit raw-Authorization test against the caller's Discord user session.
/// Discord disallows automated normal user accounts. This read-only command is
/// experimental, undocumented, and not an approved authentication integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawProbe {
    Account,
    Quests,
}

impl RawProbe {
    fn path(self) -> &'static str {
        match self {
            Self::Account => "/users/@me",
            Self::Quests => "/quests/@me",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RawHttpResult {
    pub authentication: &'static str,
    pub endpoint: &'static str,
    pub http_status: u16,
    pub response_valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quest_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_quest_count: Option<usize>,
    pub note: &'static str,
}

fn build_raw_request(
    client: &Client,
    probe: RawProbe,
    token: &str,
) -> Result<RequestBuilder, LiveError> {
    // Never silently prepend Bot/Bearer, forward to another host, or print
    // the credential. Restrict the request to two fixed read-only GET paths.
    if token.is_empty()
        || token.starts_with("Bearer ")
        || token.starts_with("Bot ")
        || token
            .bytes()
            .any(|b| !b.is_ascii() || b.is_ascii_whitespace() || b.is_ascii_control())
    {
        return Err(LiveError::InvalidRawCredentialInput);
    }
    let mut header =
        HeaderValue::from_str(token).map_err(|_| LiveError::InvalidRawCredentialInput)?;
    header.set_sensitive(true);
    Ok(client
        .get(format!("{API}{}", probe.path()))
        .header(AUTHORIZATION, header))
}

fn assess_raw_response(probe: RawProbe, value: &Value) -> Result<RawHttpResult, LiveError> {
    let (quest_count, excluded_quest_count) = match probe {
        RawProbe::Account => {
            if !value
                .get("id")
                .and_then(Value::as_str)
                .is_some_and(valid_quest_id)
            {
                return Err(LiveError::InvalidResponse);
            }
            (None, None)
        }
        RawProbe::Quests => {
            let quests = value
                .get("quests")
                .and_then(Value::as_array)
                .ok_or(LiveError::InvalidResponse)?;
            let excluded = value
                .get("excluded_quests")
                .and_then(Value::as_array)
                .map(Vec::len);
            (Some(quests.len()), excluded)
        }
    };
    Ok(RawHttpResult {
        authentication: "raw_personal_token_unsupported_by_official_api",
        endpoint: probe.path(),
        http_status: 200,
        response_valid: true,
        quest_count,
        excluded_quest_count,
        note: "Read-only response; not official user-token API support, verified Quest eligibility, or completion",
    })
}

/// Consumes a locally supplied bare token from stdin. Never accepts token argv,
/// retrieves token from env/files, persists a token or emits user identifiers.
/// Exactly one non-redirecting HTTP GET, no retries, no activity/claim endpoints.
pub fn raw_http_check_from_stdin(probe: RawProbe) -> Result<RawHttpResult, LiveError> {
    let token = read_token_from_stdin(true).map_err(|_| LiveError::InvalidRawCredentialInput)?;
    let client = make_client()?;
    let request = build_raw_request(&client, probe, token.as_str())?;
    let value = send_json(request)?;
    assess_raw_response(probe, &value)
}

#[cfg(test)]
mod tests {
    use super::{
        assess_raw_response, build_raw_request, make_client, parse_public_config, valid_quest_id,
        LiveError, RawProbe,
    };
    use super::{send_json, Client, Duration, Policy, Value, MAX_BODY_BYTES};
    use reqwest::header::AUTHORIZATION;
    use serde_json::json;

    #[test]
    fn raw_header_has_no_oauth_or_bot_prefix_and_stays_on_fixed_host() {
        let client = make_client().unwrap();
        for probe in [RawProbe::Account, RawProbe::Quests] {
            let request = build_raw_request(&client, probe, "SYNTHETIC_USER_TOKEN_ONLY")
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                request.headers().get(AUTHORIZATION).unwrap(),
                "SYNTHETIC_USER_TOKEN_ONLY"
            );
            assert_eq!(request.url().host_str(), Some("discord.com"));
            assert_eq!(request.url().path(), format!("/api/v10{}", probe.path()));
            assert_eq!(request.method(), reqwest::Method::GET);
            assert!(request.headers()[AUTHORIZATION].is_sensitive());
            assert!(!request.url().as_str().contains("SYNTHETIC_USER_TOKEN_ONLY"));
        }
    }

    #[test]
    fn raw_token_prefixes_and_control_characters_are_refused() {
        let client = make_client().unwrap();
        for candidate in ["Bearer test", "Bot test", "test\nother", "", "bad token"] {
            assert!(matches!(
                build_raw_request(&client, RawProbe::Account, candidate),
                Err(LiveError::InvalidRawCredentialInput)
            ));
        }
    }

    #[test]
    fn raw_response_reports_only_counts_or_validity_not_account_details() {
        let account = assess_raw_response(
            RawProbe::Account,
            &json!({"id":"123456789012345678","username":"DO_NOT_PRINT"}),
        )
        .unwrap();
        let json = serde_json::to_string(&account).unwrap();
        assert!(account.response_valid);
        assert!(!json.contains("DO_NOT_PRINT"));
        assert!(!json.contains("123456789012345678"));
        let quests = assess_raw_response(
            RawProbe::Quests,
            &json!({"quests":[{"id":"PRIVATE_A"},{"id":"PRIVATE_B"}],"excluded_quests":[{}]}),
        )
        .unwrap();
        assert_eq!(quests.quest_count, Some(2));
        assert_eq!(quests.excluded_quest_count, Some(1));
        assert!(!serde_json::to_string(&quests)
            .unwrap()
            .contains("PRIVATE_A"));
        assert!(assess_raw_response(RawProbe::Quests, &json!({})).is_err());
    }

    // Loopback responses are synthetic; these tests never contact Discord.
    #[test]
    fn synthetic_http_status_json_size_redirect_and_timeout() {
        use std::{
            io::{Read, Write},
            net::TcpListener,
            thread,
        };
        fn response(status: u16, body: &str, delay: bool) -> Result<Value, LiveError> {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let body = body.to_owned();
            let worker = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0; 4096];
                let length = stream.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..length]).to_ascii_lowercase();
                assert!(request.starts_with("get /api/v10/users/@me http/1.1"));
                assert!(request.contains("\r\nauthorization: synthetic_user_token_only\r\n"));
                if delay {
                    thread::sleep(Duration::from_millis(150));
                }
                let _ = write!(stream, "HTTP/1.1 {status} Synthetic\r\nContent-Length: {}\r\nLocation: http://127.0.0.1:1/PRIVATE\r\nConnection: close\r\n\r\n{body}", body.len());
            });
            let client = Client::builder()
                .no_proxy()
                .redirect(Policy::none())
                .retry(reqwest::retry::never())
                .timeout(Duration::from_millis(80))
                .build()
                .unwrap();
            let mut request =
                build_raw_request(&client, RawProbe::Account, "SYNTHETIC_USER_TOKEN_ONLY")
                    .unwrap()
                    .build()
                    .unwrap();
            // Only tests replace the origin; production has no endpoint override.
            *request.url_mut() = format!("http://{address}/api/v10/users/@me")
                .parse()
                .unwrap();
            let result = send_json(reqwest::blocking::RequestBuilder::from_parts(
                client, request,
            ));
            worker.join().unwrap();
            result
        }
        assert_eq!(
            response(200, r#"{"ok":true}"#, false).unwrap(),
            serde_json::json!({"ok":true})
        );
        for (status, expected) in [
            (401, LiveError::AuthenticationRequired),
            (403, LiveError::PermissionDenied),
            (429, LiveError::RateLimited),
            (500, LiveError::HttpStatus(500)),
            (503, LiveError::HttpStatus(503)),
            (302, LiveError::UnexpectedStatus),
        ] {
            assert_eq!(
                response(status, "PRIVATE_BODY", false).unwrap_err(),
                expected
            );
        }
        assert_eq!(
            response(200, "PRIVATE_MALFORMED", false).unwrap_err(),
            LiveError::InvalidResponse
        );
        assert_eq!(
            response(200, &"x".repeat(MAX_BODY_BYTES + 1), false).unwrap_err(),
            LiveError::OversizeResponse
        );
        assert_eq!(
            response(200, "{}", true).unwrap_err(),
            LiveError::RemoteUnavailable
        );
    }

    #[test]
    fn quest_id_is_strictly_numeric_and_bounded() {
        assert!(valid_quest_id("123456789012345678"));
        assert!(!valid_quest_id("1/../users/@me"));
        assert!(!valid_quest_id("0\nAuthorization:bad"));
        assert!(!valid_quest_id(""));
        assert!(!valid_quest_id("1".repeat(21).as_str()));
    }

    #[test]
    fn remote_config_is_explicitly_not_a_user_status() {
        let id = "123456789012345678";
        let input = json!({
            "id": id,
            "starts_at": "2026-11-01T00:00:00+00:00",
            "expires_at": "2026-11-04T00:00:00+00:00",
            "messages": {"quest_name":"Example"},
            "task_config_v2":{"tasks":{"WATCH_VIDEO":{"target":15}}},
            "user_status":{"completed_at":"2026-11-02T00:00:00+00:00"}
        });
        let config = parse_public_config(id, &input).unwrap();
        let output = serde_json::to_string(&config).unwrap();
        assert_eq!(config.task_types, ["WATCH_VIDEO"]);
        assert!(!output.contains("completed_at"));
        assert!(!output.contains("user_status"));
        assert!(output.contains("unverified_remote_public_config"));
    }

    #[test]
    fn missing_id_or_expiry_fails_closed() {
        let id = "123456789012345678";
        for value in [
            json!({}),
            json!({"id":id}),
            json!({"id":"99","expires_at":"date"}),
        ] {
            assert_eq!(
                parse_public_config(id, &value).unwrap_err(),
                LiveError::InvalidResponse
            );
        }
    }
}
