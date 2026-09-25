---
Type:
  - windows Server VPN
---
### DirectAccess topology

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/01-DirectAccess topology.png|01-DirectAccess topology.png]]

Window 10 的版本一定要是企業版專業版沒有DirectAccess的功能官網說的

[DirectAccess | Microsoft Docs](https://docs.microsoft.com/zh-tw/windows-server/remote/remote-access/directaccess/directaccess)

步驟

1. 先將ADDS裝起來在DC上和分享出出一個檔案
2. 將DirectAccess-VPN裝在DA上
3. 將CLT和DA放在內部網路加入網域update GPO
4. 在DC上創建一個directAccess的群組
5. 將CLT遷移到外部網路
6. 在DHCP上安裝DHCP和DNS
7. 測試在外部的CLT可不可以連線到分享的檔案或內部網頁

在DC上

```PowerShell
Install-WindowsFeature ad-domain-services,web-server -IncludeManagementTools
```

在創建完directaccess之後會再DC上自動新增兩個GPO

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/02-DirectAccess topology.png|02-DirectAccess topology.png]]

需要將directAccess Client Setting的WMI filter關掉，因為它會偵測電腦是否為筆電，如果不是他部會套用GPO

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/03-DirectAccess topology.png|03-DirectAccess topology.png]]

內部分享的資料夾

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/04-DirectAccess topology.png|04-DirectAccess topology.png]]

在DA上

```PowerShell
Install-WindowsFeature directAccess-VPN -IncludeManagementTools
```

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/05-DirectAccess topology.png|05-DirectAccess topology.png]]

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/06-DirectAccess topology.png|06-DirectAccess topology.png]]

打上外往IP

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/07-DirectAccess topology.png|07-DirectAccess topology.png]]

點here改東西

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/08-DirectAccess topology.png|08-DirectAccess topology.png]]

在DC上創建的群組

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/09-DirectAccess topology.png|09-DirectAccess topology.png]]

再加入網域後打上這個指令讓GPO刷新

```PowerShell
gpupdate /force
```

然後用這個指令查看GPO是否有刷新

```PowerShell
gpreslut /r
```

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/10-DirectAccess topology.png|10-DirectAccess topology.png]]

在DHCP上

```PowerShell
Install-WindowsFeature dhcp,dhs -IncludeManagementTools
```

在CLT上

```PowerShell
add-computer skills39.com
```

再加入網域後打上這個指令讓GPO刷新

```PowerShell
gpupdate /force
```

然後用這個指令查看GPO是否有刷新

```PowerShell
gpresult /r
```

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/11-DirectAccess topology.png|11-DirectAccess topology.png]]

將網卡切到外往

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/12-DirectAccess topology.png|12-DirectAccess topology.png]]

測試結果可以連到內部的分享

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/13-DirectAccess topology.png|13-DirectAccess topology.png]]

在CLT測試內部網頁

![[Assets/Note/Tech/Windows Server DirectAccess VPN 設定指南/14-DirectAccess topology.png|14-DirectAccess topology.png]]