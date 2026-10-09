# Matt Pocock：解決 PR 審查瓶頸的五個心法

- source: `raw/web-clipper/EP412 - Matt Pocock 怎麼 Code Review？解決海量 PR 的五個心法.md`
- source_sha256: `ca72cd1e8f66cc1f277dfc78723ebcf84cccb92896c216ee31fc50698ff8c6a7`
- source link: https://www.youtube.com/watch?v=T-5FW3KRpgI
- published: 2026-10-02
- source_created: 2026-10-09
- ingested_at: 2026-10-09
- type: source summary

## 摘要與關鍵資訊

這份 AI 懶人報節目逐字稿整理 Matt Pocock 的演講，主張 AI 讓 PR 產生更快後，品質檢查與人的審查能力成為瓶頸；應改善產生程式的系統，避免重複出現同類錯誤。

1. 建立三層檢查：lint／測試／型別檢查、AI review、人工 review。既有程式也是 agent 的工作環境，品質下降會影響後續生成。
2. 避免只重述常數或實作的 tautological tests；測試應經由模組對外介面驗證行為，深模組以較小介面封裝較多實作。
3. 將實作與按規範重構分成兩個 context window；講者建議把詳細 coding standards 放在 review 階段讀取的檔案，並讓 reviewer 對確定問題直接修改與 commit，疑義再提問。
4. 依改動可恢復性與 blast radius 分配人工注意力；寄送大量郵件、不可逆資料損失或昂貴遷移，即使改動很小也需仔細審查。
5. 把重複 review 意見轉成流程改善；機械性問題優先變成會執行的檢查，retro 用來回寫工具與規範。

## 來源邊界與關聯

這是二手節目整理，不是原始演講全文；原始演講連結為 https://www.youtube.com/watch?v=LlgiOCmFG_w 。訂閱數、星數及模型評語僅是節目當時敘述，未另行查證。

「詳細規範移出全域規則檔」補充 [[context-engineering]] 的按需載入方法，並不推翻 [[harness-engineering]] 對必要規則及硬控制的要求。可 revert 程式不代表可撤回外部副作用；測試全通過也不等於需求或整體品質已獲保證。

## 相關概念

- [[harness-engineering]]
- [[ai-coding-tools]]
- [[context-engineering]]
