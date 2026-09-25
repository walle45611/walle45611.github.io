# Linux OpenSSH 設定指南

- source: `raw/my-vault/Note/Tech/Linux OpenSSH 設定指南.md`
- source_sha256: `4acc079f4e23e1c0dd324a0031d0da3849ca8ba019998d4c96b4eeb3565aea41`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 安裝 OpenSSH Server、設定檔位置、限制登入帳號與來源、金鑰登入設定（停用密碼登入）、更改預設 SSH Port（預設為 22）、防止暴力破解（搭配 Fail2ban）。

## Source Notes

- OpenSSH 是最常見的遠端連線協定之一，提供安全的 SSH 連線、SCP、SFTP 等功能。以下整理常見的 OpenSSH 設定方式，適用於 Linux 系統。
- 或手動將 ~/.ssh/id_rsa.pub 內容寫入伺服器的 ~/.ssh/authorized_keys
- 建立設定檔 /etc/fail2ban/jail.local

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
