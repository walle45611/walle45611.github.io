## iptables 檢查流程和 table 優先順序

![[Assets/Note/Tech/iptables 防火牆完整指南/01-iptables 檢查流程和 table 優先順序.png]]

## iptables Tables 和 Chain

|Chain|**filter (重要)**|**nat (重要)**|mangle|raw|
|---|---|---|---|---|
|**input**|✓||✓||
|**forward**|✓||✓||
|**output**|✓||✓|✓|
|**prerouting**||✓|✓|✓|
|**postrouting**||✓|✓||

> [!important] 處理優先順序 **raw > mangle > nat > filter**

## iptables 安裝

```bash
apt install iptables
update-alternatives --set iptables /usr/sbin/iptables-nft
update-alternatives --set ip6tables /usr/sbin/ip6tables-nft
update-alternatives --set arptables /usr/sbin/arptables-nft
update-alternatives --set ebtables /usr/sbin/ebtables-nft
```

## iptables 基本語法

```bash
iptables {-t TABLE} COMMAND CHAIN [number] 匹配條件 -j 處理動作
```

### 自定義 Chain

```bash
iptables -N test
iptables -A test -p tcp --dport 22 -j ACCEPT
iptables -A INPUT -d 172.16.10.10 -j test
```

### 匹配條件

#### 通用匹配

```bash
-s          # 來源 IP
-d          # 目標 IP
-p {tcp | udp | icmp}  # 協定
-i          # 輸入介面
-o          # 輸出介面
```

#### 擴展匹配

##### 1. 隱藏式擴展

**TCP 擴展**：

```bash
-p tcp
    --sport PORT_NUMBER      # 來源 port
    --dport PORT_NUMBER      # 目標 port
    --tcp-flags mask comp    
    --tcp-flags SYN,FIN,ACK,RST SYN,ACK  # 只檢查 mask 的標誌位
    --tcp-flags SYN,FIN,ACK,RST SYN == --syn
    --syn

    # 範例：-p tcp -m tcp --dport -> -p tcp --dport（隱藏了所以稱為隱藏式擴展）
```

**ICMP 擴展**：

```bash
-p icmp
    --icmp-type 
        8: Request
        0: Response
```

##### 2. 顯示式擴展（使用額外的匹配條件）

```bash
-m EXTENSION --spec-opt  # 大概的擴展語法
```

**狀態擴展**：

```bash
-m state :
    # 結合 ip_conntrack 追蹤會話的狀態
    NEW         # 新連接請求
    ESTABLISHED # 已建立的連線
    INVALID     # 非法連線 SYN=1,FIN=1
    RELATED     # 相關的連線
    
    # 範例
    -m state --state NEW -j ACCEPT                    # 允許狀態為 NEW
    -m state --state NEW,ESTABLISHED -j ACCEPT       # 允許狀態為 NEW,ESTABLISHED
```

**多 Port 擴展**：

```bash
-m multiport :
    --source-ports
    --destination-ports
    --ports
    
    # 範例
    -m multiport --destination-ports 21,22,80 -j ACCEPT
```

**範圍 IP 擴展**：

```bash
-m iprange :
    --src-range
    --dst-range
    
    # 範例
    iptables -A INPUT -p tcp -m iprange --src-range 172.16.100.3-172.16.100.100 --dport 22 -m state --state NEW,ESTABLISHED -j ACCEPT
```

**連接數限制擴展**：

```bash
-m connlimit :
    ! --connlimit-above {number}  # 通常會加驚嘆號代表反向
    
    # 範例
    iptables -A INPUT -d 172.16.100.7 -p tcp --dport 80 -m connlimit ! --connlimit-above 2 -j ACCEPT

-m limit
    --limit {RATE}
    --limit-burst {number}
    
    # 範例
    iptables -A INPUT -d 172.16.100.7 -p icmp --icmp-type 8 -m limit --limit 1/minute --limit-burst 6 -j ACCEPT
```

