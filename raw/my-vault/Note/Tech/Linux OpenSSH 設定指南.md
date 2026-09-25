---
Type:
  - Linux SSH
OS:
  - Debian
---

OpenSSH 是最常見的遠端連線協定之一，提供安全的 SSH 連線、SCP、SFTP 等功能。以下整理常見的 OpenSSH 設定方式，適用於 Linux 系統。

---

### 1. 安裝 OpenSSH Server

```bash
sudo apt update
sudo apt install openssh-server
```

啟用並設定開機自動啟動：

```bash
sudo systemctl enable ssh
sudo systemctl start ssh
```

檢查 SSH 狀態：

```bash
sudo systemctl status ssh
```

---

### 2. 設定檔位置

OpenSSH Server 的主要設定檔為：

```bash
/etc/ssh/sshd_config
```

修改完畢後請執行：

```bash
sudo systemctl restart sshd
```

---

### 3. 限制登入帳號與來源

```conf
# 禁止 root 登入（建議）
PermitRootLogin no

# 僅允許指定使用者登入
AllowUsers axer
AllowUsers root@10.200.*
AllowUsers root@192.168.10.1

# 或者只允許特定群組
AllowGroups sshusers
```

---

### 4. 金鑰登入設定（停用密碼登入）

建立 SSH 金鑰（用戶端操作）：

```bash
ssh-keygen -t rsa -b 4096
```

將金鑰複製到伺服器：

```bash
ssh-copy-id username@server_ip
```

或手動將 `~/.ssh/id_rsa.pub` 內容寫入伺服器的 `~/.ssh/authorized_keys`

修改伺服器設定，停用密碼登入：

```conf
PasswordAuthentication no
PubkeyAuthentication yes
```

---

### 5. 更改預設 SSH Port（預設為 22）

```conf
Port 2222
```

記得開放防火牆：

```bash
sudo ufw allow 2222/tcp
```

---

### 6. 防止暴力破解（搭配 Fail2ban）

安裝並啟用 fail2ban：

```bash
sudo apt install fail2ban
sudo systemctl enable fail2ban
```

建立設定檔 `/etc/fail2ban/jail.local`：

```ini
[sshd]
enabled = true
port    = ssh
logpath = /var/log/auth.log
maxretry = 5
```

---

### 7. 限制 SSH 最大連線數與速率

```conf
MaxAuthTries 3
MaxSessions 2
LoginGraceTime 30
```

---

### 8. 強化加密協定與禁用不安全版本

```conf
Protocol 2
PermitEmptyPasswords no
HostKey /etc/ssh/ssh_host_rsa_key
HostKey /etc/ssh/ssh_host_ed25519_key

# 僅允許安全演算法
Ciphers aes256-ctr,aes192-ctr,aes128-ctr
MACs hmac-sha2-512,hmac-sha2-256
KexAlgorithms curve25519-sha256@libssh.org
```

---

### 9. 常見問題與測試

重新載入設定：

```bash
sudo systemctl restart sshd
```

檢查設定是否正確：

```bash
sudo sshd -t
```

查看登入紀錄：

```bash
last
journalctl -u ssh
```

---

這些設定可確保 SSH 安全、只開放必要帳號與來源。搭配防火牆與 Fail2ban 可以有效防止暴力攻擊。