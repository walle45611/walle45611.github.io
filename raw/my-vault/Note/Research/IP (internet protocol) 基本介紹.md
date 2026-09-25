# 概述

- 是一種不可靠(unreliable)的傳輸協定，不能保證IP能夠成功的到目的地，IP僅提供最好的傳輸服務，如果生錯誤也部會理會。
- 無連接(connectionless)意思是IP並不維護任何後續封包的狀態訊息，妹個封包的處理是互相獨立的，也就是說不會通過seq number來判斷順序

# IPv4

## IPv4 header

![[Assets/Note/Research/IP (internet protocol) 基本介紹/01-IPv4 header-2.png]]

- IP第一層
    - Version = 0x0100 代表 ipv4 Version = 0x0110 代表 ipv6
    - IHL = Header Length不可能超過65535
    - Type os Service = ToS
    - Total length = 全部封包的大小
- IP第二層
    - Identification (fragment ID) fragment Offset這整個東做也稱為分片示意圖
        - 每發一個完整封包就加1，
    - MF = More fragment 代表了可以分片
    - DF = Don’t fragment 代表了不行分片如果最大支持的封包大小超過了就會傳送錯誤訊息
        
        ![[Assets/Note/Research/IP (internet protocol) 基本介紹/01-IPv4 header.png|01-IPv4 header.png]]
        
- IP第三層
    - Time To Live = 存活時間 也就是每過一個閘道會見減一從255減到0，如果為0就將封包丟棄這項功能可以防止路由迴還。
    - Protocol = 定義是那些協議例如tcp,udp,icmp這些只能選一個，這些協議header會放在Data裡面
        
| Protocol | Number |
|----------|--------|
| ICMP | 1 |
| TCP | 6 |
| UDP | 17 |
| GRE | 47 |
| ESP | 50 |
| AH | 51 |
        
    - Header checksum=檢查header有沒有錯誤每過一個閘道這個checksum就會改變

> [!important] 不可能超過40 byte，不包括Option和Data也會有20byte

---

## QoS

### ToS

定義了資料包的優先順序(Precedence)，並指定同樣優先等級下，資料封包的丟棄原則(ToS)。

Type of Service最早定義在RFC-791。之後的RFC-2474則改變了這些區域的代表意義，稱作DS（Differentiated Services）。ToS的定義問題，使得支援性比較低，實際網路上都是用DS，因此在IPv6階段，ToS已經不支援了。後面還有其他RFC文件，提到再帶出。沒有設定都是0000

- 可以細分成Precedence和ToS
    
| Precedence (3 bit) | ToS (4) | Unused (1) |
|--------------------|---------|------------|
| 值 | 值 | 值 |
    

### DSCP


---

## MTU

data link layer上面所能通過的最大封包大小，IP允許IP分片，它使用的是將分組傳送到鏈路上的網路介面的最大傳輸單元的值。原始分組的分片都被加上了標記，這樣目的主機的IP層就能將分組重組成原始的資料報了，DF這個位址就是代表了到底要不要分片，如果是1就代表不用分片，在Cisco advanced ping可以去設定DF這個位置的value，如果不分片且大小超過設定的MTU就會丟棄封包。

在大多情況都是使用1500這個MTU值，像是PPPoE會減小這個數值通常是1492（=1500-2（PPP  
）-6（oE））  

那位甚麼不把MTU設定到很大呢，因為不能確定其他設備可以支持到很大的MTU數值。

- advanced ping DF bit
    
    ```Plain
    R1#ping 192.168.1.1 df-bit repeat 1
    ```
    
- 常見的MTU
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/02-MTU.png|02-MTU.png]]
    

> [!important] 在哪個方向查MTU呢?是在出的方向查MTU

---

### Loopback

![[Assets/Note/Research/IP (internet protocol) 基本介紹/02-Loopback.gif]]

- 傳給loopback interface 一般是127.x.x.x的任何data作為IP輸入
- 傳給廣播地址或是群播地址的數據複製一份給loopback interface，然後送到ethernet上。因為廣播傳送和群播的定義包含主機本身
- 任何該主機IP位置均送到loopback

---

## 子網切割

### 特殊的IP位置

