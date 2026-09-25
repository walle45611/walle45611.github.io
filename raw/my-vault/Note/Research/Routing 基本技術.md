
## 路由的功能

- 路由中的routing就是拿來轉送資料到正確的區域網路區域中

![[Assets/Note/Research/Routing 基本技術/01-路由的功能.png|01-路由的功能.png]]

## 路由封裝

假設現在 PC1 傳送資料到 PC2，會經過 R1。在 PC1 傳到 R1 的時候，是以 PC1 的 MAC address 傳遞的。當 R1 要將資料傳給 PC2 的時候，會將 MAC address 換成自己的 MAC address。

---

## 路由表

```Plain
Router#sh ip router

路由種類代號
Codes: C - connected, S - static, R - RIP, M - mobile, B - BGP
       D - EIGRP, EX - EIGRP external, O - OSPF, IA - OSPF inter area
       N1 - OSPF NSSA external type 1, N2 - OSPF NSSA external type 2
       E1 - OSPF external type 1, E2 - OSPF external type 2
       i - IS-IS, su - IS-IS summary, L1 - IS-IS level-1, L2 - IS-IS level-2
       ia - IS-IS inter area, * - candidate default, U - per-user static route
       o - ODR, P - periodic downloaded static route

路由訊息
C    192.168.20.0/24 is directly connected, FastEthernet0/1
```

![[Assets/Note/Research/Routing 基本技術/02-路由表.png|02-路由表.png]]

---

## 路由表刪除新增事件

```Plain
Router#debug ip routing
```

```Plain
Router(config)#int f0/1
Router(config-if)#shutdown
```

- 刪除事件
    
    ```Plain
    *May 31 14:22:13.699: RT: is_up: FastEthernet0/1 0 state: 6 sub state: 1 line: 1 has_route: True
    *May 31 14:22:13.699: RT: interface FastEthernet0/1 removed from routing table
    *May 31 14:22:13.703: RT: del 192.168.20.0 via 0.0.0.0, connected metric [0/0]
    *May 31 14:22:13.703: RT: delete network route to 192.168.20.0
    *May 31 14:22:13.703: RT: NET-RED 192.168.20.0/24
    R1(config-if)#
    *May 31 14:22:15.687: %LINK-5-CHANGED: Interface FastEthernet0/1, changed state to administratively down
    R1(config-if)#
    *May 31 14:22:15.691: RT: is_up: FastEthernet0/1 0 state: 6 sub state: 1 line: 1 has_route: False
    *May 31 14:22:16.687: %LINEPROTO-5-UPDOWN: Line protocol on Interface FastEthernet0/1, changed state to down
    R1(config-if)#
    *May 31 14:22:16.691: RT: is_up: FastEthernet0/1 0 state: 6 sub state: 1 line: 1 has_route: False
    ```
    
- 新增事件
    
    ```Plain
    *May 31 14:24:46.039: RT: is_up: FastEthernet0/1 1 state: 4 sub state: 1 line: 1 has_route: False
    *May 31 14:24:46.043: RT: add 192.168.20.0/24 via 0.0.0.0, connected metric [0/0]
    *May 31 14:24:46.043: RT: NET-RED 192.168.20.0/24
    *May 31 14:24:46.047: RT: interface FastEthernet0/1 added to routing table
    R1(config-if)#
    *May 31 14:24:48.015: %LINK-3-UPDOWN: Interface FastEthernet0/1, changed state to up
    R1(config-if)#
    *May 31 14:24:48.019: RT: is_up: FastEthernet0/1 1 state: 4 sub state: 1 line: 1 has_route: True
    *May 31 14:24:49.015: %LINEPROTO-5-UPDOWN: Line protocol on Interface FastEthernet0/1, changed state to up
    R1(config-if)#
    *May 31 14:24:49.019: RT: is_up: FastEthernet0/1 1 state: 4 sub state: 1 line: 1 has_route: True
    ```
    

---

## 路由選擇原理

### 路由訊息的來源

- 直連路由
    - 接口配置的IP，該接口的物理層和數據鏈路層UP
    - 通過接口感知道的直連網路
- 靜態路由
    - 使用靜態路由命令手工配置的路由
