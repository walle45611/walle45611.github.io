- 比賽常用禁止定時斷開，ssh或是telnet連線
    
    ```Plain
    Router#no exec-timeout
    ```
    
- 限制遠端控制
    
    ```Plain
    switch(config)# access-list 99 premit host 192.168.1.100
    switch(config)#line vty 0 4
    switch(config-line)# access-class 99 in
    ```