# Walle Blog 與知識庫工具

`sites/` 保存 Rust 網站工具、樣式與 favicon。文章由 `raw/my-vault/` 的筆記產生；`wiki/` 保存摘要與索引，不參與 Blog 文章建置。建置暫存與輸出集中於專案根目錄的 `.build/`，不提交 Git。

## 目錄與資料流

```text
專案根目錄/
├── raw/
│   ├── my-vault/              # My vault 同步副本
│   └── web-clipper/           # 原始網頁剪藏
├── wiki/                     # 摘要、索引、概念、規則與紀錄
├── sites/
│   ├── builder-rs/
│   │   ├── Cargo.toml        # Rust workspace
│   │   └── crates/
│   │       ├── site-builder/ # 建站、同步、歸檔、附件整理
│   │       └── renderer-wasm/# Markdown 渲染 crate
│   ├── styles.css
│   └── public/favicon.svg
├── .build/
│   ├── cargo-target/         # Rust 編譯產物
│   └── site/
│       ├── vault-posts/      # 轉換後的暫存文章
│       ├── vault-assets/     # 文章引用的附件副本
│       ├── content-manifest.json
│       └── client/           # 可部署的靜態網站
└── .github/workflows/pages.yml
```

My vault 經授權同步至 `raw/my-vault/`；建站讀取副本中的發佈 metadata，轉換 Obsidian 連結與附件，再輸出靜態網站。知識整理則由 `raw/` 寫入 `wiki/`，與建站分開進行。

## 文章來源與 metadata

專案發佈規則限定 `raw/my-vault/Note/Research/` 與 `raw/my-vault/Note/Tech/`，以 My vault 原文為準，只發佈使用者選定的文章。筆記 frontmatter 範例：

```yaml
---
blog: true
blog_title: 範例文章
blog_date: '2026-09-29'
blog_url: https://blog.walle4561.com/articles/posts/example-article/
---
```

`blog_url` 必須符合上述網址形式，slug 不可重複；`blog_date` 使用 `YYYY-MM-DD`。選填的 `blog_topic` 支援 `data-structures`、`algorithms`、`problem-solving`、`operating-systems`、`networking`。帶有 `網路與作業系統` tag 的文章依「作業系統 Overview」連結分為 OS，其餘歸入 Networking，在 OS & Networking 頁面分區呈現；也可用 `blog_topic` 明確指定分類。未發佈筆記的內部連結會轉為文字，避免產生不存在的文章網址。

目前匯出器實際掃描整個 `raw/my-vault/Note/`，建站器也保留 `raw/web-clipper/` 發佈 metadata 的相容入口；這不改變本專案限定的發佈範圍。同 slug 時，My vault 文章優先。建站還會讀取 `raw/my-vault/00_Dashboard/資料結構和演算法 Overview.md`，該檔案及其中的演算法區段需保留。

## 本機建置

需有 Rust／Cargo。以下命令皆從專案根目錄執行；首次建置需能取得 Cargo 相依套件。

```bash
export CARGO_TARGET_DIR="$PWD/.build/cargo-target"
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- --project-dir "$PWD"
```

成品位於 `.build/site/client/`，來源與雜湊清單位於 `.build/site/content-manifest.json`。建站會重新建立網站輸出目錄，不要在其中手動維護內容。一般建站不需要先同步、歸檔或整理 Git 附件。

## 來源同步、歸檔與附件

以下命令同樣從專案根目錄執行，沿用上述 `CARGO_TARGET_DIR`。

### 同步 My vault

只在使用者已授權同步來源時執行：

```bash
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- sync-vault
# 使用其他來源 Vault：
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- sync-vault --source "/absolute/path/to/My vault"
```

預設來源為 `$HOME/Library/Mobile Documents/iCloud~md~obsidian/Documents/My vault`。這是鏡像同步：會覆寫 `raw/my-vault/` 並刪除來源已不存在的檔案；也會複製外掛、snippets 並合併根目錄 Obsidian 的啟用外掛清單。

`.gitignore` 排除本機工作區、垃圾桶、上傳金鑰設定及本機代理設定；Git 排除不代表同步時不會複製這些檔案。

### 檢查與更新摘要

```bash
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- archive --check
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- archive
```

`archive --check` 唯讀檢查 My vault 筆記、Dashboard 與 Web Clipper 的摘要涵蓋率，並比對帶有 `source_sha256` 的既有摘要。缺漏或來源變更時會回報失敗。

`archive` 補建缺少的 My vault 摘要並更新索引，不會自動重寫全部既有摘要。`archive --refresh-generated` 可重建工具產生的 My vault 摘要；人工摘要仍需人工整理。缺少摘要的 Web Clipper 來源需先按 `wiki/rules/ingest-rules.md` 整理，否則歸檔會報錯。wiki 修改與工作紀錄遵循根目錄 `AGENTS.md` 及 `wiki/rules/`。

### 整理發佈附件

完成建置後，才執行：

```bash
cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- stage-assets
```

`stage-assets` 讀取最近建置的 `content-manifest.json`，核對來源雜湊，將文章 Obsidian 嵌入所引用的附件加入 Git 暫存區；來源變更後需重新建置。它也會將已追蹤但不再引用的 `raw/my-vault/Assets/` 檔案移出 Git 索引，保留本機檔案。執行後檢查暫存差異；筆記與 wiki 摘要需依任務範圍另行提交。

## 檢查與部署

Rust 程式碼變更可從根目錄執行：

```bash
cargo test --manifest-path sites/builder-rs/Cargo.toml --workspace
cargo fmt --manifest-path sites/builder-rs/Cargo.toml --all -- --check
cargo clippy --manifest-path sites/builder-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings
```

`.github/workflows/pages.yml` 在推送至 `main` 時執行：安裝 stable Rust 與 `wasm32-unknown-unknown`、建置 workspace 和 WASM renderer、產生網站、執行 tests／fmt／clippy，通過後將 `.build/site/client/` 部署至 Cloudflare Pages 專案 `walle45611-github-io`。部署使用 `CLOUDFLARE_API_TOKEN` 與 `CLOUDFLARE_ACCOUNT_ID` secrets；對外網址為 https://blog.walle4561.com。

CI 不會同步本機 My vault 或自動加入被忽略的附件。發佈前需將選定筆記及其必要附件納入提交，避免遠端建置缺少檔案。
