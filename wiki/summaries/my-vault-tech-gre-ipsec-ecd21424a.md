# GRE 與 IPSec 隧道技術指南

- source: `raw/my-vault/Note/Tech/GRE 與 IPSec 隧道技術指南.md`
- source_sha256: `016d18eeabc38bf12044ae6b4a3c1ccd42a3b71399b92735b91936cdba06d8c9`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 GRE Over IPSec、理論概念、網路拓撲、設定步驟、基本 IP 設定、IPSec 設定。

## Source Notes

- GRE Over IPSec 是把 IPSec 在最外面，意思就是在 R1 和 R2 建立 IPSec tunnel，把 GRE tunnel 加密。Routing Protocol 在 GRE Tunnel 裡面完成 Route 交換，最後 Data 在 GRE Tunnel 裡面傳送，所以 Routing Protocol 和 Data 都會被加密。
- 把 GRE 放在最外面，R1 與 R2 建立 GRE Tunnel，在 GRE Tunnel 裡面建立 IPSec。由於 IPSec 沒辦法支援組播，因此通常把 Routing Protocol 建立在 GRE tunnel 進行交換，沒有加密，只有在 Data 有經過 IPSec tunnel 加密。如堅持把 Routing 放入到 IPSec tunnel 中可以使用單播建立鄰居，如果這樣設定會很痛苦，因為要自己設定鄰居。
- IPSec interesting traffic

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
