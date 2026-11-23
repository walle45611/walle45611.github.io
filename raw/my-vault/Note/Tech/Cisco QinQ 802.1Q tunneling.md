---
blog: true
blog_title: "Cisco QinQ 802.1Q tunneling"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-qinq-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

QinQ（802.1Q tunneling）在客戶 VLAN 標籤外再加上服務供應商 VLAN 標籤，讓供應商網路依外層標籤轉送，同時保留客戶原本的 VLAN 標籤。不同客戶因此可以使用重複的 VLAN ID。

QinQ 是乙太網路封裝與隔離方式，不會加密流量，也不保證隱藏 ISP 的身分或網路拓樸。

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/01-這個也可以讓ISP隱藏自己.png|01-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/02-這個也可以讓ISP隱藏自己.png|02-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/03-這個也可以讓ISP隱藏自己.png|03-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/04-這個也可以讓ISP隱藏自己.png|04-這個也可以讓ISP隱藏自己.png]]

- 在SW1和SW2設定
    
    ```Plain
    interface range e0/1-2
        switchport mode dot1q-tunnel
        switchport access vlan 666
        l2protocol-tunnel cdp
    ```
    
- show
    
    ![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/05-show.png|05-show.png]]
    
    l2protocol-tunnel cdp 是選用設定，用來跨越 tunnel port 傳送 CDP；是否能看到遠端鄰居，取決於兩端設定及平台支援。
    
    ![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/06-show.png|06-show.png]]
