use go_quest_port::{
    gateway_trace::summarize_gateway_ndjson,
    mock_http::{MockError, MockHttpClient, MockPolicy, MockResponse},
};

fn response(status: u16, body: &str, latency_ms: u64) -> MockResponse {
    MockResponse {
        status,
        body: body.as_bytes().to_vec(),
        latency_ms,
    }
}

#[test]
fn synthetic_http_fixture_drives_real_offline_parser_without_network() {
    let fixture = include_str!("../examples/sample-quests.json");
    let mut mock = MockHttpClient::new(
        vec![response(200, fixture, 1)],
        MockPolicy::default(),
    ).unwrap();
    let parsed = mock.fetch_quest_document().unwrap();
    assert!(!parsed.quests.is_empty());
    assert_eq!(mock.attempts(), 1);
}

#[test]
fn status_errors_fail_closed_without_reading_server_body() {
    for (status, expected) in [
        (401, MockError::Unauthorized),
        (403, MockError::Forbidden),
        (302, MockError::RedirectRefused),
        (404, MockError::HttpStatus(404)),
        (429, MockError::RateLimited),
        (503, MockError::HttpStatus(503)),
    ] {
        let mut mock = MockHttpClient::new(
            vec![response(status, "TOKEN_LIKE_SECRET_SHOULD_NOT_BE_REPORTED", 1)],
            MockPolicy::default(),
        ).unwrap();
        assert_eq!(mock.fetch_quest_document().unwrap_err(), expected);
        assert_eq!(mock.attempts(), 1);
    }
}

#[test]
fn bounded_retries_and_virtual_timeout_are_deterministic() {
    let policy = MockPolicy {
        max_attempts: 3,
        deadline_ms: 100,
        ..MockPolicy::default()
    };
    let mut mock = MockHttpClient::new(vec![
        response(429, "{}", 10),
        response(503, "{}", 20),
        response(200, include_str!("../examples/sample-quests.json"), 30),
    ], policy).unwrap();
    assert!(!mock.fetch_quest_document().unwrap().quests.is_empty());
    assert_eq!(mock.attempts(), 3);

    let mut timeout = MockHttpClient::new(
        vec![response(429, "{}", 50), response(200, "{}", 60)],
        policy,
    ).unwrap();
    assert_eq!(timeout.fetch_quest_document().unwrap_err(), MockError::TimedOut);
    assert_eq!(timeout.attempts(), 2);
}

#[test]
fn exhausted_and_invalid_mock_policies_do_not_spin() {
    let policy = MockPolicy { max_attempts: 4, ..MockPolicy::default() };
    let mut exhausted = MockHttpClient::new(vec![response(503, "{}", 0)], policy).unwrap();
    assert_eq!(exhausted.fetch_quest_document().unwrap_err(), MockError::NoResponse);
    assert_eq!(exhausted.attempts(), 1);
    assert!(matches!(
        MockHttpClient::new(vec![], MockPolicy { max_attempts: usize::MAX, ..policy }),
        Err(MockError::InvalidPolicy)
    ));
    assert!(matches!(
        MockHttpClient::new(vec![response(200, "{}", 0); 9], policy),
        Err(MockError::TooManyFixtures)
    ));
}

#[test]
fn invalid_utf8_and_oversized_body_are_not_exposed() {
    let mut bad = MockHttpClient::new(vec![MockResponse {
        status: 200,
        body: vec![0xff, 0xfe],
        latency_ms: 0,
    }], MockPolicy::default()).unwrap();
    assert_eq!(bad.fetch_quest_document().unwrap_err(), MockError::InvalidDocument);
    let mut large = MockHttpClient::new(
        vec![response(200, "abcdef", 0)],
        MockPolicy { max_body_bytes: 3, ..MockPolicy::default() },
    ).unwrap();
    assert_eq!(large.fetch_quest_document().unwrap_err(), MockError::ResponseTooLarge);
}

#[test]
fn gateway_trace_summary_does_not_retain_user_payload_or_assert_login() {
    let raw = br#"{"op":10,"d":{"heartbeat_interval":45000}}
{"op":0,"s":1,"t":"READY","d":{"token":"PRIVATE_SECRET","user":{"id":"123"}}}
{"op":1,"d":1}
{"op":11,"d":null}
{"op":7,"d":null}
{"op":0,"s":1,"t":"QUESTS_UPDATE","d":{}}
{"op":9,"d":false}
"#;
    let trace = summarize_gateway_ndjson(raw).unwrap();
    assert_eq!(trace.frames, 7);
    assert_eq!(trace.ready_events_in_untrusted_trace, 1);
    assert_eq!(trace.heartbeat_interval_ms, Some(45000));
    assert_eq!(trace.heartbeat_acks_without_outbound_proof, 1);
    assert_eq!(trace.reconnects, 1);
    assert_eq!(trace.invalid_sessions, 1);
    let serialized = serde_json::to_string(&trace).unwrap();
    assert!(!serialized.contains("PRIVATE_SECRET"));
    assert!(!serialized.contains("\"123\""));
    assert!(serialized.contains("unverified_local_gateway_trace"));
}

#[test]
fn gateway_trace_rejects_invalid_frames_and_sequence_regression_without_echo() {
    for input in [
        r#"{"op":10,"d":{"heartbeat_interval":"bad"}}"#,
        r#"{"op":0,"s":0,"t":"READY"}\n{"op":0,"s":0,"t":"READY"}"#,
        r#"{"op":"IDENTIFY","d":{"token":"PRIVATE_SECRET"}}"#,
        r#"{"op":0,"s":5}"#,
    ] {
        let err = summarize_gateway_ndjson(input.replace(r"\n", "\n").as_bytes()).unwrap_err();
        assert!(!err.contains("PRIVATE_SECRET"));
    }
    assert!(summarize_gateway_ndjson(b"  \n \n").unwrap().frames == 0);
}

#[test]
fn gateway_trace_has_explicit_frame_and_byte_caps() {
    let over = vec![b'a'; go_quest_port::MAX_JSON_BYTES + 1];
    assert!(summarize_gateway_ndjson(&over).is_err());
    let many = r#"{"op":11}"#.to_owned() + "\n";
    let oversized_frames = many.repeat(10_001);
    assert!(summarize_gateway_ndjson(oversized_frames.as_bytes()).is_err());
}
