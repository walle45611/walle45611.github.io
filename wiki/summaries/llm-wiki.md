# LLM Wiki

- source: `raw/llm-wiki.md`
- source link: https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f
- original title: llm-wiki
- author: (not specified)
- published: (not specified)
- created: 2026-04-21
- type: idea note

## Summary

這份 idea note 提出以 LLM 持續編譯與維護個人 wiki，取代只在查詢時從 raw 文件重新拼接答案的做法。核心差異是：來源被讀取後，關鍵資訊、實體、概念、矛盾與交叉連結會累積到持久化的 Markdown wiki，使後續查詢直接使用已整理的知識，而不是每次從零開始重建。

## Key Claims

1. Query-time RAG 能從文件片段產生回答，但本身不會自然形成可累積、可交叉連結與可持續更新的知識資產。
2. 持久化 wiki 應由 raw sources、可維護的 wiki 與描述工作流程的 schema 三層組成；raw 是不可變來源，wiki 是 LLM 維護層。
3. Ingest 不只是索引來源，而是讀取、提煉、整合既有頁面、標註衝突並更新索引與 log 的維護流程。
4. Query 與 lint 也是知識庫操作：好的回答、比較或分析若值得保留，應可回寫成新頁面；lint 則檢查矛盾、過時主張、孤立頁面與缺少的交叉連結。
5. `index.md` 適合做內容導向導覽，`log.md` 則提供可解析的時間序列；兩者用途不同，不應互相取代。
6. Obsidian、Git、Web Clipper 與可選的本地搜尋工具可作為工作介面，但具體目錄、規則與工具邊界應依使用情境實作。

## Limitations and Boundaries

來源明確把自己定位為抽象模式，而非完整實作規格。它沒有決定特定 agent、資料庫、搜尋器或檔案格式，也不保證 LLM 維護出的內容正確；實作仍需要來源追溯、規則路由、權限邊界與人工驗證。

## Related Concepts

- [agentic-knowledge-base-maintenance](../concepts/agentic-knowledge-base-maintenance.md)
- [context-engineering](../concepts/context-engineering.md)

## Alignment With Current Wiki

這份來源是目前知識庫「raw / wiki 分層、規則路由、索引與 log」設計的概念性前身；既有 `llm-wiki-worker` 與 `llm-wiki-worker-project` 摘要則分別記錄規則邊界與具體專案實作，三者應保持來源層級區分。
