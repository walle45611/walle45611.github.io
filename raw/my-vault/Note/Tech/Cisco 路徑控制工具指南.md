## ACL

[[Cisco ACL 存取控制清單指南]]

## Prefix-list

Prefix-list 是抓取路由的專業工具，預設 seq 從 5 開始，每多一條 prefix-list 遞增 10。

### 基本語法

```cisco
Router(config)#ip prefix-list {list-name [seq number] {deny | permit} network/length [ge ge-value] [le le-value]}
```

### 參數說明

|參數|描述|
|---|---|
|`ge ge-value`|匹配遮罩範圍，greater than 到 32|
|`le le-value`|匹配遮罩範圍，less than|

### 常用範例

```cisco
# 匹配預設路由
ip prefix-list list1 permit 0.0.0.0/0

# 匹配所有路由
ip prefix-list list1 permit 0.0.0.0/0 le 32

# 匹配所有主機路由
ip prefix-list list1 permit 0.0.0.0/0 ge 32

# 設定前八位等於 10，遮罩為 /24
ip prefix-list list1 permit 10.0.0.0/8 ge 24 le 24
```

---

# 路徑控制工具

## Offset-list

### 功能說明

- **作用**：用於在進入或出去路由時增加 metric，只能增加不能減少
- **適用協定**：EIGRP、RIP
- **限制**：只能使用 ACL，不能使用 prefix-list

### 基本語法

```cisco
offset-list {access-list-number | name} {in | out} offset [interface-type interface-number]
```

### 實際案例

**目標**：優先選擇路由器 B 當作主要路徑

![[Assets/Note/Tech/Cisco 路徑控制工具指南/01-實際案例.png]]

**設定**：

```cisco
D(config)#access-list 1 permit 10.1.1.0
D(config)#router rip
D(config-router)#offset-list 1 out 2 serial 0/0
```

## Distribute-list

### 功能說明

- **作用**：控制路由表更新內容
- **用途**：在接收或發出路由時，過濾不想要的路由
- **DV 路由協定**：在發出和接收方向都有效果
- **LS 路由協定**：
    - 指定介面（out 方向）過濾不被允許，除非預設全部介面
    - 在 OSPF 中傳送資料使用 LSA 而非路由訊息，除非是 ASBR 上的 type 5 LSA 外部路由
    - in 方向：接收方有效，但下游鄰居不會刪除此路由

### 基本語法

```cisco
Router(config-router)#distribute-list {access-list-number | prefix-list-name} [out | in] [interface | routing-process]
```

## Route-map

### 用途

Route-map 是多功能的工具，常用於：

- Route redistribute（路由重分發）
- PBR（策略路由）
- NAT（網路位址轉換）
- BGP（邊界閘道協定）

### Route-map 路由映射表概念

Route-map 類似於程式語言中的 if-then-else 邏輯：

![[Assets/Note/Tech/Cisco 路徑控制工具指南/02-Route-map 路由映射表概念.png]]

### 使用 Route-map 進行 Redistribute

#### 網路拓撲

![[Assets/Note/Tech/Cisco 路徑控制工具指南/03-網路拓撲.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/04-網路拓撲.png]]

#### 路由處理需求

|Prefix|動作|
|---|---|
|172.16.101.0/24|拒絕|
|172.16.102.0/25|拒絕|
|172.16.103.0/26|允許|
|172.16.104.0/27|允許|
|172.16.105.0/28|拒絕|
|172.16.106.0/29|拒絕|
|172.16.107.0/30|允許|

#### 設定方案一：使用 ACL

**RD1 設定**：

```cisco
ip access-list extended match-101
    permit ip host 172.16.101.0 host 255.255.255.0
!
ip access-list extended match-104-105
    permit ip host 172.16.104.0 host 255.255.255.224
    permit ip host 172.16.105.0 host 255.255.255.240
!
route-map option1 deny 10
    match ip address match-101
!
route-map option1 deny 20
    match ip address match-104-105
!
route-map option1 permit 100
!
router eigrp 1
    redistribute ospf 1 route-map option1
```

#### 設定方案二：使用 Prefix-list

**RD1 設定**：

