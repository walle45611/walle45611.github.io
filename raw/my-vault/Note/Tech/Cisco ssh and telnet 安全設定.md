---
blog: true
blog_title: "Cisco ssh and telnet 安全設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-ssh-telnet-security/
blog_tag: OS & Networking
blog_topic: Networking
---

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