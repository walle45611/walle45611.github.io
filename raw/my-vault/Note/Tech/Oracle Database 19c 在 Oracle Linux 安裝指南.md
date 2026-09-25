
> [!info] Oracle Database 19c Download for Linux x86-64 | Oracle 台灣  
> [https://www.oracle.com/tw/database/technologies/oracle19c-linux-downloads.html](https://www.oracle.com/tw/database/technologies/oracle19c-linux-downloads.html)

---

## 📦 1. 建立 YUM repo 檔案

```bash
vim /etc/yum.repos.d/ol8-temp.repo
```

```ini
[ol8_baseos_latest]
name=Oracle Linux 8 BaseOS Latest ($basearch)
baseurl=https://yum.oracle.com/repo/OracleLinux/OL8/baseos/latest/$basearch/
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-oracle
gpgcheck=1
enabled=1

[local_ol8_appstream]
name=Oracle Linux AppStream $releasever Latest ($basearch)
baseurl=https://yum.oracle.com/repo/OracleLinux/OL8/appstream/$basearch/
gpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-oracle
gpgcheck=1
enabled=1
```

---

## 🔐 2. 匯入 GPG 金鑰

```bash
curl -o /etc/pki/rpm-gpg/RPM-GPG-KEY-oracle https://yum.oracle.com/RPM-GPG-KEY-oracle-ol8
```

---

## 🏗️ 3. 安裝 Oracle Database 19c

```bash
dnf update -y
dnf install -y oracle-database-preinstall-19c
export ORACLE_DOCKER_INSTALL=true
dnf localinstall oracle-database-ee-19c-1.0-1.x86_64.rpm
/etc/init.d/oracledb_ORCLCDB-19c configure
```

---

## 👤 4. 設定 oracle 使用者環境

切換至 oracle 使用者並編輯：

```bash
vim ~/.bash_profile
```

```bash
umask 022
export ORACLE_SID=ORCLCDB
export ORACLE_BASE=/opt/oracle/oradata
export ORACLE_HOME=/opt/oracle/product/19c/dbhome_1
export PATH=$PATH:$ORACLE_HOME/bin
```

---

## 🔄 5. 重新載入環境變數

```bash
source ~/.bash_profile
```

---

## 🎧 6. 設定 LISTENER

```bash
vim $ORACLE_HOME/network/admin/listener.ora
```

```ini
SID_LIST_LISTENER =
  (SID_LIST =
    (SID_DESC =
      (GLOBAL_DBNAME = ORCLCDB)
      (ORACLE_HOME = /opt/oracle/product/19c/dbhome_1)
      (SID_NAME = ORCLCDB)
    )
  )
```

---

## 🚀 7. 重啟 listener 並開放防火牆通訊埠

```bash
lsnrctl stop
lsnrctl start
firewall-cmd --zone=public --add-port=1521/tcp --permanent
firewall-cmd --reload
```

---

完成以上步驟後，Oracle 19c 應已成功在 Oracle Linux 上安裝完成。可使用 SQL*Plus 或外部工具（如 SQL Developer）連線使用。