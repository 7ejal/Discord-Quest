//! Opt-in real HTTPS reads using documented public Quest config and OAuth2 identity.
//! Never accepts personal-account session tokens, spoofs activity, or sends Quest progress.
use reqwest::{blocking::Client, redirect::Policy, StatusCode};
use serde::Serialize;
use serde_json::Value;
use std::{
    io::{self, IsTerminal, Read},
    time::Duration,
};

const API: &str = "https://discord.com/api/v10";
const MAX_BODY_BYTES: usize = 1024 * 1024;
const MAX_OAUTH_TOKEN_BYTES: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub enum LiveError {
    InvalidQuestId,
    InvalidCredentialInput,
    AuthenticationRequired,
    PermissionDenied,
    RateLimited,
    NotFound,
    RemoteUnavailable,
    UnexpectedStatus,
    OversizeResponse,
    InvalidResponse,
}

impl std::fmt::Display for LiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::InvalidQuestId => "Quest ID must contain 1-20 ASCII digits",
            Self::InvalidCredentialInput => "Expected one OAuth2 Bearer access token from non-terminal stdin",
            Self::AuthenticationRequired => "OAuth2 authorization failed or expired (HTTP 401)",
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
        .connect_timeout(Duration::from_secs(4))
        .timeout(Duration::from_secs(10))
        .user_agent(concat!("go-quest-port/", env!("CARGO_PKG_VERSION"), " (read-only)"))
        .build()
        .map_err(|_| LiveError::RemoteUnavailable)
}

fn get_json(client: &Client, path: &str, oauth: Option<&str>) -> Result<Value, LiveError> {
    let mut request = client.get(format!("{API}{path}"));
    if let Some(token) = oauth {
        request = request.bearer_auth(token);
    }
    let response = request.send().map_err(|_| LiveError::RemoteUnavailable)?;
    match response.status() {
        StatusCode::OK => {}
        StatusCode::UNAUTHORIZED => return Err(LiveError::AuthenticationRequired),
        StatusCode::FORBIDDEN => return Err(LiveError::PermissionDenied),
        StatusCode::NOT_FOUND => return Err(LiveError::NotFound),
        StatusCode::TOO_MANY_REQUESTS => return Err(LiveError::RateLimited),
        status if status.is_redirection() => return Err(LiveError::UnexpectedStatus),
        _ => return Err(LiveError::UnexpectedStatus),
    }
    if response
        .content_length()
        .is_some_and(|n| n > MAX_BODY_BYTES as u64)
    {
        return Err(LiveError::OversizeResponse);
    }
    let mut bytes = Vec::new();
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
    if !object.get("id").and_then(Value::as_str).is_some_and(|v| v == id)
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
pub fn check_oauth_identity_from_stdin() -> Result<OAuthCheck, LiveError> {
    if io::stdin().is_terminal() {
        return Err(LiveError::InvalidCredentialInput);
    }
    let mut bytes = Vec::new();
    io::stdin()
        .take((MAX_OAUTH_TOKEN_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| LiveError::InvalidCredentialInput)?;
    if bytes.len() > MAX_OAUTH_TOKEN_BYTES {
        return Err(LiveError::InvalidCredentialInput);
    }
    // A pipe can end with one newline. Extra lines and whitespace are refused.
    if bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    let token = std::str::from_utf8(&bytes).map_err(|_| LiveError::InvalidCredentialInput)?;
    if token.is_empty()
        || token.len() > MAX_OAUTH_TOKEN_BYTES
        || token.bytes().any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
    {
        return Err(LiveError::InvalidCredentialInput);
    }
    let client = make_client()?;
    let json = get_json(&client, "/users/@me", Some(token))?;
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

#[cfg(test)]
mod tests {
    use super::{parse_public_config, valid_quest_id, LiveError};
    use serde_json::json;

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
        for value in [json!({}), json!({"id":id}), json!({"id":"99","expires_at":"date"})] {
            assert_eq!(parse_public_config(id, &value).unwrap_err(), LiveError::InvalidResponse);
        }
    }
}
