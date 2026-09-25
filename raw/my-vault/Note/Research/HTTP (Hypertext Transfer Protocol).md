---
Created: 2023-02-27T20:50
---
- [[#HTTP簡介]]
- [[#HTTP特點]]
- [[#URL ]]
    - [[#URL的組成]]
    - [[#cookie]]
    - [[#多連接問題]]
    - [[#HTTP RFC command]]
- [[#HTTPS]]

## HTTP簡介

---

適用於www server傳輸超文本到本地瀏覽器的傳輸協定，是一個客戶端和server請求和應答的標準(TCP)。HTTP是internet最廣泛的一種協定WWW文件都必須遵守這個協定

## HTTP特點

---

Http servers area pretty dum servers，也就是說不會有歡迎訊息除非有請求，可以使用nc command來查看。

HTTP是一個瞬間的協定，不會維護會話，也就是說給到網頁就結束連線。

## URL

---

### URL的組成

![[Assets/Note/Research/HTTP (Hypertext Transfer Protocol)/01-URL的組成.png|01-URL的組成.png]]

### cookie

- 一個人只會有一個cookie，且這個cookie會一樣
    
    ![[Assets/Note/Research/HTTP (Hypertext Transfer Protocol)/02-cookie.png|02-cookie.png]]
    
    ![[Assets/Note/Research/HTTP (Hypertext Transfer Protocol)/03-cookie.png|03-cookie.png]]
    
- cookie就是保持當前訪問的人的狀態
- cookie可以讓http可以有連續性

### 多連接問題

也就是說可能網頁有很多的東西，但是一次性載入會花費大量的資源，這時候可以利用分段的Get請求讓，使用者可以看到廣告卻不是在同時請求，所以http有很多的Get其時很正常

### HTTP RFC command

OPTIONS

==GET : 可以指定要求的網頁內容，也就是說header和data可以透過這個指令得知==

==HEAD : 指要求header，可以得知一些server的版本或是ip位置等等==

POST

PUT

DELETE

TRACE

CONNECT

---

## HTTPS

- 先具備的知識
- HTTPS其實就是HTTP加上TLS就成為HTTPS

[[PKI (Public key infrastructure)公鑰基礎設施]]

![[Assets/Note/Research/HTTP (Hypertext Transfer Protocol)/01-HTTPS.png]]