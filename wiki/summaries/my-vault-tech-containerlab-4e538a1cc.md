# Containerlab 安裝與設定指南

- source: `raw/my-vault/Note/Tech/Containerlab 安裝與設定指南.md`
- source_sha256: `818dc6cf8c4d1346e944ac30c1157cdd8abc8fc71b9ab68f54ff3687c6ccdd7d`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 環境架構、建立並進入 Debian machine、安裝 Docker 與 Containerlab、建立最小測試 Lab、建置 Cisco IOL 映像、部署 Cisco Router 與 Switch。

## Source Notes

- 以 Apple Silicon Mac 搭配 OrbStack 的 Debian ARM64 Linux machine 建立網路實驗環境，先驗證 Containerlab，再加入 Cisco IOL Router 與 Switch。
- 來源：Mac安裝網路模擬器；整理日期：2026-09-20。
- Containerlab 透過 YAML 定義節點與連線，再建立容器及實驗網路。它需要 Linux 的 network namespace、veth、netlink 等功能，因此本流程在 Debian 裡執行 Containerlab 與 Docker。這是 Containerlab macOS 官方指南列出的方式。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
