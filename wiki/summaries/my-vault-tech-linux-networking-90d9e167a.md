# Linux networking 常用指令

- source: `raw/my-vault/Note/Tech/Linux networking 常用指令.md`
- source_sha256: `83b6901752959f401004d50359780671e66e7e9cab8917818a94d4d3bf6dcdf4`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 安裝與用途速查、NetworkManager 與 IP 設定、舊版 RHEL／CentOS 設定方式、連通性、介面與路由：ping、ip、Socket 與連線：ss、TCP／UDP 測試：nc。

## Source Notes

- 集中整理網路設定、連線診斷與封包分析。診斷範例以 Ubuntu／Debian 為主，適用於 Linux VM、Container 與一般主機；NetworkManager、舊版 RHEL／CentOS 設定與 macOS 差異另列。範例中的 eth0、IP、port 請依實際環境替換。
- Linux 是否預裝取決於發行版與映像，精簡 Container 尤其可能缺少工具。先用 command -v ss、command -v tcpdump 等確認。
- Homebrew 的 wireshark formula 提供命令列工具；GUI 是另外的 cask。套件安裝完成不代表已有抓包權限，Linux 可能需要設定 dumpcap 權限，Container 也受 capabilities 限制。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
