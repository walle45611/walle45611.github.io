# 知識庫維護

本專案保留原始資料於 `raw/`，並將整理後的知識寫入 `wiki/`。

- `raw/my-vault/`：My vault 的副本，包含筆記、附件與 Obsidian 設定。
- `raw/web-clipper/`：Obsidian Web Clipper 與其他網頁剪藏的原始資料。
- `wiki/`：可更新的摘要、索引、規則及工作紀錄。

每次任務先讀 `wiki/rules/router-rules.md`，再依路由讀必要規則；回覆前讀 `wiki/rules/output-rules.md`。需記錄時按 `wiki/rules/log-rules.md` 追加 `wiki/log.md`。

`raw/` 預設只讀；使用者要求匯入、同步或重新分類時，才修改相關來源。保留來源內容與可追溯路徑。新建的 wiki 筆記使用小寫連字號檔名；每份來源最多對應一份摘要，避免重複歸檔。Rust 工具 `site-builder archive --check` 用於檢查來源涵蓋率及已產生摘要的來源版本。

Blog 每次建置掃描 `raw/` 的發佈 metadata。`raw/my-vault/Note/Research` 與 `Note/Tech` 是 Blog 文章的唯一筆記來源；只有明確標記 `blog: true` 的 My vault 筆記會發佈。網站轉換檔及圖片為暫存產物，不作為另一份筆記維護。