- 動態路由
    - 通過路由協定學習的路由
    - 常見的動態路由協定 : RIP、OSPF 、IS-IS、EIGRP、BGP

---

### 管理距離(AD、Administrator Distance值)

只有去往同一個網路的路由訊息才會對比AD值

- IPv4 AD 全部都要記住
    
|   |   |
|---|---|
|直接連接|0|
|靜態路由 Metric=0|1|
|EIGRP Smmary Router|5|
|外部BGP|20|
|EIGRP|90|
|IGRP|100|
|OSPF|110|
|IS-IS|115|
|RIPv1,RIPv2|120|
|EGP(Exterior Gateway Protocol)|140|
|ODR(On Demand Routing)|160|
|External EIGRP|170|
|internal BGP|200|
|DHCP-learned|254|
|未知的方式|255|
    
- IPv6 AD 全部都要記住
    
|   |   |
|---|---|
|直連路徑|0|
|static|1|
|NDP|2|
|EIGRPv6|90|
|OSPFv3|110|
|RIPng|120|
|未知的方式|255|
    

---

### 有類及無類路由查詢

- 無類的路由查詢(ip classless)：
    
    - 最長前綴匹配
        
        ![[Assets/Note/Research/Routing 基本技術/03-有類及無類路由查詢 - 最長前綴匹配.png|03-有類及無類路由查詢 - 最長前綴匹配.png]]
        
        - 路由協定 : OSPF 、EIGRP 、IS-IS、BGP、RIPv2
        - 路由器不會在意目的地位置的類別，會在目的地位置和已知的路由之間逐位(bit by bit)執行最長匹配
    
    - example : 
        
        ```Plain
        假設有一IP : 192.168.20.19需要通過路由器
        
        routing table :
        192.168.20.16/28
        192.168.0.0/16
        
        會發現兩個都可以，所以就需要"最長前綴匹配"原則也就是說
        精準的先走，所以就是192.168.20.16/28這個路由會被選中
        ```
        
    - command
        
        ```Plain
        ip classless
        ```
        
- 有類路由查詢(no ip classless)：
    
    - 路由協定 : RIPv1 、 IGP
    - 路由器收到一個封包時，會先查找目的地位置所屬的主類，如果路由表中有主類路由，則再去爪子網，如果子網路由，則查詢被限定在子網中，並進一步查找，如果最終查找失敗則丟棄封包，即使有默認路由存在;如果本地沒有該主路由，則看是否有默認路由，如果有責轉發，沒有就丟棄，==基本不用了==
    - 流程
        
        ![[Assets/Note/Research/Routing 基本技術/04-有類及無類路由查詢 - 流程.png|04-有類及無類路由查詢 - 流程.png]]
        
    
    - example 1
        
        ![[Assets/Note/Research/Routing 基本技術/05-有類及無類路由查詢 - example 1.png|05-有類及無類路由查詢 - example 1.png]]
        
        routing table
        
|Destination Network|Exit interface|
|---|---|
|192.168.1.0/24|Fa0/0|
|192.168.1.0/27|Fa1/0|
        
        - 解說
            1. 首先會先去找是否有主類Routing table
            2. 有主類192.168.1.0/24
            3. 查找子網192.168.1.0/27
            4. 從F1/0出去
    - example 2
        
        ![[Assets/Note/Research/Routing 基本技術/06-有類及無類路由查詢 - example 2.png|06-有類及無類路由查詢 - example 2.png]]
        
        routing table
        
|Destination Network|Exit interface|
|---|---|
|192.168.1.0/24|Fa0/0|
|192.168.1.0/27|Fa1/0|
        
        - 解說
            1. 有主類192.168.1.0/24
            2. 查找子網沒有包含在192.168.1.0/27裡面
            3. 丟棄
    
    - command
        
        ```Plain
        no ip classless
        no ip cef
        ```
        
- 分辨有無類路由 :
    
    > [!important] ==簡單來說查看路由更新訊息中有沒有遮罩，如果有遮罩就是無類的，如果沒有就是有類的==
    

---

### 路由表的查找

- 不同的前綴，在路由表中屬於不同的路由
- 相同的前綴，先比AD值，後比metric
- 匹配，轉發;不匹配，丟棄
- 路由器的行為是逐跳得，到目標網路的沿路徑每個路由器都必須要有關於目標的路由
- Data是雙向的，所以要考慮流量的時候，需要注意流量的往返

