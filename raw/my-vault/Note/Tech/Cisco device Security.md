---
blog: true
blog_title: "Cisco device Security"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-device-security/
blog_tag: OS & Networking
blog_topic: Networking
---

  

## 關閉一些不使用的服務

在Cisco上跑了很多預設的服務系統，這時候你可能不需要某些服務，所以你可以選擇性的將這些服務關閉，因為這些服務不僅引響安全問題也會引向設備運行效率。

- 常見服務以及port

|服務|協議|PORT|
|---|---|---|
|TELNET|TCP|23|
|SSH|TCP|22|
|DHCP server|UDP|67|
|DHCP client|UDP|68|
|SNMP|UDP|161|
|HTTP|TCP|80|
|HTTPS|TCP|443|
|SNMP trap/inform receiver|UDP|162|

- 全部關閉CDP
    
    ```Shell
    Router(config)#no cdp run
    ```
    
- 在某些街口上關閉CDP，經常使用在連接其他廠商設備上也可以在連上ISP的那台設備關閉，以免設備資訊外流。
    
    ```Plain
    Router(config-if)#no cdp enable
    ```
    
- Cisco IOS XE 的 DHCP service 功能預設啟用；只有設定 DHCP pool 或 relay 等功能時才會提供對應服務。設備不需要 DHCP 功能時可關閉
    
    ```Plain
    Router(config)#no service dhcp
    ```
    
- 關閉HTTP服務，HTTP服務用於讓管理員使用Web葉面設備進行管理沒必要Cisco不推薦，所以就把關閉吧。
    
    ```Plain
    Router(config)#no ip http server
    ```
    
- 關閉HTTPS服務，跟HTTP一樣，所以就關吧，沒啥意義
    
    ```Plain
    Router(config)#no ip http secure-server
    ```
    
- 若啟用 Web 管理，使用 HTTPS 與本機驗證。HTTP Basic 認證使用 Base64 編碼傳送帳密，Base64 不是加密。
    
    ```Plain
    R(config)#ip http secure-server
    R(config)#ip http authentication local
    R(config)#username admin privilege 15 secret use-a-unique-secret
    ```
    

## 保護IOS密碼

- service password-encryption
    
    service password-encryption 會把部分明文密碼以 Type 7 可逆混淆形式存入設定，不是安全的雜湊。移除這項設定不會自動還原既有密碼；重新設定密碼後才會以目前設定方式儲存。
    
    - password password (console port 或是 vty mode)
    - enable name password password (global)
    - username name password password (global)
    - command
        
        ```Plain
        R1(config)#service password-encryption
        ```
        
- 單向密碼雜湊
    
    - enable secret 會使用該 IOS/IOS XE 版本預設的 secret 格式；若要指定雜湊類型，可明確設定 algorithm-type。
        
        ```Plain
        R1(config)#enable algorithm-type md5 secret [password]
        ```
        
    - Type 7 是可逆混淆；Type 8 使用 PBKDF2-SHA-256，Type 9 使用 scrypt。
        
        ```Plain
        R1(config)#enable algorithm-type sha256 secret [password]
        ```
        
    - 比較
        
        |命令|類型|演算法|
        |---|---|---|
        |enable algorithm-type md5 secret [password]|5|MD5（舊式雜湊）|
        |enable algorithm-type sha256 secret [password]|8|PBKDF2-SHA-256|
        |enable algorithm-type scrypt secret [password]|9|scrypt|

Type 5（MD5）與 Type 7 都不建議用於新設定；演算法可用性會依 IOS/IOS XE 平台與版本而異。參考：[Cisco Protecting Secrets](https://www.cisco.com/c/en/us/about/trust-center/resilient-infrastructure/protecting-secrets.html)、[Cisco IOS XE Hardening Guide](https://sec.cloudapps.cisco.com/security/center/resources/IOS_XE_hardening)。
