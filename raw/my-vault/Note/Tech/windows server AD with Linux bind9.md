- vim /etc/bind/named.conf.local
    
    ![[Assets/Note/Tech/windows server AD with Linux bind9/01-vim etc bind named.conf.local.png|01-vim etc bind named.conf.local.png]]
    
- vim /var/cache/bind/dynamic/skills39.com
    
    ![[Assets/Note/Tech/windows server AD with Linux bind9/02-vim var cache bind dynamic skills39.png|02-vim var cache bind dynamic skills39.png]]
    
- vim /var/cache/bind/dynamic/_msccs.skills39.com
    
    ![[Assets/Note/Tech/windows server AD with Linux bind9/03-vim var cache bind dynamic msccs.ski.png|03-vim var cache bind dynamic msccs.ski.png]]
    
- 權限設定
    
    ![[Assets/Note/Tech/windows server AD with Linux bind9/04-權限設定.png|04-權限設定.png]]
    

> [!important] 安裝windows AD時不要勾選dns server，且ip設定dns指向linux bind9

> [!info] Using Linux BIND DNS Servers for Active Directory Domains - Serverlab  
> Learn how to configure a BIND DNS domain for Active Directory on a Linux server, and also the pros and cons of doing so.  
> [https://www.serverlab.ca/tutorials/linux/network-services/using-linux-bind-dns-servers-for-active-directory-domains/](https://www.serverlab.ca/tutorials/linux/network-services/using-linux-bind-dns-servers-for-active-directory-domains/)