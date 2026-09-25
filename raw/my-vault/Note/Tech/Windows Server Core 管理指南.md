## Windows Server Core 概述

Windows Server Core 是 Windows Server 的最小化安裝選項，提供無圖形使用者介面的伺服器環境。它具有較小的攻擊面、更低的資源消耗，以及更少的維護需求，適合用於專門的伺服器角色部署。

## Server Core 支援的服務

Windows Server Core 可以支援以下主要服務和角色：

### 目錄服務

- **ADCS** (Active Directory Certificate Services) - 憑證服務
- **ADDS** (Active Directory Domain Services) - 網域服務
- **ADLDS** (Active Directory Lightweight Directory Services) - 輕量型目錄服務
- **ADRMS** (Active Directory Rights Management Services) - 權限管理服務

### 網路服務

- **DHCP** - 動態主機配置協定
- **DNS** - 網域名稱系統
- **RRAS** (Routing and Remote Access Service) - 路由與遠端存取服務

### 檔案與列印服務

- **File Services** - 檔案服務
- **列印服務** - 列印和文件服務

### Web 與應用程式服務

- **IIS** (Internet Information Services) - 包含 ASP.NET 支援

### 虛擬化與更新服務

- **Hyper-V** - 虛擬化平台
- **WSUS** (Windows Server Update Services) - 更新服務

## Sconfig 設定工具

### 啟動 Sconfig

Sconfig 是 Server Core 的主要設定工具，提供文字模式的設定介面。

```powershell
# 設定 Sconfig 自動啟動
Set-SConfig -AutoLaunch $true
```

### Sconfig 主要功能

Sconfig 提供以下設定選項：

#### 1. 網域/工作群組設定

- 加入網域或工作群組
- 變更電腦名稱
- 網域帳戶管理

#### 2. 網路設定

- IP 位址設定
- DNS 伺服器設定
- 網路介面卡管理

#### 3. Windows Update 設定

- 自動更新設定
- 更新下載和安裝選項

#### 4. 遠端管理設定

- 遠端桌面設定
- Windows Remote Management (WinRM)
- PowerShell 遠端管理

#### 5. 系統資訊和狀態

- 系統資訊檢視
- 網路設定檢視
- 服務狀態監控

## PowerShell 管理指令

### 基本系統管理

```powershell
# 檢視系統資訊
Get-ComputerInfo

# 變更電腦名稱
Rename-Computer -NewName "SERVER01" -Restart

# 加入網域
Add-Computer -DomainName "contoso.com" -Credential (Get-Credential) -Restart

# 檢視網路設定
Get-NetIPConfiguration

# 設定靜態 IP
New-NetIPAddress -InterfaceAlias "Ethernet" -IPAddress "192.168.1.100" -PrefixLength 24 -DefaultGateway "192.168.1.1"
Set-DnsClientServerAddress -InterfaceAlias "Ethernet" -ServerAddresses "192.168.1.10", "192.168.1.11"
```

### 角色和功能管理

```powershell
# 檢視可用的功能
Get-WindowsFeature

# 安裝 DNS 伺服器角色
Install-WindowsFeature DNS -IncludeManagementTools

# 安裝 DHCP 伺服器角色
Install-WindowsFeature DHCP -IncludeManagementTools

# 安裝 Active Directory 網域服務
Install-WindowsFeature AD-Domain-Services -IncludeManagementTools

# 安裝檔案服務
Install-WindowsFeature File-Services -IncludeManagementTools

# 安裝 IIS
Install-WindowsFeature IIS-WebServerRole -IncludeManagementTools

# 安裝 Hyper-V
Install-WindowsFeature Hyper-V -IncludeManagementTools -Restart
```

### 遠端管理設定

```powershell
# 啟用 PowerShell 遠端管理
Enable-PSRemoting -Force

# 設定 WinRM
winrm quickconfig

# 啟用遠端桌面
Set-ItemProperty -Path "HKLM:\System\CurrentControlSet\Control\Terminal Server" -Name "fDenyTSConnections" -Value 0
Enable-NetFirewallRule -DisplayGroup "Remote Desktop"

# 檢視防火牆規則
Get-NetFirewallRule | Where-Object {$_.Enabled -eq "True"}
```

## 服務設定範例

### DNS 伺服器設定

```powershell
# 安裝 DNS 角色
Install-WindowsFeature DNS -IncludeManagementTools

# 建立正向查詢區域
Add-DnsServerPrimaryZone -Name "contoso.com" -ZoneFile "contoso.com.dns"

# 建立反向查詢區域
Add-DnsServerPrimaryZone -NetworkId "192.168.1.0/24" -ZoneFile "1.168.192.in-addr.arpa.dns"

# 新增 A 記錄
Add-DnsServerResourceRecordA -ZoneName "contoso.com" -Name "server01" -IPv4Address "192.168.1.100"
```

### DHCP 伺服器設定

```powershell
# 安裝 DHCP 角色
Install-WindowsFeature DHCP -IncludeManagementTools

# 建立 DHCP 範圍
Add-DhcpServerv4Scope -Name "LAN Scope" -StartRange "192.168.1.100" -EndRange "192.168.1.200" -SubnetMask "255.255.255.0"

# 設定 DHCP 選項
Set-DhcpServerv4OptionValue -ScopeId "192.168.1.0" -Router "192.168.1.1" -DnsServer "192.168.1.10" -DnsDomain "contoso.com"

# 啟用 DHCP 伺服器
Set-DhcpServerv4Binding -BindingState $true -InterfaceAlias "Ethernet"
```

