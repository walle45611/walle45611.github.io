## 概述

基於 Windows 上的 Linux 子系統，這個工具通常用來做開發使用或是 DevOps，並不會拿來架站，不推薦使用

## 開啟虛擬平台功能和支援 WSL

### Windows PowerShell

```PowerShell
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux virtualmachineplatform
```

### Windows Server

```PowerShell
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

### Windows Server GUI

![[Assets/Note/Tech/WSL2 安裝指南/01-Windows Server GUI.png]]

### Windows GUI

![[Assets/Note/Tech/WSL2 安裝指南/02-Windows GUI.png]]

## 下載 Linux kernel update package

### 下載連結

```
https://wslstorestorage.blob.core.windows.net/wslblob/wsl_update_x64.msi
```

### 安裝

![[Assets/Note/Tech/WSL2 安裝指南/03-安裝.png]]

### 設定預設為 WSL2

```PowerShell
wsl --set-default-version 2
```

## 下載 Linux 發行版

### 下載 Debian

```PowerShell
Invoke-WebRequest -Uri https://aka.ms/wsl-debian-gnulinux -OutFile Ubuntu.appx -UseBasicParsing
```

### 離線安裝（無網路環境）

#### 解壓縮 appx 檔案

```PowerShell
Expand-Archive -LiteralPath "C:\Users\Administrator\Desktop\debian.zip" -DestinationPath C:\Users\Administrator\Desktop\debian\
```

#### 改成 zip 格式並解壓縮

![[Assets/Note/Tech/WSL2 安裝指南/04-改成 zip 格式並解壓縮.png]]

#### 解壓縮後的檔案

![[Assets/Note/Tech/WSL2 安裝指南/05-解壓縮後的檔案.png]]

#### 執行 Debian，創建使用者帳號和密碼

![[Assets/Note/Tech/WSL2 安裝指南/06-執行 Debian,創建使用者帳號和密碼.png]]

#### 加入到環境變數

![[Assets/Note/Tech/WSL2 安裝指南/07-加入到環境變數.png]]

## 參考資料

> [!info] WSL2 官方文檔 舊版 WSL 的手動安裝步驟
> 
> 為了簡單起見，我們通常會建議使用 wsl --install 來安裝 Windows 子系統 Linux 版，但如果您執行較舊的 Windows 組建，可能不受支援。我們已包含下列手動安裝步驟。如果您在安裝程式期間遇到問題，請檢查疑難排解指南的安裝區段。您必須先啟用「Windows 子系統 Linux 版」選用功能，然後才能在 Windows 上安裝任何 Linux 發行版本。以系統管理員身分開啟 PowerShell ([開始] 功能表 > PowerShell > 以滑鼠右鍵按一下 > [以系統管理員身分執行])，然後輸入下列命令：dism.
> 
> 參考連結：https://docs.microsoft.com/zh-tw/windows/wsl/install-manual

> [!info] PowerShell 解壓縮指令 Expand-Archive (Microsoft.PowerShell.Archive) - PowerShell
> 
> ```
> Expand-Archive [-Path] [[-DestinationPath] ] [-Force] [-PassThru] [-WhatIf] [-Confirm] [ ]
> Expand-Archive -LiteralPath [[-DestinationPath] ] [-Force] [-PassThru] [-WhatIf] [-Confirm] [ ]
> ```
> 
> The Expand-Archive cmdlet extracts files from a specified zipped archive file to a specified destination folder.
> 
> 參考連結：https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.archive/expand-archive?view=powershell-7.2