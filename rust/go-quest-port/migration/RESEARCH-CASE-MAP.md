# Research fixture disposition for the offline CLI

The F01–F22 source matrix is a research plan, not 22 passing engagement tests.
This ledger follows TEST-MATRIX.md's rule: executable offline test or justified
N/A. No fake transport, OAuth workflow or retry engine was added to inflate
coverage. N/A means the component does not exist in this CLI.

| Case | Executable evidence or N/A | Expected result |
|---|---|---|
| F01 | offline_regressions::group_operator_and_missing_progress_table | AND needs both tasks |
| F02 | same table | OR satisfaction never implies Quest completion |
| F03 | src/lib.rs::number_at_target_is_not_server_confirmed | Numeric progress alone is unconfirmed |
| F04 | offline_regressions::all_ten_identifiers_are_classification_only | Video value at target is not completion |
| F05 | N/A: no HTTP response consumer; empty JSON is rejected by src/lib.rs::invalid_full_quest_objects_are_rejected | No HTTP 200 success inference |
| F06 | N/A: no transport/retry recursion; network-free execution checked by scripts/test-linux.sh | No retry engine exists |
| F07 | N/A: no polling; model_parity::input_heartbeat_and_freshness_are_preserved_not_fabricated classifies an expired observation | No indefinite heartbeat loop |
| F08 | offline_contract::application_identity_is_kept_per_quest | Independent campaign/app identity |
| F09 | model_parity::blank_task_application_ids_fall_back_to_nonblank_campaign_id; application_id_source in golden report | Explicit campaign fallback provenance |
| F10 | offline_regressions::v2_precedence_never_merges_or_falls_back_from_empty_map | V2/legacy precedence |
| F11 | offline_regressions::unknown_task_in_or_group_never_proves_group_support | Unsupported task/operator fails closed |
| F12 | manager::worker_tests::worker_error_returns_no_partial_success | Errors produce no partial success |
| F13 | manager::worker_tests::worker_panic_returns_no_partial_success_and_joins_other_workers | Panics propagate after joins; no EventEmitter dependency |
| F14 | N/A: no final request or local engagement timer; numeric false-success covered by F03 | No fabricated request success |
| F15 | offline_contract::milestones_have_local_provenance_and_independent_redemption | Reported completed/unclaimed only |
| F16 | same test | External redemption remains unknown |
| F17 | N/A: no OAuth grant, token or privilege lifecycle | Nothing to authorize or revoke |
| F18 | offline_contract::malformed_json_never_echoes_input_in_errors | Sanitized errors |
| F19 | manager::worker_tests::cancellation_is_observed_before_scheduling and worker_error_returns_no_partial_success | Cooperative library cancellation between evaluations; no stalled network call |
| F20 | offline_regressions::all_ten_identifiers_are_classification_only | Console recognition is not authentic activity proof |
| F21 | same test and unverified_local_json provenance | Stream recognition is not viewer evidence |
| F22 | offline_contract::application_identity_is_kept_per_quest; model_parity::blank_application_ids_are_skipped_and_reported | Same task label across Quests retains distinct application IDs |

Six cases (F05/F06/F07/F14/F17 and the stalled-network portion of F19) have
inapplicable network behavior; F05/F07/F19 also have related executable parser,
freshness or cancellation assertions. The table makes no live engagement claim.
