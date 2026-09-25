---
Type:
  - Linux RADIUS
OS:
  - Debian
---
## Cisco With Free**RADIUS**

> [!important] 模擬器非常難模擬，問題一堆

![[Assets/Note/Tech/Linux FreeRADIUS 設定指南/01-Cisco With Free RADIUS.png|01-Cisco With Free RADIUS.png]]

- freeRADUIS
    - 設定client 也就是 router 
        
        ```Shell
        client 設備的ipaddr {
        	secret = 連線AAA時的密碼
        	nastype = 認證的type
        	shortname = 設備的名稱
        }
        ```
        
        ![[Assets/Note/Tech/Linux FreeRADIUS 設定指南/02-Cisco With Free RADIUS.png|02-Cisco With Free RADIUS.png]]
        
    - 設定可以登入的使用者
        
        ```Shell
        cisco Cleartext-Password := "cisco"  //用戶cisco，密碼cisco
        Service-Type = NAS-Prompt-User,
        Cisco-AVPair = "shell:priv-lvl=15"  //用戶端權限為15
        ```
        
        ![[Assets/Note/Tech/Linux FreeRADIUS 設定指南/03-Cisco With Free RADIUS.png|03-Cisco With Free RADIUS.png]]
        
- cisco設定
    
    ```Shell
    conf t
    aaa new-model
    aaa group server radius AAA
     server-private 192.168.1.1 auth-port 1812 acct-port 1813 key secretkey
    aaa authentication login default group AAA
    aaa authorization exec default group AAA
    aaa accounting exec default start-stop group AAA
    aaa accounting system default start-stop group AAA
    line vty 0 4
    	transport input telnet ssh
    	login authentication default
    ```