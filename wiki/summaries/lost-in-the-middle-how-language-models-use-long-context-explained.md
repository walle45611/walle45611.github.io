# Lost in the Middle: How Language Models use Long Context - Explained!

- source: `raw/Lost in the Middle How Language Models use Long Context - Explained!.md`
- source link: https://www.youtube.com/watch?v=Kf3LeaUGwlg&t=2s
- original title: Lost in the Middle: How Language Models use Long Context - Explained!
- author: [[Weaviate vector database]]
- discussed paper: https://arxiv.org/pdf/2307.03172.pdf
- published: 2023-07-17
- created: 2026-05-15
- type: YouTube transcript summary

## Summary

這份影片是 Weaviate 對 Liu et al. 2023〈Lost in the Middle〉的解說。核心訊息是：長 context 並不等於模型能平均利用整段輸入。當 RAG 的關鍵資訊落在輸入前段或末段時，回答表現較好；落在中段時，表現明顯下滑，形成 U-shaped curve。這使得「檢索排序品質」比單純擴大 context window 更重要。

## Key Claims

1. 在 multi-document QA 中，模型對 relevant document 的利用存在明顯的位置效應，最差位置通常出現在輸入中段。
2. 當 relevant document 位於第一個位置時，RAG 對問答準確率的提升非常大；落到中段時，甚至可能比 closed-book 回答更差。
3. 隨著輸入文件數增加，平均表現不會持續上升，反而常因更多干擾內容而下降。
4. 這個現象不是單一模型特例；影片整理的實驗涵蓋 Claude、GPT-3.5、MPT、LongChat，連 GPT-4 也仍能觀察到類似趨勢。
5. 在更接近真實檢索的設定中，提高 recall 並不自動轉成更好的最終回答，因為模型未必能有效利用排在中後段的相關內容。
6. key-value retrieval 實驗顯示，模型即使在「純複製」任務上也會出現位置偏差，表示問題不只在語意理解，也牽涉到長輸入中的資訊提取能力。
7. query-aware contextualization 可能顯著改善 decoder-only 模型的長 context 利用率，因為把問題先放在前面，能讓模型在閱讀後續內容時帶著檢索目標。

## Important Details

- 影片把研究問題拆成兩個控制變數：輸入中放了多少文件，以及正確答案位於第幾個文件。
- 在 controlled setup 中，作者使用有 ground truth 的問答資料，刻意移動正確文件位置，避免把排序誤差與模型能力混在一起。
- 在 more realistic setup 中，retriever 的 recall 隨著 top-k 上升而提高，但最終 QA 表現很快飽和，說明「找得到」與「模型能用好」是兩件事。
- 影片特別強調 closed-book 與 oracle context 的差距很大，這代表高品質檢索仍是 RAG 系統的主要槓桿。
- query-aware contextualization 的例子是先放問題、再放候選內容；影片引用的結果指出，這能把某些 key-value retrieval 設定從低表現拉到接近完美。
- 影片作者把這篇研究連到 serial position effect，指出模型在 list-like input 中對首尾項目較敏感，與人類記憶研究有可比較之處。

## Practical Takeaways

- 不要把「更大 context window」當成 RAG 品質問題的直接解法；先處理排序與上下文組裝。
- 如果系統會傳很多候選文件給模型，應優先投資 re-ranking、cutoff 或其他減少中段噪音的策略。
- 在 decoder-only 模型上，問題敘述與任務指令應盡量提前，避免模型先被大量無導向內容淹沒。
- 評估 RAG 時，不只看 retrieval recall，也要看 relevant chunk 在 prompt 中的實際位置分布。
- 若應用常需跨多來源拼接 context，應額外測試「來源混合後的排序品質」而不只測單一索引的 top-k。

## Related Concepts

- [context-engineering](../concepts/context-engineering.md)
- [long-context-position-effects](../concepts/long-context-position-effects.md)

## Alignment With Current Wiki

- 這份來源補強了 `context-engineering` 中對 RAG 與長 context 管理的觀點：問題不只是能塞多少內容，而是模型能否在正確位置讀到真正有用的內容。
- 它也促成一個更具體的概念層整理：把長 context 中的 serial-position 式偏差獨立成 `long-context-position-effects`，方便後續接到檢索排序、prompt 組裝與多來源 context 設計。
