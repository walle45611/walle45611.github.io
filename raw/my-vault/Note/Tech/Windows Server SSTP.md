---
Type:
  - windows Server VPN
---
前置作業先將VPNS加入到DC網域內

在DC上安裝CA

```PowerShell
Install-WindowsFeature adcs-cert-authority,adcs-web-enrollment -IncludeManagementTools
```

在DC上的IIS打開Browsing功能

![[Assets/Note/Tech/Windows Server SSTP/01-Windows Server SSTP.png|01-Windows Server SSTP.png]]

![[Assets/Note/Tech/Windows Server SSTP/02-Windows Server SSTP.png|02-Windows Server SSTP.png]]

在DC上將CRL打開因為SSTP會自動下載CRL去比對vpn server憑證是否正確

![[Assets/Note/Tech/Windows Server SSTP/03-Windows Server SSTP.png|03-Windows Server SSTP.png]]

在DC上發布CRL

![[Assets/Note/Tech/Windows Server SSTP/04-Windows Server SSTP.png|04-Windows Server SSTP.png]]

![[Assets/Note/Tech/Windows Server SSTP/05-Windows Server SSTP.png|05-Windows Server SSTP.png]]

在VPNS上

記得將VPNS上的憑證renew這樣才會有CRL

![[Assets/Note/Tech/Windows Server SSTP/06-Windows Server SSTP.png|06-Windows Server SSTP.png]]

```PowerShell
Install-WindowsFeature directAccess-VPN,Routing -IncludeManagementTools
```

Config時選擇nat和VPN

![[Assets/Note/Tech/Windows Server SSTP/07-Windows Server SSTP.png|07-Windows Server SSTP.png]]

將192.168.10.1 nat 到 192.0.0.1這樣外網的人才可以下載到CRL

![[Assets/Note/Tech/Windows Server SSTP/08-Windows Server SSTP.png|08-Windows Server SSTP.png]]

![[Assets/Note/Tech/Windows Server SSTP/09-Windows Server SSTP.png|09-Windows Server SSTP.png]]

![[Assets/Note/Tech/Windows Server SSTP/10-Windows Server SSTP.png|10-Windows Server SSTP.png]]

DHCP relay記得要設定不然會失敗

OutClt

```PowerShell
Add-VpnConnection -name "sstp" -TunnelType "sstp" -ServerAddress "vpns1.skills39.com"
```

因為沒有模擬dns所以需要再hosts file動點手腳因為SSTP會檢查common name，所以需要這樣做

![[Assets/Note/Tech/Windows Server SSTP/11-Windows Server SSTP.png|11-Windows Server SSTP.png]]