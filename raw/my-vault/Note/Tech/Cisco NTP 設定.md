---
blog: true
blog_title: "Cisco NTP 設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-ntp-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

## 設定時間及時區

- 日光節約時間DST一種在夏季月份犧牲正常的日出時間，而將時間調快的做法。通常使用夏令時間的地區，會在接近春季開始的時候，將時間調快一小時，並在秋季調回正常時間。實際上，夏令時間會造成在春季轉換當日的睡眠時間減少一小時，而在秋季轉換當日則會多出一小時的睡眠時間。
- NTP 的同步不要求各設備使用相同的顯示時區。Cisco IOS 以 UTC 作為內部時間基準；時區與夏令時間設定用於顯示本地時間。若沒有有效的時間來源，才用 clock set 設定軟體時鐘。
    
    ```Plain
    R1(config)#clock timezone EST -5
    R1(config)#clock summer-time EDT recurring 2 Sunday March 2:00 1 Sunday November 2:00
    R1#clock set 20:52:49 21 October 2022
    ```
    
    1. clock timezone 後的時區名稱是顯示標籤；-5 表示 UTC-5，也就是比 UTC 慢 5 小時。
    2. clock summer-time 的時區標籤用 EDT 表示夏令時間。本範例採用美國夏令時間規則；其他地區要依當地規則設定，沒有使用夏令時間的地區不需設定此命令。
- 最後可以使用此命令查看時間時區
    
    ```Plain
    R1#show clock
    ```
    

## NTP用戶端、伺服器、用戶端\伺服器設定

若要讓 NTP 請求固定使用某個來源位址，可設定 ntp source interface；使用 loopback 前，須確認路由可達且 ACL 允許該來源。Loopback 並非所有拓樸下都較佳。

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
    !Configuration on R3:
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
    Switch(config)#access-list acl-num permit ip-address wildcard-mask
    Switch(config)#ntp access-group {peer | serve | serve-only | query-only} acl-num
    ```
    
    - peer：允許時間請求與 NTP 控制查詢，也允許本機與符合 ACL 的系統同步。
    - serve：允許時間請求與控制查詢，但不允許本機與該系統同步。
    - serve-only：只允許時間請求。
    - query-only：只允許 NTP 控制查詢。

設定 NTP access-group 後，僅允許符合 ACL 且符合所選存取類型的請求。參考：[Cisco IOS XE NTP](https://www.cisco.com/c/en/us/td/docs/routers/ios/config/17-x/syst-mgmt/b-system-management/m_bsm-time-calendar-set.html)。

## SNTP (simplified network time protocol)

部分平台提供獨立的 SNTP 命令；SNTP 是功能較精簡的時間同步協定，命令集不能一概視為將每個 ntp 命令替換成 sntp。請依平台文件確認支援的功能。

```Plain
SW(config)#sntp authentication-key key-number md5 key-string
SW(config)#sntp authenticate
SW(config)#sntp trusted-key key-number
SW(config)#sntp server ip-address key key-number
```
