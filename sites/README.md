# Walle Blog

Blog 從 `raw/` 的 Markdown 與發佈 metadata 建置，部署至 Cloudflare Pages。

## 來源

- `raw/my-vault/` 保存 My vault 筆記、附件和 Obsidian 設定。資料結構與演算法文章以這裡的 `blog: true`、`blog_title`、`blog_date`、`blog_url` 為準。
- `raw/web-clipper/` 保存網頁剪藏；網址本身不會觸發發佈。
- `wiki/` 是整理後的知識庫，不存放這 40 篇文章的第二份正文。

`sites/sync-vault-posts.py` 在每次建置時掃描 `raw/my-vault/Note/`，轉換 Obsidian 連結和圖片，輸出到被忽略的 `sites/.generated/`。產生檔不需編輯或提交。CI 會執行同一流程，再由 Rust builder 產生網站。

## 本機建置

```bash
python3 sites/sync-vault-posts.py
cd sites/builder-rs
cargo run -p site-builder --release -- --project-dir ../.. --raw-dir ../../raw --out-dir ../../sites/dist/client
```

修改 My vault 後，先執行 `python3 sites/sync-my-vault.py`，再建置網站，最後執行 `python3 sites/stage-blog-sources.py`。後者只將發佈筆記、文章圖片和資料結構與演算法 Dashboard 加入公開 Git 倉庫；完整 My vault 副本仍保存在本機的 `raw/my-vault/`。是否發佈以筆記 metadata 為準。