---

### 遞迴查詢

![[Assets/Note/Research/Routing 基本技術/07-遞迴查詢.png|07-遞迴查詢.png]]

|number|路由協定|Destination Network|Next-hop|
|---|---|---|---|
|1|S|192.168.30.0|192.168.23.3|
|2|S|192.168.23.0|192.168.12.2|
|3|C|192.168.12.0|Serial0/0|

==查找順序 1 → 2 → 3，再第三個路由表中找到出接口這個過程就是遞迴查詢==

  

---

## Routing loop

### 水平分割(split Horizon)

水平分割的功能用於防止從某個介面學習的路徑再經由相同介面通告回去

### 路徑毒害(route poisoning)

逆向毒害的功能促使從某個介面學習到的路徑以帶無限大的權值由該介面通告回去

![[Assets/Note/Research/Routing 基本技術/08-路徑毒害(route poisoning).png|08-路徑毒害(route poisoning).png]]

---

## 路由認證

- 許多路由協定支持認證，使得路由器驗證接手到每個路由的封包

- 明文認證
    - 路由器發送封包和密鑰
    - 鄰居檢查密鑰是否相同
    - 安全性很低

- MD5 authentication
    - 設定密鑰和密鑰ID
    - 路由器產生一個訊息摘要
    - 路由器發送封包和摘要訊息，密鑰不發送
    - 安全性高

- 支持明文認證的路由協定
    - IS-IS
    - OSPF
    - RIPv2

- 支持密文的路由協定
    - OSPF
    - RIPv2
    - BGP
    - EIGRP

---

## IPv4 routing

### 默認路由static default router

當查找所有路由表之後找不到路由訊息，就會走默認路由這條路徑

```Plain
R1(config)#ip route 0.0.0.0 0.0.0.0 <Gateway IP address>
```

### static routing config

- 使用指定next hop的靜態路由
    
    ```Plain
    ip route 192.168.10.0 255.255.255.0 192.168.1.1
    ```
    
    ==廣播型的網路通常建議使用此方法，Ethernet就是常見的廣播型==
    
- 使用指定出接口的靜態路由
    
    ```Plain
    ip route 192.168.10.0 255.255.255.0 fa0/0
    ```
    
    ==使用場景通常是在p2p的網路環境中例如pppoe，如果使用此方法進行在廣播型網路ARP的泛洪，可能會造成下個節點的負擔==
    
    - APR的影響
        
        如果把代理ARP關閉就會造成錯誤，因為泛洪時拿不到目標mac address所以造成路由錯誤
        
- 查看路由表
    
    ```Plain
    R1#sh ip route
    ```
    
    ![[Assets/Note/Research/Routing 基本技術/09-static routing config - 查看路由表.png|09-static routing config - 查看路由表.png]]
    

### case 1

![[Assets/Note/Research/Routing 基本技術/10-case 1.png|10-case 1.png]]

一般推薦的寫法(這樣比較清楚)

```Plain
路由規則
R1(config)#ip route 192.168.1.0 255.255.255.0 192.168.254.1
R1(config)#ip route 192.168.2.0 255.255.255.0 192.168.254.1
R1(config)#ip route 192.168.3.0 255.255.255.0 192.168.254.1
IOU1(config)#ip route 0.0.0.0 0.0.0.0 192.168.254.2
```

不推薦路由匯總的寫法(不清楚哪裡到哪裡)

```Plain
ip route 192.168.0.0 255.255.0.0 192.168.254.1
ip route 0.0.0.0 0.0.0.0 192.168.254.2
```

### 浮動靜態路由

- 通過修改AD值可以實現冗於路由

![[Assets/Note/Research/Routing 基本技術/11-浮動靜態路由 - 通過修改AD值可以實現冗於路由.png|11-浮動靜態路由 - 通過修改AD值可以實現冗於路由.png]]

```Plain
假設台哥大壞掉了就可以走中華電
但是這種也是有缺陷只有
f1/1 down才會走f1/0這種在高附載的狀況沒辦法達到附載均衡

R1(config)#int f1/1
R1(config-if)#ip route 0.0.0.0 0.0.0.0

R1(config)#int f1/0
R1(config-if)#ip route 0.0.0.0 0.0.0.0 10
```

