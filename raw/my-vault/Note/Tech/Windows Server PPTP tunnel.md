---
Type:
  - windows Server P2P
---
![[Assets/Note/Tech/Windows Server PPTP tunnel/01-windows Server P2P.png|01-windows Server P2P.png]]

### pptp/GRE tunnel

安裝VPN和路由表

```PowerShell
Install-WindowsFeature directAccess-VPN,Routing -IncludeManagementTools
```

VPNS1

![[Assets/Note/Tech/Windows Server PPTP tunnel/02-pptp GRE tunnel.png|02-pptp GRE tunnel.png]]

名稱不能亂取

![[Assets/Note/Tech/Windows Server PPTP tunnel/03-pptp GRE tunnel.png|03-pptp GRE tunnel.png]]

選擇VPN

![[Assets/Note/Tech/Windows Server PPTP tunnel/04-pptp GRE tunnel.png|04-pptp GRE tunnel.png]]

選擇對方的public ip也就是VPNS2的public ip

![[Assets/Note/Tech/Windows Server PPTP tunnel/05-pptp GRE tunnel.png|05-pptp GRE tunnel.png]]

繞送和撥入都要打勾

![[Assets/Note/Tech/Windows Server PPTP tunnel/06-pptp GRE tunnel.png|06-pptp GRE tunnel.png]]

增加static routing table對方的子網路也就是VPNS2的內部網路

![[Assets/Note/Tech/Windows Server PPTP tunnel/07-pptp GRE tunnel.png|07-pptp GRE tunnel.png]]

設定撥入使用者密碼帳號是interface所以第一步驟不能亂設定

![[Assets/Note/Tech/Windows Server PPTP tunnel/08-pptp GRE tunnel.png|08-pptp GRE tunnel.png]]

對方撥入的使用者密碼和帳號

![[Assets/Note/Tech/Windows Server PPTP tunnel/09-pptp GRE tunnel.png|09-pptp GRE tunnel.png]]

按下connect

![[Assets/Note/Tech/Windows Server PPTP tunnel/10-pptp GRE tunnel.png|10-pptp GRE tunnel.png]]

VPNS2步驟如同VPNS1