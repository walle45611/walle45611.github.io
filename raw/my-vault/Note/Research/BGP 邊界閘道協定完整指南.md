## 概念使用原因

### AS (Autonomous System) 自治系統

- **定義**：指在同一個組織管理下使用相同策略的設備集合
- **ASN 編號範圍**：1-65535
    - **64512-65434**：私有 ASN
    - **65535**：保留使用
- **用途**：不同的 AS 透過 ASN 分區，AS 之間傳遞路由訊息

### BGP 版本

- **目前 BGP 版本**：V1、V2、V4、V4+ (MBGP)

### BGP 的優勢

1. **大量路由承載能力**：IGP 只能容納千條路由，BGP 可以容納上萬條
2. **MPLS/VPN 應用**：支援 VPN 功能
3. **強大的策略能力**：可以實現路由決策和數據控制

## 路徑向量協定

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/01-路徑向量協定.png]]

### 特性

- 路徑向量訊息中包括 BGP AS table
- BGP 路由器會拒收包含自己 ASN 的路由更新，確保無環路
- BGP 支援對 BGP 自治系統路徑的路由策略
- BGP 路由器只能將其使用的路由通告給對等自治系統中的 peer

## BGP 封包類型

|封包名稱|作用|發送時機|
|---|---|---|
|**OPEN**|協商 BGP peer 的各項參數，建立 peer|透過 TCP 建立 BGP 後發送 OPEN|
|**UPDATE**|進行路由訊息交換|連線建立後，有路由需要發送或路由變化時|
|**NOTIFICATION**|報告錯誤，中止鄰居關係|BGP 運行中發現錯誤時|
|**KEEPALIVE**|維持鄰居關係|定時發送以保持 peer 關係|
|**Route-refresh**|觸發更新路由的機制|路由策略改變時觸發|

## Peer 關係

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/02-Peer 關係.png]]

### 基本概念

- **傳輸協定**：BGP 使用 TCP，port 179
- **連線方式**：BGP 路由器之間建立 TCP 連接，這些路由器稱為 BGP peer
    - **eBGP**：ASN 不一樣的 peer
    - **iBGP**：ASN 一樣的 peer
- **更新機制**：BGP 路由器只能發送增量更新
- **效能最佳化**：BGP 通告成千上萬的路由，採用 TCP 滑動視窗機制，停止等待確認前可以發送 65576 bytes

### BGP Peer 狀態機 (FSM)

|Peer 狀態|發送封包|狀態說明|
|---|---|---|
|**Idle**|嘗試建立 TCP|開始準備 TCP 連接並監視遠端 peer 啟動 TCP 連接，使用 BGP 時要準備足夠的資源和記憶體|
|**Connect**|發送 TCP|正在進行 TCP 連接，等待完成中，認證都是在 TCP 建立期間完成。如果 TCP 連接失敗則會進入 Active 狀態，反覆嘗試連接|
|**Active**|發送 TCP|TCP 連接沒建立成功，反覆嘗試 TCP 連接|
|**OpenSent**|發送 Open|TCP 連接已成功，開始發送 Open，但尚未收到相符的 Open 訊息|
|**OpenConfirm**|發送 Keepalive|已發送並收到 Open 訊息，下一步是接收 BGP keepalive 訊息（確認鄰居參數相符）或 BGP Notification 訊息|
|**Established**|發送 Update|所有鄰居參數相符，鄰居關係生效，peer 開始交換 Update 訊息|

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/03-BGP Peer 狀態機 (FSM).png]]

### BGP Timer 設定

#### KEEPALIVE Timer

- **預設值**：60 秒
- **計算規則**：keepalive 計時器不會在 Open 消息中交換，BGP 計時器如何決定？
    - 如果手動設定的 keepalive timer < min(holdtime)/3，則取設定值
    - 如果手動設定的 keepalive timer > min(holdtime)/3，則取 int[min(holdtime)/3]
    - 其中 min(holdtime) 為兩台鄰居間最小的 holdtime 值