### dynamic routing protocols

### IGP

|特性|RIPv2|EIGRP|OSPFv2|
|---|---|---|---|
|計算metric的方式|hop|bandwidth|cost|
|定期傳送完整更新|yes|no|no|
|定期傳送hello|no|yes|yes|
|針對無效路徑使用route poisoning|yes|yes|yes|
|利用split Horizon去限制關於有效路徑之更新|yes|yes|no|
|組播地址|224.0.0.9|224.0.0.10|224.0.0.5、224.0.0.6|
|max metric|16|2^32-1|2^24-1|

- 鏈路狀態 (link-state) (LS)
    
	[[OSPFv2 與 OSPFv3 全面整理：原理、流程、LSA 類型與實作案例]]
    
    [[IS-IS (Intermediate System - Intermediate System)]]
    
- 距離向量路由 (distance-vertor) (DV)
    
    [[RIP (Routing information Protocols)]]
    
- 混和路由 (balanced hybrid routing)
    
    [[EIGRP (Enhanced interior Gateway Routing Protocol)]]
    

### EGP

- 路徑向量路由
    
    [[BGP 邊界閘道協定完整指南]]
    

## IPv6 routing

### 默認路由static default router

當查找所有路由表之後找不到路由訊息，就會走默認路由這條路徑

```Plain
R1(conifg)#ipv6 route ::/0 s0/0
```

### connected routing

![[Assets/Note/Research/Routing 基本技術/12-connected routing.png|12-connected routing.png]]

```Plain
!configuration on R1
!
ipv6 unicast-routing
!
interface Ethernet0/0
 no ip address
 ipv6 address 2001:DB8:1111:1::1/64
!
interface Serial1/0
 no ip address
 ipv6 address 2001:DB8:111:4::1/64
 serial restart-delay 0
!
interface Serial1/1
 no ip address
 ipv6 address 2001:DB8:1111:5::1/64
 serial restart-delay 0
```

```Plain
!configuration on R2
!
ipv6 unicast-routing
!
interface Ethernet0/0
 no ip address
 ipv6 address 2001:DB8:1111:2::2/64
!
interface Serial1/0
 no ip address
 ipv6 address 2001:DB8:1111:4::2/64
 serial restart-delay 0
```

```Plain
!configuration on R3
!
ipv6 unicast-routing
!
interface Ethernet0/0
 no ip address
 ipv6 address 2001:DB8:1111:3::3/64
!
interface Serial1/1
 no ip address
 ipv6 address 2001:DB8:1111:5::3/64
 serial restart-delay 0
```

```Plain
R1#sh ipv6 router connected
```

![[Assets/Note/Research/Routing 基本技術/13-connected routing.png|13-connected routing.png]]

### static routing config

![[Assets/Note/Research/Routing 基本技術/14-static routing config.png|14-static routing config.png]]

```Plain
!config on R7
!
unicast-routing
!
interface Serial1/0
 no ip address
 ipv6 address 2001:DB8:1111:4::1/64
 serial restart-delay 0
!
interface Ethernet0/0
 no ip address
 ipv6 address 2001:DB8:1111:1::1/64
```

```Plain
!config on R8
!
unicast-routing
!
interface Serial1/0
 no ip address
 ipv6 address 2001:DB8:1111:4::2/64
 serial restart-delay 0
!
interface Ethernet0/0
 no ip address
 ipv6 address 2001:DB8:1111:2::2/64
```

```Plain
!config on VPC9
!
ip 2001:db8:1111:1::11/64
```

```Plain
!config on VPC10
!
ip 2001:db8:1111:2::22/64
```

可以使用ping來做測試

### 浮動靜態路由

```Plain
ipv6 route 3334:4:4:4::/64 3444:2:2:2::2 130 AD值
```

### dynamic routing protocols

### IGP

[[OSPFv2 與 OSPFv3 全面整理：原理、流程、LSA 類型與實作案例]]

[[EIGRP (Enhanced interior Gateway Routing Protocol)]]

[[RIP (Routing information Protocols)]]

[[IS-IS (Intermediate System - Intermediate System)]]

### EGP

[[BGP 邊界閘道協定完整指南]]