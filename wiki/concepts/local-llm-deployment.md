# Local LLM Deployment

## Current View

Local LLM deployment 指的是把開放權重模型與 serving stack 放在本機、區域網路、私有雲或組織控制的硬體上，再接到 agent harness 與工具。真正需要設計的單位不是單一模型檔案，而是模型、推論引擎、資料路徑、工具權限與執行隔離所形成的整體系統。

## Stable Conclusions

1. 地端模型可以讓資料路徑更容易留在組織控制範圍內，但不會自動消除 prompt injection、惡意工具、憑證外洩或錯誤操作風險。
2. 模型記憶體需求不能只用參數量估算；權重、KV cache、context window、併發量與系統保留空間都要納入，MoE 的 active parameters 也不等於載入時的總權重。
3. unified memory 硬體可用較大的容量換取部署彈性，但頻寬、延遲、溫度與功耗限制仍會影響實際吞吐；容量足夠不代表互動速度或併發一定足夠。
4. 小型地端機器適合先做 POC：用代表性任務、目標 context、併發量與安全配置驗證可行性，再依實測需求擴展到更大的工作站、私有雲或 cluster。
5. 模型格式、tokenizer/config、tool parser、推論引擎、容器映像檔與硬體架構必須一起驗證；Docker 有助於固定啟停與清理方式，但不保證已有適用的 image。
6. 模型位置與 harness 隔離是兩條不同的安全軸：私有模型可縮小資料外流範圍，container/VM/sandbox 可縮小 agent 失誤的爆炸半徑，網路政策則要另外明確設定。
7. 可由固定工具或 CPU 程式完成的轉換、擷取與檢查，應優先交給可審查、可重複的工具，讓模型負責選擇與編排，降低自由生成程式碼的必要性。

## Working Heuristics

- 先畫出資料流與信任邊界：模型在哪裡、harness 在哪裡、哪些目錄可寫、哪些工具可執行、sandbox 是否能連網。
- 估算權重後保留 KV cache、context、併發與系統空間；再用實際請求長度與多使用者情境做 smoke test。
- 讀取 model card，確認檔案格式、量化方式、tokenizer、tool parser、MTP 與目標 inference engine 的支援，再決定硬體。
- 對 DGX Spark、Mac 或其他 unified-memory 平台，同時記錄記憶體占用、溫度、時脈、單流延遲與併發吞吐，不用單一 tok/s 判斷整體可用性。
- 以假資料和代表性文件先驗證去識別化、摘要、程式執行等目標任務；成功的 demo 不等於合規、隱私或安全保證。
- 模型 license、組織政策、資料分類與網路出口應在部署前一起審查，尤其是需要外部模型或外部 skills 的工作流。

## Open Questions

- 不同 Qwen 版本、量化格式與 inference engine 在 DGX Spark 等硬體上的可重現延遲與品質差異，仍需要固定 workload 的 benchmark。
- context 長度、KV cache、併發與量化品質之間的最佳配置，會如何隨模型家族與工具使用模式變化？
- sandbox 需要下載 skills 或連接版本庫時，如何在最小網路權限與可維運性之間取得平衡？
- 單機 POC 的吞吐、電力、溫度與維運成本，應用什麼指標推導到私有雲或多機部署？

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [llm-serving-compatibility](./llm-serving-compatibility.md)
- [model-quantization](./model-quantization.md)
- [context-engineering](./context-engineering.md)

## Sources

- [qwen-3-8-27b-dgx-spark-agent-harness](../summaries/qwen-3-8-27b-dgx-spark-agent-harness.md)
