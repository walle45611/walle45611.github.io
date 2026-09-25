# **IPAM介紹**

IPAM代表"IP地址管理"，是一種用於管理和監控網絡中IP地址的工具或系統。它可以幫助企業或組織更有效地管理其IP地址，並減少由IP地址冲突或錯誤配置引起的網絡問題。

IPAM系統通常包括以下功能：

1. IP地址分配：IPAM系統可以自動分配IP地址，也可以手動分配IP地址。此外，它還可以管理IP地址池以確保可用性和節省IP地址。
2. IP地址跟踪和監控：IPAM系統可以監控IP地址的使用情況，並警告用戶使用超出預期的IP地址或在網絡中使用重複的IP地址。
3. DNS和DHCP管理：IPAM系統可以集成DNS和DHCP服務器，以便更好地管理IP地址和主機名稱。
4. 报告和分析：IPAM系統可以提供關於IP地址使用情況和趨勢的報告和分析，幫助用戶更好地了解其網絡。

總之，IPAM系統可以提高網絡的可靠性和安全性，同時減少網絡管理的工作量和成本。

# IPAM安裝

1. 安裝
    
    ```PowerShell
    Install-WindowsFeature ipam -includeManagement
    ```
    
2. Connect to IPAM Server
    
    ![[Assets/Note/Tech/Windows Server IPAM/01-IPAM安裝.png|01-IPAM安裝.png]]
    
3. GPO
    
    ```PowerShell
    Invoke-IpamGpoProvisioning -Domain skills39.com -gpoprefixname SKILLS39_IPAM -ipamServerFqdn ipam.skills39.coom -delegetedgpouuser administrator@skills39.com
    ```
    
4. 新增server
    1. 新增按鈕
        
        ![[Assets/Note/Tech/Windows Server IPAM/02-IPAM安裝 - 新增按鈕.png|02-IPAM安裝 - 新增按鈕.png]]
        
    2. 新增
        
        ![[Assets/Note/Tech/Windows Server IPAM/03-IPAM安裝 - 新增.png|03-IPAM安裝 - 新增.png]]
        
    3. 新增後 這裡應為紅色的
        
        ![[Assets/Note/Tech/Windows Server IPAM/04-IPAM安裝 - 新增後 這裡應為紅色的.png|04-IPAM安裝 - 新增後 這裡應為紅色的.png]]
        
5. 再新增的server上
    
    ```PowerShell
    gpupdate /force
    restart-computer now
    ```
    

---

# 問題解決

> [!important] Domain Controller不能裝
> 
> > [!info] Server 2012 - IPAM failed on provisioning - Alexandre VIOT  
> > IPAM failed to install with error Provisioning IPAM has failed.  
> > [http://www.alexandreviot.net/2015/07/17/server-2012-ipam-failed-on-provisioning/](http://www.alexandreviot.net/2015/07/17/server-2012-ipam-failed-on-provisioning/)  

# 推薦影片

> [!info] 22. Install and Configure IPAM in Windows Server 2019  
> Video Series on Advance Networking with Windows Server 2019:  
> [https://www.youtube.com/watch?v=8qoAfsAzrew&ab_channel=MSFTWebCast](https://www.youtube.com/watch?v=8qoAfsAzrew&ab_channel=MSFTWebCast)