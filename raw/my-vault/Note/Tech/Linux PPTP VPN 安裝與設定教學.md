---
Type:
  - Linux VPN
OS:
  - Debian
---
## 📦 安裝 PPTP 伺服器

使用 apt 套件安裝 PPTP Daemon：

```bash
apt install pptpd
```

---

## 🌐 設定 DNS（可選）

若需自定義 DNS，例如讓 VPN 用戶端使用內部 DNS，可以編輯以下檔案：

```bash
vim /etc/ppp/pptpd-options
```

取消註解下列行或自行新增 DNS 伺服器：

```bash
ms-dns 8.8.8.8
ms-dns 8.8.4.4
```

> 此設定會在用戶端連線成功後，自動將 DNS 設為這裡指定的地址。

---

## 🖧 設定 VPN Server IP 範圍

設定伺服器本身的 VPN IP 與分配給用戶端的 IP 範圍：

```bash
vim /etc/pptpd.conf
```

範例如下：

```conf
localip 10.0.0.1
remoteip 10.0.0.100-200
```

- `localip`：VPN Server 的虛擬 IP（不與 LAN 衝突即可）
    
- `remoteip`：分配給連線進來的用戶端的 IP 範圍
    

---

## 🔐 新增帳號

在 `chap-secrets` 檔案中加入用戶名稱、密碼與允許的伺服器：

```bash
vim /etc/ppp/chap-secrets
```

格式：

```
<username>  <server>  <password>  <allowed IP>
```

範例：

```
client pptpd password *
```

- `client`：用戶名稱
    
- `pptpd`：伺服器識別名稱，通常設為 `pptpd`
    
- `password`：登入密碼
    
- `*`：允許從所有來源 IP 登入（也可指定 IP 限制）
    

---

## 🚀 啟用與重新啟動服務

```bash
systemctl enable pptpd
systemctl restart pptpd
```

---

## 🖥️ 用戶端連線指令（Linux）

使用 `pptpsetup` 工具建立連線（如未安裝可安裝 `pptp-linux`）：

```bash
pptpsetup --create your-vpn-name \
          --server your-vpn-server-ip \
          --username your-username \
          --password your-password \
          --encrypt \
          --start
```

### 說明：

- `--create`：設定的 VPN 名稱（本地識別用）
    
- `--server`：VPN Server 公網或內網 IP
    
- `--username` / `--password`：對應 `chap-secrets` 中的帳密
    
- `--encrypt`：使用 MPPE 加密通訊
    
- `--start`：立即發起連線
    

---

## 🧱 額外建議

- 建議搭配防火牆開放 TCP port `1723` 及 GRE 協定（protocol 47）
    
- 若出現 MTU 問題，可於用戶端加入 `mru 1400 mtu 1400` 等參數
    
- 若需支援多用戶同時連線，請確認 IP 分配範圍足夠，並設定 `localip` 不與 LAN 衝突
    

---

如需補充 Windows 用戶端設定、iptables NAT轉發、連線失敗排錯，我可以協助擴充！