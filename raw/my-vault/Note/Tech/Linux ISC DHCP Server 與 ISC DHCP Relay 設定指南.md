## 先備知識

DHCP (Dynamic Host Configuration Protocol) 是一種網路協定，用於自動分配 IP 位址和其他網路設定給網路設備。本指南將介紹如何在 Linux 系統上設定 ISC DHCP Server 和 DHCP Relay。

## DHCP Server 設定

### 1. 安裝 DHCP 服務

```bash
apt update
apt install isc-dhcp-server
```

### 2. 設定 DHCP 預設網路介面

編輯 DHCP 服務的預設設定檔：

```bash
vim /etc/default/isc-dhcp-server
```

在檔案中指定要提供 DHCP 服務的網路介面：

![[Assets/Note/Tech/Linux ISC DHCP Server 與 ISC DHCP Relay 設定指南/01-2. 設定 DHCP 預設網路介面.png]]

**設定範例**：

```bash
# 指定 DHCP 服務要監聽的網路介面
INTERFACESv4="eth0"
INTERFACESv6=""
```

### 3. 設定 DHCP 主要配置

編輯 DHCP 主要設定檔：

```bash
vim /etc/dhcp/dhcpd.conf
```

![[Assets/Note/Tech/Linux ISC DHCP Server 與 ISC DHCP Relay 設定指南/02-3. 設定 DHCP 主要配置.png]]

#### 基本設定範例

```bash
# 全域設定
default-lease-time 600;
max-lease-time 7200;
authoritative;

# DNS 設定
option domain-name "example.com";
option domain-name-servers 8.8.8.8, 8.8.4.4;

# 子網路設定
subnet 192.168.1.0 netmask 255.255.255.0 {
    # IP 位址範圍
    range 192.168.1.100 192.168.1.200;
    
    # 預設閘道
    option routers 192.168.1.1;
    
    # 子網路遮罩
    option subnet-mask 255.255.255.0;
    
    # 廣播位址
    option broadcast-address 192.168.1.255;
    
    # 租約時間
    default-lease-time 86400;
    max-lease-time 172800;
}
```

#### 進階設定選項

```bash
# 多個子網路設定
subnet 192.168.2.0 netmask 255.255.255.0 {
    range 192.168.2.50 192.168.2.150;
    option routers 192.168.2.1;
    option domain-name-servers 192.168.2.1;
}

# 排除特定 IP 位址
subnet 192.168.1.0 netmask 255.255.255.0 {
    range 192.168.1.100 192.168.1.120;
    range 192.168.1.140 192.168.1.200;
    option routers 192.168.1.1;
}

# PXE 開機設定
option space PXE;
option PXE.mtftp-ip    code 1 = ip-address;
option PXE.mtftp-cport code 2 = unsigned integer 16;
option PXE.mtftp-sport code 3 = unsigned integer 16;
option PXE.mtftp-tmout code 4 = unsigned integer 8;
option PXE.mtftp-delay code 5 = unsigned integer 8;

# TFTP 伺服器設定
next-server 192.168.1.10;
filename "pxelinux.0";
```

### 4. MAC 位址綁定設定

為特定設備綁定固定 IP 位址：

![[Assets/Note/Tech/Linux ISC DHCP Server 與 ISC DHCP Relay 設定指南/03-4. MAC 位址綁定設定.png]] ![[Assets/Note/Tech/Linux ISC DHCP Server 與 ISC DHCP Relay 設定指南/04-4. MAC 位址綁定設定.png]]

#### 設定範例

```bash
# 主機特定設定
host desktop-pc {
    hardware ethernet 00:11:22:33:44:55;
    fixed-address 192.168.1.10;
    option host-name "desktop-pc";
}

host laptop {
    hardware ethernet AA:BB:CC:DD:EE:FF;
    fixed-address 192.168.1.11;
    option host-name "laptop";
    option routers 192.168.1.1;
    option domain-name-servers 8.8.8.8;
}

host printer {
    hardware ethernet 12:34:56:78:90:AB;
    fixed-address 192.168.1.20;
    option host-name "office-printer";
}
```

### 5. 重啟 DHCP 服務

套用設定變更：

```bash
# 重啟服務
systemctl restart isc-dhcp-server

# 檢查服務狀態
systemctl status isc-dhcp-server

# 啟用開機自動啟動
systemctl enable isc-dhcp-server
```

## DHCP Relay 設定

DHCP Relay 用於轉發不同子網路間的 DHCP 請求。

### 1. 安裝 DHCP Relay

```bash
apt install isc-dhcp-relay
```

### 2. 設定 DHCP Relay

編輯 DHCP Relay 設定檔：

```bash
vim /etc/default/isc-dhcp-relay
```

#### 基本設定

```bash
# DHCP 伺服器位址
SERVERS="192.168.1.10"

# 要監聽的網路介面
INTERFACES="eth0 eth1"

# 其他選項
OPTIONS=""
```

### 3. 進階 Relay 設定

```bash
# 多個 DHCP 伺服器
SERVERS="192.168.1.10 192.168.1.11"

# 指定特定介面
INTERFACES="eth0 eth1 eth2"

# 附加選項
OPTIONS="-d -q"
```

