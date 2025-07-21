---
title: 在 Windows 上正確設定 OpenSSH Server 和 Guacamole SFTP
tags:
  - System Administration
  - Windows
  - Network
  - SSH
categories: System Administration
abbrlink: 16f9
date: 2024-08-27 14:45:03
---

## 前情提要

我最近有遇到一個問題就是我 Windows 10 要使用 share devices 然後給外包廠商上傳檔案在 Guacamole 那麼我知道 Guacamole 有 SFTP 的服務可以串接讓外包廠商上傳檔案，但是中間一直連線失敗有點弄到快爆氣哈哈哈...，但是後面發現滿蠢的哈哈哈哈，廢話不多說開始吧。

<!--more-->

![圖 1. : Blog Image](https://imgur.com/X7H6aOq.png)

## Windows 10 / Windows Server 2019 安裝內建的 OpenSSH 工具

1. 可以先用以下指令確認有哪些內建的 OpenSSH 工具

    ```powershell
    Get-WindowsCapability -Online | Where-Object Name -like 'OpenSSH*'
    ```

    * 輸出內容 :

        ```text
        Name  : OpenSSH.Client~~~~0.0.1.0
        State : Installed

        Name  : OpenSSH.Server~~~~0.0.1.0
        State : Installed
        ```

2. 安裝 OpenSSH-Client，這個預設有些是安裝的

   ```powershell
    Add-WindowsCapability -Online -Name OpenSSH.Client~~~~0.0.1.0
   ```

3. 安裝 OpenSSH-Server，這樣的話就 Windows 就會有這個 Service

    ```powershell
    Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
    ```

4. 可以使用 `win+r` 然後輸入 `services.msc`

   1. 檢查是否有 `OpenSSH SSH Server` 這個 `Service`

        ![圖 2. : 檢查 Service](https://imgur.com/VG7CGgR.png)

   2. 然後開啟 Serivce 和調整成 Automatic，那麼有兩種啟動方式一種是 powershell 的方式，和 GUI 的方式

        ```powershell
        #啟動 sshd service
        Start-Service sshd

        # setting sshd Automatic start
        Set-Service -Name sshd -StartupType 'Automatic'
        ```

        ![圖 3. : 自動啟動 Service GUI](https://imgur.com/O7DQrVz.png){width=50%}

5. 再來確定防火牆規則吧！

    ```powershell
    Get-NetFirewallRule -Name "OpenSSH-Server-In-TCP" -ErrorAction SilentlyContinue | Select-Object Name, Enabled
    ```

    * 輸出內容 :

        ```text
        Name                  Enabled
        ----                  -------
        OpenSSH-Server-In-TCP    True
        ```

6. 可以先手動連線一下

    1. 先測試是否可以連線

        ```bash
        ssh Administrator@xxx.xxx.xxx.xxx
        ```

    2. 再來測試是否可以使用 sftp，並且確認是否有檔案到該使用者桌面

        ```scp
        scp test.txt Administrator@xxx.xxx.xxx.xxx:~/Desktop
        ```

## Guacamole SFTP 設定

那這邊的設定及其簡單，但是讓我崩潰的點可能是我的網路太慢了，所以導致我連線的時候會一直連線不上，但是不要慌去連接一個好網路就好了，這個問題我還特別去翻了 Windows Event 看 SSH 有沒有連線成功，但是都顯示連線成功，那時候真的就通靈了哈哈哈。

* 設定 Guacamole SFTP 那麼 hostname 當然要用自己需要遠端的 IP Address，至於我不用是因為我把它塗掉了。

    ![圖 4. Guacamole SFTP 設定](https://imgur.com/YONbogM.png)

* 設定成功後會按下 command + ctrl + shift + esc (mac)，就會有 share devices，圖示了那麼就可以上傳檔案到這台電腦了

    ![圖 5. 完成圖](https://imgur.com/4KhJy0H.png)

## 參考

* [如何在 Windows 架設高安全性的 SFTP (SSH File Transfer Protocol) 伺服器](https://blog.miniasp.com/post/2021/12/12/Enhanced-Security-for-SFTP-SSH-File-Transfer-Protocol-on-Windows)
* [如何在 Windows 正確的安裝與設定 OpenSSH Server 服務](https://blog.miniasp.com/post/2021/12/11/How-to-setup-OpenSSH-Server-in-Windows)
* [Get started with OpenSSH for Windows](https://learn.microsoft.com/en-us/windows-server/administration/openssh/openssh_install_firstuse?WT.mc_id=DT-MVP-4015686&tabs=powershell&pivots=windows-server-2025)