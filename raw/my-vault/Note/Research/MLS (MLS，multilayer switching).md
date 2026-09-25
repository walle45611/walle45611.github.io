- [[#多層交換機概述(MLS，multilayer switching)]]
    - [[#CAM]]
    - [[#TCAM]]
- [[#實現多層交換]]
    - [[#RoS (單臂路由)]]
    - [[#L3交換]]
    - [[# VRF-Lite (Virtual Routing and Forwarding)]]

## 多層交換機概述(MLS，multilayer switching)

catalys switch 基本可以分為兩種基本MLS

- route caching
    
    第一代MLS，需要路由器處理器(RP，route processor)與SE，switch engine。RP必須處理資料流的第一個封包已決定其目的地;SE則是聆聽第一個封包與決定過的目的地，並且在其MLS快取中建立捷徑，RP必須檢查每個SE定的捷徑。
    
    這種稱為netflos lan switching
    
- topology based
    
    MLS採用特殊的硬體，同樣分成SE和PR。第三層會見立即預先占用一個具備整體已知的網路拓樸資料;資料庫以硬體的方提供表格查詢，使得SE能迅速的轉發。如果topology有發生變更database 資料也會更新
    
    這種稱為cisco express forwarding(CEF)，其中的資料庫又成為FIB(forwarding informatin base)
    

### CAM

mac address 記錄在CAM，所以switch會在cam中進行轉發，為了管理CAM通常路由器會刪除老舊的mac address 300秒後刪除。

- command
    - 可以改變cam刪除時間的指令
        
        ```Plain
        Switch(config)#mac addrss-table aging-time seconds
        ```
        
    - 查看CAM表內容
        
        ```Plain
        Switch#show mac address-table dynamic [address mac-address | interface type mod/num | vlan vlan-id] 
        ```
        
    - 清除CAM
        
        ```Plain
        Switch#clear mac address-table dynamic [address mac-address | interface type mode/num | valn vlan-id]
        ```
        

### TCAM

傳統的設備上都是依序比對ACL，ACL是由一個或多個ACE(Access control entiy)所控制，但是在MLS中會有提供TCAM這個硬體執行。有了TCAM就能單一表格來查詢整個存取清單比對封包。大部分交換器具有多個TCAM，因此可以同時比對ACL和QoS ACL，或是同時做出L2或是L3的轉送策略。

- FM(Feature Manager)
    
    建立清單或設定ACL清單，FM軟體將ACE編譯合併到TCAM表中，然後查詢
    
- SDM(switch database manager)
    
    TCAM可以被劃分多個區域，如有需要可以使用SDM調整TCAM分割區，以便提供交換充足的空間catalyst 4500 6500不能更改TCAM以固定
    
- command
    - 顯示tcam使用資源
        
        ```Plain
        Switch#show platform tcam utilizatoin
        ```
        
    - 顯示SDM
        
        ```Plain
        Switch#show sdm prefer
        ```
        

---

## 實現多層交換

### RoS (單臂路由)

![[Assets/Note/Research/MLS (MLS，multilayer switching)/01-RoS (單臂路由).png|01-RoS (單臂路由).png]]

---

### L3交換

- 路由介面
    
    預設L3 switch 的介面是使用switch模式，但是這樣就沒辦法配IP，所以需要
    
    ```Plain
    SW(config)#no siwthcport
    ```
    
- SVI交換機虛擬介面 (**Switch virtual interface**)
    
    - L2 SW只能UP一個
    - 必須要有雙up的物理介面，vlan要有創建的，也必須要有物理接口屬於某個vlan
    - 存在trunk情況下也可以，vlan是自己創建的，前提是vlan不是not allowed
    - BW是取自物理界面*10
    
    ![[Assets/Note/Research/MLS (MLS，multilayer switching)/02-L3交換 - BW是取自物理界面 10.png|02-L3交換 - BW是取自物理界面 10.png]]
    
    ```Plain
    SW(config)#vlan 10
    SW(config-vlan)#name test
    SW(config-if)#int e0/0
    SW(config-if)#no sh
    SW(config)#int vlan 10
    SW(config-if)#ip addr 10.1.1.1 255.255.255.0
    ```
    
    ```Plain
    SW1#sho int vlan 10
    Vlan10 is up, line protocol is up
      Hardware is Ethernet SVI, address is aabb.cc80.1000 (bia aabb.cc80.1000)
      Internet address is 10.1.1.1/24
      MTU 1500 bytes, BW 1000000 Kbit/sec, DLY 10 usec,
         reliability 255/255, txload 1/255, rxload 1/255
      Encapsulation ARPA, loopback not set
      Keepalive not supported
      ARP type: ARPA, ARP Timeout 04:00:00
      Last input never, output never, output hang never
      Last clearing of "show interface" counters never
      Input queue: 0/75/0/0 (size/max/drops/flushes); Total output drops: 0
      Queueing strategy: fifo
      Output queue: 0/40 (size/max)
      5 minute input rate 0 bits/sec, 0 packets/sec
      5 minute output rate 0 bits/sec, 0 packets/sec
         0 packets input, 0 bytes, 0 no buffer
         Received 0 broadcasts (0 IP multicasts)
         0 runts, 0 giants, 0 throttles
         0 input errors, 0 CRC, 0 frame, 0 overrun, 0 ignored
         1 packets output, 60 bytes, 0 underruns
         0 output errors, 0 interface resets
         0 unknown protocol drops
    ```
    

### VRF-Lite (Virtual Routing and Forwarding)

- VRF是可以分流不同類型流量的工具，在一台路由器上有許多個virtual router的概念
- 這個日本網站有好的解釋
    
    > [!info] VRF-Liteとは、Ciscoコンフィグ設定例  
    > ◆　です。 VRF-Lite の VRF-Liteとは 　主な特徴は以下の4点です。 　2.  
    > [https://www.infraexpert.com/study/mpls12.html](https://www.infraexpert.com/study/mpls12.html)  
    

- case
    
    ![[Assets/Note/Research/MLS (MLS，multilayer switching)/03-VRF-Lite (Virtual Routing and Forwar.png|03-VRF-Lite (Virtual Routing and Forwar.png]]
    
    ![[Assets/Note/Research/MLS (MLS，multilayer switching)/04-VRF-Lite (Virtual Routing and Forwar.png|04-VRF-Lite (Virtual Routing and Forwar.png]]
    
    ```Plain
    R1(config)#ip vrf VOICE
    R1(config-vrf)#ip vrf VOIDE
    R1(config-vrf)#ip vrf DATA
    !
    interface Loopback0
     ip address 1.1.1.1 255.255.255.255
    !
    interface Ethernet0/0
     no ip address
    !
    interface Ethernet0/0.2
     encapsulation dot1Q 2
     ip vrf forwarding VOICE
     ip address 192.0.2.1 255.255.255.252
     ip ospf 1 area 0
    !
    interface Ethernet0/0.3
     encapsulation dot1Q 3
     ip vrf forwarding DATA
     ip address 198.51.100.1 255.255.255.252
     ip ospf 2 area 0
    !
    interface Ethernet0/0.4
     encapsulation dot1Q 4
     ip vrf forwarding VIDEO
     ip address 203.0.113.1 255.255.255.252
     ip ospf 3 area 0
    !
    router ospf 1 vrf VOICE
     router-id 1.1.1.1
    !
    router ospf 2 vrf DATA
    !
    router ospf 3 vrf VIDEO
    ```
    
    ```Plain
    SW1
    !
    interface GigabitEthernet0/0
     switchport trunk encapsulation dot1q
     switchport mode trunk
     media-type rj45
     negotiation auto
    !
    interface GigabitEthernet0/1
     switchport access vlan 2
     switchport trunk encapsulation dot1q
     switchport mode access
     media-type rj45
     negotiation auto
    !
    interface GigabitEthernet0/2
     switchport access vlan 3
     switchport trunk encapsulation dot1q
     switchport mode access
     media-type rj45
     negotiation auto
    !
    interface GigabitEthernet0/3
     switchport access vlan 4
     switchport trunk encapsulation dot1q
     switchport mode access
     media-type rj45
     negotiation auto
    ```
    
    ```Plain
    R2
    !
    interface Loopback0
     ip address 2.2.2.2 255.255.255.255
    !
    interface Ethernet0/0
     ip address 192.0.2.2 255.255.255.252
     ip ospf 1 area 0
    !
    interface Ethernet0/1
     ip address 10.1.1.1 255.255.255.0
     ip ospf 1 area 0
    !
    router ospf 1
     router-id 2.2.2.2
    ```
    
    ```Plain
    R3
    !
    interface Loopback0
     ip address 3.3.3.3 255.255.255.255
    !
    interface Ethernet0/0
     ip address 198.51.100.2 255.255.255.252
     ip ospf 1 area 0
    !
    interface Ethernet0/1
     ip address 172.16.1.1 255.255.255.0
     ip ospf 1 area 0
    !
    router ospf 1
     router-id 3.3.3.3
    ```
    
    ```Plain
    R4
    !
    interface Loopback0
     ip address 4.4.4.4 255.255.255.255
    !
    interface Ethernet0/0
     ip address 203.0.113.2 255.255.255.252
     ip ospf 1 area 0
    !
    interface Ethernet0/1
     ip address 192.168.1.1 255.255.255.0
     ip ospf 1 area 0
    !
    router ospf 1
     router-id 4.4.4.4
    ```