| 網路位 | 主機位 | Source Address | 目的位址使用 | 代表意思 |
|--------|--------|----------------|--------------|----------|
| 0 | 0 | 可 | 不可 | 在本網路上的所有主機 |
| 0 | host-id | 可 | 不可 | 在本網路上的某台主機 |
| 全1 | 全1 | 不可 | 可 | 在本網路上進行廣播(個路由器均不轉發) |
| net-id | 全1 | 不可 | 可 | 對net-id上的所有主機進行廣播 |
| 127 | 非全0或全1 | 可 | 可 | 用本地loopback測試之用 |

### VLSM

### CIDR

---

# IPv6

## IPv4存在的問題

- 地址位址耗盡
- internet用戶太多
- internet路由表增大
- 缺乏真正的p2p模型
- 無法適應新技術的發展，物聯網
- 所有的行業都是IPv6的淺在用戶

## IPv6取代或升級

- OSPFv2升級到OSPFv3
- ICMP升級到ICMPv6
- ARP被NDP(Neighbor Discovery Protocol)取代

## IPv6特點

- 128 bit
- 多等層級有助於路由聚合
- 自動配置過程允許IPv6網路節點更加便捷的接入IPv6
- 重新編址機制使得IPv6 ISP之間的主換對最終的用戶是透明的
- 無須NAT
- 不再有廣播，不再有ARP，ICMPv6取代ARP
- IPv6的header比IPv4更有效率，數據字段更少，去掉header sum check。更簡單的header提高了路由器處理的效率。新的擴展取代了IPv4的選項字段，並且提供了更多靈活性
- 更有效的支持移動性和安全性
- 子網路長度是固定的
- v4v6過度方式豐富多彩

### IPv4和IPv6性值的差別

- NO NAT
- Protocol is not compatible
- NOT interconnected
- NOT to be Deployed As IPv4
- By Design, NO Auto assign Default Gateway
- Many thing to be discussed

## IPv6 header

![[Assets/Note/Research/IP (internet protocol) 基本介紹/03-IPv6 header.png|03-IPv6 header.png]]

### IPv6 header 和IPv4 header差別

![[Assets/Note/Research/IP (internet protocol) 基本介紹/04-IPv6 header 和IPv4 header差別.png|04-IPv6 header 和IPv4 header差別.png]]

![[Assets/Note/Research/IP (internet protocol) 基本介紹/05-IPv6 header 和IPv4 header差別.png|05-IPv6 header 和IPv4 header差別.png]]

![[Assets/Note/Research/IP (internet protocol) 基本介紹/06-IPv6 header 和IPv4 header差別.png|06-IPv6 header 和IPv4 header差別.png]]

- Traffic Class = ToS
- Protocol = Next Header
- Hop Limit = TTL
- Payload Length = Total Length

### IPv6 分組header和擴展header

IPv4 Option = IPv6分組header或擴展

![[Assets/Note/Research/IP (internet protocol) 基本介紹/07-IPv6 分組header和擴展header.png|07-IPv6 分組header和擴展header.png]]

- 擴展header只有目標節點查看，其他節點不查看和處理大部分擴展header
- 要按順序查看擴展header的內容

![[Assets/Note/Research/IP (internet protocol) 基本介紹/08-IPv6 分組header和擴展header.png|08-IPv6 分組header和擴展header.png]]

- IPv6擴展header，使用擴展header時，封包順序如下(RFC2460) :
    1. 基本的IPv6 header
    2. 逐跳選項header
    3. 目標選項header (如果使用路由選舉header)
    4. 路由選舉header
    5. 分段header
    6. 身分驗證(AH)和封裝安全有效負載(ESP)header
    7. 目的選項header
    8. 上層header : 主要為TCP UDP ICMPv6等等

  

### IPv6 header 的改進

- 取消了IP的校驗
    - 第二層和第四層已經夠強壯了，因此IPv6取消了IP的三層校驗。
- 取消中間節點的分片功能
    - 分片重組功能由源目的端自己進行，通過PMTU機制來發現MTU
- 定義最長的IPv6 header
    - 有利於硬體的快速處理，如此一來中間節點可以避免處理而節省大量資源
- 安全選項
    - IPv6提供了IPsec的完美支持，如此上層協議可以節省許多安全選項，如OSPFv3取消了驗證
