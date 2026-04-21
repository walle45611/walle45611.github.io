# Agent Design Patterns

## Current View

在目前知識庫中，agent design patterns 指的是：為了讓 AI agent 系統在不同任務條件下兼顧控制力、延遲、成本與可維護性，而採用的工作流拓撲設計。重點不是單一 prompt，而是代理如何分工、依序執行、並行執行，以及如何回收中間結果。

## Stable Conclusions

1. Single agent 適合簡單任務與少量工具，優勢是實作成本低、彈性高，但流程一旦變長，可靠性與可控性就容易下降。
2. Sequential agent 以固定步驟串接多個子代理，適合高結構、可重複的流程，能提升 predictability，但會犧牲適應性。
3. Parallel agent 適合彼此獨立的子任務，可用併發換取較低延遲，但常需要額外 aggregator 才能產出可用最終結果。
4. 多代理設計的核心不是「更多 agent」，而是明確定義資料流、共享狀態與彙整責任。
5. pattern 選擇本質上是 trade-off：flexibility、control、latency、cost 不可能同時最佳。

## Working Heuristics

- 先判斷任務是單步工具使用、固定流水線，還是可拆成互不依賴的子任務，再決定 pattern。
- 若流程順序固定且常重複，優先用 sequential agent，而不是把所有步驟塞回單一 prompt。
- 若子任務可獨立執行，優先考慮 parallel agent，但要同步設計 gather / synthesize 階段。
- 不論哪種 pattern，都要讓子代理的輸入輸出格式夠清楚，避免共享狀態變成雜亂暫存區。
- 可先從 single agent 起步，只有在可靠性、可控性或延遲真的成為瓶頸時再升級到多代理。

## Open Questions

- 在真實系統中，何時多代理帶來的協調成本會超過它改善的可靠性與延遲，仍需更多實務基準。
- 共享 session state 的最佳結構與保真方式，會隨框架與模型能力而變化。
- orchestrator、review loop、agent as tool 等進階 pattern 在何種條件下值得引入，還需要更多來源支撐。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [context-engineering](./context-engineering.md)

## Sources

- [ai-agent-design-patterns](../summaries/ai-agent-design-patterns.md)
