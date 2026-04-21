# AI agent design patterns

- source: `raw/AI agent design patterns.md`
- source link: [https://www.youtube.com/watch?v=GDm_uH6VxPY](https://www.youtube.com/watch?v=GDm_uH6VxPY)
- original title: AI agent design patterns
- author: Google Cloud Tech
- speaker: Annie Wang
- published: 2026-02-28
- type: YouTube transcript summary

## Summary

這份來源用 Google ADK 的旅行規劃 demo，介紹三種基礎 agent 設計模式：single agent、sequential agent、parallel agent。核心重點不是哪一種「最好」，而是如何依任務複雜度、控制需求與延遲成本，在單代理的簡潔性、多代理串接的可預測性，以及並行拆解的低延遲之間做取捨。

## Key Claims

1. Single agent 最簡單，適合工具數量少、流程不複雜的任務，但當多步邏輯變長時，行為可靠性會快速下降。
2. Sequential agent 把流程拆成固定順序的子代理，讓每一步輸出成為下一步輸入，可提高可控性與穩定性。
3. Parallel agent 讓多個獨立子任務同時執行，可明顯降低等待時間，但通常需要額外的聚合步驟來整合結果。
4. 多代理系統不只是多開幾個模型，而是要明確設計代理間的責任分工、資料傳遞與最終彙整方式。
5. 短期共享狀態（session state）是 sequential / parallel pattern 中常見的協作機制，用來讓子代理交換中間結果。

## Important Details

- Single agent 範例用單一代理搭配 Google Search tool 規劃旅行，示範模型自行決定查詢步驟的彈性。
- 來源指出，若把多步規則全部堆進單一 system instruction，會形成巨大 prompt，且因模型非決定性而難以穩定遵守。
- Sequential agent 以「找餐廳 -> 找交通方式」的流水線示範固定步驟工作流，重點在 predictable, reliable execution。
- 子代理之間透過 shared session state 溝通；前一步將結果寫入共享 scratchpad，後一步再從 system prompt 中讀取。
- Parallel agent 以「博物館 / 演唱會 / 餐廳」三個搜尋代理並行執行，最後再交給彙總代理產出整體旅程規劃。
- 來源明確點出 parallel pattern 的 trade-off：延遲降低，但多代理同時運行會提高初始成本，也增加結果合併的設計複雜度。
- 本影片只涵蓋基礎三種 pattern，並預告後續進階 pattern 如 orchestrator、review/critique loop、agent as tool。

## Practical Takeaways From This Source

- 任務簡單、工具少時，先用 single agent，避免過早把流程拆得過碎。
- 當流程順序固定且可重複時，改成 sequential agent，能用結構換取穩定度。
- 當多個子任務彼此獨立時，優先考慮 parallel agent，再補一個聚合步驟收斂結果。
- 設計多代理時，先畫清楚資料流與責任邊界，不要只從「多開幾個 agent」的角度思考。

## Related Concepts

- [agent-design-patterns](../concepts/agent-design-patterns.md)
- [harness-engineering](../concepts/harness-engineering.md)

## Alignment With Current Wiki

- 這份來源補上目前 wiki 尚未明確整理的 execution pattern 視角，把 agent workflow 拆成 single / sequential / parallel 三種結構選擇。
- 與既有 `harness-engineering` 一致，但更聚焦在工作流拓撲與 latency / control / flexibility 的取捨，而不是安全與工具邊界本身。
