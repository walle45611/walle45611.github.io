---
Created: 2023-02-27T20:50
---
## 用意

- 在syslog server可以更準確的知道各個設備的log event是在什麼時候發生的也可以知道攻擊者在甚麼時候開始發起攻擊的
- 在digital certificate非常重要時間的日期，因為digital certificate的有效期限，所以我們需要的NTP的原因

---

## NTP (Cont.)

- NTP可以從一個內部或外部的NTP取得正確的時間
    - Master Clock也可以使用本地的時間
    - Master Clock也可以使用internet 的時間
- 在一個設備上可以同時擔任NTP Server或NTP Client，其他的NTP Client向NTP Server同步

---

## 總結上面兩點

1. 設備需要高度精確的時間和高度同步的時間意義 (一定不是拿來看時間)。
2. 查看syslog，有精確的時間比較便於判log發生的先後順序，如果時間不夠精確，那麼設備產的syslog，根本就看不懂或困惑。
3. 所有設備時間同步，便於將一系列事件關聯起來。
4. 對於digital certifcate正確的時間非常的關鍵，有效期限 (==不能早也不能晚幾秒都不能==)，可能導致有效的證書被視為無效的證書，因為有效期限。

---

## NTP的特色

1. NTP有階級的系統，所有的NTP設備被分為不同的階級。時鐘同步一次，階級就加一，階級用高，同步優先權低(階級用高說明同步次數多，時鐘誤差會很大)。internet上的一些主要NTP Server都具有較低的階級。
    
    ![[Assets/Note/Research/NTP (Network Time Protocol)/01-NTP的特色.png|01-NTP的特色.png]]
    
2. NTP Server與NTP Client，多數應用，Server就是Server，只提供服務，客戶段就是客戶端使用服務，NTP的Client同時可以作為NTP Server，都使用UDP 123 PORT。
    
    NTP推薦的時鐘源:與之同步的時間。一般使用internet上大型的NTP Server就好。也可使用GPS提供服務