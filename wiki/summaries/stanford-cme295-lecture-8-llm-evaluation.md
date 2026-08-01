# Stanford CME295 Transformers & LLMs｜Autumn 2025｜Lecture 8 - LLM Evaluation

- source: `raw/Stanford CME295 Transformers & LLMs  Autumn 2025  Lecture 8 - LLM Evaluation.md`
- source link: https://www.youtube.com/watch?v=8fNP4N46RRo
- original title: Stanford CME295 Transformers & LLMs | Autumn 2025 | Lecture 8 - LLM Evaluation
- author: [[Stanford Online]]
- published: 2025-12-03
- lecture date: 2025-11-21
- created: 2026-07-11
- type: YouTube lecture transcript

## Summary

本講以「沒有可靠的評估，就不知道模型該改進什麼」為主軸，整理 LLM 輸出品質的評估方法。內容從人工評分的成本與主觀性出發，經過 inter-rater agreement 與 METEOR、BLEU、ROUGE 等 reference-based metrics，進一步介紹不依賴固定參考答案的 LLM-as-a-Judge。後半段延伸到事實性、agent 工作流錯誤診斷，以及知識、推理、程式碼、安全與工具使用等 benchmark 類型。

## Key Claims

1. LLM 評估不只包含輸出品質，也可能包含延遲、價格、可用性等系統指標；本講主要聚焦輸出是否有用、正確、相關、安全且符合格式。
2. 人工評分雖然理想，卻昂貴、緩慢且可能主觀；Cohen's kappa、Fleiss's kappa 與 Krippendorff's alpha 透過「相對於隨機一致」的基線來衡量評分者一致性。
3. METEOR、BLEU、ROUGE 等規則式指標能讓固定 prompt 集合被重複評估，但依賴 reference，容易低估同義改寫與風格變化，也不等同於事實性或實用性。
4. LLM-as-a-Judge 將 prompt、模型回答與評估準則交給另一個 LLM，輸出分數與理由；它可減少對固定 reference 和大量人工標註的依賴，但仍是近似人類偏好的 proxy。
5. Judge 最好先產生 rationale 再產生 score，並使用 structured outputs 保證結果可解析；評估通常偏好低 temperature，以提高重現性。
6. Pairwise judge 可比較兩個回答並產生偏好資料，但必須注意 position bias、verbosity bias 與 self-enhancement bias。常見緩解方式包括交換答案順序、明確要求忽略長度、加入示例，以及避免使用同一模型同時生成與評分。
7. 評估準則應清楚、可操作；二元的 pass/fail 往往比過度細緻的量表更容易讓 judge 與人工評分對齊。不過仍應以人工樣本校準 judge，避免過度優化評估 proxy。
8. 事實性評估可先把長文本拆成可核查的 facts，再逐項透過 RAG、web search 或其他知識來源驗證，最後依事實重要性加權聚合；因此局部錯誤不必然使整段文字被視為完全錯誤。
9. Agent 評估應拆解成工具選擇與參數預測、工具執行、結果合成等階段，分別追蹤 punt、tool router recall error、tool hallucination、錯誤參數、工具輸出不具意義、空回應，以及無法正確引用工具結果等 failure modes。
10. Benchmark 應依使用情境選擇，而非用單一分數宣稱模型全面較好。講義以 MMLU 評估知識、AIME 與 PIQA 評估推理、SWE-bench 評估程式碼修復、HarmBench 評估安全、Tau-bench 評估工具型 agent；還要注意資料污染與 Goodhart's law。

## Practical Evaluation Workflow

1. 先界定要評估的層級：輸出品質、任務成功、格式與安全，或 latency、cost、availability 等系統面向。
2. 對可有多種正確表達的任務，不要只依賴字面 overlap；選擇能對應目標品質的 reference-based metric、judge 或 executable evaluator。
3. 若使用 LLM-as-a-Judge，明確定義準則與輸出 schema，要求 rationale 在 score 之前，並設定低 temperature。
4. 以交換順序、不同長度、不同生成模型的測試檢查 judge bias，並用一批人工評分資料做 correlation/calibration。
5. 對 factuality 與 agentic workflow，將整體結果拆成可診斷的 atomic checks，而不是只看最後一個總分。
6. 使用 benchmark 描述模型 profile，並結合實際任務、成本、安全與可靠性做選型；不要把 benchmark 直接當成使用者價值的替代品。

## Important Limitations

- 沒有 universal metric 能同時涵蓋自然語言、程式碼、數學推理、工具使用與安全。
- LLM-as-a-Judge 會受到 judge 自身能力、訓練偏好與 prompt 設計影響；即使沒有固定 reference，也不代表評估客觀。
- Benchmark 的分數可能受到資料污染、測試格式與政策定義影響；當指標成為最佳化目標時，可能失去原本的代表性。
- Agent 的錯誤可能來自模型、context、tool router、API 描述、工具實作或結果格式，不能只歸因於模型能力。

## Related Concepts

- [[llm-evaluation]]
- [[nlp-evaluation-metrics]]
- [[self-correction-in-language-models]]
- [[harness-engineering]]
- [[long-context-position-effects]]

## Alignment With Current Wiki

本來源將既有的 reference-overlap 指標放進更大的 LLM 評估框架，補上 LLM-as-a-Judge、factuality decomposition、agent failure taxonomy 與 benchmark selection。它與既有 ROUGE 摘要一致，但進一步明確指出 ROUGE、judge 與 benchmark 都只是針對特定目標的 proxy，不能單獨代表整體品質。
