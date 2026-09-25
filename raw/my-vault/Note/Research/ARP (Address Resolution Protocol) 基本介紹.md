## ARP 概述

- 當一台主機把ethernet data frame送到位於同意局域網路上的另ㄧ台主機時，根據48bit的mac address確定送到哪個接口。設備driver從不檢查IP數據
- 地址解析位者兩種不同的地址，也就是mac和IP address去做對應關西 : 32 bit 的 IP地址和data link layer使用的任何類型的地址 (Frame relay DLC)
- ARP的目的其實就是用mac address去找IP位址
- 在還每有找到mac address之前都是ARP的封包不會有IP的封包

---

## ARP流程

假定ARP cache table沒有東西

1. 192.168.1.5要向192.168.1.6發送訊息
2. 發現沒有mac address的內容，所以先廣播FF:FF:FF:FF:FF:FF
3. 192.168.1.6就會發現是自己的該收的封包，並以自己的source mac address和對方dst mac address單播回去
4. 192.168.1.5就會記錄在自己的ARP cache table

- 後者優先

---

## ARP packet

![[Assets/Note/Research/ARP (Address Resolution Protocol) 基本介紹/01-ARP packet.png|01-ARP packet.png]]

- HW type : 他的值為1。表示ethernet，表示協議類型表示映射的協議地址類型。他的值為0x0800表示IP
- Protocol : 他表示0x0800
- Oper : 為1的時候是ARP請求，如果是2 ARP回覆

---

## Porxy ARP

![[Assets/Note/Research/ARP (Address Resolution Protocol) 基本介紹/02-Porxy ARP.png|02-Porxy ARP.png]]

- R1現在沒有路由能力，所以只能透過ARP，但是有沒有辦法通過gateway，所以只能向R2請求ARP，所以R2代替R3給了R1，R3的mac address

---

## Free ARP

會自己先問自己，也就是說會自己發起ARP請求看沒有重複的IP位址，也可以說是有沒有相同的IP但卻有兩個mac address來判斷自己有沒有重複設定IP。

---

## ARP command

- windows
    - 查看ARP
        
        ```Plain
        PS C:\Users\walle> arp -a
        
        介面: 192.168.0.1 --- 0xa
          網際網路網址          實體位址               類型
          192.168.0.106         00-0c-29-9d-13-48     動態
          192.168.0.254         00-fc-8d-e9-ec-a2     動態
          192.168.0.255         ff-ff-ff-ff-ff-ff     靜態
          224.0.0.22            01-00-5e-00-00-16     靜態
          224.0.0.251           01-00-5e-00-00-fb     靜態
          224.0.0.252           01-00-5e-00-00-fc     靜態
          239.255.102.18        01-00-5e-7f-66-12     靜態
          239.255.255.250       01-00-5e-7f-ff-fa     靜態
        
        介面: 192.168.40.1 --- 0xe
          網際網路網址          實體位址               類型
          192.168.40.255        ff-ff-ff-ff-ff-ff     靜態
          224.0.0.22            01-00-5e-00-00-16     靜態
          224.0.0.251           01-00-5e-00-00-fb     靜態
          224.0.0.252           01-00-5e-00-00-fc     靜態
          239.255.255.250       01-00-5e-7f-ff-fa     靜態
        
        介面: 169.254.220.111 --- 0xf
          網際網路網址          實體位址               類型
          169.254.255.255       ff-ff-ff-ff-ff-ff     靜態
          224.0.0.22            01-00-5e-00-00-16     靜態
          224.0.0.251           01-00-5e-00-00-fb     靜態
          224.0.0.252           01-00-5e-00-00-fc     靜態
          239.255.255.250       01-00-5e-7f-ff-fa     靜態
          255.255.255.255       ff-ff-ff-ff-ff-ff     靜態
        ```
        
    - ARP刪除
        
        ```Plain
        arp -d
        ```
        
- cisco
    - 查看ARP
        
        ```Plain
        Router#show arp
        Protocol  Address          Age (min)  Hardware Addr   Type   Interface
        Internet  192.168.1.1           144   aabb.cc00.6000  ARPA   Ethernet0/0
        Internet  192.168.1.2             -   aabb.cc00.5000  ARPA   Ethernet0/0
        ```
        
    - 刪除ARP
        
        ```Plain
        Router#clear arp
        Router(config-if)#no sh
        ```