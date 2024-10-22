---
title: DOMjudge 安裝指南
categories: [競賽系統]
tags: [DOMjudge, 安裝, Linux, Shell Script]
date: 2024-10-22
---

![圖 1. : Blog Image](https://imgur.com/YJsR9lN.png)

## 簡介

DOMjudge 是一個開源的程式設計競賽管理系統，通常用於各類程式設計競賽，例如 ACM ICPC。本篇文章將介紹如何透過自動化 Shell 腳本來在 Ubuntu 系統上安裝 DOMjudge 競賽系統。我們會逐步解釋每個步驟，幫助讀者了解整個過程的每一個細節。

### 安裝 DOMServer 流程

#### 0. 簡介

此為 DOMjudge 的計分版和登入的主程式所以之後對外開放需要透過此台讓參賽選手登入和操作。

#### 1. 安裝必要套件

首先更新系統，然後安裝包括 MariaDB、Apache、PHP 等在內的必要套件，這些套件是 DOMServer 正常運行所需的。

```bash
sudo apt-get update
sudo apt-get install -y libcgroup-dev build-essential acl zip unzip mariadb-server apache2 \
    php php-fpm php-gd php-cli php-intl php-mbstring php-mysql php-curl php-json php-xml \
    php-zip composer ntp ssh pkg-config make
```

#### 2. 下載並安裝 DOMjudge

使用 `wget` 下載 DOMjudge 壓縮檔案，然後使用 `tar` 解壓縮。

```bash
wget https://www.domjudge.org/releases/domjudge-8.3.1.tar.gz
tar -zxf domjudge-8.3.1.tar.gz
cd domjudge-8.3.1
```

#### 3. 設定 MariaDB

利用 `mysql_secure_installation` 工具來增強 MariaDB 的安全性，包含設定 root 密碼、移除匿名使用者、禁止遠端 root 登入等。

```bash
sudo mysql_secure_installation <<EOF

y
y
$mariadb_root_password
$mariadb_root_password
y
y
y
y
EOF
```

#### 4. 對 DOMjudge 進行編譯並安裝 DOMServer

```bash
./configure --prefix=/opt/domjudge --with-domjudge-user=root
make domserver
sudo make install-domserver
```

#### 5. 設置 DOMjudge 資料庫

使用 DOMjudge 提供的工具生成隨機密碼，並安裝 DOMjudge 資料庫，記得要打上自己的密碼。

```bash
sudo /opt/domjudge/domserver/bin/dj_setup_database genpass
sudo /opt/domjudge/domserver/bin/dj_setup_database -u root -p"$mariadb_root_password" install
```

#### 6. 設定 Apache 和 PHP FPM

創建符號連結，將 DOMjudge 的 Apache 和 PHP FPM 設定文件添加到系統設定中，並啟用所需的模組，最後重新加載服務以應用變更。

```bash
sudo ln -s /opt/domjudge/domserver/etc/apache.conf /etc/apache2/conf-available/domjudge.conf
sudo ln -s /opt/domjudge/domserver/etc/domjudge-fpm.conf /etc/php/8.3/fpm/pool.d/domjudge.conf
sudo a2enmod proxy_fcgi setenvif rewrite
sudo a2enconf php8.3-fpm domjudge
sudo service apache2 reload
sudo service php8.3-fpm reload
```

### 安裝 Judgehost 流程

#### 0. 簡介

Judgehost 節點負責自動化測試參賽者提交的程式碼。

#### 1. 安裝必要軟體包

更新系統並安裝 Judgehost 所需的軟體包，包括 PHP、Java、GHC 等編譯環境。

```bash
sudo apt-get update
sudo apt-get install -y make pkg-config debootstrap libcgroup-dev \
    php-cli php-curl php-json php-xml php-zip lsof procps gcc g++ \
    openjdk-8-jre-headless openjdk-8-jdk ghc fp-compiler libjsoncpp-dev build-essential
```

#### 2. 下載並安裝 Judgehost

下載 DOMjudge 並安裝 Judgehost，使用 `configure` 設定安裝目錄，並使用 `make` 進行編譯安裝。

```bash
wget https://www.domjudge.org/releases/domjudge-8.3.1.tar.gz
tar -zxf domjudge-8.3.1.tar.gz
cd domjudge-8.3.1
./configure --prefix=/opt/domjudge --with-domjudge-user=$USER
make judgehost
sudo make install-judgehost
```

#### 4. 設定 Judgehost 環境

添加 `domjudge-run` 使用者，並將 `sudoers-domjudge` 設定文件複製到 `/etc/sudoers.d`，使 Judgehost 能正確執行需要的命令。

```bash
cd /opt/domjudge/judgehost/etc
sudo useradd -d /nonexistent -U -M -s /bin/false domjudge-run
sudo cp sudoers-domjudge /etc/sudoers.d
```

#### 5. 創建 chroot 環境

使用 DOMjudge 提供的工具創建隔離的 chroot 環境，以保證 Judgehost 可以安全地執行提交的程式碼。

```bash
sudo /opt/domjudge/judgehost/bin/dj_make_chroot
```

#### 7. 設定 GRUB

```bash
sudo sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="[^"]*"/GRUB_CMDLINE_LINUX_DEFAULT="quiet splash cgroup_enable=memory swapaccount=1 systemd.unified_cgroup_hierarchy=0"/' /etc/default/grub
  sudo update-grub
```

#### 8. 設定 REST API KEY

```bash
vim /opt/domjudge/judgehost/etc/restapi.secret"

default $new_api_url $new_user $new_password"
```

#### 9. 創建 judgedaemon 的 systemd 服務

創建一個 `systemd` 服務，來管理 judgedaemon 的啟動和停止。

```bash
cat <<EOL | sudo tee /etc/systemd/system/judgedaemon.service >/dev/null
[Unit]
Description=DOMjudge Judgedaemon Service
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=/opt/domjudge/judgehost
ExecStart=/opt/domjudge/judgehost/bin/judgedaemon
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOL
```

#### 8. 安裝完成後重新開機

```bash
reboot
```

### 自動化腳本
如果您希望更輕鬆地安裝 DOMjudge 系統，可以參考我在 GitHub 上發布的自動化腳本：[DOMjudge 安裝腳本](https://github.com/walle45611/DOMjudge-AutoInstall)。

此 GitHub 專案包含完整的安裝腳本和相關說明，能幫助您快速完成 DOMjudge 的安裝與設定，減少手動操作並避免潛在錯誤。

### 結論

透過這兩個 Shell 腳本，我們可以輕鬆地在 Ubuntu 上自動化安裝並設定 DOMjudge 競賽系統及其 Judgehost 節點。這些腳本不僅節省了大量的手動設定時間，還大幅減少了因操作失誤而導致安裝失敗的風險。希望這篇文章對有需要搭建競賽系統的讀者有所幫助！