#### HOLDTIME Timer

- **預設值**：Cisco 預設 180 秒（3 倍的 keepalive timer）
- **功能**：該計時器包含在 open 封包中，必須收到一個 keepalive 或更新消息前所允許的最大時間
- **協商**：如果兩端 holdtime 不一致，雙方接受較小的時間

> [!important] 重要提醒 允許兩邊時間不一致

#### Timer 設定指令

```cisco
router bgp [AS-number]
timer bgp [keepalive-time] [holdtime]
```

### 觀念說明

BGP 會使用 TCP 在 BGP 鄰居之間傳送訊息，使用 port 179。BGP 路由器會開啟 179 port number，等待對方路由器連接，一旦連接上鄰居，TCP 連線就算建立完成。

一旦建立完 BGP 連線，每台路由器的 BGP 程序都必須判斷是否應該成為鄰居。整個觀念很像 OSPF 和 EIGRP 透過鄰居交換判斷是否要與其成為鄰居。

兩台 BGP 路由器互相傳送 BGP 訊息，對參數一一檢查，以判斷兩台是否要成為鄰居。如果都檢查對了，就會成為 BGP peer（也就是鄰居），之後就可以交換路由資訊。

> [!warning] 重要限制 在建立 BGP peer 時需要有一方的路由是明細路由，兩邊都是預設路由是無法建立 BGP peer

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/04-觀念說明.png]]

### Peer Group 設定

Peer Group 常見於 iBGP 中：

- 我們可以將更新相同策略的鄰居放在一個組中，簡化設置
- peer group member 繼承所有 peer group 的 config

```cisco
Router(config-router)#neighbor peer-group-name peer-group
Router(config-router)#neighbor ip-address peer-group peer-group-name
```

### 設定範例

#### Case 1: 基本設定

##### 設定鄰居

```cisco
# Configuration on R1
router bgp 1001
neighbor 198.51.100.2 remote-as 1

# Configuration on R2
router bgp 1
neighbor 198.51.100.1 remote-as 1001
```

##### 檢查鄰居

```cisco
R1#show tcp brief
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/05-檢查鄰居.png]]

```cisco
R1#show ip bgp summary
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/06-檢查鄰居.png]]

#### Case 2: 複雜拓撲設定

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/07-Case 2 複雜拓撲設定.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/08-Case 2 複雜拓撲設定.png]]

##### R1 設定

```cisco
hostname R1
!
no aaa new-model
mmi polling-interval 60
no mmi auto-configure
no mmi pvc
mmi snmp-timeout 180
interface Loopback0
 no shutdown
 ip address 1.1.1.1 255.255.255.255
!
interface Ethernet0/0
 no shutdown
 ip address 12.1.1.1 255.255.255.252
!
interface Ethernet0/1
 no shutdown
 ip address 12.1.1.5 255.255.255.252
!
router eigrp 1
 network 1.1.1.1 0.0.0.0
 network 12.1.1.0 0.0.0.255
!
line con 0
 exec-timeout 0 0
 logging synchronous
line aux 0
line vty 0 4
 login
 transport input none
!
end
```

##### R2 設定

```cisco
hostname R2
!
interface Loopback0
 no shutdown
 ip address 2.2.2.2 255.255.255.255
!
interface Ethernet0/0
 no shutdown
 ip address 12.1.1.2 255.255.255.252
!
interface Ethernet0/1
 no shutdown
 ip address 13.1.1.2 255.255.255.252
!
router eigrp 1
 network 2.2.2.2 0.0.0.0
 network 12.1.1.0 0.0.0.255
!
router bgp 123
 bgp router-id 2.2.2.2
 bgp log-neighbor-changes
 neighbor 3.3.3.3 remote-as 123
 neighbor 3.3.3.3 update-source Loopback0
 neighbor 13.1.1.1 remote-as 4
!
line con 0
 exec-timeout 0 0
 logging synchronous
line aux 0
line vty 0 4
 login
 transport input none
!
end
```

