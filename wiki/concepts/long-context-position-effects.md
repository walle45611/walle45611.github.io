# Long-Context Position Effects

## Current View

在目前知識庫中，long-context position effects 指的是：當語言模型面對長輸入或多文件 context 時，對資訊的利用不是平均分布，而會受到內容所在位置影響。最典型的現象是 relevant information 在前段或末段較容易被模型有效使用，在中段則明顯退化，也就是所謂的 lost in the middle。

## Stable Conclusions

1. 長 context window 不代表模型能平均利用整段輸入；位置效應本身就是系統能力邊界。
2. 在 RAG 中，relevant document 的排序品質會直接影響 generation 品質，因為中段位置往往是利用率最低區。
3. 單純增加 top-k 或放入更多候選文件，常只會提升 recall，未必能提升最終回答品質。
4. 這個現象不只出現在多文件問答，也能出現在 key-value retrieval，表示它同時牽涉語意推理與長輸入中的基本提取能力。
5. 對 decoder-only 模型而言，把 query 或任務目標放在前面，有助於提升後續 context 的可利用性。

## Working Heuristics

- 先優化 retrieval ranking，再考慮擴大 context 長度。
- 對長 prompt 優先控制內容順序，讓最關鍵的證據靠前，必要時把重要補充放在尾端而非埋在中間。
- 若系統會混合多個來源的檢索結果，應額外做 re-ranking 或分區組裝，避免相關內容被大量中段噪音吞沒。
- 評估 RAG pipeline 時，同步追蹤 recall、最終 answer quality、以及 relevant chunk 的實際 prompt position。
- 對 decoder-only 模型，把問題、schema 或 extraction target 放在前面，再附上候選內容。

## Open Questions

- 不同模型架構與 instruction tuning 方式，對 lost-in-the-middle 的敏感度差多少，仍需要更多跨模型實測。
- 哪些 prompt 組裝策略最能穩定緩解中段退化，目前仍偏經驗法則。
- 多來源 context、長篇摘要、工具輸出拼接等更複雜場景，位置效應會如何與來源多樣性疊加，仍缺足夠整理。

## Related Concepts

- [context-engineering](./context-engineering.md)
- [harness-engineering](./harness-engineering.md)

## Sources

- [lost-in-the-middle-how-language-models-use-long-context-explained](../summaries/lost-in-the-middle-how-language-models-use-long-context-explained.md)
