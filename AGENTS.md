# 知識庫與 Blog 維護

- `raw/my-vault/`：My vault 筆記、附件與 Dashboard 的來源副本。
- `raw/web-clipper/`：網頁剪藏；`wiki/`：整理後的摘要、概念、索引與紀錄。
- `sites/`：Blog 工具，操作見 [sites/README.md](sites/README.md)。`.build/` 與根目錄 `.obsidian/` 不提交 Git。

## 工作原則

1. 先讀 `wiki/rules/router-rules.md`，按任務讀必要規則；回覆遵循 `wiki/rules/output-rules.md`，紀錄依 `wiki/rules/log-rules.md` 追加。
2. `raw/` 預設唯讀；使用者要求或同意新增、修改、匯入、重新分類或發佈筆記，即授權處理相關來源與必要連結，不需重複確認。
3. 依目前 vault 的實際路徑定位筆記。若有獨立的原始 My vault，優先修改原始來源；保留出處，避免來源與副本分歧。
4. 優先整合既有頁面；新建 wiki 檔名使用小寫連字號，每份來源最多一份摘要。

## Blog 與工具

- Blog 僅發佈使用者選定的 `raw/my-vault/Note/` 下 `Research/`、`Tech/` 筆記，須有 `blog: true` 及有效的 `blog_title`、`blog_date`、`blog_url`；連結到的筆記不連帶發佈。
- 文章以筆記來源為準；`wiki/` 不作為文章來源，`.build/site/` 是產物，不直接編輯。
- `sync-vault`：僅在授權同步時執行；會覆寫、刪除同步副本內容並更新外掛與 snippets，先核對來源與目標。
- `archive`：更新 My vault 摘要與索引；唯讀檢查用 `archive --check`，Web Clipper 摘要依攝取規則整理。
- `stage-assets`：建置後執行，暫存引用附件並取消追蹤未引用附件（保留本機檔案）；完成後檢查暫存差異。
