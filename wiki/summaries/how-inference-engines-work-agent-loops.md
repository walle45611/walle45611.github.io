# LLM 推論引擎：從 token、KV Cache 到 Agent Loop

- source: `raw/web-clipper/LLM 推理引擎到底是怎么工作的：从一个 token 到 KV 缓存、连续批处理，再到 Agent Loop 如何把 GPU 榨干.md`
- source link: https://x.com/shao__meng/status/2098783138118041760?s=46
- original title: LLM 推理引擎到底是怎么工作的：从一个 token 到 KV 缓存、连续批处理，再到 Agent Loop 如何把 GPU 榨干
- author: @shao__meng（整理 Zain Hasan／Together AI 演講）
- published: 2026-09-12（剪藏欄位）
- source_created: 2026-09-14
- ingested_at: 2026-09-14
- type: 社群技術解說／演講二手整理
- referenced slides: https://docs.google.com/presentation/d/1ljkXsGI8fCtHEwg7XVaTLc8R2aGtaZIox2bpjhJiVnY/edit?usp=sharing

## 摘要

文章沿單一請求的生命週期，說明 tokenization、prefill、decode、KV Cache、排程、採樣與串流輸出，再將連續批次、分頁快取及前綴共享連到多輪 agent 的負載特性。核心觀點是：推論效能取決於如何安排請求、token 預算與快取空間，不能只看模型前向運算。

## 請求與排程

- **輸入與輸出**：文字經 tokenizer 轉成 token IDs；模型輸出 logits，再依採樣設定產生 token 並反分詞。文中詞表約十萬、500 英文詞約 700 tokens 是示例，並非固定換算。
- **Prefill 與 decode**：前者建立 prompt 的狀態，後者利用歷史 K/V 延續生成。來源以 compute-bound 與 memory-bound 概括常見負載差異，用來解釋 TTFT 和輸出速率，但不是所有批次與硬體都符合的絕對分類。
- **引擎分工**：來源以 API server → scheduler → GPU workers 描述管線；scheduler 決定每步參與的請求、token 預算及 KV blocks，CPU 輸出處理可與 GPU 運算重疊。
- **Continuous batching**：每步重新組批，已結束的請求可讓位給等待中的請求，降低靜態批次等待最長請求的浪費；仍受可用流量、token 預算與記憶體約束。
- **PagedAttention**：將 KV 切成固定大小 blocks，依需求從共用池配置並以 block table 對應，降低預留最大長度造成的浪費。文中 60–80% 降至 4% 以下的碎片率屬轉述示例，未獨立核對測量條件。
- **Chunked prefill**：將長 prompt 切入多次排程，與活躍請求的 decode 分享預算。8K prompt／2K 預算的四片是簡化示例；若同時保留 decode 預算，實際每片可處理的 prefill tokens 可能更少。此安排著重減少長 prefill 對既有輸出的干擾，不能保證新請求 TTFT 必然下降。

## Agent Loop 與前綴快取

作者描述常見的無狀態對話呼叫：工具結果追加至歷史後，再送出一輪請求，形成長前綴、短新增尾段及較短輸出的模式。Prefix caching 可復用相同前綴的 KV，減少重複 prefill；連續批次承接並行工具與子 agent 的突發請求，chunked prefill 或 prefill/decode 分離處理不均衡負載，offload 則將暫時不活躍的快取移出 GPU。

來源以 100 輪迴圈提出約 55 倍對 10 倍的工作量比較，但剪藏未提供完整基準與推導，不能當作一般加速比。若每輪固定增加 tokens，重複處理全部歷史與只處理新增 tokens 的累積 token 數成長方式不同；這也不等於整體 attention FLOPs 或端到端延遲直接具有相同比例。

## 限制與知識庫關聯

- 本次讀取的是社群剪藏，未另讀投影片；示意排程、數值和講者原始說法的對應仍待核實。
- 「一次 forward 一個 token」是文中標準自回歸 decode 的簡化視角，不能延伸為所有引擎路徑；prefill 本身就處理多個輸入位置。
- 前綴快取依賴引擎支援、啟用狀態、相容且相同的前綴及尚未淘汰的狀態；原文「無需配置」「速度自動到手」不是普遍保證。
- [[deepseek-v4-1-flash-kv-cache-compression]] 側重縮小 KV 表示與跨層副本，本篇側重請求排程與復用，兩者互補。兩篇均不能把歷史 K/V 重算的減少等同於所有注意力成本消失。
- [[local-llm-deployment]] 承接部署與量測框架，[[transformer-attention-fundamentals]] 承接模型機制與快取的區分。
