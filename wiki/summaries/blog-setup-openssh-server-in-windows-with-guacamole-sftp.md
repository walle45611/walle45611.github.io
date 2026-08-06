# 在 Windows 上設定 OpenSSH Server 和 Guacamole SFTP

- source: `raw/setup-openssh-server-in-windows-with-guacamole-sftp.md`
- original title: 在 Windows 上正確設定 OpenSSH Server 和 Guacamole SFTP
- author: Walle
- published: 2024-08-27
- source_created: 2024-08-27
- ingested_at: 2026-08-03
- type: migrated blog article summary

## Summary

這篇文章記錄 Windows 10/Windows Server 2019 上安裝與檢查 OpenSSH Client/Server、服務與防火牆，再讓 Guacamole 透過 SFTP 提供檔案上傳的排錯過程。

## Key Points

- 先確認內建 OpenSSH 工具，再安裝或啟用 Client、Server 與 `OpenSSH SSH Server` 服務。
- 服務啟動模式、防火牆規則、SSH 連線與 SFTP 寫入測試需要分開驗證。
- Guacamole 的 SFTP 設定本身不一定是唯一問題，網路延遲也可能造成連線失敗。
- 原文保留 Windows 服務與測試命令；實際操作需依目前 Windows 版本重新確認。

## Related Concepts

- [blog-knowledge-migration](../concepts/blog-knowledge-migration.md)
