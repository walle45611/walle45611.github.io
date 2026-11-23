---
blog: true
blog_title: "EtherChannel 介紹"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/etherchannel-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

## 概觀

EtherChannel 將多條實體連線組成一個邏輯連線，讓 STP 將整個 port-channel 視為一條路徑，而不是把每條平行連線分開阻擋。只要成員連線正常，流量可分散到多條鏈路；若 port-channel 本身被 STP 阻擋，成員鏈路也不會因此全部轉為 forwarding。

## EtherChannel限制

成員埠的設定必須相容。Access 埠需屬於相同 VLAN；Trunk 埠需使用相容的 trunk 設定（包括 native VLAN 與允許 VLAN）。成員埠的速度和 duplex 也需一致；具體限制依平台而異。

## EtherChannel流量分配

交換器會依平台支援的欄位計算雜湊，為每個流量選擇一條成員鏈路。常見欄位包括來源/目的 MAC、IP 或 TCP/UDP port。這是依流量計算的分配方式，不保證每條鏈路的負載平均；雜湊欄位與演算法細節依平台而異。

## EtherChannel load-balance設定

|常見選項|參與雜湊的欄位|
|---|---|
|src-ip、dst-ip、src-dst-ip|來源和/或目的 IP|
|src-mac、dst-mac、src-dst-mac|來源和/或目的 MAC|
|src-port、dst-port、src-dst-port|來源和/或目的 TCP/UDP port（僅部分平台支援）|

命令選項會依交換器型號和軟體版本而異，設定前請查看該設備的 CLI 說明。

- 設定方式
    
    ```Plain
    Switch(config)#port-channel load-balance src-dst-ip
    ```
    
- 查看方式
    
    ```Plain
    SW#show etherchannel load-balance
    ```
    

---

## Etherchannel protocol

如果設定完成後，會有Po這個新的邏輯介面，這個接面做設定會動態的給實體介面。

| |被動，直到要求建立|主動建立通道|是否有協商封包|是否為公開標準|
|---|---|---|---|---|
|LACP|passive|active|yes|yes|
|PAgP|auto|desirable|yes|no cisco 私有|
|on|N/A|N/A|no|N/A|

### PAgP

| |desirable|auto|
|---|---|---|
|Desirable|V|V|
|Auto|V|X|

- command
    
    ```Plain
    Switch(config)#interface range GigabitEthernet1/0/1-4
    Switch(config-if-range)#channel-protocol pagp
    Switch(config-if-range)#channel-group number mode {auto | desirable}
    ```
    
- example
    
    這會將 g1/0/1-4 加入 PAgP channel-group 1，並使用 desirable 模式。
    
    ```Plain
    SW(config)#port-channel load-balance src-dst-port
    SW(config)#interface range gig 1/0/1-4
    SW(config-if-range)#channel-protocol pagp
    SW(config-if-range)#channel-group 1 mode desirable
    ```
    

### LACP IEEE 802.3ad

| |Active|Passive|
|---|---|---|
|Active|V|V|
|Passive|V|X|

LACP system priority 預設為 32768，數值越低代表優先權越高。Cisco 常見 CLI 的 port priority 範圍為 1–65535，預設 32768；平台行為仍須查該機型文件。部分 Catalyst 平台的 LACP channel 最多可設定 16 條相容鏈路，其中最多 8 條同時 active，其餘作為 hot standby；其他平台限制可能不同。

- command
    
    ```Plain
    SW(config)#lacp system-priority priority
    SW(config)#interface range GigabitEthernet1/0/1-4
    SW(config-if-range)#channel-protocol lacp
    SW(config-if-range)#channel-group 1 mode active
    SW(config-if-range)#lacp port-priority 100
    ```
    
- 範例
    
    ```Plain
    SW(config)#lacp system-priority 100
    SW(config)#interface range GigabitEthernet1/0/1-4, GigabitEthernet2/0/1-4
    SW(config-if-range)#channel-protocol lacp
    SW(config-if-range)#channel-group 1 mode active
    SW(config-if-range)#lacp port-priority 100
    SW(config-if-range)#exit
    ```
    

---

## EtherChannel error

若兩端 EtherChannel 成員或設定不一致，可能造成迴圈或轉送異常；EtherChannel misconfiguration guard 可偵測部分此類錯誤，實際支援與預設行為依平台而異。

```Plain
sw(config)#spanning-tree etherchannel guard misconfig
sw(config)#no spanning-tree etherchannel guard misconfig
```

可使用 show interfaces status err-disabled 檢查進入 err-disabled 的埠；排除原因後，在介面設定模式執行 shutdown，再執行 no shutdown 重新啟用。

---

## show etherchannel

|顯示功能| |
|---|---|
|目前etherchannel每個成員port狀態|show etherchannel summary|
|目前etherchannel每個成員port狀態|show etherchannel port|
|etherchannel變更的時間戳記|show etherchannel port-channel|
|使用哪種hash load balance|show etherchannel load-balance|
|每個switch port使用的load balance索引|show etherchannel port-channel|
|每個switch port上 etherChannel neighbor|show { lacp \| pagp } neighbor|
|LACP系統ID|show lacp sys-id|

參考：[Cisco IOS XE EtherChannel guide](https://www.cisco.com/c/en/us/td/docs/switches/lan/c9000/lyr2-fwd/etherchannel/etherchannel-configuration-guide/etherchannels.html)。