##### R3 設定

```cisco
hostname R3
!
interface Loopback0
 no shutdown
 ip address 3.3.3.3 255.255.255.255
!
interface Ethernet0/0
 no shutdown
 ip address 13.1.1.6 255.255.255.252
!
interface Ethernet0/1
 no shutdown
 ip address 12.1.1.6 255.255.255.252
!
router eigrp 1
 network 3.3.3.3 0.0.0.0
 network 12.1.1.0 0.0.0.255
!
router bgp 123
 bgp router-id 3.3.3.3
 bgp log-neighbor-changes
 neighbor 2.2.2.2 remote-as 123
 neighbor 2.2.2.2 update-source Loopback0
 neighbor 13.1.1.5 remote-as 4
!
end
```

##### R4 設定

```cisco
hostname R4
!
interface Loopback0
 no shutdown
 ip address 4.4.4.4 255.255.255.255
!
interface Ethernet0/0
 no shutdown
 ip address 13.1.1.5 255.255.255.252
!
interface Ethernet0/1
 no shutdown
 ip address 13.1.1.1 255.255.255.252
!
router bgp 4
 bgp router-id 4.4.4.4
 bgp log-neighbor-changes
 neighbor 13.1.1.2 remote-as 123
 neighbor 13.1.1.6 remote-as 123
!
line con 0
 exec-timeout 0 0
 logging synchronous
line aux 0
line vty 0 4
 login
 transport input none
!
end
```

### Peer 驗證

BGP 支援 MD5 鄰居身分驗證，認證都是在建立 TCP 連接時完成的：

```cisco
Router(config-router)#neighbor {ip-address | peer-group-name} password string
```

## BGP 路由傳遞

### 傳遞路由常用的方法

#### 1. Network 宣告

宣告時需要嚴格按照路由表中的形式：

```cisco
network 4.4.4.4 mask 255.255.255.255
```

#### 2. Redistribute 重分發

- 將 OSPF 重分發進 BGP 時，預設只能將 internal 的重分發進 BGP
    - **解決辦法**：
        
        ```cisco
        redistribute ospf 1 match internal external 1 external 2 nssa-external 1 nssa-external 2
        ```
        
- 預設情況 eBGP 的路由重分發進 IGP
    - **原因**：因為 iBGP 建立時就應該要有路由，如果沒有路由就沒辦法建立 TCP 連線，所以 eBGP 的路由才會重分發到 IGP
- 重分發進 OSPF 或 EIGRP 時，會將 AS-path 中最前面一個 ASN 作為 TAG

#### 3. 下發

### BGP 路由表檢視

#### BGP Table

```cisco
R1#show ip bgp
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/09-BGP Table.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/10-BGP Table.png]]

##### Status Code 說明（在最前面）

|符號|意義|
|---|---|
|`>`|代表最佳路徑|
|`*`|代表這條路徑是有效的|
|`r`|RIB-failure，表示要加入路由表時失敗了，可能是競爭不過其他路由協定|
|`i`|代表 iBGP|
|`m`|代表負載均衡|
|`d`|懲罰|

##### 其他欄位說明

- **Metric**：在官方的書上面是寫 MED
- **LocPrf**：本地優先級
- **Weight**：Cisco 私有

##### Origin Code 說明（在 path 底下）

|字母|意義|
|---|---|
|`i`|IGP（常代表 network 指令宣告的）|
|`e`|EGP（現在應該都沒有了）|
|`?`|常代表重分發進來的路由|

#### 詳細的 BGP Table

```cisco
show ip bgp x.x.x.x/xx
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/11-詳細的 BGP Table.png]]

#### R1 通告給 x.x.x.x 的路由

```cisco
R1#show ip bgp neighbors x.x.x.x advertised-routes
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/12-R1 通告給 x.x.x.x 的路由.png]]

#### 從 x.x.x.x 收到的路由

```cisco
R1(config-router)#neighbor x.x.x.x soft-reconfiguration inbound  # 要先打開這個功能
R1#show ip bgp neighbors 1.1.1.1 received-routes
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/13-從 x.x.x.x 收到的路由.png]]

