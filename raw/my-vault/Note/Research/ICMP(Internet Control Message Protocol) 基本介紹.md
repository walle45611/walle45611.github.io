- [[#ICMP概述]]
- [[#ICMP header]]
- [[#ICMP type]]
- [[#icmp unreachable packet ]]

## ICMP概述

- ICMP只是IP protocol的附屬協議
- 例如，一個剛搭建好的網路，驗證網路設定是否正確以外，為了確保網路能夠按照域其正常工作，一旦遇到什麼問題需要立即制止問題的蔓延。為了減輕administrtor的負擔。
- ICMP主要的功能確定是否能成功送往目的地，通知過程當中IP封包被丟棄的具體訊息，改上網路設定等等。有了這些訊息，就可以獲得網路是否正常、設定是否又誤以及設備有何異常等等訊息，以便檢查網路

---

## ICMP header

![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/01-ICMP header.png|01-ICMP header.png]]

ICMP總共是8byte，ICMP前面4byte一定是固定的，也就是說type code checksum都是固定會有的，但是剩下的4byte不一定會有

---

## ICMP type

![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/02-ICMP type.png|02-ICMP type.png]]

- 最常用的查詢應該是type 8 code0這個，也就是echo-request這個封包和回復00 echo-reply

---

## icmp unreachable packet

![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/03-icmp unreachable packet.png|03-icmp unreachable packet.png]]

- ==8byte裡面放著tcp或udp的port，為什麼需要這個port number，因為這樣就可以去判斷說是哪個app出事了，因為port對應了某個app，發送源的app==
- destination unreachable
    
    - 發現沒有路由訊息會host unreachable
        
        ![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/04-icmp unreachable packet.png|04-icmp unreachable packet.png]]
        
    - 在cisco中出現這種錯誤代表了type 3的錯誤訊息
        
        ![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/05-icmp unreachable packet.png|05-icmp unreachable packet.png]]
        
        ```Plain
        R1#debug ip icmp
        ```
        
        ![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/06-icmp unreachable packet.png|06-icmp unreachable packet.png]]
        
    - cisco 使用telnet連線沒有路由的情況會直接掛斷，因為有了unreachable的訊息了
        
        ![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/07-icmp unreachable packet.png|07-icmp unreachable packet.png]]
        
    
      
    
- administrative prohibited unreachable
    - 也就是說被過濾掉了，也就是說有可能設定了什麼東西以至於讓ip不可以到達
        
        ![[Assets/Note/Research/ICMP(Internet Control Message Protocol) 基本介紹/08-icmp unreachable packet.png|08-icmp unreachable packet.png]]