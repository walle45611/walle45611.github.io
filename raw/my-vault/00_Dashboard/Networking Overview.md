# TCP/OSI 網路模型完整指南

![[Assets/00_Dashboard/Networking Overview/01-TCP OSI 網路模型完整指南.png]]

## 為什麼需要分層

### 分層的必要性

- **標準統一**：沒有分層會導致每家廠商都需從 Physical Layer 開始開發，無法統一標準，不利於專業分工
- **模組化設計**：層與層之間有清晰邊界，有助於理解與模組化
- **功能分離**：每層處理特定功能，互不影響：
  1. 應用層處理應用細節
  2. 傳輸層提供主機間 p2p 通訊
  3. 網路層處理選路等分組轉送

### 分層的優勢

- **服務層次**：每層是**服務者也是被服務者**，對上層提供服務，對下層提出需求
- **標準制定**：有利於國際標準制定：
  - IEEE 管 Data Link Layer 以下標準
  - IETF 管 Network Layer 以上標準
- **設計原則**：層數足夠多，有利功能分離與不重複設計

> [!important] 建議由下往上閱讀

## WAN (Wide Area Network) 廣域網路

### 為什麼需要 WAN

- **分支通訊**：分區或分支機構的員工需要與總部通信並共享數據
- **組織間協作**：組織經常需要與其他組織遠距離共享資訊
- **移動辦公**：經常出差的員工需要訪問公司的內網

### WAN 在 OSI 模型中的位置

WAN 的操作主要集中在第 1 層和第 2 層：
- **物理層**：描述連接通信服務提供商所需的設備
- **數據鏈路層**：協定定義如何封裝向遠端位置的數據以及最終數據 frame 的傳輸機制

![[Assets/00_Dashboard/Networking Overview/02-WAN 在 OSI 模型中的位置.png]]

### WAN 接入方式

#### 1. 專線（Point-to-Point）

- **常見技術**：DDN、POS、E1、Ethernet 專線
- **特點**：安全穩定、傳輸品質佳、設置簡單
- **適用場景**：高流量、高穩定性的需求，但價格較高

| 連結類型   | 頻寬容量          |
| ------ | ------------- |
| 56     | 56 kbps       |
| 64     | 64 kbps       |
| T1     | 1.544 Mbps    |
| E1     | 2.048 Mbps    |
| J1     | 2.048 Mbps    |
| E3     | 34.064 Mbps   |
| T3     | 44.736 Mbps   |
| OC-1   | 51.84 Mbps    |
| OC-3   | 155.54 Mbps   |
| OC-12  | 622.08 Mbps   |
| OC-48  | 2488.32 Mbps  |
| OC-192 | 9953.28 Mbps  |
| OC-768 | 39813.12 Mbps |

#### 2. 電路交換

- **常見技術**：ISDN、ADSL、PSTN 模擬撥號（異步）
- **特點**：利用語音模擬信號的電話線傳送數據，須撥號連接
- **缺點**：數/模轉換、頻寬小、穩定性較差、設定維護複雜

![[Assets/00_Dashboard/Networking Overview/03-2. 電路交換.png]]

#### 3. 分組交換

- **原理**：根據資料封包第二層地址選擇路徑
- **虛擬電路類型**：
  - **PVC**（永久虛擬電路）：預先建立路由
  - **SVC**（交換虛擬電路）：需時建立
- **常見協定**：X.25、Frame Relay

![[Assets/Note/Research/Frame Relay 介紹/00-Frame Relay WAN 雲端拓撲.png]]

### WAN 物理層

- 描述連接 WAN 所需的電氣、機械、操作、功能特性
- 包括 DTE 與 DCE 之間的連接方式

![[Assets/00_Dashboard/Networking Overview/04-WAN 物理層.png]]

### WAN 常見封裝協定

#### 專線協定
![[Assets/00_Dashboard/Networking Overview/05-專線協定.png]]

參考：
- [[HDLC 介紹]]
- [[PPP 和 PPPoE 介紹]]

#### 電路交換協定
![[Assets/00_Dashboard/Networking Overview/06-電路交換協定.png]]

#### Frame Relay
![[Assets/00_Dashboard/Networking Overview/07-Frame Relay.png]]

參考：[[Frame Relay 介紹]]

