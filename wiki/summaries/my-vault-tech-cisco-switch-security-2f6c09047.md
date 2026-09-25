# Cisco Switch Security 設定

- source: `raw/my-vault/Note/Tech/Cisco Switch Security 設定.md`
- source_sha256: `588fdfed2a89aa703366fec8386ac07eea57a24aee4aced276a72a456a9859a1`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 Port Protect。

## Source Notes

- 在某些情況呢，server不需搬動的，可以使用mac位址的對應去鎖定port，如果是移動式server可以使用switch動態的去學習mac address，將其加入對應port的列表。
- 指定一組允許的mac address，可以手動設定位址，或是根據交換機的port動態學習到位址，以下指令是指定最大mac address的數目
- 預設上每個使用port-secutity可動態的學到MAC address，並期望這些port可以出現在該介面。當主機發送data到switch介面上時就會主動的去學習，主動學習的數目不超過設定的maximum，如果相連的主機沒有動靜，mac address可能會到期的關係，而從ARP表中刪除，預設情況不刪除

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
