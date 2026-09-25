# Windows Server SSTP

- source: `raw/my-vault/Note/Tech/Windows Server SSTP.md`
- source_sha256: `59bfb7ec30577dc8a42192c447d4f5d30d6774a04efd564d196bf086a98f8b11`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

在DC上將CRL打開因為SSTP會自動下載CRL去比對vpn server憑證是否正確

## Source Notes

- 將192.168.10.1 nat 到 192.0.0.1這樣外網的人才可以下載到CRL
- 因為沒有模擬dns所以需要再hosts file動點手腳因為SSTP會檢查common name，所以需要這樣做

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
