---
Type:
  - windows Server VPN
---
## l2tp/ipsec pre-share-key

  

## l2tp/ipsec certificate

  

## IKEv2

在DC上安裝CA

```PowerShell
Install-WindowsFeature adcs-cert-authority,adcs-web-enrollment -IncludeManagementTools
```

在DC上創建複製IPSec範本並改內容

![[Assets/Note/Tech/Windows Server IPSec 設定指南/01-IKEv2.png|01-IKEv2.png]]

在DC上改範本名稱

![[Assets/Note/Tech/Windows Server IPSec 設定指南/02-IKEv2.png|02-IKEv2.png]]

在DC上可匯出密鑰

![[Assets/Note/Tech/Windows Server IPSec 設定指南/03-IKEv2.png|03-IKEv2.png]]

在DC上增加伺服器驗證功能

![[Assets/Note/Tech/Windows Server IPSec 設定指南/04-IKEv2.png|04-IKEv2.png]]

在DC上改權限

![[Assets/Note/Tech/Windows Server IPSec 設定指南/05-IKEv2.png|05-IKEv2.png]]

在DC上支援請求

![[Assets/Note/Tech/Windows Server IPSec 設定指南/06-IKEv2.png|06-IKEv2.png]]

![[Assets/Note/Tech/Windows Server IPSec 設定指南/07-IKEv2.png|07-IKEv2.png]]

在DC上安裝DHCP

```PowerShell
Install-WindowsFeature dhcp -IncludeManagementTools
Add-DhcpServerv4Scope -Name "internal" -StartRange 192.168.10.100 -EndRange 192.168.10.200 -SubnetMask 255.255.255.0
Set-DhcpServerv4OptionValue -Router 192.168.10.254 -DnsDomain "sayms.local" -DnsServer 192.168.10.1
```

在VPNS上安裝

```PowerShell
Install-WindowsFeature directAccess-VPN -IncludeManagementToolsInstall-WindowsFeature directAccess-VPN -IncludeManagementTools
```

申請憑證

![[Assets/Note/Tech/Windows Server IPSec 設定指南/08-IKEv2.png|08-IKEv2.png]]

打勾IKEv2

![[Assets/Note/Tech/Windows Server IPSec 設定指南/09-IKEv2.png|09-IKEv2.png]]

加上common name和dns

![[Assets/Note/Tech/Windows Server IPSec 設定指南/10-IKEv2.png|10-IKEv2.png]]

允許IKEv2在rrasmgmt.msc

![[Assets/Note/Tech/Windows Server IPSec 設定指南/11-IKEv2.png|11-IKEv2.png]]

右鍵DHCP Relay Agent → Properties → 加入DHCP Server

右鍵DHCP Relay Agent → New interface → LAN

![[Assets/Note/Tech/Windows Server IPSec 設定指南/12-IKEv2.png|12-IKEv2.png]]

在VPNS上安裝NPAS

```PowerShell
Install-WindowsFeature npas -IncludeManagementTools將測
```

複製範本

![[Assets/Note/Tech/Windows Server IPSec 設定指南/13-IKEv2.png|13-IKEv2.png]]

改成這樣

![[Assets/Note/Tech/Windows Server IPSec 設定指南/14-IKEv2.png|14-IKEv2.png]]

加入你要的全組或使用者

![[Assets/Note/Tech/Windows Server IPSec 設定指南/15-IKEv2.png|15-IKEv2.png]]

OutClt

```PowerShell
Add-VpnConnection -Name "test" -TunnelType "ikev2" -ServerAddress "192.0.0.1"
```

OutClt的憑證要匯入

![[Assets/Note/Tech/Windows Server IPSec 設定指南/16-IKEv2.png|16-IKEv2.png]]