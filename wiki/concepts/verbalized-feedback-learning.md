# Verbalized Feedback Learning

## Current View

在目前知識庫中，verbalized feedback learning 指的是：不依賴明確標籤或分數，直接利用自然語言回饋（人類或環境訊息）改變 agent 後續行為，並在部分方法中進一步轉化為參數更新信號。

## Stable Conclusions

1. 在真實任務中，最常見回饋型態常是文字訊息（指正、錯誤訊息、偏好描述），而非標準答案。
2. 模型通常能利用高品質回饋改善行為；錯誤或噪音回饋可能導致表現下滑。
3. 回饋內容的可執行性與可驗證性，通常比語氣強弱更影響改進效果。
4. 迴圈式流程（生成 -> 評估 -> 回饋 -> 再生成）是 verbalized feedback 發揮作用的主要結構。
5. 在長期 agent 場景，verbalized feedback 可結合 skill 檔、記憶整理或持續微調，形成累積式能力演進。
6. 由工具、checklist、錯誤訊息或搜尋結果帶來的 external feedback，通常比模型單靠 internal self-reflection 更穩定。

## Working Heuristics

- 優先給具體、可操作、可驗證的 feedback，避免情緒化責備。
- 讓回饋直接對應任務成功條件，而不是只給抽象評語。
- 若任務可執行檢查，優先把工具回傳訊息納入迴圈回饋。
- 當上下文過長時，先摘要再進下一輪，維持回饋資訊密度。
- 反思提示詞會影響模型的 confidence 與修正傾向；肯定式與質疑式 wording 應依模型特性調整。

## Open Questions

- 如何穩定辨識多輪對話中哪些句子是真正可學習的 feedback，仍是關鍵研究點。
- verbalized feedback 何時只該影響當前上下文、何時值得轉成參數更新，尚缺統一判準。
- AI-judge 參與的評量流程可能高估表現，需更多人類互動基準校正。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [self-correction-in-language-models](./self-correction-in-language-models.md)

## Sources

- [harness-engineering-language-models-need-human-guidance](../summaries/harness-engineering-language-models-need-human-guidance.md)
- [ai-self-correction-decoding-workflow-reasoning](../summaries/ai-self-correction-decoding-workflow-reasoning.md)
