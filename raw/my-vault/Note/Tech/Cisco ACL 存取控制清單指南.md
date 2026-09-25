
## ACL 概念

- **ACL 只能抓取網路號，無法精確匹配遮罩**
- 假設有兩條路由：`192.168.1.0/24` 和 `192.168.1.0/30`，只要抓取 `192.168.1.0/24` 使用 ACL 無法實現，只能兩條都抓，或都不抓
- **在每條 ACL 後都隱藏了 `deny any`**

### ACL 編號分類

|ACL 類型|編號範圍|說明|
|---|---|---|
|標準型 ACL|1-99|只能比對來源 IP|
|擴充型 ACL|100-199|可比對來源、目的地 IP 及協定|
|新增標準型|1300-1999|擴充的標準型編號範圍|
|新增擴充型|2000-2699|擴充的擴充型編號範圍|
|命名型 ACL|-|使用名稱而非編號|

### 改良的編輯方式

使用序號（sequence number）進行精確的 ACL 條目管理。

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/01-改良的編輯方式.png]]

## WC (Wildcard Mask)

Wildcard mask 是一個用於決定哪些 IP 位址該精確匹配的 32 位元數值：

- **0 代表精確匹配**
- **1 代表忽略該位元**
- 通常用於 ACL、OSPF 和 EIGRP 等地方

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/02-WC (Wildcard Mask).png]]

### 計算範例

**問題**：有一個子網 `172.16.8.0 255.255.252.0`，WC mask 為何？

**答案**：WC mask = `0.0.3.255`

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/03-計算範例.png]]

## 標準型 ACL

### 基本語法

```cisco
Router(config)#access-list access-list-number {permit | deny} source [wildcard-mask]
```

#### 參數說明

- **編號選擇**：1-99 或 1300-1999
- **通配符**：若無指定，預設為 `0.0.0.0`（精確匹配）
- **刪除 ACL**：`no access-list access-list-number`

### 套用到介面

```cisco
Router(config-if)#ip access-group access-list-number {in | out}
```

#### 參數說明

- **in**：套用到進入介面的封包
- **out**：套用到離開介面的封包（預設）
- **移除 ACL**：`no ip access-group access-list-number`

### 設定範例

```cisco
# 允許任何封包
Router(config)#access-list 1 permit any

# 允許 10.1.1.1
Router(config)#access-list 1 permit 10.1.1.1

# 禁止 10.1.1.0/24 網段
Router(config)#access-list 1 deny 10.1.1.0 0.0.0.255

# 使用 ACL 抓取感興趣的流量
Router(config)#access-list 1 permit 192.168.1.0
```

### 查看 ACL

```cisco
Router#show ip access-lists
```

## 擴充型 ACL

擴充型 ACL 可以比對：

- **來源 IP**
- **目的地 IP**
- **協定類型**

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/04-擴充型 ACL - 協定類型.png]]

### 基本語法

```cisco
access-list access-list-number {deny | permit} protocol source source-wildcard destination destination-wildcard [log | log-input]
```

#### 參數說明

- **access-list-number**：100-199 或 2000-2699

### TCP/UDP 專用語法

```cisco
access-list access-list-number {deny | permit} {tcp | udp} source source-wildcard [operator [port]] destination destination-wildcard [operator [port]] [established] [log]
```

### 套用到介面

```cisco
Router(config-if)#ip access-group access-list-number {in | out}
```

### 設定範例

```cisco
# 拒絕所有 TCP 封包
Router(config)#access-list 101 deny tcp any any

# 拒絕從主機 1.1.1.1 到 2.2.2.2 的封包
Router(config)#access-list 101 deny ip host 1.1.1.1 host 2.2.2.2

# 拒絕從子網 1.1.1.0/24 到任何地點的 UDP 封包
Router(config)#access-list 101 deny udp 1.1.1.0 0.0.0.255 any
```

## TCP 與 UDP Port 對照

|Port|Protocol|Application|ACL 關鍵字|
|---|---|---|---|
|20|TCP|FTP Data|ftp-data|
|21|TCP|FTP Control|ftp|
|23|TCP|Telnet|telnet|
|25|TCP|SMTP|smtp|
|53|UDP/TCP|DNS|domain|
|67|UDP|DHCP Client|bootpc|
|68|UDP|DHCP Server|bootps|
|69|UDP|TFTP|tftp|
|80|TCP|HTTP|www|
|110|TCP|POP3|pop3|
|161|UDP|SNMP|snmp|
|443|TCP|HTTPS|443|

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/05-TCP 與 UDP Port 對照.png]]

### Port 相關範例

```cisco
# 拒絕任意來源 port > 1023 到主機 10.1.1.1:23 的 TCP 連線
Router(config)#access-list 101 deny tcp any gt 1023 host 10.1.1.1 eq 23
Router(config)#access-list 101 deny tcp any host 10.1.1.1 eq telnet

# 添加描述
Router(config)#access-list 101 remark Block Telnet Access
```

## 命名型 ACL

命名型 ACL 基本功能與編號型相同，但採用子命令模式，更容易管理。

