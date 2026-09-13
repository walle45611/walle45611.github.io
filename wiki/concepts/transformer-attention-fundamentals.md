# Transformer 與 Attention 基礎

## 核心機制

依 Stanford CME295 第一講，tokenization 決定文字如何切分，embedding 將 token 轉成向量，attention 則讓每個位置依相關上下文重新取得表示。這三個步驟的用途不同。

$$
Q=XW_Q,\quad K=XW_K,\quad V=XW_V
$$

$$
\operatorname{Attention}(Q,K,V)=\operatorname{softmax}\left(\frac{QK^\top}{\sqrt{d_k}}\right)V
$$

Query-key 分數經縮放與 softmax 形成權重，再對 value 加權求和。Self-attention 的三者来自同一序列；原始 Transformer 的 cross-attention 則由 decoder 提供 query、encoder 提供 key/value。多個 head 的輸出拼接後再投影。

## 應分清的層次

- RNN 逐步傳遞 hidden state；attention 提供 token 間較直接的資訊關係，但自回歸解碼仍逐 token 產生。
- 位置資訊須另外編入；decoder 的 causal mask 限制讀取目前及先前位置，避免使用未來 token。
- FFN、位置編碼與輸出 softmax 各有角色，不宜把整個 Transformer 都稱為 attention。
- Attention 產生上下文化表示，不等於把整段文字摘要成較短文字。[[context-engineering]] 的 compaction 與檢索是應用工作流程層的管理方法。

## 邊界與來源

本頁以原始 encoder-decoder 架構建立基礎；不從單一入門講座推定所有現代模型的架構或長文本利用能力。

- [[context-engineering]]
- [[stanford-cme295-lecture-1-transformer]]

## KV Cache 與計算邊界

[[deepseek-v4-1-flash-kv-cache-compression]] 補充 causal 自回歸推論中的 K/V 重用：避免重算歷史 K/V，不等於免除新 query 對歷史的注意力計算。來源的 O(N) 說法不能直接套用到完整生成成本。容量與讀取頻寬的部署取捨見 [[local-llm-deployment]]；降低 K/V 儲存精度則屬於 [[model-quantization]]。

[[how-inference-engines-work-agent-loops]] 將機制延伸到請求流程：prefill 建立多個輸入位置的狀態，decode 延續生成，跨請求的 prefix caching 則復用已存在的前綴狀態。這與单次生成內的 KV Cache 有關，但復用範圍及存活條件不同。