**限制封包內容文字擴展**：

```bash
-m string
    --algo {bm|kmp}
    --string "STRING"
    
    # 範例
    iptables -A OUTPUT -s 172.16.100.7 -m string --algo kmp --string "test" -j REJECT
```

### 命令類型

#### 管理規則

```bash
-A CHAIN               # 加一條規則到某條 CHAIN
-I CHAIN [number]      # 插入一條規則到某條 CHAIN
-D CHAIN [number]      # 從某條 CHAIN 刪除第 number 規則
-R CHAIN [number]      # 從某條 CHAIN 替換第 number 規則
```

#### 管理鏈

```bash
-F [chain]             # flush，清除指定規則鏈，如果省略 chain 則刪除所有
-X Name                # 刪除一個空的鏈，如果不是空的需要用 -F
-E Name                # 重命名自定義的鏈名稱
-Z chain               # 重置指定鏈中所有規則的計數器
-P [chain] {DROP | ACCEPT}  # 指定預設 chain 的規則
-N Name                # 指定一個新的鏈
```

#### 查看類型

```bash
-L                     # 顯示指定表中的規則
    -n                 # 以數字類型格式顯示主機地址和 port
    -v                 # 鏈和規則顯示詳細訊息
    -vv                # 更詳細資訊
    -x                 # 顯示計數器的精確數值
    --line-number      # 顯示行號
```

#### 動作

```bash
ACCEPT      # 允許
DROP        # 丟棄
REJECT      # 拒絕
DNAT        # 目的地位址轉換
SNAT        # 來源位址轉換
REDIRECT    # Port 重新定向
MASQUERADE  # 地址偽裝也是來源位址轉換
LOG         # 記錄日誌
    --log-prefix "STRING"
    # 範例：iptables -A INPUT -d 172.16.100.7 -p icmp --icmp-type 8 -j LOG --log-prefix "test"
MARK        # 標記
```

## iptables 儲存

```bash
apt install iptables-persistent
iptables-save > /etc/iptables/rules.v4     # 保存
iptables-restore < /etc/iptables/rules.v4  # 讀取
```

## 追蹤 iptables 狀態

ip_conntrack 模組追蹤封包，可以利用 `cat /proc/net/ip_conntrack` 來看有哪些封包經過 iptables，最大只能儲存 32768 個。

```bash
apt install iptstate
iptstate
```

## NAT 拓撲

![[Assets/Note/Tech/iptables 防火牆完整指南/02-NAT 拓撲.png]]

### SNAT (Source NAT)

![[Assets/Note/Tech/iptables 防火牆完整指南/03-SNAT (Source NAT).png]]

```bash
iptables -t nat -A POSTROUTING -o eth2 -s 192.168.10.0/24 -j SNAT --to 192.0.0.1
```

### DNAT (Destination NAT)

![[Assets/Note/Tech/iptables 防火牆完整指南/04-DNAT (Destination NAT).png]]

```bash
iptables -t nat -A PREROUTING -i eth2 -d 192.0.0.1 -p tcp --dport 80 -j DNAT --to 192.168.10.1:80
```

## DMZ 拓撲

![[Assets/Note/Tech/iptables 防火牆完整指南/05-DMZ 拓撲.png]]

### 流量控制表

|來源/目標|DMZ|internal|external|
|---|---|---|---|
|**DMZ**|**可以**|需要在 input 和 output、forward 開啟對應的 port|只允許需要的 port 號開放和開放 dmz 可以上網|
|**internal**|需要在 input 和 output、forward 開啟對應的 port|**可以**|只允許內部可以存取外部 (snat)|
|**external**|只允許需要的 port 號開放|**不可以**|**可以**|

### 流量拓撲

**紅色代表不允許、黃色代表可以但是需要轉送、綠色代表可以**

