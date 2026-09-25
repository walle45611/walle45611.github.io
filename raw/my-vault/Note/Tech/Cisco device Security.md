  

## 關閉一些不使用的服務

在Cisco上跑了很多預設的服務系統，這時候你可能不需要某些服務，所以你可以選擇性的將這些服務關閉，因為這些服務不僅引響安全問題也會引向設備運行效率。

- 常見服務以及port

|服務|協議|PORT|
|---|---|---|
|TELNET|TCP|23|
|SSH|TCP|22|
|DHCP|UDP|67|
|SNMP|UDP|161|
|DHCP|UDP|67|
|HTTP|TCP|80|
|HTTPS|TCP|443|

- 全部關閉CDP
    
    ```Shell
    Router(config)no cdp run
    ```
    
- 在某些街口上關閉CDP，經常使用在連接其他廠商設備上也可以在連上ISP的那台設備關閉，以免設備資訊外流。
    
    ```Plain
    Router(config-if)#no cdp enable
    ```
    
- 把不必要的DHCP關閉，默認是開的
    
    ```Plain
    Router(config)#no ip service dhcp
    ```
    
- 關閉HTTP服務，HTTP服務用於讓管理員使用Web葉面設備進行管理沒必要Cisco不推薦，所以就把關閉吧。
    
    ```Plain
    Router(config)#no ip http server
    ```
    
- 關閉HTTPS服務，跟HTTP一樣，所以就關吧，沒啥意義
    
    ```Plain
    Router(config)#no ip http server security
    ```
    
- 如果要開啟http可以做驗證，但是密碼只是base64編碼…
    
    ```Plain
    R(config)#ip http server
    R(config)#ip http authentication local
    R(config)#username admin privilege 15 password ccna
    ```
    

## 保護IOS密碼

- service password-encryption
    
    某些舊式的IOS密碼安全會產生問題，可以使用service password-encryption這條global。對明文的密碼進行加密會加密以下密碼，加密後的編碼類型為 「7」，但是這種方法早已被破解，這種方法如果使用no service password-encryption不會立即顯示出明文的密碼需要等到更改密碼後:
    
    - password password (console port 或是 vty mode)
    - enable name password password (global)
    - username name password password (global)
    - command
        
        ```Plain
        R1(config)#service password-encryption
        ```
        
- hash encryption
    
    - MD5 加密後的編碼類型為 「5」
        
        ```Plain
        R1(config)#enable secret [password]
        ```
        
    - sha256 MD5 加密後的編碼類型為 「7」
        
        ```Plain
        R1(config)#enable algorithm-type-sha-256 secret [password]
        ```
        
    - 比較
        
        |命令|類型|演算法|
        |---|---|---|
        |enable [algorithm-type md5] secret [password]|5|md5|
        |enable algorithm-type sha-256 secret [password]|8|sha256|
        |enable algorithm-type scrypt secret [password]|9|sha256|