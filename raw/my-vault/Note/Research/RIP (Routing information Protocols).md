### 摘要

這個動態路由協定是一種基於距離的路由協定，它已經被其他協定所取代，但它是動態路由協定的經典之一。了解這個協定的運作方式可以幫助學習者更好地理解和學習更複雜的路由協定。除此之外，學習這個協定還有助於理解網路路由的基本原理，以及如何更好地設計和管理網絡架構。

### RIP協定概述

- 是應用在早期、使用較普遍的IGP。
- 適用於小型網路。
- RIP基於UDP，PORT 520。
- 群播地址為224.0.0.9。

### 距離使量路由的基本運作過程

![[Assets/Note/Research/RIP (Routing information Protocols)/01-距離使量路由的基本運作過程.png|01-距離使量路由的基本運作過程.png]]

R1 routing table

|network|interface|hop|
|---|---|---|
|10.0.1.0|G0/0/0|0|
|10.0.2.0|G0/0/1|0|
|==10.0.3.0==|==G0/0/1==|1|
|10.0.4.0|G0/0/1|2|

R2 routing table

|network|interface|hop|
|---|---|---|
|10.0.2.0|G0/0/0|0|
|10.0.3.0|G0/0/1|0|
|10.0.1.0|G0/0/0|1|
|10.0.4.0|G0/0/1|1|

R3 routing table

|network|interface|hop|
|---|---|---|
|10.0.2.0|G0/0/0|0|
|10.0.3.0|G0/0/1|0|
|10.0.2.0|G0/0/0|1|
|10.0.1.0|G0/0/0|2|

==藍色代表第一次學習到的路由，綠色代表第二次學習到的路由訊息==

- 路由器收斂完成的標示
    - 當所有路由表飽含相同網路的可達訊息
    - 網路(路由)進入下一個穩態
- 路由器繼續交換路由訊息
    - 當無新的路由訊息被跟新時的收斂結束
    - 網路在達到收練前無法完全正常工作

### RIP路由協定的度量值

RIP是以跳數(Hops)作為度量值，隨然簡單，但是充滿了漏洞

![[Assets/Note/Research/RIP (Routing information Protocols)/02-RIP路由協定的度量值.png|02-RIP路由協定的度量值.png]]

### 路由環路避免

以下是發生回環的路由表狀態

![[Assets/Note/Research/RIP (Routing information Protocols)/01-距離使量路由的基本運作過程.png|01-距離使量路由的基本運作過程.png]]

R1 routing table

|network|interface|hops|
|---|---|---|
|10.0.1.0|G0/0/0|0|
|10.0.2.0|G0/0/1|0|
|10.0.3.0|G0/0/1|1|
|10.1.4.0|G0/0/1|16|

R2 routing table

|network|interface|hops|
|---|---|---|
|10.0.2.0|G0/0/0|0|
|10.0.3.0|G0/0/1|0|
|10.0.1.0|G0/0/0|1|
|10.1.4.0|G0/0/1|16|

R3 routing table

|network|interface|hops|
|---|---|---|
|10.0.3.0|G0/0/0|0|
|10.0.4.0|G0/0/1|16|
|10.0.2.0|G0/0/0|1|
|10.0.1.0|G0/0/0|2|

- 定義最大跳數(16 hops為不可以達到)

- 水平分割 split Horizon
    
    ![[Assets/Note/Research/RIP (Routing information Protocols)/03-路由環路避免.png|03-路由環路避免.png]]
    
    R1 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.1.0|G0/0/0|0|
    |10.0.2.0|G0/0/1|0|
    |10.0.3.0|G0/0/1|1|
    |10.1.4.0|G0/0/1|2|
    
    R2 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.2.0|G0/0/0|0|
    |10.0.3.0|G0/0/1|0|
    |10.0.1.0|G0/0/0|1|
    |10.1.4.0|G0/0/1|1|
    
    R3 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.3.0|G0/0/0|0|
    |~~10.0.4.0~~|~~G0/0/1~~|~~2~~|
    |10.0.2.0|G0/0/0|1|
    |10.0.1.0|G0/0/0|2|
    