- 增加流標籤
    - 提高Qos效率

## ipv6編址

- IPv6是128 byte
- 可提供很多設備
- 使用冒號分隔16進位格式
    
    2001:0da8:0207:0000:0000:0000:0000:8207
    
- IPv6 address有很多呈現的方式

### IPv6地址簡寫

![[Assets/Note/Research/IP (internet protocol) 基本介紹/09-IPv6地址簡寫.png|09-IPv6地址簡寫.png]]

- 每組16bits的單元中多個前導0可以省略，但是如果16bits單位的所有bits都為0，至少留一個0:
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/10-IPv6地址簡寫.png|10-IPv6地址簡寫.png]]
    
- 一個或多連續的16bit字符為0時，可用"::”表示，但整個地址縮寫址與許有一個"::”
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/11-IPv6地址簡寫.png|11-IPv6地址簡寫.png]]
    

### 介面ID

- 接口ID為64bits，用於標示鏈路的接口，在每條鏈路上接口ID必須唯一
- 接口ID的配置主要有以下幾種方式
    - 可以根據IEEE的EUI-64規範將48bits的MAC轉化為64bits的接口ID
    - 手工配置接口ID
    - 某些作業系統支持自動隨機產生接口ID
- 接口ID的作用
    - 可用於構成link local地址
    - 可在無狀態配置環境中用於構成全局單播IPv6地址

- EUI-64算法如下
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/12-介面ID - EUI-64算法如下.png|12-介面ID - EUI-64算法如下.png]]
    

### IPv6編址種類

![[Assets/Note/Research/IP (internet protocol) 基本介紹/13-IPv6編址種類.png|13-IPv6編址種類.png]]

### 單播 (Unicast Address)

==標示一個接口，目的地位置為單播地址的封包會被送到標示的接口==

### Aggregatable Global Unicast address 可以在internet的ip address

![[Assets/Note/Research/IP (internet protocol) 基本介紹/14-Aggregatable Global Unicast address.png|14-Aggregatable Global Unicast address.png]]

- 相當於IPv4全局單播地址
- 由48位的全局路由選則前綴+16位的子網ID+64位的接口ID組成
- 一般ISP申請到的IPv6地址空間為/48，再由自己根據的需求規劃
- ==可以想像成是IPv4的A、B、C公共IP==
- 分配IP
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/15-Aggregatable Global Unicast address.png|15-Aggregatable Global Unicast address.png]]
    
    - IANA分配給ARIN prefix為2001::/16
    - ARIN分配給NA-ISP1 prefix 2001:0DB8::/32
    - NA-ISP1分配給公司1 prefix 2001:0DB8:1111/48

- Aggregatable Global Unicast address地址範圍:
    - 2000:: 到 3FFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF
    - 2002::/16是拿來IPv4 to IPv6
    - 由此可以看出，Aggregatable Global Unicast address只站IPv6地址的1/8
- IPv6 Global Unicast子網切割
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/16-Aggregatable Global Unicast address.png|16-Aggregatable Global Unicast address.png]]
    
    - 沒有分級的概念，其餘跟IPv4基本相同
    - 通常prefix是/32到/48之間，但也可能長達/56
    - 通常介面ID為64 byte
    - 最常用的 P=48、S=16、I=64

### Site-Local Address (RFC3870寫被廢棄的內容)基本被廢棄，被Unique Local address取代

- 類似IPv4私有地址
- 使用站點本地地址意味著需要NAT，地址不是site2site的
- 地址以FEC::/10，緊接著是連接連續38bits的0
- 對於站點本地地址來說，前48bits是固定的。在接口ID和48bits特定前綴之間有16bits子網ID字段，供機構內部建立子網
- 本地地址永遠不會用於全球IPv6 internet通訊，一般用於內網通信

### Unique Local address (RFC 4193發佈)可以在自己的公司內部路由

- 開頭 FC00::/7 like 10.0.0.0/8
- ULA是IPv6私有地址，只能夠在內網中使用，該地址空間再IPv6供網不可路由，因此不能直接訪問公網
- 地址FC00/7被劃分為兩個/8的塊，其中FC00::/7暫未定義，FD00::/8定義如圖
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/17-Unique Local address (RFC 4193發佈)可以在.png|17-Unique Local address (RFC 4193發佈)可以在.png]]
    

