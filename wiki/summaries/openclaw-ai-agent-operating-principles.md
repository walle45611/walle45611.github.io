# 解剖小龍蝦 — 以 OpenClaw 為例介紹 AI Agent 的運作原理

- source: `raw/解剖小龍蝦 — 以 OpenClaw 為例介紹 AI Agent 的運作原理.md`
- source link: [https://www.youtube.com/watch?v=2rcJdFuNbZQ&t=1s](https://www.youtube.com/watch?v=2rcJdFuNbZQ&t=1s)
- original title: 解剖小龍蝦 — 以 OpenClaw 為例介紹 AI Agent 的運作原理
- speaker: 李一駿助教（課程素材由 Hung-yi Lee 團隊發布）
- published: 2026-03-09
- type: YouTube transcript summary

## Summary

這份來源用 OpenClaw 拆解 AI agent 的運作：agent 本身不是模型，而是位於「人類指令」與「LLM 回應」之間的執行框架。它透過 system prompt、工具調用、記憶檔案、排程與上下文壓縮，讓只會文字接龍的模型看起來像可長期運作的助理；同時也暴露了高權限工具、prompt injection 與記憶遺失造成的安全風險。

## Key Claims

1. OpenClaw 是 AI agent 中「非 AI」的程式框架，主要負責組 prompt、調工具、轉發輸入輸出。
2. 模型是否「有人格、有記憶、有目標」多半來自 system prompt 與地端 `.md` 檔案注入，不是模型天生具備。
3. 工具調用是 agent 可執行任務的核心機制；尤其 `execute` 類高權限工具同時帶來能力與風險。
4. 長期運作的關鍵是 context engineering：按需載入 skill、子代理分工、記憶檢索、壓縮與裁剪上下文。
5. 安全控制不應只靠模型服從，還需 harness 層的硬規則（如執行前人工核准、工具白名單、隔離環境）。
6. 重要規則若只存在對話歷史，可能在 compact 後消失；需寫入長期記憶檔，才更穩定保留。

## Important Details

- 來源明確區分：LLM 只做 token 接龍；agent 框架提供身份設定、上下文拼接與工具執行閉環。
- system prompt 由多個本地檔案組成（如 `agent.md`、`soul.md`、`memory.md`、skills 說明），並包含工具手冊。
- `skill.md` 被定位為可重用 SOP（非程式本體），可按需讀取，避免把全文常駐在 prompt。
- subagent（spawn）用於把重任務切給子代理，讓父代理只保留摘要結果，節省父級 context window。
- heartbeat + cron job 讓 agent 從被動回應改為可定期自動執行任務，支援「等待後再檢查」的流程。
- context compaction / pruning 用摘要或裁剪降低上下文長度，但可能丟失關鍵約束。
- 案例指出：讓 agent 接觸外部內容（如留言、外部 skill）會引入注入與供應鏈風險。
- 實務隔離建議包括：專用機器、獨立帳號與最小化憑證暴露，不把 agent 裝在日常主機。

## Practical Takeaways From This Source

- 把高風險規則放在固定注入位置（如長期記憶/規則檔），不要只口頭交代在聊天裡。
- 高權限工具預設加人工核准與白名單，避免一次回傳即自動執行。
- 對外部 skill 與下載指令先做人類審閱，尤其涉及壓縮檔、執行檔與腳本安裝。
- 多步任務優先拆成子代理 + 摘要回傳，減少主代理上下文污染。
- 生產使用採隔離機與獨立身份，將 agent 失誤的爆炸半徑降到最低。

## Related Concepts

- [harness-engineering](../concepts/harness-engineering.md)
- [context-engineering](../concepts/context-engineering.md)

## Alignment With Current Wiki

- 與既有 `harness-engineering-language-models-need-human-guidance` 一致，且補上更工程化的執行細節（system prompt 組裝、工具閉環、compact 機制、heartbeat/cron）。
- 本來源強化了「harness 層硬防禦」與「隔離部署」的重要性，對既有概念頁屬於補強而非衝突。
