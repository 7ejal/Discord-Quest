//! Deterministic in-memory HTTP fixtures. Never opens a socket or accepts credentials.
use crate::{parse_document, ParsedDocument, MAX_JSON_BYTES};
use std::collections::VecDeque;

pub const MAX_MOCK_RESPONSES: usize = 8;
pub const MAX_MOCK_ATTEMPTS: usize = 4;
pub const MAX_MOCK_TIMEOUT_MS: u64 = 30_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockError {
    InvalidPolicy,
    TooManyFixtures,
    NoResponse,
    TimedOut,
    ResponseTooLarge,
    RedirectRefused,
    Unauthorized,
    Forbidden,
    RateLimited,
    HttpStatus(u16),
    InvalidDocument,
}

#[derive(Debug, Clone)]
pub struct MockResponse {
    pub status: u16,
    pub body: Vec<u8>,
    /// Virtual elapsed time: no sleeping or network activity.
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MockPolicy {
    /// Total number of attempts, including the initial request.
    pub max_attempts: usize,
    pub deadline_ms: u64,
    pub max_body_bytes: usize,
}

impl Default for MockPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 1,
            deadline_ms: 5_000,
            max_body_bytes: MAX_JSON_BYTES,
        }
    }
}

pub struct MockHttpClient {
    responses: VecDeque<MockResponse>,
    policy: MockPolicy,
    attempts: usize,
}

impl MockHttpClient {
    pub fn new(responses: Vec<MockResponse>, policy: MockPolicy) -> Result<Self, MockError> {
        if responses.len() > MAX_MOCK_RESPONSES {
            return Err(MockError::TooManyFixtures);
        }
        if policy.max_attempts == 0
            || policy.max_attempts > MAX_MOCK_ATTEMPTS
            || policy.deadline_ms == 0
            || policy.deadline_ms > MAX_MOCK_TIMEOUT_MS
            || policy.max_body_bytes == 0
            || policy.max_body_bytes > MAX_JSON_BYTES
        {
            return Err(MockError::InvalidPolicy);
        }
        Ok(Self {
            responses: responses.into(),
            policy,
            attempts: 0,
        })
    }

    pub fn attempts(&self) -> usize {
        self.attempts
    }

    /// Decode a synthetic successful HTTP response using the production offline parser.
    /// No Authorization headers, target URLs, or real HTTP client are involved.
    pub fn fetch_quest_document(&mut self) -> Result<ParsedDocument, MockError> {
        let mut elapsed = 0_u64;
        for attempt in 0..self.policy.max_attempts {
            let response = self.responses.pop_front().ok_or(MockError::NoResponse)?;
            self.attempts += 1;
            elapsed = elapsed
                .checked_add(response.latency_ms)
                .ok_or(MockError::TimedOut)?;
            if elapsed > self.policy.deadline_ms {
                return Err(MockError::TimedOut);
            }
            if response.body.len() > self.policy.max_body_bytes {
                return Err(MockError::ResponseTooLarge);
            }
            match response.status {
                200 => {
                    let text = std::str::from_utf8(&response.body)
                        .map_err(|_| MockError::InvalidDocument)?;
                    return parse_document(text).map_err(|_| MockError::InvalidDocument);
                }
                401 => return Err(MockError::Unauthorized),
                403 => return Err(MockError::Forbidden),
                300..=399 => return Err(MockError::RedirectRefused),
                429 => {
                    if attempt + 1 == self.policy.max_attempts {
                        return Err(MockError::RateLimited);
                    }
                }
                500..=599 => {
                    if attempt + 1 == self.policy.max_attempts {
                        return Err(MockError::HttpStatus(response.status));
                    }
                }
                status => return Err(MockError::HttpStatus(status)),
            }
        }
        unreachable!("a valid policy has at least one attempt")
    }
}
