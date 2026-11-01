// Offline reference harness. Pure helpers copied from upstream MIT source
// 352d7f457c9511bc9e7c9b940a71012bfb47572e; never links the upstream client.
package main

import (
    "encoding/json"
    "fmt"
    "io"
    "os"
    "sort"
)

type QuestManager struct { quests map[string]*Quest }

// FilterValid returns quests that are not completed and not expired.
func (m *QuestManager) FilterValid() []*Quest {
	var result []*Quest
	for _, q := range m.quests {
		if !q.IsCompleted() && !q.IsExpired() {
			result = append(result, q)
		}
	}
	return result
}

// FilterRedeemable returns quests that are completed but not yet claimed.
func (m *QuestManager) FilterRedeemable() []*Quest {
	var result []*Quest
	for _, q := range m.quests {
		if q.IsCompleted() && !q.HasClaimedRewards() {
			result = append(result, q)
		}
	}
	return result
}


func main() {
    var response AllQuestsResponse
    if err := json.NewDecoder(io.LimitReader(os.Stdin, 8*1024*1024)).Decode(&response); err != nil {
        fmt.Fprintln(os.Stderr, "invalid synthetic input")
        os.Exit(1)
    }
    if response.QuestEnrollmentBlockedUntil != nil {
        fmt.Println("{\"blocked\":true,\"quests\":[]}")
        return
    }
    manager := QuestManager{quests: make(map[string]*Quest)}
    for i := range response.Quests { manager.quests[response.Quests[i].ID] = &response.Quests[i] }
    valid, redeemable := map[string]bool{}, map[string]bool{}
    for _, q := range manager.FilterValid() { valid[q.ID] = true }
    for _, q := range manager.FilterRedeemable() { redeemable[q.ID] = true }
    ids := make([]string,0,len(manager.quests))
    for id := range manager.quests { ids = append(ids,id) }
    sort.Strings(ids)
    results := make([]map[string]interface{},0,len(ids))
    for _,id := range ids {
        q := manager.quests[id]
        results = append(results,map[string]interface{}{"id":id,"expired":q.IsExpired(),
            "completed":q.IsCompleted(),"enrolled":q.IsEnrolled(),"claimed":q.HasClaimedRewards(),
            "android":q.IsAndroidQuest(),"first_task":q.GetTaskName(),
            "valid":valid[id],"redeemable":redeemable[id]})
    }
    if err := json.NewEncoder(os.Stdout).Encode(map[string]interface{}{"blocked":false,"quests":results}); err != nil { os.Exit(1) }
}

