---
Type:
  - Linux Basic
OS:
  - Debian
---
### 將網卡名稱改成eth0

![[Assets/Note/Tech/Linux Ubuntu or Debian Set hostname and IP address/01-將網卡名稱改成eth0.png|01-將網卡名稱改成eth0.png]]

- ATTR{address}==””這個是這個網卡的mac address
- KERNEL==””這個是需要改名的nic 可以使用 `ip a > example` 到某個檔案複製mac address
- NAME=””這個就是設定你要的名子

---

### 設定IP

![[Assets/Note/Tech/Linux Ubuntu or Debian Set hostname and IP address/02-設定IP.png|02-設定IP.png]]

---

## 設定Hostname

```Shell
sudo hostnamectl set-hostname ldap.mydomain.local
```