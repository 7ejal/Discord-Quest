//! Ensure the repository's supported task taxonomy remains in sync with Rust.
//! Offline only: no account, network, or Quest API access.
use go_quest_port::manager::task_family;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf};

#[test]
fn all_research_catalog_task_ids_are_classified_consistently() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let content = fs::read_to_string(root.join("data/task-types.json"))
        .expect("repository task catalog must be present in this source checkout");
    let document: Value = serde_json::from_str(&content).unwrap();
    let tasks = document["tasks"].as_array().expect("task catalog array");

    let expected = BTreeMap::from([
        ("WATCH_VIDEO", ("video", false)),
        ("WATCH_VIDEO_ON_MOBILE", ("video", false)),
        ("PLAY_ON_DESKTOP", ("desktop_game", false)),
        ("PLAY_ON_DESKTOP_V2", ("historical_deprecated", true)),
        ("STREAM_ON_DESKTOP", ("desktop_stream", false)),
        ("PLAY_ACTIVITY", ("embedded_activity", false)),
        ("ACHIEVEMENT_IN_ACTIVITY", ("publisher_or_activity_achievement", false)),
        ("ACHIEVEMENT_IN_GAME", ("publisher_or_activity_achievement", false)),
        ("PLAY_ON_XBOX", ("linked_console", false)),
        ("PLAY_ON_PLAYSTATION", ("linked_console", false)),
    ]);

    assert_eq!(tasks.len(), expected.len(), "review new or removed task identifiers");
    let mut observed = BTreeMap::new();
    for task in tasks {
        let id = task["id"].as_str().expect("task id");
        let deprecated = task["deprecated"].as_bool().expect("deprecated flag");
        assert!(observed.insert(id, deprecated).is_none(), "duplicate catalog entry {id}");
        let (family, expected_deprecated) = expected
            .get(id)
            .unwrap_or_else(|| panic!("unrecognized catalog identifier {id}"));
        assert_eq!(task_family(id), *family, "Rust classifier drift for {id}");
        assert_eq!(deprecated, *expected_deprecated, "catalog deprecation drift for {id}");
    }
    assert_eq!(observed.len(), expected.len());
    assert_eq!(task_family("FUTURE_UNSUPPORTED_TASK"), "unknown");
}