![[Assets/Note/Tech/iptables 防火牆完整指南/06-流量拓撲.png]]

## DMZ 簡易腳本

```bash
#!/bin/bash
# 清空 filter table
iptables -X
iptables -F
iptables -Z

# 清空 nat table
iptables -t nat -X
iptables -t nat -F
iptables -t nat -Z

# 預設全部都使用 DROP
iptables -P INPUT DROP
iptables -P FORWARD DROP
iptables -P OUTPUT DROP

# 讓 DMZ 和 internal 區域能夠上網
iptables -t nat -A POSTROUTING -o eth2 -s 192.168.10.0/24 -j SNAT --to 192.0.0.1
iptables -t nat -A POSTROUTING -o eth2 -s 192.168.1.0/24 -j SNAT --to 192.0.0.1

# 讓外部機器可以訪問內部 80 port 的 web server
iptables -t nat -A PREROUTING -i eth2 -d 192.168.10.0/24 -p tcp --dport 80 -j DNAT --to 192.168.10.2:80

# 讓 80 port 封包可以從 192.168.10.2 這台 web server 轉送
iptables -A FORWARD -i eth2 -s 0.0.0.0/0 -o eth1 -d 192.168.10.2 -p tcp --dport 80 -j ACCEPT
iptables -A FORWARD -o eth2 -d 0.0.0.0/0 -i eth1 -s 192.168.10.2 -p tcp --sport 80 -j ACCEPT

# 讓內部的機器可以訪問外部的 web server
iptables -A FORWARD -i eth0 -s 192.168.1.0/24 -o eth2 -d 0.0.0.0/0 -p tcp --dport 80 -j ACCEPT
iptables -A FORWARD -o eth0 -d 192.168.1.0/24 -i eth2 -s 0.0.0.0/0 -p tcp --sport 80 -j ACCEPT

# 讓內部的機器可以訪問 DMZ 的 web server
iptables -A FORWARD -i eth0 -s 192.168.1.0/24 -o eth1 -d 192.168.10.0/24 -p tcp --dport 80 -j ACCEPT
iptables -A FORWARD -o eth0 -d 192.168.1.0/24 -i eth1 -s 192.168.10.0/24 -p tcp --sport 80 -j ACCEPT

# 允許內部主機發出 http 請求
iptables -A INPUT -i eth0 -s 192.168.1.0/24 -p tcp --dport 80 -j ACCEPT
iptables -A OUTPUT -o eth0 -d 192.168.1.0/24 -p tcp --sport 80 -j ACCEPT

# 內部主機和 dmz 可以互 ping
iptables -A INPUT -i eth0 -p icmp -j ACCEPT
iptables -A OUTPUT -o eth0 -p icmp -j ACCEPT
iptables -A INPUT -i eth1 -p icmp -j ACCEPT
iptables -A OUTPUT -o eth1 -p icmp -j ACCEPT
```

### 測試結果

![[Assets/Note/Tech/iptables 防火牆完整指南/07-測試結果.png]] ![[Assets/Note/Tech/iptables 防火牆完整指南/08-測試結果.png]]

### 網域加入所需 Port

如果需要將 inClt 加入到網域，需要開啟這些 port，從 internal 到 dmz：

- **TCP 88** (Kerberos Key Distribution Center)
- **TCP 135** (Remote Procedure Call)
- **TCP 139** (NetBIOS Session Service)
- **TCP 389** (LDAP)
- **TCP 445** (SMB, Net Logon)
- **UDP 53** (DNS)
- **UDP 389** (LDAP, DC Locator, Net Logon)

## 推薦影片

- [金槍魚之夜：壞人的 iptables 小講堂](https://www.youtube.com/watch?v=w_vGD-96O54&ab_channel=TUNA)
- [金槍魚之夜：壞人的 iptables 小講堂第二彈](https://www.youtube.com/watch?v=Vnh8hYk6wZE&ab_channel=TUNA)