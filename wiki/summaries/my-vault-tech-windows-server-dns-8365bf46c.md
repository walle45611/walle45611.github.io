# Windows Server DNS 設定指南

- source: `raw/my-vault/Note/Tech/Windows Server DNS 設定指南.md`
- source_sha256: `f33ffd7fb407340f1e3c4765062810cdc33ac8d95a7fd2d5b2800a76e85131bb`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 建立primary zone、secondary zone、reverse、subdomain 和 delegatin domain、SOA、新增DNS server。

## Source Notes

- 建立子網域，然後將記錄輸入到此子望路內，這些紀錄是儲存在這台DNS Server
- 也可以將子網域的紀錄委派到給其他DNS server來管理，也就是此子網域內的紀錄是除存被委派的DNS伺服器
- serial number 發生異動，serial number增加，secondary server和master server，可以根據雙方的serial number是否新紀錄，以便透過zone transfer，複寫新紀錄到secondary server。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
