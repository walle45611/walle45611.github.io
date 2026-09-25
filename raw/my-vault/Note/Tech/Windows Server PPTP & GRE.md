---
Type:
  - windows Server VPN
---
![[Assets/Note/Tech/Windows Server PPTP & GRE/01-windows Server VPN.png|01-windows Server VPN.png]]

### pptp

在 VPNS 上安裝VPN

```PowerShell
Install-WindowsFeature directAccess-VPN -IncludeManagementTools
```

OutClt

```PowerShell
Add-VpnConnection -Name "test" -TunnelType "pptp" -ServerAddress "192.0.0.1"
```