#### VPN 技術
參考：
- [[GRE 介紹]]
- [[IPSec (IP Security) 介紹]]
- [[GRE 與 IPSec 隧道技術指南]]
- [[DMVPN (Dynamic Multipoint VPN) 介紹]]

## 基本的網路連線

### 靜態 IP 位址設定

1. 在設定模式中，利用 `ip address ip_address mask` 將 IP address 設定到 ISP 介面
2. 設定預設路由 `ip route 0.0.0.0 0.0.0.0 ISP_address`

![[Assets/00_Dashboard/Networking Overview/08-靜態 IP 位址設定.png]]

```cisco
R1(config)#ip route 0.0.0.0 0.0.0.0 10.1.1.1
```

### 動態 IP 指派

![[Assets/00_Dashboard/Networking Overview/09-動態 IP 指派.png]]

```cisco
R1(config-if)#ip address dhcp
```

## OSI 七層詳解

### Physical Layer (實體層)

#### 實體傳輸與網路線

- **接頭標準**：RJ45 T568B/T568A 接法
- **線材類型**：
  - **Straight**（兩端相同）
  - **Crossover**（兩端不同）
- **線芯使用**：
  - 10M 使用線芯 1,2,3,6
  - 100M/1000M 使用 8 條線芯
#### RJ45 T568B T568A

![[Assets/00_Dashboard/Networking Overview/01-RJ45 T568B T568A.jpg]]

- 兩端一樣的稱為Straight，兩端不一樣的稱為Crossover
- 做網路線的時候必須把網路線完全進到網路頭中

💡 事實上10M乙太網的網線只使用 1、2、3、6編號的芯線傳遞數據，即1、2用於發送，3、6用於接收，按顏色來說：橙白、橙兩條用於發送；綠白、綠兩條用於接收；4、5，7、8是雙向線。100M和1000M網卡需要使用四對線，即8根芯線全部用於傳遞數據。由於10M網卡能夠使用按 100M方式製作的網線；而且雙絞線又提供有四對線，所以日常生活中不再區分，10M網卡一般也按 100M方式製作網線。


#### 跳線與平行線使用規則

| 設備類型 | HUB | Switch | Router | NIC |
|----------|-----|--------|--------|-----|
| **HUB** | Crossover | Crossover | Straight | Straight |
| **Switch** | Crossover | Crossover | Straight | Straight |
| **Router** | Straight | Straight | Crossover | Crossover |
| **NIC** | Straight | Straight | Crossover | Crossover |

> [!important] 連接規則
> 相同性質設備 → Crossover；不同性質設備 → Straight

### full-duplex, half-duplex

網路線中，知道8條上面其時只有兩對使用，一對是在傳輸另一對在接收，如果電腦可以同時支援就帶表示全雙工，input/output均可以達到10/100Mbps，亦即資料的傳送與接收同時均可達到 10/100bps 的意思，所以總頻寬就代表可以到達20/200Mbps，但是在Hub中是不可能實現的。

- cisco command
    
    ```
    R1(config-if)#duplex [full | half]
    ```
    

### auto-negotiation

在網路中會有很多的線路，也會有很多的分級，可以知道ethernet nic可以向下支援其速度的網路，在早期需要手動設定網路的speed，現在的switch大部分都有auto-speed的功能了

- cisco command
    
    ```
    R1(config-if)#speed auto
    ```
    

### **Auto MDI/MDIX**

也就是說可以自動判斷網路線的種類是跳線還是平行線，所以這樣就可以不用特別去買跳線或是特別需要做跳現了

- cisco command
	
	```
	R1(config-if)#mdix auto
	```

### Data Link Layer (資料鏈路層)

負責實體介面的邏輯封裝與介面控制

#### Switching 相關協定

參考資料：
- [[EtherChannel 介紹]]
- [[GLBP  熱備份路由協定指南]]
- [[HSRP 與 VRRP 熱備份路由協定指南]]
- [[MLS (MLS，multilayer switching)]]
- [[Cisco PoE 電力供應指南]]
- [[Cisco StackWise FlexStack]]
- [[STP (Spanning tree) 完整指南]]
- [[Cisco VTP (VLAN Trunking Protocol)]]
- [[Cisco 監控模組與路由處理器備援]]
- [[Cisco QinQ 802.1Q tunneling]]

