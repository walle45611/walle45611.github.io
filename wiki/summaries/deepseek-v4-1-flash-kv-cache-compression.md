# 圖解 KV Cache——DS V4.1-Flash 的制勝秘密

- source: `raw/图解 KV Cache——DS V4.1-Flash的制胜秘密.md`
- source link: https://x.com/wquguru/status/2099119760248389730?s=46
- original title: 图解 KV Cache——DS V4.1-Flash的制胜秘密
- author: @wquguru
- published: 2026-09-13（剪藏欄位）
- source_created: 2026-09-14
- ingested_at: 2026-09-14
- type: 社群長文剪藏／技術解說

## 摘要

文章以 KV Cache 的容量與讀取成本解釋長上下文推論，將優化分成每筆資料大小、保留序列長度、跨層份數，以及分頁、前綴共享與分層儲存。接著以作者對 DeepSeek V4.1-Flash 的解讀，說明跨層復用、稀疏索引、低精度快取和持久化策略如何配合。

## 核心內容

- **快取用途**：在 causal 自回歸推論中保留歷史 token 的 K/V，避免每一步重新產生歷史 K/V；prefill 建立輸入的狀態，decode 逐 token 延續。快取仍須被注意力讀取。
- **容量與頻寬分開看**：作者用「每條大小 × 條數 × 份數」拆解容量。稀疏選取可以減少注意力讀取，但若全部 KV 仍保留，就不等於降低儲存容量。
- **架構與系統分開看**：GQA 減少 KV heads，MLA 儲存潛在表示，滑動窗口限制局部歷史，跨層復用減少獨立副本。分頁改善配置碎片，前綴共享減少重複，offload 改變儲存位置；不能把這些都理解成同一種壓縮。
- **量化對象**：文章區分 main KV、索引器 K 與 SWA KV 的精度；快取量化與權重量化是不同對象，降低儲存精度也不代表注意力直接使用同精度計算。

## 作者對 V4.1-Flash 的描述

以下保留為來源主張，本次未另讀官方技術報告核實：

1. **CED**：40 層分為前 20 層因果編碼器與後 20 層解碼器，後段全局 KV 由第 20 層輸出投影取得；局部 SWA 另行處理。文末仍描述 decoder 對 prompt 尾端的計算，因此不宜把「prefill 只跑前 20 層」理解為完全沒有後段成本。
2. **CSA2**：Full 產生 main KV 與索引，Reindex 共享 KV 但重算候選，Reuse 共享 KV 與候選索引；文章稱只有 4 層保存獨立全局 KV，但每層仍有自身 Q 與 SWA 狀態。「其餘 36 層不占空間」只可理解為該項全局 KV 副本，不能套用到全部快取。
3. **階層索引**：作者描述 decoder 先以每 8 個位置分塊，選 2,048 塊形成 16,384 個候選位置，再供後續 Reindex 使用；各層選 Top-512。
4. **精度與估算**：作者以 main KV 的 512 維 FP4 加 32 bytes scale 得 288 bytes，索引器 128 維加 8 bytes scale 得 72 bytes，再算 `3 × (288 + 72) / 2 + (288 + 72) = 900 bytes/token`。這是作者反推的近似值，並非文中所稱報告的 890 bytes 精確分解，且只針對全局 KV。
5. **分層保存**：文中將 runtime KV 放 HBM、短命的 encoder SWA 放 DRAM 池、全局持久化 KV 放 SSD；SWA 過期後以最近 128 tokens 近似重放，區別於精確重建所需的更長依賴範圍。
6. **結果主張**：作者稱相較 V4-Flash，全局 HBM KV 降至 1/4、持久化 KV 降至 1/8；另引用 DeepSWE v1.1 的 74.2、Terminal-Bench 3.0 的 30.0 與 HLE 的 36.8。這些是不同測試的來源數字，不能互相比較或推導全面能力提升。

## 限制與待核對事項

- 本頁依本地剪藏歸檔；外部圖片與所述官方報告未另行檢查。發布日期、模型配置、benchmark 與 API 價格均不作為目前已驗證事實。
- 原文「使用快取後總計算量由 O(N²) 降為 O(N)」未限定計算項目；不能據此聲稱完整 dense attention 生成總成本變成線性。重用歷史 K/V 與持續對歷史做 attention 是兩件事。
- 「KV 比權重占更多顯存」與 prefill／decode 的瓶頸取決於上下文、併發、架構及實作，不是所有負載都成立。
- 原文稱編碼器 18 層分組，模型前段則為 20 層；其餘兩層的配置未在剪藏文字中交代，不自行補齊。
- 作者將快取縮小與 API 降價、能力提升相連，但未提供可分離各因素的因果證據；本文不把定價比例當成部署成本或能力的保證。

## 知識庫關聯

- [[transformer-attention-fundamentals]]：補上自回歸 K/V 重用與 attention 計算的區分。
- [[local-llm-deployment]]：補強容量、頻寬、併發與前綴共享應分別量測的框架。
- [[model-quantization]]：區分快取量化與權重量化，保留 scale 開銷及品質驗證邊界。

- [[how-inference-engines-work-agent-loops]]：互補說明連續批次、chunked prefill 與跨請求前綴復用。
