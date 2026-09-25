# ARP (Address Resolution Protocol) 基本介紹

- source: `raw/my-vault/Note/Research/ARP (Address Resolution Protocol) 基本介紹.md`
- source_sha256: `13727507b90919f7b987f41088a1afe4a210bcf446b53eb60c4f6ef928ca0fa3`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 ARP 概述、ARP流程、ARP packet、Porxy ARP、Free ARP、ARP command。

## Source Notes

- 當一台主機把ethernet data frame送到位於同意局域網路上的另ㄧ台主機時，根據48bit的mac address確定送到哪個接口。設備driver從不檢查IP數據
- 地址解析位者兩種不同的地址，也就是mac和IP address去做對應關西 : 32 bit 的 IP地址和data link layer使用的任何類型的地址 (Frame relay DLC)
- ARP的目的其實就是用mac address去找IP位址

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