## Next Hop 屬性

### 概念說明

- BGP 的 next hop 並不意味著下一台路由器，而是到下一個 AS 的 IP 地址
- **eBGP**：預設 next-hop 為發送更新的鄰居路由器 IP 地址
- **iBGP**：從 eBGP 傳來的 next-hop 屬性在 iBGP 中保持不變傳遞下去

這個也是屬於 BGP 的屬性之一，是因為在 BGP 傳遞路由時會把自己的外部介面 IP 通告成 next hop，但是在 eBGP 因為兩個是 connected 的鏈路所以可以正常溝通，但是內部的 peer 不知道怎麼到達 eBGP 鄰居，所以就可以將內部向外部 AS 的設備設定為 next hop self，這樣 iBGP peer 就會知道 next hop 可以先經過內部向外部 AS 的設備。

### Next-Hop-Self 範例

假設 R4 宣告了 4.4.4.4 這段網路：

#### 在 R3 上沒有使用 next hop self

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/14-在 R3 上沒有使用 next hop self.png]]

#### 使用了 next hop self

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/15-使用了 next hop self.png]]

### Case 1: Next-Hop 設定實例

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/16-Case 1 Next-Hop 設定實例.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/17-Case 1 Next-Hop 設定實例.png]]

- peer 關係全連接的狀態：R4 和 R2、R3 建立 peer，R2、R3、R1 互相建立 peer
- 在 R2 和 R3 因為是直連線路所以會知道怎麼到 R4，但是 R1 是內部設備所以不知道怎麼到 R4，所以把 R1 到 R4 的路徑 show ip bgp 44.44.44.44 時就會出現 inaccessible 不可達的訊息，所以就需要將 Next-Hop 設定在 AS 邊界的設備上也就是 R2 和 R3

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/18-Case 1 Next-Hop 設定實例.png]]

#### 增加指令

預設是 next-hop-unchanged： ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/19-增加指令.png]]

##### R2 設定

```cisco
router bgp 123
 bgp log-neighbor-changes
 neighbor IBGP peer-group
 neighbor IBGP remote-as 123
 neighbor IBGP update-source Loopback0
 neighbor IBGP next-hop-self
 neighbor 1.1.1.1 peer-group IBGP
 neighbor 3.3.3.3 peer-group IBGP
 neighbor 13.1.1.1 remote-as 4
```

##### R3 設定

```cisco
router bgp 123
 bgp log-neighbor-changes
 neighbor IBGP peer-group
 neighbor IBGP remote-as 123
 neighbor IBGP update-source Loopback0
 neighbor IBGP next-hop-self
 neighbor 1.1.1.1 peer-group IBGP
 neighbor 2.2.2.2 peer-group IBGP
 neighbor 13.1.1.5 remote-as 4
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/20-R3 設定.png]]

## iBGP Split Horizon

### 規則說明

從 iBGP peer 收到的路由不能再傳遞給其他 iBGP peer，換句話說從 eBGP 學到的路由只能傳 1 Hop。eBGP 本身靠著 ASN 防止 loop，但是 iBGP 都是相同的 ASN，所以只能靠水平分割。

peer 關係改為 R1、R2、R3 成為 peer，但是 R2 和 R3 不成為 peer，R4、R3、R2 互相成為 peer（Next Hop same topology）。原本 R3 可以收到 R2 的路由訊息，R2 也可以收到 R3，但是 R2 沒有跟 R3 建立鄰居，又因為水平分割的原因 R2→R1→R3 已經 2 hop 了所以就沒有傳遞路由，R3→R1→R2 同理。

### 解決辦法

#### 1. RR (Route Reflector 路由反射器)

##### 觀念

RR 有分 client 和 not client，路由傳遞規則：

|來源/目標|Client|Not Client|
|---|---|---|
|**Client**|✓|✓|
|**Not Client**|✓|✗|

##### 常用拓撲 (Hub and Spoke)

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/21-常用拓撲 (Hub and Spoke).png]]

##### Case（Same "Next-Hop" Topology）

peer 關係改為 R1、R2、R3 成為 peer，但是 R2 和 R3 不成為 peer，R4、R3、R2 互相成為 peer。如果有成功 R3 的 bgp table，R2：

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/22-Case(Same Next-Hop Topology).png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/23-Case(Same Next-Hop Topology).png]]

###### 設定指令

```cisco
# R1 設定
router bgp 123
 bgp log-neighbor-changes
 neighbor 2.2.2.2 remote-as 123
 neighbor 2.2.2.2 update-source Loopback0
 neighbor 2.2.2.2 route-reflector-client
 neighbor 3.3.3.3 remote-as 123
 neighbor 3.3.3.3 update-source Loopback0