### Extended 命名型 ACL

```cisco
Router(config)#ip access-list extended barney
Router(config-ext-nacl)#permit tcp host 10.1.1.2 eq www any
Router(config-ext-nacl)#deny udp host 10.1.1.1 10.1.2.0 0.0.0.255
Router(config-ext-nacl)#deny ip 10.1.3.0 0.0.0.255 10.1.2.0 0.0.0.255
Router(config-ext-nacl)#deny ip 10.1.2.0 0.0.0.255 10.2.3.0 0.0.0.255
Router(config-ext-nacl)#permit ip any any
Router(config-ext-nacl)#exit
Router(config)#interface serial 1/0
Router(config-if)#ip access-group barney out
```

#### 移除命名型 ACL 條目

```cisco
Router(config)#ip access-list extended barney
Router(config-ext-nacl)#no deny ip 10.1.2.0 0.0.0.255 10.2.3.0 0.0.0.255
```

### Standard 命名型 ACL

```cisco
Router(config)#ip access-list standard 24
Router(config-std-nacl)#permit 10.1.1.0 0.0.0.255    # 隱藏編號 10
Router(config-std-nacl)#permit 10.1.2.0 0.0.0.255    # 隱藏編號 20
Router(config-std-nacl)#permit 10.1.3.0 0.0.0.255    # 隱藏編號 30
```

#### 使用序列號管理

```cisco
# 移除序列號 20 的條目
Router(config-std-nacl)#no 20

# 指定序列號新增條目
Router(config-std-nacl)#5 deny 10.1.1.1
```

## Object Group

Object Group 是減少 ACL 條目的工具，可以將多個網路、服務分組管理。

### Network Object Group

```cisco
object-group network net1
    host 1.1.1.1
    2.2.2.0 255.255.255.0
    range 3.3.3.100 3.3.3.200
```

### 巢狀 Object Group

```cisco
object-group network net2
    group-object net1
    group-object net3
```

### Service Object Group

```cisco
object-group service ser1
    gre
    ospf
    tcp eq 23
    tcp eq 22
    icmp echo-reply
```

### 在 ACL 中使用 Object Group

```cisco
ip access-list extended test2
    permit object-group ser1 any object-group net1
```

## 其他功能

### 重新排序序列號

```cisco
Router(config)#ip access-list resequence forccna 10 10
```

### 清除計數器

```cisco
Router#clear access-list counters forccna
```

### 記錄功能

```cisco
# log-input 會增加進入介面的資訊
Router(config)#access-list 101 deny ip any host 1.1.1.1 log-input
```

### 時間存取控制

```cisco
Router(config)#time-range TIME
Router(config-time-range)#periodic weekdays 09:00 to 18:00
Router(config)#access-list 102 permit tcp any any eq 80 time-range TIME
```

### 註解功能

```cisco
Router(config)#ip access-list extended Test
Router(config-ext-nacl)#remark permit traffic for BJSEC webserver
Router(config-ext-nacl)#permit tcp any host 1.1.1.1 eq www
```

## 實際案例

![[Assets/Note/Tech/Cisco ACL 存取控制清單指南/06-實際案例.png]]

### 需求分析

1. **允許 TCP-based internet responses for all networks**
2. **Allow incoming SMTP traffic to the public IP address 202.100.1.101**
3. **Allow incoming HTTP/HTTPS traffic to the public IP address 202.100.1.102**
4. **Prevent IP spoofing attack by denying RFC 1918 addresses from internet**

### RFC 1918 私有位址範圍

|起始位址|結束位址|網路前綴|
|---|---|---|
|10.0.0.0|10.255.255.255|10/8|
|172.16.0.0|172.31.255.255|172.16/12|
|192.168.0.0|192.168.255.255|192.168/16|

### 設定實作

```cisco
GW(config)#ip access-list extended forccna
GW(config-ext-nacl)#permit tcp any any established          # 允許已建立的 TCP 連線 (ACK 或 RST)
GW(config-ext-nacl)#permit tcp any host 202.100.1.101 eq smtp    # 允許 SMTP 流量
GW(config-ext-nacl)#permit tcp any host 202.100.1.102 eq www     # 允許 HTTP 流量
GW(config-ext-nacl)#permit tcp any host 202.100.1.102 eq 443     # 允許 HTTPS 流量
GW(config-ext-nacl)#5 deny ip 10.0.0.0 0.255.255.255 any        # 拒絕 10/8 私有位址
GW(config-ext-nacl)#6 deny ip 172.16.0.0 0.15.255.255 any       # 拒絕 172.16/12 私有位址
GW(config-ext-nacl)#7 deny ip 192.168.0.0 0.0.255.255 any       # 拒絕 192.168/16 私有位址
```

> [!important] 最佳實務
> 
> - 標準型 ACL 應套用在靠近目的地的位置
> - 擴充型 ACL 應套用在靠近來源的位置
> - 記得在 ACL 最後會有隱含的 `deny any`
> - 使用 `established` 關鍵字來允許回應流量