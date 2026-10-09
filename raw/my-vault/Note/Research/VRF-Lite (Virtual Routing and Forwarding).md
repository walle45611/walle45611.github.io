---
title: VRF-Lite (Virtual Routing and Forwarding)
aliases:
  - VRF
  - VRF-Lite
---

# VRF-Lite (Virtual Routing and Forwarding)

- VRF是可以分流不同類型流量的工具，在一台路由器上有許多個virtual router的概念
- 這個日本網站有好的解釋
    
    > [!info] VRF-Liteとは、Ciscoコンフィグ設定例  
    > ◆　です。 VRF-Lite の VRF-Liteとは 　主な特徴は以下の4点です。 　2.  
    > [https://www.infraexpert.com/study/mpls12.html](https://www.infraexpert.com/study/mpls12.html)  
    

- case
    
    ![[Assets/Note/Research/VRF-Lite (Virtual Routing and Forwarding)/03-VRF-Lite (Virtual Routing and Forwar.png|03-VRF-Lite (Virtual Routing and Forwar.png]]
    
    ![[Assets/Note/Research/VRF-Lite (Virtual Routing and Forwarding)/04-VRF-Lite (Virtual Routing and Forwar.png|04-VRF-Lite (Virtual Routing and Forwar.png]]
    
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

## 相關筆記

- [[MLS (MLS，multilayer switching)]]
