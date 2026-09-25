# Window Server Active Directory Certificate Services

- source: `raw/my-vault/Note/Tech/Window Server Active Directory Certificate Services.md`
- source_sha256: `19f0e0209b248854bf93d9d8c9c9622b11b34b2e204c6c85b07d8780b29bdee4`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 種類概述、安裝、申請證書。

## Source Notes

- 企業CA又分為企業CA與企業次CA，他需要Active Directory Domain Services，可以將企業CA安裝到DC或成員server。發放憑證的對象僅限網域使用者，當網域使用者來申請certificate，企業CA會從Active Directory來得知該使用者帳戶資訊，並據以決定是否有權利來申請憑證。
- 企業次級CA須向其父系CA(獨立根CA或是企業CA)取得憑證後才正常運作。企業次級CA也可以發放給下層使用
- 獨立根與獨立次級CA，他不需要Active Directory網域。他可以是獨立server、成員server或DC。無論是否使用網域使用者，都可以向獨立CA申請。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
