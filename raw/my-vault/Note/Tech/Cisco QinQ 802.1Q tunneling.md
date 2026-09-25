- 這個技術等於是因為沒有這麼多vlan id或是可以隔離內部使用的vlan id跟ISP是不同的。
- 這個也可以讓ISP隱藏自己

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/01-這個也可以讓ISP隱藏自己.png|01-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/02-這個也可以讓ISP隱藏自己.png|02-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/03-這個也可以讓ISP隱藏自己.png|03-這個也可以讓ISP隱藏自己.png]]

![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/04-這個也可以讓ISP隱藏自己.png|04-這個也可以讓ISP隱藏自己.png]]

- 在SW1和SW2設定
    
    ```Plain
    interface range e0/1-2
    	switchport mode dot1q-tunnel
    	switchport access vlan 666
    	l2protocol-tunel cdp 這個可以不用只是方便查看封包內容
    ```
    
- show
    
    ![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/05-show.png|05-show.png]]
    
    可以看到cdp鄰居只剩下SW4了
    
    ![[Assets/Note/Tech/Cisco QinQ 802.1Q tunneling/06-show.png|06-show.png]]