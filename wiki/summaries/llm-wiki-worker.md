# LLM-Wiki-Worker

- source: `raw/LLM-Wiki-Worker.md`
- source link: https://github.com/walle45611/LLM-Wiki-Worker
- original title: LLM-Wiki-Worker
- author: [[Walle]]
- published: (not specified)
- type: project overview / repository summary

## Summary

這份來源介紹 LLM-Wiki-Worker 的整體設計：把日常看到的文章、影片與筆記先收進唯讀的 `raw/`，再由 Cloudflare Worker 上的 query agent 依規則讀寫 `wiki/`，把零散素材整理成可查詢、可回顧、可持續維護的知識庫。系統同時整合 LINE webhook、Cloudflare Queue、GitHub 私有 repo 與 Obsidian 本地編輯流程，目標是把「我好像看過這件事」轉成能被 AI 檢索與重組的活資料。

## Key Claims

1. LLM-Wiki-Worker 的核心不是單次摘要，而是建立一條可反覆運作的知識整理管線，讓原始素材持續被編譯進 wiki。
2. `raw/` 唯讀、`wiki/` 可維護的分層，是避免原始來源與 AI 整理結果互相污染的關鍵設計。
3. agent 的穩定性來自規則檔與工具邊界，而不只是模型本身；query agent 先讀 `AGENTS.md` 與 router rules，再決定要讀哪些任務規則與檔案。
4. LINE webhook、Queue consumer 與排程共用同一條背景處理路徑，可避開 request 壽命限制，並統一查詢與每日整理的執行方式。
5. GitHub 私有 repo 與 Obsidian 雙軌同步，讓知識庫既能被雲端 Worker 存取，也保留本地 Markdown 維護體驗。

## Important Details

- 收集階段以 Obsidian Web Clipper 或本地整理把素材寫成 Markdown，作為 `raw/` 與 `wiki/` 的輸入來源。
- Worker 端的工具被刻意切小：`get_file`、`get_file_tree`、`upsert_file`、`append_file`、`replace_in_file` 各自負責單一能力，且寫入限制在 `wiki/`。
- `POST /webhook` 進來的 LINE 事件不直接同步做完整 AI 任務，而是先 enqueue 到 `LLM_WIKI_QUEUE`，再由 queue consumer 執行 `buildLineQueryReply()`。
- 每日摘要同樣走 queue job（`scheduled_summary`），表示即時查詢與排程整理共用同一套知識操作邏輯。
- 規則系統由 `templates/AGENTS.md`、`templates/wiki/rules/router-rules.md` 與多個 task rules 組成，行為控制是顯式路由，不是單一大 prompt。
- 回 LINE 前會做 Markdown 清理與長度截斷，確保輸出適合通訊軟體閱讀。
- 目前必要 secrets 包含 LINE 與 GitHub 憑證，必要 vars 則包含 GitHub repo 位置、時區、排程推播對象與模型設定。

## Practical Takeaways From This Source

- 若要做可持續更新的 AI 知識庫，先把「來源層」與「整理層」切開，再讓 agent 只對整理層有寫入權。
- 對通訊軟體或 webhook 入口，不要把長任務直接塞在同步 request；用 queue 把推理與寫檔搬到背景更穩定。
- 用 router rules 明確規定「第一步讀什麼、不同任務再讀哪些 rules」，能降低 agent 直接跳步亂寫的風險。
- 若知識庫需要同時支援人類編輯與 AI 維護，GitHub 遠端儲存加本地 Markdown 編輯是實用折衷。
- 最終回覆格式應與知識庫寫檔格式分離，避免把內部 Markdown 結構直接暴露到聊天介面。

## Related Concepts

- [harness-engineering](../concepts/harness-engineering.md)
- [context-engineering](../concepts/context-engineering.md)
- [agentic-knowledge-base-maintenance](../concepts/agentic-knowledge-base-maintenance.md)

## Alignment With Current Wiki

- 這份來源延續既有 `harness-engineering` 與 `context-engineering` 主題，但把焦點落在「知識庫維護型 agent」的具體落地方式，而不是泛談 agent 架構。
- 相較於既有 OpenClaw 與 harness 來源，本頁更強調來源分層、規則路由、queue 背景執行與通訊軟體輸出約束，適合抽成獨立概念持續累積。