#### 通訊協定

參考資料：
- [[NDP (Neighbor Discovery Protocol) 基本介紹]]
- [[CDP 與 LLDP 鄰居探索協定指南]]
- [[ARP (Address Resolution Protocol) 基本介紹]]
- [[Ethernet 乙太網路協定指南]]

### Network Layer (網路層)

提供 hop-to-hop routing，負責封包選路與邏輯位址（IP）處理

#### 通訊協定

參考資料：
- [[NAT (Network Address Translation) 介紹]]
- [[IP (internet protocol) 基本介紹]]
- [[ICMP(Internet Control Message Protocol) 基本介紹]]

#### Routing 協定

參考資料：
- [[Routing 基本技術]]
- [[BGP 邊界閘道協定完整指南]]
- [[OSPFv2 與 OSPFv3 全面整理：原理、流程、LSA 類型與實作案例]]
- [[EIGRP (Enhanced interior Gateway Routing Protocol)]]
- [[RIP (Routing information Protocols)]]
- [[IS-IS (Intermediate System - Intermediate System)]]
- [[Cisco 路徑控制工具指南]]
- [[Cisco 路由重分配 (route redistribute)]]

#### Traceroute 工具

**功用**：檢查路由下一個 hop 是否正確

**原理**：利用 TTL 值來達到目的

![[Assets/00_Dashboard/Networking Overview/10-Traceroute 工具.png]]

##### Traceroute 運作原理

1. **發送封包**：發送 TTL=1 的封包到目的地
2. **第一跳回應**：第一個路由器收到後 TTL 減 1 變成 0，回傳 ICMP Time Exceeded
3. **逐步增加**：TTL=2, 3, 4... 逐步增加，直到到達目的地
4. **路徑追蹤**：每個 hop 都會回傳其 IP 位址，形成完整路徑

##### 使用範例

```cisco
# Cisco 設備使用 traceroute
Router#traceroute 8.8.8.8

# Windows 使用 tracert
C:\> tracert 8.8.8.8

# Linux 使用 traceroute
$ traceroute 8.8.8.8
```

### Transport Layer (傳輸層)

提供主機與主機之間端對端（end-to-end）可靠或不可靠的資料傳輸，負責分段與重組

#### 常見協定

參考資料：
- [[TCP 和 UDP 的基本介紹]]

### Application Layer (應用層)

應用程式所使用的通訊協定，例如 HTTP、DNS 等，處理應用層協定細節與介面

#### Cisco 裝置管理相關

參考資料：
- [[Cisco ACL 存取控制清單指南]]
- [[Cisco device Security]]
- [[Cisco DHCP 設定]]
- [[Cisco errordisable 指令使用]]
- [[Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定]]
- [[Cisco NTP 設定]]
- [[Cisco SPAN (Switch port Analysis) 簡介和使用方式]]
- [[Cisco ssh and telnet 安全設定]]
- [[Cisco Switch Security 設定]]
- [[Cisco SNMP (Simple Network Management Protocol) 介紹和使用方式]]
- [[Cisco Syslog 設定]]

#### 常見協定

參考資料：
- [[防火牆基本術語]]
- [[DNS (Domain Name Service)]]
- [[DHCP (Dynamic Host Configuration Protocol)]]
- [[HTTP (Hypertext Transfer Protocol)]]
- [[LDAP (Light-weight Directory Access Protocol)]]
- [[Windows Server NTP 網路時間協定設定指南]]
- [[PKI (Public key infrastructure)公鑰基礎設施]]
- [[RADIUS (Remote Authentication Dial In User Service)]]
- [[NTP (Network Time Protocol)]]
#### 網路模擬器

參考資料：
- [[EVE-NG 安裝與設定指南]]
- [[GNS3 安裝與設定指南]]
- [[Containerlab 安裝與設定指南]]


## Kubernetes Networking

[[K8s CNI 比較：Cilium、Calico 與 Flannel]]
[[K8s Cilium 與 Containerlab 網路實驗]]

## 網路認證與 VPN 排錯

[[induction-ldap|LDAP 入門]]
[[windows-l2tp-connect-error-720|Windows L2TP 錯誤 720]]
