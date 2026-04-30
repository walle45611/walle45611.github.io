# Context Engineering

## Current View

在目前知識庫中，context engineering 指的是：為了讓模型在有限 context window 內持續完成多步任務，對資訊載入、壓縮、分工與檢索進行系統化設計。它是 AI agent 能長時間穩定運作的核心工程層。

## Stable Conclusions

1. context window 是硬限制；不管理上下文，長任務遲早退化或失敗。
2. 有效策略不是「把所有資訊塞進 prompt」，而是按需載入、摘要回寫與結構化取用。
3. 子代理分工（subagent）可把中間繁瑣互動封裝成摘要，減少主代理上下文負擔。
4. 記憶系統可用檢索式路徑（如 chunk + 相似度排序）補足長期歷史，但召回品質不保證完美。
5. 壓縮（compact/pruning）能延長續航，但也可能遺失關鍵約束；關鍵規則需放在穩定注入區。

## Working Heuristics

- 固定規則放 system prompt/長期記憶，過程資料用檢索回填。
- 長流程優先拆任務，讓主代理只接收子任務摘要與結論。
- 工具輸出預設做截斷、摘要或分段讀取，避免原文大量灌入。
- 針對會等待的流程使用排程（cron/heartbeat）而非同步阻塞等待。
- 每次壓縮後檢查是否保留安全與授權條件，避免規則在摘要中消失。
- 若任務目標是維護知識庫，應優先讓 agent 讀取已整理的 wiki 層，避免每次查詢都回頭重掃原始素材。

## Open Questions

- 何種壓縮策略在不同任務型態下最能兼顧保真與 token 成本，仍缺統一答案。
- RAG 記憶檢索在跨天、跨任務場景的穩定召回如何提升，仍是主要瓶頸。
- 子代理分工的最佳粒度與回傳格式，仍依模型能力與任務而變動。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [agentic-knowledge-base-maintenance](./agentic-knowledge-base-maintenance.md)

## Sources

- [openclaw-ai-agent-operating-principles](../summaries/openclaw-ai-agent-operating-principles.md)
- [harness-engineering-language-models-need-human-guidance](../summaries/harness-engineering-language-models-need-human-guidance.md)
- [llm-wiki-worker](../summaries/llm-wiki-worker.md)
- [andrej-karpathy-from-vibe-coding-to-agentic-engineering](../summaries/andrej-karpathy-from-vibe-coding-to-agentic-engineering.md)
