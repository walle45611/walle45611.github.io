# DHCP (Dynamic Host Configuration Protocol)

- source: `raw/my-vault/Note/Research/DHCP (Dynamic Host Configuration Protocol).md`
- source_sha256: `e019fbe8f41a7a61eb5e93e680ff53c2983a092fbf10110d2a0c51ab1498a2f6`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 DHCPv4、可以使用的設備、原理、DHCPv4 Relay、Automatic Private IP address (APIPA)、DHCPv6。

## Source Notes

- 用意 : 自動分配IP address，gateway，dns，網路中無盤系統提供引導加載，bootP是DHCP的前身所以在cisco上有的時候設定bootPC就是dhcp client，bootPS就是dhcp server
- 可以使DHCP Server在不同往段分配IP給需要IP的網段
- 當windows client無法從DHCP server租用到IP會自動把IP變成169.254.0.0/16的IP位址，並使用這個P位址更其他主機溝通，會先廣播詢問其他台主機有沒有使用這個IP位址，會5min來尋找一次DHCP

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
