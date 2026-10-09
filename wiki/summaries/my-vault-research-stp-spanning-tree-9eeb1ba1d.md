# STP (Spanning tree) 完整指南

- source: `raw/my-vault/Note/Research/STP (Spanning tree) 完整指南.md`
- source_sha256: `49c829d997cc2ca6310f2783810ab371542fdc608bb2e51ce07e9e2e954084a0`
- source_reviewed: 2026-10-09
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記以 BPDU、Bridge ID、Cost 與 port 選舉建立無迴圈的二層拓樸，並補充傳統 802.1D 的 TCN／TCA／TC 通報、MAC aging 與兩個直接／間接故障案例。

## Source Notes

- Configuration BPDU 在穩定拓樸中仍週期性傳播；TCN 是獨立 BPDU，Type 為 `0x80`，沒有 Root ID、Cost 或 Flags。
- 非 Root 沿 RP 向上送 TCN；上游在 DP 以 Configuration BPDU 的 TCA flag 逐跳確認。Root 以 TC flag 通知下游加速動態 MAC aging。
- Max Age 管理已保存的 STP 資訊年齡；TC 通知縮短動態 MAC aging 至 Forward Delay。兩者影響的資料不同，不能把 Max Age 到期解讀為清空 MAC 表。
- 預設 TC 通知期間為 Max Age + Forward Delay（35 秒），縮短後的 MAC aging 為 15 秒；不是一律立即刪除所有 MAC。
- 間接 BPDU 遺失可能先等舊資訊到期，再經 Listening／Learning；Blocking port 仍能處理 BPDU，控制 BPDU 不必等 Forwarding 才可傳送。
- RSTP 原生 TC 擴散與傳統 TCN 往 Root、TCA 逐跳確認的流程不同；傳統流程假設沒有 UplinkFast／BackboneFast 等加速機制。

## 來源邊界

本次更新依來源內的 2026-09-28 補充與 Cisco 參考連結整理；來源現已補上 ENCOR 350-401 Official Cert Guide 第 2 版第 2 章圖 2-4、2-5。未執行設備實驗，不以圖中簡化文字推導立即清空 MAC 或跳過 STP timer。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)

## 相關概念

- [[network-protocol-layers]]：將二層拓樸、SMTP／IMAP 服務與主機防火牆放回各自的排錯層次。

## 2026-10-09 來源複核

來源已補齊故障圖書籍出處；仍需保留傳統 STP 與 RSTP 流程、TC 通知與 MAC aging 的差異，不把圖示當成設備實測。

目前來源章節：為什麼需要、Spanning-tree standards、STP 802.1D、BPDU (bridge protocol data unit)、Bridge ID、STP Cost、port id、802.1D STP 狀態、802.1D STP Role、802.1D STP 選舉的整體流程、手動計算STP、802.1D 選路範例、case 1、case 2、802.1D timer、802.1D TC(Topology change)機制、TCN、TC、TCA 的差別、逐跳通報與確認，不是一般 Ethernet flooding、Max Age 與 MAC aging 是兩件事、圖一：SW1–SW3 link 仍 up，但 SW3 收不到有效 BPDU、圖二：SW1–SW2 實體斷線，SW3 等待舊資訊失效、與 RSTP 的界線及參考、RSTP 802.1W、802.1W 狀態、802.1W interface Role、802.1W BPDU、Link Type、802.1W Sync、802.1W TC機制、802.1W設定、MST 802.1S (**Multiple Spanning Tree Protocol**)、MST 概觀、MST area、MST內部的STP、MSTP設定、保護STP的topology、防止非預期的BPDU、portfast、RootGuard、BPDUguard、BPDUfilter、防止BPDU意外遺失、LoopGuard、UDLD (Unidirectional link detection)、uplinkfast、BackboneFast、保護STP的topology的指令總結、參考資料。
