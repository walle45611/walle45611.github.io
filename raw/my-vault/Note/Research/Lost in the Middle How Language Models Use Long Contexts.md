# Lost in the Middle: How Language Models Use Long Contexts

[[Assets/Note/Research/Papers/Lost in the Middle - How Language Models Use Long Contexts/Lost in the Middle - How Language Models Use Long Contexts.pdf|論文 PDF]]

## 初讀摘要

- 這篇檢驗模型是否真的能有效使用長上下文，而不只是在介面上接受更多 token。
- 透過多文件問答及 key-value 檢索，控制相關資訊在輸入中的位置與上下文長度。
- 模型通常較容易使用開頭或結尾的資訊，中間位置表現較差；因此 context window 長度不能直接代表資訊利用能力，RAG 文件排序也可能影響答案。

摘要依據：PDF 摘要與導論。


閱讀狀態：**已讀完，完整筆記待補。** 以下保留目前記錄的重點。

Table 1 提供 Closed-book 與 Oracle 兩個基準。Closed-book 表示模型不看文件時的基本能力；Oracle 表示只給含答案文件時的上限表現。Oracle 準確率較高，說明模型其實能根據正確文件回答問題。然而 Figure 5 顯示，當含答案文件與多個干擾文件一起放入 context，尤其位於中間位置時，準確率會明顯下降。因此問題不是模型完全不會回答，而是長上下文中資訊位置會影響模型能否有效利用該資訊。

為了排除自然語言語意造成的影響，作者又設計了 UUID key-value retrieval task。這個任務把資料簡化成 JSON 格式的 key-value pairs，key 和 value 都是隨機 UUID，模型只需要根據指定 key 找出對應 value。即使如此，部分模型在目標 key-value pair 位於 context 中間時仍然表現較差，進一步證明 lost-in-the-middle 不只是 QA 語意推理問題，而是模型對中間位置資訊的存取能力本身就不穩定。
