# Walle Blog

`sites/` 保存 Rust 網站工具、樣式與 favicon。My vault 原文在 `raw/my-vault/`，Web Clipper 在 `raw/web-clipper/`；只有標記 `blog: true` 的 My vault 筆記會成為 Blog 文章。Rust 工具同時負責建站、Vault 同步、筆記歸檔及 Blog 附件挑選。建置暫存與輸出統一放在根目錄的 `.build/`，不提交 Git。

## 本機建置

從專案根目錄執行：

```bash
CARGO_TARGET_DIR="$PWD/.build/cargo-target" cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- --project-dir "$PWD"
```

成品位於 `.build/site/client/`；部署流程使用同一組路徑。

## 更新筆記與附件

```bash
CARGO_TARGET_DIR="$PWD/.build/cargo-target" cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- sync-vault
CARGO_TARGET_DIR="$PWD/.build/cargo-target" cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- archive
CARGO_TARGET_DIR="$PWD/.build/cargo-target" cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- --project-dir "$PWD"
CARGO_TARGET_DIR="$PWD/.build/cargo-target" cargo run --manifest-path sites/builder-rs/Cargo.toml -p site-builder --release -- stage-assets
```

`archive --check` 檢查 My vault 與 Web Clipper 的摘要涵蓋率及已產生摘要的來源版本。`stage-assets` 只加入 Blog 實際引用的 `Assets/`；筆記與 wiki 摘要照一般 Git 流程提交。`sync-vault` 會覆寫 `raw/my-vault/` 既有副本，只在已獲授權同步來源時執行。原始 Vault 中的上傳金鑰、裝置工作區、垃圾桶及本機代理設定由 `.gitignore` 排除。
