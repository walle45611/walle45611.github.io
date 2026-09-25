## 設定時間及時區

- 日光節約時間DST一種在夏季月份犧牲正常的日出時間，而將時間調快的做法。通常使用夏令時間的地區，會在接近春季開始的時候，將時間調快一小時，並在秋季調回正常時間。實際上，夏令時間會造成在春季轉換當日的睡眠時間減少一小時，而在秋季轉換當日則會多出一小時的睡眠時間。
- 在設定NTP前應該要先將此設備較時，和把時區設定相同，這樣效果較佳，假設設定的時間為8.25 p.m.要先設定時區，甚至要告訴其DST，才啟用NTP
    
    ```Plain
    R1(config)#clock timezone EST -5
    R1(config)#clock summer-time EST recurring
    R1#clock set 20:52:49 21 October 2022
    ```
    
    1. 第一條指令**clock timezone EST -5**，EST這個參數可以隨意打，但是還是希望可以選用有意的值，最後**-5**是指比UTC(國際標準時間)前5個小時
    2. **clock summer-time EST recurring**，EST這個參數可以隨意打，但是還是希望可以選用有意的值，**recurring**表示在此期間時間會往前調整1小時
- 最後可以使用此命令查看時間時區
    
    ```Plain
    R1#show clock
    ```
    

## NTP用戶端、伺服器、用戶端\伺服器設定

NTP採用lookback interface會有更好的效果

![[Assets/Note/Tech/Cisco NTP 設定/01-NTP用戶端、伺服器、用戶端 伺服器設定.png|01-NTP用戶端、伺服器、用戶端 伺服器設定.png]]

- R1扮演NTP Client，R2是Server也是Client，R3是Server
    
    ```Plain
    !Configuration on R1:
    ntp server 172.16.2.2
    ```
    
    ```Plain
    !Configuration on R2:
    ntp server 172.16.3.3
    ```
    
    ```Plain
    !Configuration on R1:
    ntp master 2
    ```
    
- 檢視
    
    ```Plain
    show ntp associations
    show ntp status
    ```
    

## NTP 防護

- NTP server驗證
    
    ```Plain
    SW(config)#ntp authentication-key key-number md5 key-string
    SW(config)#ntp authenticate
    SW(config)#ntp trusted-key key-number
    SW(config)#ntp server ip-address key key-number
    ```
    
- ACL
    
    ```Plain
    Switch(config)#access-list acl-num permit ip-address mask
    SWitch(config)#ntp access-group {server-only | server | peer |query-only} acl-num
    ```
    
    - server-only : 只允許同步
    - server : 允許同步化與控制請求 ; 不允許與裝置同步期本身的時間時鐘
    - peer : 允許同步化與控制請求 ; 不允許裝置同步其本身的時間時鐘
    - query-only : 只允許控制請求

## SNTP (simplified network time protocol)

只是將ntp指令變成sntp

```Plain
SW(config)#sntp authentication-key key-number md5 key-string
SW(config)#sntp authenticate
SW(config)#sntp trusted-key key-number
SW(config)#sntp server ip-address key key-number
```