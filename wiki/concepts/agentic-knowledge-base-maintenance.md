# Agentic Knowledge Base Maintenance

## Current View

在目前知識庫中，agentic knowledge base maintenance 指的是：把原始素材、整理後的 wiki、規則系統與背景執行管線拆成明確分層，讓 LLM/agent 能持續把零散內容編譯成可查詢、可回顧、可更新的知識庫，而不直接污染原始來源。

## Stable Conclusions

1. 若知識庫要同時支援人類維護與 AI 整理，最重要的基線是把唯讀來源層與可寫工作層分開。
2. 對知識庫型 agent 來說，規則路由比單一 prompt 更重要；先判斷任務類型、再按需讀規則，可降低跳步與誤寫。
3. 將查詢、摘要、每日整理等任務統一進入同一條背景處理管線，有助於避免 webhook/request 壽命限制，也能集中治理寫檔邏輯。
4. 工具邊界要小而明確，尤其寫入工具應限制在 wiki 工作層，避免 agent 直接改動來源素材。
5. 知識庫輸出與對外聊天輸出應分離：前者偏向可維護 Markdown，後者則應針對通訊介面做格式清理與長度控制。

## Working Heuristics

- 原始文章、影片轉錄、剪藏內容先進 `raw/`，整理與概念整合再回寫到 `wiki/`。
- 讓 agent 每次任務都先讀 router，再依任務決定要讀 ingest/query/review/daily 等規則。
- 把長任務做成 queue 或排程背景處理，不依賴單次 request 完成整條知識整理流程。
- 寫檔工具分成整檔更新、附加紀錄與精準替換，避免一個模糊工具同時承擔所有副作用。
- 若最終要送到 LINE、Telegram 或其他聊天介面，加入 Markdown 清理與字數裁切層。

## Open Questions

- 規則檔數量與細粒度拆分到什麼程度，最能兼顧可維護性與 agent 執行效率，仍需更多實證。
- 當知識庫規模持續擴張時，如何在不犧牲來源可追溯性的前提下控制索引與讀檔成本，仍是後續設計重點。
- 同一套維護模式在不同訊息入口（LINE、Telegram、Web UI）下，輸出清理與互動節奏要如何共用，仍有調整空間。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [context-engineering](./context-engineering.md)

## Sources

- [llm-wiki-worker](../summaries/llm-wiki-worker.md)
