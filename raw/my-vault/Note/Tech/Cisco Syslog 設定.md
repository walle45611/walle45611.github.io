## 概念

對於IT人員，log訊息極為重要，因為可以查看在一個時間發生的事件，但是訊息很多有重要的也有不太重要的訊息，所以做好log訊息的整理是一件很重要的事情，還有當裝置OS發生event的時候亦要怎麼通知管理人員也是相當重要的議題。

## 及時發送訊息給目前使用者

- 根據預設，IOS會將所有等級的log訊息顯示給console port的使用者看。這是由於預設的**logging console** global指令所產生的，所以可以透過**no logging console**關閉。
    
    ```Plain
    R1(config)#no logging console
    R1(config)#logging console
    ```
    
- 對於其他介面的聯接方式例如ssh或telnet使用者來說，需要使用**logging monitor**來告訴IOS將log message告訴所有登入的使用者。這只指令還不足也讓使用者看到log訊息還必須下**terminal monitor**指令，來告知IOS終端會想要接收到log訊息
    
    ![[Assets/Note/Tech/Cisco Syslog 設定/01-及時發送訊息給目前使用者.png|01-及時發送訊息給目前使用者.png]]
    
    ```Plain
    R1(config)#logging monitor
    R1(config)#terminal monitor
    ```
    
- 比賽常出現，可以關閉log顯示在螢幕上，防止打斷打指令
    
    ```Plain
    Router(config-line)#logging synchronous
    ```
    
- 比賽常出現，防止打錯字被當作domain name查詢
    
    ```Plain
    Router(config)#no ip domain-lookup
    ```
    

## 儲存log以便事後檢視

- IOS可以藉由**logging buffered**的global指令將log訊息儲存到RAM裡之後可以用**show logging EXEC**命令查看log訊息
    
    ```Plain
    R1(config)#logging buffered
    R1#show logging EXEC
    ```
    
- 另一種常見的作法是讓所有裝置把他們的log訊息儲存到syslog server上，RFC 5424定義了syslog協定，讓路由器交換這類的裝置可以利用UDP協定傳送到syslog server加以儲存，也可以利用web介面更方便的查詢log訊息
    
    ![[Assets/Note/Tech/Cisco Syslog 設定/02-儲存log以便事後檢視.png|02-儲存log以便事後檢視.png]]
    
    ```Plain
    R1(config)#loggin {address|hostname}
    R1(config)#logging trap severity
    ```
    

## Log訊息格式

IOS定義了log訊息格式。一開始與訊息有關的某些資料欄位後面接續一些讓易懂的文字

```Plain
*Jun 14 05:26:02.794: %LINEPROTO-5-UPDOWN: Line protocol on Interface Ethernet0/0, changed state to up
```

根據上面的log訊息可以知道

- 時間 : Jun 14 05:26:02.794
- 產生訊息的路由器的硬體裝置，協定或是系統軟體的模組 : %LINEPROTO
- 嚴重等級 : 5
- 訊息助記符號 : UPDOWN
- 訊息說明 : Line protocol on Interface Ethernet0/0, changed state to up

我們可以自訂一些訊息內容例如時間戳記(default open)或是log序列號(default close)

```Plain
R1(config)#no service timestamps
R1(config)#service sequence-numbers
```

- 將時間計入到訊息中

```Plain
SW(config)#service timestamps log datetime [localtime] [show-timezone] [msec] [year]
```

## Log事件等級

- 事件分級
    
    |關鍵字|等級|說明|
    |---|---|---|
    |警報 (emergencies)|0 嚴重|需要立即處理|
    |緊急 (alerts)|1 嚴重|系統無法使用|
    |重大 (Critical)|2 有影響|重大事件 (最高為3)|
    |錯誤 (Error)|3 有影響|重大事件 (中間為3)|
    |警告 (Warring)|4 有影響|重大事件 (最低為3)|
    |提醒 (notifications)|5 正常|正常，比較重要|
    |資訊 (informational)|6 正常|正常，不太重要|
    |除錯 (debugging)|7 除錯|使用者要求除錯|
    
    等級為**有影響**的代表並非非常急迫的事件，通常常見的有介面異常down的log訊息會顯示嚴重等級為3的訊息
    
- 設定每個log服務的紀錄訊息等級
    
    在設定某個嚴重等級時，IOS將會傳送該嚴重等級以上的服務訊息。舉例來說，**logging console 4**命令會使IOS將等級0-4的訊息傳送到console介面使用者上。如果要停用可以使用no指令
    
    |服務|啟用log紀錄|設定訊息等級|
    |---|---|---|
    |Console|logging console|logging console level-name \| level-number|
    |Monitor|logging monitor|logging monitor level-name \| level-number|
    |Buffered|logging buffered|logging buffered level-name \| level-number|
    |Syslog|logging host address \| hostname|logging trap level-name \| level-number|
    
    ```Plain
    R1(config)#no logging console
    R1(config)#no logging monitor
    ```
    
    可以使用，檢視log設定值
    
    ```Plain
    R1#show logging
    ```