### 檔案服務設定

```powershell
# 安裝檔案服務
Install-WindowsFeature File-Services, FS-FileServer -IncludeManagementTools

# 建立共享資料夾
New-Item -Path "C:\Shares\Data" -ItemType Directory
New-SmbShare -Name "Data" -Path "C:\Shares\Data" -FullAccess "Everyone"

# 設定 NTFS 權限
$Acl = Get-Acl "C:\Shares\Data"
$AccessRule = New-Object System.Security.AccessControl.FileSystemAccessRule("Domain Users", "Modify", "ContainerInherit,ObjectInherit", "None", "Allow")
$Acl.SetAccessRule($AccessRule)
Set-Acl "C:\Shares\Data" $Acl
```

## 監控和維護

### 系統監控

```powershell
# 檢視系統效能
Get-Counter "\Processor(_Total)\% Processor Time"
Get-Counter "\Memory\Available MBytes"
Get-Counter "\LogicalDisk(C:)\% Free Space"

# 檢視事件日誌
Get-WinEvent -LogName System -MaxEvents 50
Get-WinEvent -LogName Application -MaxEvents 50

# 檢視服務狀態
Get-Service | Where-Object {$_.Status -eq "Running"}
```

### 更新管理

```powershell
# 檢查可用更新
Get-Module PSWindowsUpdate -ListAvailable
Import-Module PSWindowsUpdate
Get-WUInstall -AcceptAll -AutoReboot

# 使用 WSUS
Install-WindowsFeature UpdateServices -IncludeManagementTools
```

## 遠端管理工具

### 從 Windows 10/11 管理

```powershell
# 安裝 RSAT (Remote Server Administration Tools)
Get-WindowsCapability -Name RSAT* -Online | Add-WindowsCapability -Online

# 使用 MMC 連接到遠端伺服器
mmc.exe

# PowerShell 遠端連線
Enter-PSSession -ComputerName "SERVER01" -Credential (Get-Credential)

# 使用 Windows Admin Center
# 透過瀏覽器 https://admincenter.contoso.com 進行管理
```

### 第三方管理工具

- **Putty** - SSH 連線工具
- **Windows Terminal** - 現代化終端機
- **Visual Studio Code** - 使用 PowerShell 擴充功能

## 效能最佳化

### 記憶體管理

```powershell
# 檢視記憶體使用情況
Get-Process | Sort-Object WorkingSet -Descending | Select-Object -First 10

# 設定虛擬記憶體
$ComputerSystem = Get-WmiObject Win32_ComputerSystem -EnableAllPrivileges
$ComputerSystem.AutomaticManagedPagefile = $False
$ComputerSystem.Put()
```

### 磁碟管理

```powershell
# 檢視磁碟使用情況
Get-Volume

# 磁碟重組
Optimize-Volume -DriveLetter C -Defrag

# 清理暫存檔案
Remove-Item -Path "$env:TEMP\*" -Recurse -Force -ErrorAction SilentlyContinue
```

## 安全性設定

### 基本安全性

```powershell
# 設定本機安全性原則
secedit /export /cfg C:\temp\security.cfg

# 設定密碼原則
net accounts /minpwlen:8 /maxpwage:90 /minpwage:1

# 檢視安全性事件
Get-WinEvent -LogName Security -MaxEvents 100 | Where-Object {$_.Id -eq 4624}
```

### 防火牆設定

```powershell
# 檢視防火牆狀態
Get-NetFirewallProfile

# 啟用防火牆
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled True

# 新增防火牆規則
New-NetFirewallRule -DisplayName "Allow HTTP" -Direction Inbound -Protocol TCP -LocalPort 80 -Action Allow
```

## 故障排除

### 常見問題

#### 1. 網路連線問題

```powershell
# 測試網路連線
Test-NetConnection -ComputerName "8.8.8.8" -Port 53
Test-Connection -ComputerName "google.com"

# 檢視網路設定
ipconfig /all
```

#### 2. 服務啟動問題

```powershell
# 檢查服務狀態
Get-Service -Name "Spooler"

# 重新啟動服務
Restart-Service -Name "Spooler"
```

#### 3. 磁碟空間不足

```powershell
# 檢查磁碟空間
Get-WmiObject -Class Win32_LogicalDisk | Select-Object DeviceID, @{Name="Size(GB)";Expression={[math]::Round($_.Size/1GB,2)}}, @{Name="FreeSpace(GB)";Expression={[math]::Round($_.FreeSpace/1GB,2)}}

# 清理系統檔案
cleanmgr /sagerun:1
```

## 最佳實務建議

1. **定期備份**：設定定期備份重要資料和系統設定
2. **監控資源**：定期檢查 CPU、記憶體和磁碟使用情況
3. **安全更新**：及時安裝安全性更新
4. **日誌管理**：定期檢查系統和應用程式日誌
5. **遠端管理**：設定適當的遠端管理工具和權限
6. **文檔記錄**：記錄所有設定變更和維護活動

Windows Server Core 雖然沒有圖形介面，但透過 PowerShell 和適當的管理工具，仍能提供完整且強大的伺服器功能。