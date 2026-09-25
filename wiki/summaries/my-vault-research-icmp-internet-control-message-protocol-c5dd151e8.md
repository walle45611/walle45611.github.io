# ICMP(Internet Control Message Protocol) 基本介紹

- source: `raw/my-vault/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹.md`
- source_sha256: `db17d39e6bd808aaaf0a7b4cf4e6b0640b3a235bbc3dc9c6042e265dfddccf11`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 ICMP概述、ICMP header、ICMP type、icmp unreachable packet。

## Source Notes

- 例如，一個剛搭建好的網路，驗證網路設定是否正確以外，為了確保網路能夠按照域其正常工作，一旦遇到什麼問題需要立即制止問題的蔓延。為了減輕administrtor的負擔。
- ICMP主要的功能確定是否能成功送往目的地，通知過程當中IP封包被丟棄的具體訊息，改上網路設定等等。有了這些訊息，就可以獲得網路是否正常、設定是否又誤以及設備有何異常等等訊息，以便檢查網路
- ICMP總共是8byte，ICMP前面4byte一定是固定的，也就是說type code checksum都是固定會有的，但是剩下的4byte不一定會有

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
