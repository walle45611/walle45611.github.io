# TCP

- TCP在不可靠的IP上提供一個可靠的傳輸層

## TCP header

![[Assets/Note/Research/TCP 和 UDP 的基本介紹/01-TCP header.png]]

## source port and destination port

在來回發送的過程中，destination port和source port會相反，這個現象也滿容易理解的，因為可以想像在tcp連線的時候就是建立一個通道

![[Assets/Note/Research/TCP 和 UDP 的基本介紹/01-source port and destination port.png|01-source port and destination port.png]]

- 常用Port number
    
    |Protocol|Number|
    |---|---|
    |FTP|21、20|
    |SSH|22|
    |Telnet|23|
    |SMTP|25|
    |TACACS+|49|
    |HTTP|80|
    |HTTPS|443|
    |IKE|500|
    
    - 一般server通常是通過之名的port number
    - TCP/IP臨時分配1024~5000之間的port number
    - 大於5000的port numbe為其他server預留的(internet上並不常用的server)

---

## flag

這些這麼多的flag就是拿來確定TCP的狀態

URG = 內容可以是1或0如果是1 Urgent Pointer有效如果是零無效

ACK = 確認”確認號"是否有效

PSH = 推送代表必須立刻交訊息送給內核

==RST = Reset==

==SYN = 同步==

==FIN = 斷開連線==

A發起對B的連線狀態改變

A結束對B的連線狀態改變 MSL=120 RFC標準

---

## MSS (Maximum Segment Size)

就是MTU去掉header的資料，裡面包含了TCP的IP packet，也可以從多個IP分片中重組，MSS都會對最終的重組分段起作用。

預設TCP最大的分段式536。當一個主機想把MSS設定得時候，MSS改變得時候會以TCP SYN去定義

---

## TCP滑動視窗(**TCP sliding window)**

> [!info] TCP: Sliding Windows  
>  
> [https://www.youtube.com/watch?v=klDhO9N01c4&ab_channel=RickGraziani](https://www.youtube.com/watch?v=klDhO9N01c4&ab_channel=RickGraziani)  

  

  

---

---

# UDP

- 也是不可靠的，常見的有DNS DHCP TFTP或是有關音訊流量等協定
- 為什麼還要存在，因為像是語音如果有些訊息丟了，隨然會怪怪的但是還是聽得懂，但是如果使用TCP，在一秒鐘前丟了的訊息突然加到語音裡面，根本就聽不懂了
- UDP可以使用廣播的技術，因為他沒有三次握手的規定

## UDP header