```

###### RR 成功的話

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/24-RR 成功的話.png]]

#### 2. Confederation 聯盟

##### 概念

這個功能簡單來說就是在一個 AS 中建立一個 sub-AS：

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/25-概念.png]]

##### 常用拓撲 (Line Bus)

這個功能也沒有很常用：

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/26-常用拓撲 (Line Bus).png]]

##### Case 設定範例

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/27-Case 設定範例.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/28-Case 設定範例.png]]

###### R1 設定

```cisco
router bgp 10
 bgp log-neighbor-changes
 bgp confederation identifier 65000
 bgp confederation peers 20
 redistribute connected
 redistribute ospf 1
 neighbor 2.2.2.2 remote-as 10
 neighbor 2.2.2.2 update-source Loopback0
 neighbor 3.3.3.3 remote-as 10
 neighbor 3.3.3.3 update-source Loopback0
 neighbor 4.4.4.4 remote-as 20
 neighbor 4.4.4.4 ebgp-multihop 2
 neighbor 4.4.4.4 update-source Loopback0
 neighbor 192.0.0.2 remote-as 65100
 neighbor 192.0.0.2 next-hop-self
```

###### R2 設定

```cisco
router bgp 10
 bgp log-neighbor-changes
 bgp confederation identifier 65000
 bgp confederation peers 20
 redistribute ospf 1
 neighbor 1.1.1.1 remote-as 10
 neighbor 1.1.1.1 update-source Loopback0
 neighbor 3.3.3.3 remote-as 10
 neighbor 3.3.3.3 update-source Loopback0
```

###### R3 設定

```cisco
router bgp 10
 bgp log-neighbor-changes
 bgp confederation identifier 65000
 bgp confederation peers 20
 redistribute ospf 1
 neighbor 1.1.1.1 remote-as 10
 neighbor 1.1.1.1 update-source Loopback0
 neighbor 2.2.2.2 remote-as 10
 neighbor 2.2.2.2 update-source Loopback0
```

###### R4 設定

```cisco
router bgp 20
 bgp log-neighbor-changes
 bgp confederation identifier 65000
 bgp confederation peers 10
 redistribute connected
 redistribute ospf 1
 neighbor 1.1.1.1 remote-as 10
 neighbor 1.1.1.1 ebgp-multihop 2
 neighbor 1.1.1.1 update-source Loopback0
 neighbor 1.1.1.1 next-hop-self
 neighbor 5.5.5.5 remote-as 20
 neighbor 5.5.5.5 update-source Loopback0
```

###### R5 設定

```cisco
router bgp 20
 bgp log-neighbor-changes
 bgp confederation identifier 65000
 bgp confederation peers 10
 redistribute connected
 neighbor 4.4.4.4 remote-as 20
 neighbor 4.4.4.4 update-source Loopback0
 neighbor 4.4.4.4 next-hop-self
 neighbor 162.0.0.2 remote-as 65200