```cisco
! Area 3
ip prefix-list match-area3-permit seq 5 permit 172.16.102.0/23 ge 25 le 26
ip prefix-list match-area3-permit seq 10 permit 172.16.106.0/23 ge 29 le 30
! Area 0
ip prefix-list match-area0-permit seq 5 permit 172.16.14.0/30
ip prefix-list match-area0-permit seq 10 permit 172.16.18.0/30
ip prefix-list match-area0-permit seq 15 permit 172.16.8.0/25
ip prefix-list match-area0-permit seq 20 permit 172.16.4.0/25
ip prefix-list match-area0-permit seq 25 permit 172.16.48.0/25
!
route-map option2 permit 10
    match ip address prefix-list match-area3-permit
route-map option2 permit 20
    match ip address prefix-list match-area0-permit
!
router eigrp 1
    redistribute ospf 1 route-map option2
```

#### 驗證結果

![[Assets/Note/Tech/Cisco 路徑控制工具指南/05-驗證結果.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/06-驗證結果.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/07-驗證結果.png]]

### Set Metric 應用

#### Metric 設定需求

|Prefix|動作|
|---|---|
|172.16.101.0/24|拒絕|
|172.16.102.0/25|允許 metric 1000 44 255 1 1500|
|172.16.103.0/26|拒絕|
|172.16.104.0/27|拒絕|
|172.16.105.0/28|拒絕|
|172.16.106.0/29|允許 metric 1000 44 255 1 1500|
|172.16.107.0/30|允許 metric 100 4444 255 1 1500|
|其他|metric 1500 10 255 1 1500|

#### RD1 設定

```cisco
!
ip prefix-list match-102-103: 1 entries
   seq 5 permit 172.16.102.0/23 ge 25 le 26
!
ip prefix-list match-106-107: 1 entries
   seq 5 permit 172.16.106.0/23 ge 29 le 30
!
ip prefix-list match-area0-permit: 5 entries
   seq 5 permit 172.16.14.0/30
   seq 10 permit 172.16.18.0/30
   seq 15 permit 172.16.8.0/25
   seq 20 permit 172.16.4.0/25
   seq 25 permit 172.16.48.0/25
!
router eigrp 1
 network 172.30.0.0
 default-metric 1500 10 255 1 1500
 redistribute ospf 1 route-map set-metric
 passive-interface Loopback0
!
```

#### 驗證結果

![[Assets/Note/Tech/Cisco 路徑控制工具指南/08-驗證結果.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/09-驗證結果.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/10-驗證結果.png]]

---

# 數據控制工具

## PBR (Policy-Based Routing)

### 功能說明

PBR 可以選擇性地控制封包的路徑，不依賴傳統的路由表進行轉送。

### 基本指令

|指令|說明|
|---|---|
|`set ip next-hop ip-address [...]`|指定 next hop|
|`set ip default next-hop ip-address [...]`|功能與上一個相同，只是優先順序問題|
|`set interface interface-type interface-number [...]`|指定出路由介面|
|`set default interface interface-type interface-number [...]`|功能與上一個相同，只是優先順序問題|

### 運作邏輯

#### 沒有 default 參數

當 IOS 比對到 PBR route-map permit 時：

1. 先套用 PBR 邏輯
2. 如果 set 的出境介面為 up 或可以到達 next hop，使用 PBR 路徑
3. 如果 PBR 失敗，Cisco 會採用原本的路徑嘗試

#### 有 default 參數

1. 先使用標準路徑
2. 如果標準路徑壞掉了，才會走 PBR

### 實際案例

**需求**：假設 PC1 訪問 S1，經過 R3 再到 R4

#### 網路拓撲

![[Assets/Note/Tech/Cisco 路徑控制工具指南/11-網路拓撲.png]] ![[Assets/Note/Tech/Cisco 路徑控制工具指南/12-網路拓撲.png]]

#### R1 設定

```cisco
R1(config)#interface e0/0
R1(config-if)#ip policy route-map PC2-over-low-route
R1(config)#route-map PC2-over-low-route permit 10
R1(config-route-map)#match ip address 101
R1(config-route-map)#set ip next-hop 10.1.12.6
R1(config)#access-list 101 permit ip host 10.1.1.2 10.1.3.0 0.0.0.255
```

#### 驗證結果

**測試連線**： ![[Assets/Note/Tech/Cisco 路徑控制工具指南/13-驗證結果.png]]

**PC1 流量**： ![[Assets/Note/Tech/Cisco 路徑控制工具指南/14-驗證結果.png]]

**PC2 流量**： ![[Assets/Note/Tech/Cisco 路徑控制工具指南/15-驗證結果.png]]

## 總結

路徑控制工具提供了靈活的網路流量管理方案：

- **Offset-list**：簡單的 metric 調整工具
- **Distribute-list**：基本的路由過濾功能
- **Route-map**：功能強大的多用途工具
- **PBR**：精細的封包轉送控制

選擇適當的工具取決於具體的網路需求和控制粒度要求。