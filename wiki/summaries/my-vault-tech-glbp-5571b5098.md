# GLBP  熱備份路由協定指南

- source: `raw/my-vault/Note/Tech/GLBP  熱備份路由協定指南.md`
- source_sha256: `1c23b84bca415370ee9bc63ba53ce5c115ef54ebb4d3f9fb5952248f9069f34c`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 GLBP (Gateway Load Balancing Protocol) 概述、GLBP 原理、AVG (active virtual gateway)、AVF(active virtual forwarder)、GLBP Load balance、輪替式 (round robin)。

## Source Notes

- 也是Cisco私有的協定，用來克服現有的備援路由協定的缺點，其中有些概念跟HSRP/VRRP相同，且行為更為動態和完整，但術語有些不同。
- 為了提供虛擬路由器，得到多台交換器(路由器)，分配到同一個GLBP中。群組中所有的路由器都能參與負載平衡，分攤轉送部分的資料流，並非單獨讓active路由器代表虛擬路由來轉送資料。
- ==這種做發優點，用戶端無須指定特定的閘道，所有用戶端的default gateway 都是相同的，這個default gateway就是virtual router IP address ; 原理就是使用回送給用戶端的ARP回應中的虛擬路由mac address，來提供完整的load balance。當用戶端查詢virtual router IP address的ARP請求時，GLBP回傳一個ARP回應，其中包含從群組中選擇的路由器virtual mac…

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