```

###### R6 設定

```cisco
router bgp 65200
 bgp log-neighbor-changes
 network 6.6.6.6 mask 255.255.255.255
 neighbor 162.0.0.1 remote-as 65000
```

###### R7 設定

```cisco
router bgp 65100
 bgp router-id 7.7.7.7
 bgp log-neighbor-changes
 redistribute connected
 neighbor 192.0.0.1 remote-as 65000
 neighbor 192.0.0.1 next-hop-self
```

## Synchronization (同步規則)

### 路由黑洞問題

假設 B 和 C 和 D 和 A 建 peer，今天 A 要將 1.0 網段的路由封包傳遞給 E 以後的設備（C、D）但是不可能隔空拿到封包，所以要透過 E，但是 E 又不跑 BGP，就會把封包丟棄，反方向也是，這就是路由黑洞。可以讓 E 跑 BGP，但是性能要強，也可以用 redistribute，但是性能要夠好，也可以使用 MPLS（CCIE），Synchronization 這個方法。

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/29-路由黑洞問題.png]]

### Synchronization 規則

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/30-Synchronization 規則.png]]

#### Case（Same Next-Hop Topology）

```cisco
# config on R2 R3
R2(config-router)#synchronization
R3(config-router)#synchronization
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/31-config on R2 R3.png]]

## Backdoor 功能

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/32-Backdoor 功能.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/33-Backdoor 功能.png]]

建立完成後會發現一個問題就是 BGP AD < EIGRP AD，所以會把 BGP 6.6.6.6 的這條路由成為 Best，但是 BGP 6.6.6.6 的鄰居是靠著 EIGRP 所建立的，此時這條路由不在路由表，就會發生錯誤，也就是路由翻滾 **route flapping** 的現象。

因為當 BGP keepalive 時間到了就會發現鄰居消失了，因為 ip route 裡面沒有正確的路由（被 BGP 取代了），所以就會變成 EIGRP 的路由去替換，當 EIGRP 在路由表中，BGP 鄰居又重新建立，之後等待 keepalive 時間到了又會重複一樣的動作，使得路由都處於不穩定的狀態。前提是都使用 lo0 建立 peer，這時候就可以使用 backdoor 這個功能將 bgp 隱藏在背後。

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/34-Backdoor 功能.png]] ![[Assets/Note/Research/BGP 邊界閘道協定完整指南/35-Backdoor 功能.png]]

```cisco
R5(config-router)#network 6.6.6.6 mask 255.255.255.255 backdoor
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/36-Backdoor 功能.png]]

## Aggregate-Address 路由聚合

### 基本概念

- Topology same Next-Hop
- Like summary address
- 會同時發出一般的路由訊息和 Aggregate 的路由

### Case 設定範例

#### 基本 Aggregate

在 R2 或 R3 都會有兩條訊息，雖然不會影響路由的傳遞，但是這樣就有點多此一舉的感覺：

```cisco
# R4 config
router bgp 4
 bgp log-neighbor-changes
 network 44.44.44.44 mask 255.255.255.255
 aggregate-address 44.44.44.0 255.255.255.0
 neighbor 13.1.1.2 remote-as 123
 neighbor 13.1.1.6 remote-as 123
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/37-R4 config.png]]

#### Summary-Only 選項

所以在 R4 的 config 更改一下：

```cisco
# R4 config
router bgp 4
 bgp log-neighbor-changes
 network 44.44.44.44 mask 255.255.255.255
 aggregate-address 44.44.44.0 255.255.255.0 summary-only
 neighbor 13.1.1.2 remote-as 123
 neighbor 13.1.1.6 remote-as 123
```

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/38-R4 config.png]]

R4 會多一個 suppressed 的標示，表示明細路由會被抑制掉：

![[Assets/Note/Research/BGP 邊界閘道協定完整指南/39-R4 config.png]]

#### AS-Set 選項

現在在 R1 加上 aggregate：

```cisco
router bgp 123
 bgp log-neighbor-changes
 aggregate-address 44.44.0.0 255.255.0.0
```