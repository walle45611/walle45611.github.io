# Walle Blog

Blog 從 `raw/` 的 Markdown 與發佈 metadata 建置，部署至 Cloudflare Pages。

## 來源

- `raw/my-vault/` 保存 My vault 筆記、附件和 Obsidian 設定。資料結構與演算法文章以這裡的 `blog: true`、`blog_title`、`blog_date`、`blog_url` 為準。
- `raw/web-clipper/` 保存網頁剪藏；網址本身不會觸發發佈。
- `wiki/` 是整理後的知識庫，不存放這 40 篇文章的第二份正文。

`sites/sync-vault-posts.py` 在每次建置時掃描 `raw/my-vault/Note/`，轉換 Obsidian 連結和圖片，輸出到被忽略的 `sites/.generated/`。產生檔不需編輯或提交。CI 會執行同一流程，再由 Rust builder 產生網站。

`blog_topic: problem-solving` 讓刷題文章列在網站主題頁的「刷題」區。Obsidian 的 `刷題 Overview` 按演算法題型與 SQL 題型導覽；`SQL Overview` 則按資料庫與 SQL 語句導覽。

## 本機建置

```bash
python3 sites/sync-vault-posts.py
cd sites/builder-rs
cargo run -p site-builder --release -- --project-dir ../.. --raw-dir ../../raw --out-dir ../../sites/dist/client
```

修改 My vault 後，先執行 `python3 sites/sync-my-vault.py`，再執行 `python3 sites/archive-vault-notes.py` 更新缺少的 wiki 摘要及歸檔索引，接著建置網站並執行 `python3 sites/stage-blog-sources.py` 核對及加入 Blog 實際使用的附件。`sites/archive-vault-notes.py --check` 可檢查是否有來源尚未歸檔，或既有自動摘要所對應的來源是否已變更。My vault 的筆記、設定與 wiki 摘要納入公開 Git；`Assets/` 僅追蹤 Blog 文章引用的檔案。圖片上傳插件的金鑰設定、裝置工作區狀態、暫存垃圾桶及本機代理設定不提交。是否發佈為網站文章仍以筆記的 `blog: true` metadata 為準。
