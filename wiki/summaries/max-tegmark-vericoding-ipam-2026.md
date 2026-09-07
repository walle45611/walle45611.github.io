# EP1/23｜別部署 AI，部署它寫的程式：《Life 3.0》作者 Max Tegmark 的 vericoding（IPAM 2026）

- source: `raw/EP123｜別部署 AI，部署它寫的程式：《Life 3.0》作者 Max Tegmark 的 vericoding（IPAM 2026）.md`
- source link: https://www.youtube.com/watch?v=igWS03uN4Nk
- original title: EP1/23｜別部署 AI，部署它寫的程式：《Life 3.0》作者 Max Tegmark 的 vericoding（IPAM 2026）
- author: AI 101
- published: 2026-09-06
- source_created: 2026-09-06
- ingested_at: 2026-09-06
- type: 影片簡介與章節（二手介紹）
- original talk link: https://www.youtube.com/watch?v=nRrt7AczYV4

## 摘要

這份 AI 101 影片簡介將 Max Tegmark 在 IPAM 的方向概括為：讓 AI 產生程式與證明，使用人類定義的形式規格和可檢查的驗證器把關，再部署驗證過的產物。這是 vericoding 的工作流主張；本地素材不足以還原原演講的完整技術方案。

## 關鍵主張與數據邊界

- **可解釋性的示例**：簡介提到加法小網路形成圓形表示，以及 Llama 2 第 53 層具有世界地圖式表示。這是二手介紹，未附實驗設定或原始研究，不能據此推論一般模型已被完整理解。
- **規模限制**：簡介轉述玩具問題約一半成功率，以及對 GPT 規模缺乏明確路徑；未交代成功的分母與評估方法，保留為來源說法。
- **Vericoding 流程（章節 08:19、10:10）**：人類以形式語言定義規格，AI 產生程式和證明，再交由驗證器檢查。重點是把信任落在規格及檢查機制。
- **「300 行」與成本**：簡介以約 300 行的小型軟體描述 proof checker，章節提到另一條路貴 10–100 倍；沒有提供實作、計算基準或成本條件，不能當作通用保證。

## 來源限制與知識庫關聯

本地 raw 只有影片簡介和章節，沒有逐字稿。其記載原演講於 2026-09-01 舉行、約 51 分鐘；本次未觀看或獨立查證原演講。檔名寫 EP123，frontmatter 標題寫 EP1/23，保留兩者原樣，不自行推定集數。

此來源與 [[harness-engineering]] 的生成—驗證閉環一致。跨來源整合見 [[formal-verification-for-ai-generated-software]]：徐天音訪談補上需求、規格與故障模型的限制；驗證成功的範圍不能超出規格與驗證方法涵蓋的條件。
