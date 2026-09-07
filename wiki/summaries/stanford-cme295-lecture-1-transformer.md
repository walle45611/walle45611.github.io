# (3) Stanford CME295 Transformers & LLMs | Autumn 2025 | Lecture 1 - Transformer

- source: `raw/(3) Stanford CME295 Transformers & LLMs  Autumn 2025  Lecture 1 - Transformer.md`
- source link: https://www.youtube.com/watch?v=Ub3GoFaUcds
- original title: (3) Stanford CME295 Transformers & LLMs | Autumn 2025 | Lecture 1 - Transformer
- author: Stanford Online
- published: 未提供
- source_created: 2026-09-02
- ingested_at: 2026-09-06
- type: 影片逐字稿

## 摘要

本講從 NLP 任務、tokenization 與詞向量出發，說明 RNN 的序列依賴限制，再建立 self-attention 與原始 encoder-decoder Transformer 的資料流。核心是讓每個 token 依上下文重新取得表示，並在解碼時逐步預測下一個 token。

## 關鍵內容

- **NLP 任務（09:40 起）**：區分分類、逐 token 標記與文字生成；不同任務需要不同評估方式，不能只用一個指標衡量所有輸出。
- **Tokenization（23:08 起）**：word、subword、character 的切分粒度影響詞彙表、OOV 與序列長度。更細的粒度可處理更多字形變化，但通常增加 token 數與計算負擔；subword 不應被理解成保證符合語言學詞根。
- **表示學習（30:34 起）**：one-hot 無法直接表达詞義相近程度；Word2vec 以預測任務學到稠密向量。CBOW 用周圍詞預測中心詞，skip-gram 用中心詞預測周圍詞。靜態詞向量本身不隨當次句子改變。
- **RNN（53:31 起）**：透過 hidden state 累積序列資訊，但長距離依賴、梯度消失／爆炸與逐步計算限制了訓練；LSTM 引入 cell state 改善資訊保留。
- **Attention（約 1:05 起；1:31:36 詳解）**：輸入投影成 query、key、value，以 query-key 分數取得權重，再加權聚合 value。多個 head 使用各自投影，拼接後再投影回模型維度；不同 head 的分工不是由硬限制保證。

$$
\operatorname{Attention}(Q,K,V)=\operatorname{softmax}\left(\frac{QK^\top}{\sqrt{d_k}}\right)V
$$

- **Encoder 與 decoder（1:15–1:24、1:29:57 起）**：encoder self-attention 使用輸入序列；decoder masked self-attention 只看目前及先前位置；cross-attention 的 query 來自 decoder，key/value 來自 encoder 輸出。
- **其餘組件與生成**：位置編碼提供順序資訊，FFN 增加表示能力，label smoothing 避免訓練目標只將所有機率壓在單一 token。範例由 BOS 啟動，以線性投影及 softmax 產生詞彙分布，逐 token 解碼至 EOS。

## 來源限制與知識庫關聯

來源為課程逐字稿，未包含完整投影片；公式依逐字稿 1:32:21 的口述整理。影片發布日期未填，Autumn 2025 是課程標示，不替代發布日期。這裡說明的是原始 encoder-decoder 架構，不代表所有後續 LLM 都有 cross-attention。

本講為 [[transformer-attention-fundamentals]] 提供基礎，並補充 [[context-engineering]] 的模型機制背景：attention 的上下文化表示與工作流程中的摘要／compaction 是不同層次。
