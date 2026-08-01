# LLM Evaluation

## Current View

LLM 評估是把「模型輸出是否符合目標」拆成可觀測、可重複與可診斷的檢查。它不應被縮減成單一總分：輸出品質、任務成功、事實性、格式、安全、延遲、成本與可用性，通常需要不同的評估器與資料。

## Evaluation Layers

1. **Human evaluation**：最接近目標偏好，但成本高、速度慢，且評分任務可能主觀。應以清楚 guidelines 與 inter-rater agreement 檢查評分一致性。
2. **Reference-based metrics**：以固定 reference 重複比較模型輸出，例如 METEOR、BLEU、ROUGE。適合可定義參考答案的任務，但會受字面 overlap、風格變化與 reference 完整性限制。
3. **LLM-as-a-Judge**：輸入 prompt、回答與評分準則，產生分數與 rationale。適合開放式輸出，但必須校準人工評分並檢查 position、verbosity、self-enhancement 等 bias。
4. **Executable or structured evaluation**：對格式、程式碼、資料庫狀態或工具結果使用 schema、測試、狀態檢查等硬性驗證，通常比再交給另一個 LLM 判斷更容易重現。
5. **Benchmark evaluation**：用特定資料集描述模型在知識、推理、程式碼、安全或 agent 工具使用上的 profile；不能直接取代真實使用情境測試。

## LLM-as-a-Judge Guidelines

- 將評估準則寫成具體、可操作的 pass/fail 條件，避免模糊的整體印象。
- 要求 judge 先寫 rationale 再輸出 score，並用 structured output 固定 schema，讓結果可穩定解析。
- Pairwise 評估要交換兩個回答的順序；若結果改變，應視為 position bias 的警訊。
- 明確要求不要因回答較長就偏好它，並可用不同長度與 in-context examples 測試 verbosity bias。
- 盡量不要讓同一模型同時生成回答與擔任 judge；至少要用人工樣本確認 judge 與目標偏好的相關性。
- 以低 temperature 執行評估，保留 prompt、模型版本與評估設定，確保結果可重現。

## Factuality and Agent Evaluation

### Factuality

事實性可先把輸出拆成 atomic facts，再逐項以 RAG、web search 或其他可信知識來源驗證，最後依重要性加權聚合。這種分解能區分「整段完全錯誤」與「大致正確但含少數錯誤」，也讓錯誤更容易追蹤。

### Agents

Agent 的評估應沿著 observe、plan、act 的循環拆解：

- 工具選擇：是否在需要時使用工具、tool router 是否漏召回、是否產生不存在的工具。
- 參數預測：工具與 API 正確時，參數是否完整、正確且有足夠 context。
- 工具執行：工具是否回傳有意義且結構化的結果；錯誤或空回應是否能表達真實狀態。
- 結果合成：模型是否正確引用工具輸出，並向使用者回報真實的任務狀態。

這種分類能把問題分派給模型、prompt、context、tool router、API 描述或工具實作，而不是籠統地歸咎於 LLM。

## Benchmark Selection

Benchmark 應服務於具體決策：MMLU 可代表多領域知識，AIME 與 PIQA 可代表數學與常識推理，SWE-bench 可用測試驗證程式碼修復，HarmBench 反映特定安全政策下的風險，Tau-bench 則把工具、政策、模擬使用者與資料庫狀態結合起來評估 agent 任務成功與可靠性。

比較時要檢查資料污染、測試是否可硬性驗證、benchmark 政策與產品目標是否一致，並避免因追逐分數而違反 Goodhart's law。最終模型選擇通常要同時考量品質、成本、安全、延遲與真實任務表現。

## Related Concepts

- [[nlp-evaluation-metrics]]
- [[self-correction-in-language-models]]
- [[harness-engineering]]
- [[long-context-position-effects]]

## Sources

- [[stanford-cme295-lecture-8-llm-evaluation]]
