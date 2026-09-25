# Linux OpenVPN 完整設定與憑證管理指南

- source: `raw/my-vault/Note/Tech/Linux OpenVPN 完整設定與憑證管理指南.md`
- source_sha256: `dfd9aa2468b2fc2d8afb902dbb395bfe0de2276a2ecdf165dcf4184f4953640e`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 📥 套件安裝與 easy-rsa 初始化、🔐 建立伺服器與用戶端憑證、🗂️ 檔案複製與伺服器設定、🚀 啟用並啟動 OpenVPN Server、🔁 OpenVPN tunnel PSK 模式（Site to Site）、📌 site1 /etc/openvpn/s2s.conf。

## Source Notes

- 請見附錄完整 server.conf 範例內容（已內嵌於畫布中）
- server 10.8.0.0 255.255.255.0
- key private/server1.key # This file should be kept secret

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