### Link local address 不能去外部網路只能透過本地而已

![[Assets/Note/Research/IP (internet protocol) 基本介紹/18-Link local address 不能去外部網路只能透過本地而已.png|18-Link local address 不能去外部網路只能透過本地而已.png]]

- 本地鏈路地址，有效範圍為本地鏈路
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/19-Link local address 不能去外部網路只能透過本地而已.png|19-Link local address 不能去外部網路只能透過本地而已.png]]
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/20-Link local address 不能去外部網路只能透過本地而已.png|20-Link local address 不能去外部網路只能透過本地而已.png]]
    

- 以FE80::/10為前綴，此外11-64位為0，外加一個64位接口標示
- 用於自動地址配置，鄰居發現，路由器發現等等
- 用於一條鏈路上，必須知道對方節點的Local link address，如果不知道，將是不能通訊，所以一條鏈路中的IPv6節點需要通訊，必須擁有本地地址，並且這個Local link address只能在在一條鏈路中有效，也不能被路由，而不同的鏈路的Local link address可以重複使用。
- 手動設定，一般來說會自動設定好，必須以FE8、FE0、FEA、FEB開頭
    
    ```Plain
    R1(config-if)#ipv6 address x.x.x.x link-local
    ```
    

### 組播 (Multicast Address)

==標示多個接口，目的地址為組播地址的封包會被送到被標示的所有接口==

- 用來標示一組接口，發送給多播地址的數據流同時傳輸到多個目的地
- 範圍 : FF00::/8

![[Assets/Note/Research/IP (internet protocol) 基本介紹/21-組播 (Multicast Address) - 範圍 FF00 8.png|21-組播 (Multicast Address) - 範圍 FF00 8.png]]

- Flags
    - 用來標示永久或是零食的組播
    - 0001表示臨時
    - 0000表示永久分配或是眾所周知
        
| 簡稱 | 群播地址 | 涵義 | 相當於IPv4 |
|------|----------|------|-----------|
| 所有的節點 (all-nodes) | FF02::1 | 所有節點 | 廣播 |
| 所有路由器 (all-routers) | FF02::2 | 所有路由器 | 無 |
| 所有OSPF (All-OSPF) | FF02::5 | 鏈路上所有OSPFv3路由器 | 224.0.0.5 |
| 所有OSPF的DR(All-OSPF-DR) | FF02::6 | 鏈路上所有OSPFv3 DR路由器 | 224.0.0.6 |
| RIPng路由器 | FF02::9 | 所有RIPng路由器 | 224.0.0.9 |
| EIGRPv6 | FF02::A | 所有使用EIGRPv6的路由器 | 224.0.0.10 |
| DHCP代理 | FF02::1:2 | 所有扮演DHCPv6轉送代理的路由器 | 無 |
        
- Scope
    - 表示組播的範圍
        
| Scope | 說明 |
|-------|------|
| 0 | 預留 |
| 1 | 節點本地範圍;單接口有效，僅用於loopback通訊 |
| 2 | 鏈路本地範圍 |
| 5 | 站點本地範圍 |
| 8 | 組織本地範圍 |
| E | 全球範圍 |
| F | 預留 |
        
- Group ID
    - 組播組ID
    - 組播地址的MAC映射
        
        ![[Assets/Note/Research/IP (internet protocol) 基本介紹/22-組播 (Multicast Address) - 組播地址的MAC映射.png|22-組播 (Multicast Address) - 組播地址的MAC映射.png]]
        

### 任一傳播 (Anycast Address)

==標示多個接口，目的為任意播地址的封包會被送到最近的一個標是接口，最近的節點是路由協定所定義的==

```Plain
R1(config-if)#ipv6 address <address> anycast
```

| 地址類型 | 第一個16進位的數字 |
|----------|--------------------|
| Global Unicast | 2或3 (最初) : 基本上已無保留位置 |
| Unique Local | FD |
| Multicast | FF |
| Link-Local | FE80 |

### ==IPv6 沒有廣播無須NAT==

### command

![[Assets/Note/Research/IP (internet protocol) 基本介紹/23-command.png|23-command.png]]

