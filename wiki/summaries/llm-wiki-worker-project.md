# LLM-Wiki-Worker 開源專案

- source: `raw/LLM-Wiki-Worker 開源專案.md`
- source link: https://github.com/walle45611/LLM-Wiki-Worker/blob/main/README.md
- original title: walle45611-LLM-Wiki-Worker
- author: [[Walle]]
- published: 
- created: 2026-04-23
- type: project documentation

## 專案核心目標

LLM-Wiki-Worker 旨在解決「資訊收集後難以回溯與利用」的問題。它不只是單純的資料保存工具，而是透過 AI Agent 將零散的網頁、影片等原始素材，轉化為可查詢、可串接、可持續更新的「活資料」知識庫。

## 核心架構與流程

1. **收集 (Input)**：透過 Obsidian Web Clipper 將內容整理成 Markdown 存入 `raw/`。
2. **AI 提煉 (Processing)**：利用 Query Agent 讀取規則與工具，將原始資料編譯成結構化的 Wiki 內容。
3. **雙軌同步 (Sync)**：採用「本地編輯 + GitHub 同步」模式，確保本地 Obsidian 與雲端 AI 知識庫的一致性。
4. **Serverless 執行 (Cloudflare)**：利用 Cloudflare Worker 與 Queue 機制，在背景處理 AI 推理與資料更新，避免 Request 生命週期限制。
5. **分類隔離 (Output)**：嚴格區分 `raw/` (唯讀) 與 `wiki/` (可維護)，並透過 Telegram 進行互動查詢。

## 關鍵技術組件

- **Rule System**：基於 `AGENTS.md` 與 `router-rules.md` 的規則路由系統，確保 Agent 依任務類型（摘要、查詢、整理等）執行正確行為。
- **AI Tools**：提供 `get_file`、`upsert_file`、`append_file` 等精準的檔案操作工具，並限制寫入範圍於 `wiki/`。
- **Telegram Safety**：具備三層保護機制（Payload 驗證、Markdown 違規檢查、長度截斷），確保輸出訊息符合通訊平台格式。

## 專案目錄結構

- `src/`：核心邏輯（Webhook, Queue, AI 流程, Telegram/GitHub Client）。
- `templates/`：知識庫與規則的模板來源。
- `test/`：流程與工具行為測試。
- `wrangler.jsonc`：Cloudflare Worker 配置。

## 相關概念連結

- [agentic-knowledge-base-maintenance](../concepts/agentic-knowledge-base-maintenance.md)
- [llm-wiki-worker](../summaries/llm-wiki-worker.md)
