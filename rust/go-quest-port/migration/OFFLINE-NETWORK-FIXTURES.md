# Credential-free HTTP fixtures and offline Gateway trace analysis

**Tasks:** `NET-001`, `NET-002`, `NET-003`; partial preparation for `NET-009`.
**Scope:** Unit/integration-test library modules only; the shipping CLI remains
`demo`/`inspect`. No network sockets, HTTP client, credential, Gateway
Identify, Quest heartbeat, reward activity, or reward claim is sent.

## In-memory HTTP mock

[`src/mock_http.rs`](../src/mock_http.rs) exports `MockHttpClient`,
`MockResponse`, `MockPolicy`, and `MockError`. A response body with status
200 is fed into the **production** `parse_document` function. This proves the
fixture-to-parser flow; it does not prove any live request.

The mock consumes a finite queue (at most 8 synthetic responses), allows 1-4
attempts, uses virtual elapsed milliseconds with a deadline no greater than
30,000 ms, and caps the body to at most the existing 8 MiB JSON limit.
Requests are *not* sent and time is *not* slept. 401/403 fail immediately,
redirects fail closed (no forwarding), 429 and 5xx can be retried within the
fixed budget, and absent responses, invalid documents, deadlines and large
bodies return typed errors without including response content. No credentials,
headers, URLs, logging, environment variables or token handling are present.

[`tests/offline_network_fixtures.rs`](../tests/offline_network_fixtures.rs)
covers 200, 401, 403, 302, 404, 429, 503, bounded retry exhaustion, virtual
timeout, invalid policy and fixture limits, UTF-8, body cap and input parser.

## Offline Gateway NDJSON trace summarizer

[`src/gateway_trace.rs`](../src/gateway_trace.rs) exports
`summarize_gateway_ndjson(&[u8])`, returning a **data-only summary** with
`input_trust: "unverified_local_gateway_trace"`. Input is newline-delimited
UTF-8 JSON frames. Use only invented/sanitized frames; never provide private
tokens or other people's account traces. Limit: 8 MiB and 10,000 frames.

The parser counts the following *observed local frame types*, following
[Discord's official Gateway documentation](https://docs.discord.com/developers/events/gateway):

| Opcode | Offline interpretation |
|---|---|
| 0 (Dispatch) | Require non-decreasing session logic with **strictly increasing** sequence; count READY labels as untrusted observations |
| 1 (Heartbeat request) | Count requests only; **does not send** heartbeats |
| 7 (Reconnect) | Count, clear cached sequence |
| 9 (Invalid Session) | Count, clear cached sequence |
| 10 (Hello) | Validate positive heartbeat interval (up to an offline safety cap) |
| 11 (Heartbeat ACK) | Count, explicitly **not** matched to a real outbound heartbeat |
| Other | Count as unspecified frame without processing data |

This is an **inbound/saved-trace analyzer**, not a protocol client. A READY
label, sequence, or ACK copied into local JSON is never proof of authentication
or of Quest progress. All data fields (`d`), user IDs, tokens and arbitrary event
bodies are discarded before the returned summary; diagnostics report only frame
indices. Tests cover control events, secrets not present in output, invalid
JSON/opcodes, sequence regression, size and frame caps.

## Conditional authorized network work

[Go HTTP permission audit](GO-HTTP-AUTH-AUDIT-2026-11-01.md) documents why
the original Go raw personal-user `Authorization` mode remains unsupported.
The in-memory fixture adapters **do not authorize** `NET-006`, `NET-007`,
`NET-008`, `NET-009` or `QA-011` to claim live Quest support. True transport
timeouts, TLS, rate-limit headers and credential protection remain untested
because there is no approved live Quest client.