- static
    
    ```Plain
    R1(config)#ipv6 unicast-routing 啟動ipv6路由
    R1(config)#int l0
    R1(config-if)#ipv6 address 2001:db8:1111:1::1/64
    R1(config)#int s1/0
    R1(config-if)#ipv6 address 2001:0db8:1111:2::1/64
    ```
    
    ```Plain
    R2(config)#ipv6 unicast-routing 啟動ipv6路由
    R2(config)#int l0
    R2(config-if)#ipv6 address 2001:db8:1111:3::2/64
    R2(config)#int s1/0
    R2(config-if)#ipv6 address 2001:0db8:1111:2::2/64
    ```
    
- EUI-64
    
    ```Plain
    R1(config)#ipv6 unicast-routing 啟動ipv6路由
    R1(config)#int l0
    R1(config-if)#ipv6 address 2001:db8:1111:1::/64 eui-64
    R1(config)#int s1/0
    R1(config-if)#ipv6 address 2001:0db8:1111:2::/64 eui-64
    ```
    
    ```Plain
    R2(config)#ipv6 unicast-routing 啟動ipv6路由
    R2(config)#int l0
    R2(config-if)#ipv6 address 2001:db8:1111:3::/64 eui-64
    R2(config)#int s1/0
    R2(config-if)#ipv6 address 2001:0db8:1111:2::/64 eui-64
    ```
    
- show
    
    ```Plain
    R2#sh ipv6 route
    ```
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/24-command - show.png|24-command - show.png]]
    
    ```Plain
    R2#sh ipv6 int l0
    ```
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/25-command.png|25-command.png]]
    

### 自動分配位址給子網的主機

除了可以使用指定的方式讓子網內的IP不重複也可以是用無狀態位置自動配置(stateless address autoconfiguration，SLAAC)的IPv6內建機制來動態學習這些相同的設定或是使用有狀態的ipv6 DHCP。

```Plain
! This interface uses DHCP to learn its IPv6 address
R1(config)#interface e0/0
R1(config-if)#ip addr dhcp
!
! This interface uses SLAAC to learn its IPv6 address
R1(config)#interface e0/0
R1(config-if)#ip addr autoconfig
```

## IPv6路由

IPv6 routing 跟 IPv4 routing有很多像似的地方例如 :

- 為了建立並發送IPv6封包到某個介面，終端使用者的設備需要知道該介面的IPv6地址

- 終端使用者的主機需要知道預設閘道的IPv6 address，以便IPv6封包發送到不同子網的主機
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/26-IPv6路由.png|26-IPv6路由.png]]
    
- IPv6利用deencapsulate(解封裝)以及reencapsulate(再封裝)來routing IPv6封包
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/27-IPv6路由.png|27-IPv6路由.png]]
    
- IPv6 router對比IPv6路由表與IPv6封包的目的地位置來決定routing的方式
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/28-IPv6路由.png|28-IPv6路由.png]]
    
      
    

### IPv6 dynamic routing protocol

| Routing Protocol | Define | 說明 |
|------------------|--------|------|
| RIPng (下一代RIP) | RFC | 沒有重要的 |
| OSPFv3 (OSPF第3版) | RFC | 用於IPv4的OSPF式OSPFv2，因此OSPFv3適用於IPv6 |
| EIGRPv6 (適用於IPv6 EIGRP) | Cisco | Cisco擁有EIGRP，但是Cisco已發表EIGRP |
| MP BGP-4 (多重協定 BGP 第四版) | RFC | BGPv4具備優異的擴充性，拓過MP BGP-4的提升，BGPv4也可以支援IPv6 |

## DHCPv6

---

## IPv6過渡

### dual-stack (ds)

- 設上同時使用IPv4和IPv6，其他過渡技術的基礎
- node 上有IPv4和IPv6兩個協議
- 缺點 : 每台設備都需要設定兩種協議，需要佔資源，設備需要存儲量個路由表，兩個協議拓譜表，需要獨立處理每種協議。

### Tunnel

