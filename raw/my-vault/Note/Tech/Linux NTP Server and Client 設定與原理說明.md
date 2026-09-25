---
Type:
  - Linux NTP
OS:
  - Debian
---
## 🕰️ NTP Server（使用 `chronyd`）

### 🔧 安裝與啟用

```bash
apt install chrony -y
systemctl enable chronyd
systemctl start chronyd
```

### 🖋️ 設定 `/etc/chrony/chrony.conf`

```bash
vim /etc/chrony/chrony.conf
```

修改以下內容：

```conf
# 設定上游同步來源（可為國家時間中心）
server ntp.nict.jp iburst

# 允許內網用戶端連線
allow 192.168.0.0/16

# 本機回應 IPv4 NTP 要求
bindcmdaddress 127.0.0.1
bindcmdaddress ::1
bindaddress 0.0.0.0
```

儲存後重啟：

```bash
systemctl restart chronyd
```

### ✅ 驗證 Server 狀態

```bash
chronyc sources -v
```

---

## 🧭 NTP Client 設定（使用 `systemd-timesyncd`）

### 編輯時間同步設定檔

```bash
vim /etc/systemd/timesyncd.conf
```

![[Assets/Note/Tech/Linux NTP Server and Client 設定與原理說明/01-編輯時間同步設定檔.png]]

確認最下方加入：

```ini
NTP=your.ntp.server.ip
```

### 檢查服務狀態

```bash
systemctl status systemd-timesyncd.service
```

![[Assets/Note/Tech/Linux NTP Server and Client 設定與原理說明/02-檢查服務狀態.png]]

### 檢查同步結果

```bash
timedatectl timesync-status
```

![[Assets/Note/Tech/Linux NTP Server and Client 設定與原理說明/03-檢查同步結果.png]]

`System clock synchronized` 應為 `yes`

```bash
timedatectl status
timedatectl set-ntp yes
```

![[Assets/Note/Tech/Linux NTP Server and Client 設定與原理說明/04-檢查同步結果.png]]

---

## 🔁 自動排程同步（選用）

可透過 crontab 定期手動強制與 NTP Server 同步時間（非必要）：

```bash
crontab -e
```

加入：

```bash
0 * * * * /usr/sbin/ntpdate -u your.ntp.server.ip
```

> 建議優先使用 `systemd-timesyncd` 或 `chronyd` 提供的自動同步功能。