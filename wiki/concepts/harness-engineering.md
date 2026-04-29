# Harness Engineering

## Current View

在目前知識庫中，harness engineering 可定義為：透過規則、工具與工作流程的設計，系統性地約束並放大語言模型在多輪任務中的可用行為。它不等於 prompt wording，而是涵蓋整個 agent 執行環境的行為工程。

## Stable Conclusions

1. AI agent 的最終表現是「模型能力 × harness 設計」的共同結果，不能只歸因於模型參數大小。
2. 規則檔（如 `agents.md`、`CLAUDE.md`）可作為認知框架，雖非硬約束，仍常對效率與穩定性有明顯影響。
3. 工具與權限同時定義了能力上限與安全下限；安全性與便利性通常是可調但互相牽制的 trade-off。
4. 對 agent 友善的工具介面不必與人類習慣一致；可摘要、可結構化、低噪音的介面通常更利於任務完成。
5. 工作流設計（如 planner/generator/evaluator 或生成-驗證迴圈）是把模型輸出轉成可交付結果的關鍵中介層。
6. 真正穩定的治理通常需要「harness 層硬控制」與「模型層軟約束」並用；只靠 prompt 指示不足以覆蓋高風險場景。
7. 在 AI coding 情境中，將「架構規劃」與「代碼執行」分層，通常比單模型端到端直譯需求更能控制效能與技術債風險。
8. 在自我修正任務中，來自檢查器、搜尋、執行器或 checklist 的 external feedback，通常比單純要求模型自我反思更穩定。

## Working Heuristics

- 先定義「第一步」與「完成條件」，再讓模型展開推理與工具調用。
- 規則檔寫成導航地圖，不要塞滿百科式細節。
- 編輯或執行類任務優先形成「輸出 -> 檢查 -> 回饋 -> 重試」閉環。
- 驗證流程若要增加算力，先和同算力預算下的多樣 sampling / majority vote 比較，再決定是否值得加入昂貴 verification。
- 依任務性質選擇 workflow pattern：簡單工具使用可用 single agent，固定步驟可改成 sequential，多個獨立子任務則考慮 parallel + aggregator。
- 先要求模型產生含效能/複雜度約束的計畫，再交由執行型模型產碼與修正。
- 設計工具時優先思考模型如何節省上下文，而非只思考人類操作便利。
- 高權限工具（檔案刪除、外部執行、帳戶操作）預設加人工核准與最小權限。
- 以隔離環境承載 agent（專用機、獨立帳號、獨立憑證），控制失誤半徑。
- 在知識庫型 agent 中，把唯讀來源層與可寫 wiki 層分離，能讓規則與工具邊界更可執行。

## Open Questions

- 不同模型族（如小模型/大模型）對同一套 harness 的敏感度差異仍需更多跨模型實證。
- 規則檔長度、語氣與結構如何影響正確率，尚缺穩定可泛化的設計原則。
- 長期運作 agent 的記憶整理與自我修復流程，哪些機制最能避免能力衰退，仍在發展中。
- 「高精準執行模型」與「高抽象規劃模型」的分工邊界如何量化，仍缺公開、可重現 benchmark。

## Related Concepts

- [context-engineering](./context-engineering.md)
- [verbalized-feedback-learning](./verbalized-feedback-learning.md)
- [agentic-knowledge-base-maintenance](./agentic-knowledge-base-maintenance.md)
- [agent-design-patterns](./agent-design-patterns.md)
- [self-correction-in-language-models](./self-correction-in-language-models.md)

## Sources

- [harness-engineering-language-models-need-human-guidance](../summaries/harness-engineering-language-models-need-human-guidance.md)
- [openclaw-ai-agent-operating-principles](../summaries/openclaw-ai-agent-operating-principles.md)
- [ai-agent-design-patterns](../summaries/ai-agent-design-patterns.md)
- [codex-vs-claude-vibecoding-observations](../summaries/codex-vs-claude-vibecoding-observations.md)
- [llm-wiki-worker](../summaries/llm-wiki-worker.md)
- [ai-self-correction-decoding-workflow-reasoning](../summaries/ai-self-correction-decoding-workflow-reasoning.md)