- 路由中毒 Route Poisoning
    
    ![[Assets/Note/Research/RIP (Routing information Protocols)/04-路由環路避免.png|04-路由環路避免.png]]
    
    R1 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.1.0|G0/0/0|0|
    |10.0.2.0|G0/0/1|0|
    |10.0.3.0|G0/0/1|1|
    |10.1.4.0|G0/0/1|2|
    
    R2 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.2.0|G0/0/0|0|
    |10.0.3.0|G0/0/1|0|
    |10.0.1.0|G0/0/0|1|
    |10.1.4.0|G0/0/1|1|
    
    R3 routing table
    
    |network|interface|hops|
    |---|---|---|
    |10.0.3.0|G0/0/0|0|
    |10.0.4.0|G0/0/1|2|
    |10.0.2.0|G0/0/0|1|
    |10.0.1.0|G0/0/0|2|
    
      
    

### RIPv1

- 發出路由原則
    1. 同級的網段發出明細路由，如果是不同及的網段就發出彙總路由
    2. 發出的路由和出接口在同一及的網段，如果出接口的遮罩相同就可以發出去路由訊息，不同就不行
    3. 如果遮罩是32位，不受兩條規則約束，直接發出
- 收路由原則
    1. 如果收到的路由訊息是包含在收接口主機位，路由表寫入的內容即是32位的主機路由
        - example
            
            ![[Assets/Note/Research/RIP (Routing information Protocols)/05-RIPv1 - example.png|05-RIPv1 - example.png]]
            
            R1
            
            ```Plain
            R1(config-if)#ip addr 34.1.1.3 255.255.0.0
            R1(config)#router rip 
            R1(config-router)#version 1
            R1(config-router)#network 34.0.0.0 
            ```
            
            R2
            
            ```Plain
            R2(config)#int fa0/0
            R2(config-if)#ip addr 34.1.1.4 255.255.255.0
            R2(config)#int lo0
            R2(config-if)#ip addr 34.4.4.4 255.255.255.0
            R2(config)#router rip 
            R2(config-router)#version 1
            R2(config-router)#network 34.0.0.0 
            ```
            
            R1
            
            ```Plain
            R1#clear ip route * 清除後需要等等，因為需要學習
            R1#sh ip router rip 
            ```
            
            ![[Assets/Note/Research/RIP (Routing information Protocols)/06-RIPv1.png|06-RIPv1.png]]
            
    2. 收到的路由與入接口不在同一主類網路，且自己路由表中沒有此路由的子網路就以此路由主類的遮罩去匹配
    3. 收到的路由與入接口不在同一主類的網路，但路由表中有此路油子網路，就不接受

### RIPv2

因為RIPv2支持無類路由所以比較沒有很多的限制

```Plain
R2(config)#router rip 
R2(config-router)#version 2
```

### RIP authentication

- RIP驗證路由的方向為收方向
    - 明文
        
        發送KEY ID最小的，收到的Key-string,在key-chain中比較，有相同的通過認證
        
        ```Plain
        Router(config)#key-chain CCNP
        Router(config)#key 1
        Router(config)#key-string cisco
        ```
        
    - MD5 hash認證
        
        1. 發送KEY ID最小的，攜帶Key-ID的訊息，具有相同的Key-ID的Key-string相互比較，相同的通過認證，不同的，不收對方的路由訊息
        2. 當沒有相同的Key ID時，比較key-id+1，如果存在key-id+1比較後相同通過認證不同認證失敗，如果沒有key-id+1就找key-id+1+1以此類推
        
        ```Plain
        Router(config)#int s1/0
        Router(config-if)#ip rip authentication mode [md5 | text]
        Router(config-if)#ip rip authentication key-chain CCNP
        ```
        