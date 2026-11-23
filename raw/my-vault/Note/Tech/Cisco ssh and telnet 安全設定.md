---
blog: true
blog_title: "Cisco ssh and telnet 安全設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-ssh-telnet-security/
blog_tag: OS & Networking
blog_topic: Networking
---

- 以下設定會停用 VTY 閒置逾時，SSH 或 Telnet 連線可能一直占用 VTY。正式環境通常保留合理的逾時值。
    
    ```Plain
    Router(config)#line vty 0 4
    Router(config-line)#exec-timeout 0 0
    ```
    
- 限制遠端控制
    
    ```Plain
    switch(config)# access-list 99 permit host 192.168.1.100
    switch(config)#line vty 0 4
    switch(config-line)# access-class 99 in
    ```
