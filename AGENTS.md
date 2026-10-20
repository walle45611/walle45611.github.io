# 筆記與 Blog

- `raw/my-vault/`：直接編輯的筆記與附件；`raw/my-vault/Maps/` 為主題入口；`raw/web-clipper/` 為網頁剪藏。
- `sites/`：Blog 工具；`.build/` 與根目錄 `.obsidian/` 不提交 Git。

## 筆記

- `raw/` 預設唯讀；使用者要求修改、匯入、分類或發佈，即授權處理相關來源與連結，不需重複確認。
- 直接修改本 Vault 的筆記，按實際路徑定位。
- 優先整合既有筆記，用 Dashboard 與內部連結導航；不建立 wiki、重複摘要或例行日誌。
- 保留出處與既有內容；改名或搬移時更新相關連結。保留與任務無關的修改。
- 論文命名為 `@<精簡標題關鍵字><年份>.md`，`id` 與去掉 `@`、`.md` 的檔名一致；保留完整 `title`、`aliases`，年份須有依據。

## Blog

- 僅發佈使用者選定的 `raw/my-vault/Note/Research/`、`Tech/` 筆記，須有 `blog: true` 及有效的 `blog_title`、`blog_date`、`blog_url`；連結筆記不連帶發佈。
- 分類只讀 `blog_tag`（字串或清單）、`blog_topic`（字串），不讀一般 tags 或 Dashboard。
- Blog 筆記只保留必要的 `blog_*` metadata；論文文獻 metadata 保留。
- 圖片優先用本機附件；不直接編輯建置產物。
- `stage-assets` 在建置後執行，完成後檢查暫存差異。
- 筆記修改核對內容與連結；工具修改跑相關測試。發佈後確認實際部署結果。
