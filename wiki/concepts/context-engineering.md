# Context Engineering

## Current View

在目前知識庫中，context engineering 指的是：為了讓模型在有限 context window 內持續完成多步任務，對資訊載入、壓縮、分工與檢索進行系統化設計。它是 AI agent 能長時間穩定運作的核心工程層。

## Stable Conclusions

1. context window 是硬限制；不管理上下文，長任務遲早退化或失敗。
2. 有效策略不是「把所有資訊塞進 prompt」，而是按需載入、摘要回寫與結構化取用。
3. 子代理分工（subagent）可把中間繁瑣互動封裝成摘要，減少主代理上下文負擔。
4. 記憶系統可用檢索式路徑（如 chunk + 相似度排序）補足長期歷史，但召回品質不保證完美。
5. 壓縮（compact/pruning）能延長續航，但也可能遺失關鍵約束；關鍵規則需放在穩定注入區。
6. 在 RAG 或多文件 prompt 中，relevant information 的位置本身就是變數；長 context 常出現前後高、中段低的利用率落差，不能假設模型會平均讀懂所有片段。
7. context window 與 KV cache 會共同消耗部署記憶體；把 window 開得更大不保證模型更有效，也可能壓縮權重、併發與系統保留空間。
8. 把固定工作交給 skills、結構化工具或可重複程式，可以減少模型自由生成與上下文負擔，但工具輸出、權限與錯誤結果仍要納入 context 設計。

## Working Heuristics

- 固定規則放 system prompt/長期記憶，過程資料用檢索回填。
- 長流程優先拆任務，讓主代理只接收子任務摘要與結論。
- 工具輸出預設做截斷、摘要或分段讀取，避免原文大量灌入。
- 針對會等待的流程使用排程（cron/heartbeat）而非同步阻塞等待。
- 每次壓縮後檢查是否保留安全與授權條件，避免規則在摘要中消失。
- 若任務目標是維護知識庫，應優先讓 agent 讀取已整理的 wiki 層，避免每次查詢都回頭重掃原始素材。
- 若依賴檢索拼接 context，先做 ranking、re-ranking 或 cutoff，再決定是否增加 top-k，避免把關鍵片段埋進中段噪音。
- 以目標請求長度和併發量測試 context/KV cache 的記憶體預算，不要只依照模型標稱的最大 window 配置。

## Open Questions

- 何種壓縮策略在不同任務型態下最能兼顧保真與 token 成本，仍缺統一答案。
- RAG 記憶檢索在跨天、跨任務場景的穩定召回如何提升，仍是主要瓶頸。
- 子代理分工的最佳粒度與回傳格式，仍依模型能力與任務而變動。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [agentic-knowledge-base-maintenance](./agentic-knowledge-base-maintenance.md)
- [long-context-position-effects](./long-context-position-effects.md)
- [local-llm-deployment](./local-llm-deployment.md)

## Sources

- [openclaw-ai-agent-operating-principles](../summaries/openclaw-ai-agent-operating-principles.md)
- [harness-engineering-language-models-need-human-guidance](../summaries/harness-engineering-language-models-need-human-guidance.md)
- [llm-wiki-worker](../summaries/llm-wiki-worker.md)
- [andrej-karpathy-from-vibe-coding-to-agentic-engineering](../summaries/andrej-karpathy-from-vibe-coding-to-agentic-engineering.md)
- [lost-in-the-middle-how-language-models-use-long-context-explained](../summaries/lost-in-the-middle-how-language-models-use-long-context-explained.md)
- [qwen-3-8-27b-dgx-spark-agent-harness](../summaries/qwen-3-8-27b-dgx-spark-agent-harness.md)
