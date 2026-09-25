# NAT (Network Address Translation) 介紹

- source: `raw/my-vault/Note/Research/NAT (Network Address Translation) 介紹.md`
- source_sha256: `ec3613ba4de782fca6053a164a3e354df80e48756c374c93aa2f6f9110f81bcf`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 DNAT (dynamic NAT)、SNAT (static NAT)、PAT (Port address translation)、NVI (NAT Virtual interface，NVI)。

## Source Notes

- NAT容許私有地只轉換成外部網路位址(public ip)，這樣可以隱藏IP位址，也可以讓IPv4位址大大的足夠使用。
- 動態的指定內部位址從一組可用的位址裡自動指派內部全域位址 NAT Pool。
- 在全域ip nat pool _pool_name strat_ip ending_ip_ netmask _mask_ 設定nat集群

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