### 4. 重啟 DHCP Relay 服務

```bash
systemctl restart isc-dhcp-relay
systemctl status isc-dhcp-relay
systemctl enable isc-dhcp-relay
```

## 常用管理指令

### DHCP Server 管理

```bash
# 檢查設定檔語法
dhcpd -t -cf /etc/dhcp/dhcpd.conf

# 檢視租約資訊
cat /var/lib/dhcp/dhcpd.leases

# 檢視服務日誌
journalctl -u isc-dhcp-server -f

# 手動啟動 DHCP 服務（除錯模式）
dhcpd -f -d -cf /etc/dhcp/dhcpd.conf eth0
```

### DHCP Relay 管理

```bash
# 檢查 Relay 狀態
systemctl status isc-dhcp-relay

# 檢視 Relay 日誌
journalctl -u isc-dhcp-relay -f

# 手動啟動 Relay（除錯模式）
dhcrelay -d -i eth0 -i eth1 192.168.1.10
```

## 故障排除

### 常見問題與解決方案

#### 1. DHCP 服務無法啟動

**檢查步驟**：

```bash
# 檢查設定檔語法
dhcpd -t

# 檢查網路介面設定
ip addr show

# 檢查防火牆設定
ufw status
iptables -L
```

#### 2. 客戶端無法取得 IP

**檢查步驟**：

```bash
# 檢查 DHCP 服務狀態
systemctl status isc-dhcp-server

# 檢查網路連通性
ping 192.168.1.1

# 檢查 UDP 67 埠是否開啟
netstat -ulnp | grep :67
```

#### 3. Relay 無法轉發請求

**檢查步驟**：

```bash
# 檢查 Relay 設定
cat /etc/default/isc-dhcp-relay

# 檢查路由設定
ip route show

# 檢查封包轉發
echo 1 > /proc/sys/net/ipv4/ip_forward
```

## 安全設定建議

### 1. 防火牆設定

```bash
# 允許 DHCP 服務
ufw allow 67/udp
ufw allow 68/udp

# 限制特定來源
ufw allow from 192.168.1.0/24 to any port 67
```

### 2. 存取控制

```bash
# 在 dhcpd.conf 中設定存取限制
class "allowed-clients" {
    match hardware;
}

subclass "allowed-clients" 1:00:11:22:33:44:55;
subclass "allowed-clients" 1:AA:BB:CC:DD:EE:FF;

subnet 192.168.1.0 netmask 255.255.255.0 {
    pool {
        allow members of "allowed-clients";
        range 192.168.1.100 192.168.1.200;
    }
}
```

### 3. 日誌監控

```bash
# 設定詳細日誌
vim /etc/dhcp/dhcpd.conf

# 加入日誌設定
log-facility local7;

# 設定 rsyslog
echo "local7.*    /var/log/dhcpd.log" >> /etc/rsyslog.conf
systemctl restart rsyslog
```

## 設定檔範本

### 完整的 dhcpd.conf 範例

```bash
# /etc/dhcp/dhcpd.conf
default-lease-time 600;
max-lease-time 7200;
authoritative;

# DNS 設定
option domain-name "local.lan";
option domain-name-servers 8.8.8.8, 8.8.4.4;

# 主要子網路
subnet 192.168.1.0 netmask 255.255.255.0 {
    range 192.168.1.100 192.168.1.200;
    option routers 192.168.1.1;
    option subnet-mask 255.255.255.0;
    option broadcast-address 192.168.1.255;
    default-lease-time 86400;
    max-lease-time 172800;
}

# 伺服器子網路
subnet 192.168.10.0 netmask 255.255.255.0 {
    range 192.168.10.50 192.168.10.100;
    option routers 192.168.10.1;
    option domain-name-servers 192.168.10.1;
    default-lease-time 3600;
}

# 固定 IP 分配
host file-server {
    hardware ethernet 00:50:56:12:34:56;
    fixed-address 192.168.1.10;
    option host-name "file-server";
}

host web-server {
    hardware ethernet 00:50:56:AB:CD:EF;
    fixed-address 192.168.1.11;
    option host-name "web-server";
}
```

## 效能最佳化

### 1. 記憶體設定

```bash
# 在 dhcpd.conf 中設定
min-lease-time 300;
default-lease-time 3600;
max-lease-time 86400;
```

### 2. 資料庫最佳化

```bash
# 定期清理租約檔案
# 建立清理腳本
cat > /usr/local/bin/dhcp-cleanup.sh << 'EOF'
#!/bin/bash
systemctl stop isc-dhcp-server
cp /var/lib/dhcp/dhcpd.leases /var/lib/dhcp/dhcpd.leases.backup
dhcpd-pools -c /etc/dhcp/dhcpd.conf -l /var/lib/dhcp/dhcpd.leases -f t -o c > /tmp/dhcp-active.leases
mv /tmp/dhcp-active.leases /var/lib/dhcp/dhcpd.leases
systemctl start isc-dhcp-server
EOF

chmod +x /usr/local/bin/dhcp-cleanup.sh

# 設定定時任務
echo "0 2 * * 0 /usr/local/bin/dhcp-cleanup.sh" >> /etc/crontab
```
