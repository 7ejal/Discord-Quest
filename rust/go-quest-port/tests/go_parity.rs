//! Executable comparison against pinned Go pure helpers; no upstream client.
use go_quest_port::{manager::QuestManager, parse_document};
use serde_json::{json, Value};
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn go(input: &Value) -> Value {
    let binary = std::env::var_os("GO_QUEST_REFERENCE_BIN")
        .expect("Run tools/verify-quest-linux.sh on main-server");
    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn q(id: &str) -> Value {
    json!({"id":id,"config":{"expires_at":"2100-01-01T00:00:00Z",
        "task_config_v2":{"tasks":{"WATCH_VIDEO":{"target":60}}}}})
}
#[test]
fn pinned_go_pure_helpers_match_valid_synthetic_records() {
    let mut completed = q("completed");
    completed["user_status"] =
        json!({"enrolled_at":"2020-01-01T00:00:00Z","completed_at":"2020-01-02T00:00:00Z"});
    let mut claimed = completed.clone();
    claimed["id"] = json!("claimed");
    claimed["user_status"]["claimed_at"] = json!("2020-01-03T00:00:00Z");
    let mut expired = q("expired");
    expired["config"]["expires_at"] = json!("2000-01-01T00:00:00Z");
    let mut mobile = q("mobile");
    mobile["config"]["task_config_v2"]["tasks"] = json!({"WATCH_VIDEO_ON_MOBILE":{"target":60}});
    let input = json!({"quests":[q("active"),completed,claimed,expired,mobile]});
    let reference = go(&input);
    let doc = parse_document(&input.to_string()).unwrap();
    let manager = QuestManager::new(doc.quests, None);
    for quest in manager.quests() {
        let original = reference["quests"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == quest.id)
            .unwrap();
        assert_eq!(original["completed"], quest.is_completed());
        assert_eq!(original["enrolled"], quest.is_enrolled());
        assert_eq!(original["claimed"], quest.has_claimed_rewards());
        assert_eq!(original["expired"], quest.is_expired(chrono::Utc::now()));
        assert_eq!(original["android"], quest.is_android_video_only());
        assert_eq!(
            original["first_task"],
            quest.original_go_first_task().unwrap_or("")
        );
        assert_eq!(
            original["valid"],
            manager
                .filter_valid(chrono::Utc::now())
                .iter()
                .any(|q| q.id == quest.id)
        );
        assert_eq!(
            original["redeemable"],
            manager.filter_redeemable().iter().any(|q| q.id == quest.id)
        );
    }
}
#[test]
fn intentional_go_rust_divergences_are_executable() {
    let mut malformed = q("invalid");
    malformed["user_status"] = json!({"completed_at":""});
    let input = json!({"quests":[malformed]});
    assert_eq!(go(&input)["quests"][0]["completed"], true);
    assert!(!parse_document(&input.to_string()).unwrap().quests[0].is_completed());
    let mut unknown_expiry = q("unknown");
    unknown_expiry["config"]["expires_at"] = json!("bad");
    let input = json!({"quests":[unknown_expiry]});
    assert_eq!(go(&input)["quests"][0]["valid"], true);
    assert!(
        QuestManager::new(parse_document(&input.to_string()).unwrap().quests, None)
            .filter_valid(chrono::Utc::now())
            .is_empty()
    );
    let input =
        json!({"quests":[q("visible")],"quest_enrollment_blocked_until":"2100-01-01T00:00:00Z"});
    assert_eq!(go(&input)["blocked"], true);
    assert_eq!(parse_document(&input.to_string()).unwrap().quests.len(), 1);
    let mut changed = q("duplicate");
    changed["preview"] = json!(true);
    let input = json!({"quests":[q("duplicate"),changed]});
    assert_eq!(go(&input)["quests"].as_array().unwrap().len(), 1);
    assert!(parse_document(&input.to_string()).is_err());
}