- 把IPv6封包封裝在IPv4封包中，IPv6網路之間穿越IPv4網路進行通訊
- 手動隧道 : GRE 、 手工隧道
    
    - GRE隧道和手工隧道很相似
    - GRE隧道是cisco開發的，在Cisco路由器上，默認隧道就是使用GRE封裝，可部屬在多種協議之上，還可以乘載多種協議
    - 傳輸協議 IPv6或是IPv4
    - 隧道協議 GRE
    - GRE 協議號 47
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/29-Tunnel - GRE 協議號 47.png|29-Tunnel - GRE 協議號 47.png]]
    
    ```Plain
    interface fa0/0
    	ipv6 enable
    	ipv6 addr 2001:0001::1/64
    interface tunnel 0
    	ipv6 enable
    	tunnel mode ipv6ip
    	tunnel source fa0/1
    	tunnel destination 10.1.12.2
    ipv6 route xxxx tunnel 0
    ```
    
    ```Plain
    interface 0/0
    	ipv6 enable
    	ipv6 addr 2001:0002::1/64
    
    interface tunnel 0
    	ipv6 enable
    	tunnel mode ipv6ip
    	tunnel source fa0/1
    	tunnel destination 10.1.12.1
    
    ipv6 route xxx tunnel 0
    ```
    
- 自動隧道 : 6to4 、IPv4兼容IPv6自動隧道、ISATAP隧道
    - IPv4兼容IPv6自動隧道
        
        ![[Assets/Note/Research/IP (internet protocol) 基本介紹/30-Tunnel - IPv4兼容IPv6自動隧道.png|30-Tunnel - IPv4兼容IPv6自動隧道.png]]
        
        ```Plain
        ipv6 unicast-routing
        interface tunnel 0
        	ipv6 enable
        	tunnel source s0/1                    //s0/1外網介面
        	tunnel mode ipv6ip 6to4
        interface s0/0                      
        	ipv6 address 2002:640A:A0A::1/64     //ipv6內網gateway介面  
        	ipv6 enable
        	no ipv6 nd suppress-ra
        interface s0/1
        	ip addr 100.10.10.10 255.255.255.0
        	ip route 2002::/16 tunnel0               //注意要配置去網6to4的static route
        ```
        
    - ISATAP隧道
        
        ![[Assets/Note/Research/IP (internet protocol) 基本介紹/31-Tunnel - ISATAP隧道.png|31-Tunnel - ISATAP隧道.png]]
        
- 6PE 6PE技術依賴於BGP，BGP的peer式需要手工指定，可以算是一種半自動隧道

### 協議轉換技術(NAT-PT)

- 具備IPv4和IPv6協議轉換功能的轉換設備，修改協議header，使IPv6網路與IPv6網路可以溝通
- IPv6網路和IPv4網路同時存在且需要互相通訊
- NAT-PT (Network Addreess Translation-Protocol Translation)技術是將IPv6協定與IPv4協定之間轉換RFC2765有定義。
- 設定
    
    ![[Assets/Note/Research/IP (internet protocol) 基本介紹/32-協議轉換技術(NAT-PT) - 設定.png|32-協議轉換技術(NAT-PT) - 設定.png]]
    
    ```Plain
    ipv6 unicast-routing
    !
    interface fa0/0
    	ipv6 address 2001::2/64
    	ipv6 enable
    	ipv6 nat
    !
    interface f0/1
    	ip addr 12.1.1.2 255.255.255.0
    	ipv6 nat
    !
    ipv6 nat v6v4 sourc 2001::1 12.1.1.1
    ipv6 nat v4v6 sourc 12.1.1.1 2001::1 
    ipv6 nat prefix 2001:1:1:1::/96
    ```
    

### 兩者的差別tunnel NAT-PT

- NAT-PT和tunnel都是IPv6過渡的技術
- 封包和轉發的方式不同，這是NAT-PT和tunnel最根本的差別
- NAT-PT是對網路中的封包的網路層內進行修改轉換，剝離原先的header，替換之後的header;而隧道是對初始封包做另一層的封裝
- NAT-PT會檢查並且可以更改封包中的port;而tunnel不會檢查封包傳輸層的內容
- NAT-PT一般用於IPv4與IPv6不同網路中主機互相訪問的環境中;而tunnel一般用於實現一種網路協議跨越另一種網路協議之間的通訊
- NAT-PT的環境址需要有一台可以進行NAT-PT轉換的設備即可;而構成tunnel需要兩台設備