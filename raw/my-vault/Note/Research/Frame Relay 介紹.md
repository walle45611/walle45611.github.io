![[Assets/Note/Research/Frame Relay 介紹/00-Frame Relay WAN 雲端拓撲.png|00-Frame Relay WAN 雲端拓撲.png]]

![[Assets/Note/Research/Frame Relay 介紹/01-Frame Relay 介紹.png|01-Frame Relay 介紹.png]]

![[Assets/Note/Research/Frame Relay 介紹/02-Frame Relay 介紹.png|02-Frame Relay 介紹.png]]

- 使用虛擬電路(VC)連接
- 提供面相對向的服務
- 應用非常廣泛在WAN的協議中
- FR交換設備在用戶路由器間建立VC，提供基於分組交換的二層通道
- 面向連接的數據鏈路技術
- 速度56k - 2m
- 是一種NBMA的網路(非廣播型的多路訪問)

## Frame Relay VC

![[Assets/Note/Research/Frame Relay 介紹/03-Frame Relay VC.png|03-Frame Relay VC.png]]

### 虛電路(VC) :

- 通過Frame Relay網路實現邏輯連接叫做VC
- 利用虛電路，Frame Relay允許多個用戶共享頻寬，而無須使用多條物理專線，VC是以DLCI標示的

### DLCI(Data Link Connection Identifier)數據鏈路標示

- 通常由Frame Relay的提供商ISP分配
- Frame Relay DLCI number具有本地的意義
- DLCI 0到15和1008到1023留做特殊用途。ISP提供的DLCI範圍通常是16到1007

### LMI(本地管理接口)

- 是一種信令標準(也就是keepalivec括弧的那段距離)，用於管理鏈路連接和keepalive的機制
- 終端路由器(DTE)和Frame Relay交換機(DCE)之間的Frema Relay設備每10秒(或大概如此)輪詢一次
- Cisco Router支持以下三種LMI:Cisco ,Ansi,q933a

![[Assets/Note/Research/Frame Relay 介紹/04-LMI(本地管理接口).png|04-LMI(本地管理接口).png]]

- Active state 正常狀態
- Inactive state 遠端路由器沒有工作
- Deleted state 街口沒有收到交換機的任何LMI訊息，可能是映射問題或是線路問題

## Freme Relay Topology

![[Assets/Note/Research/Frame Relay 介紹/05-Freme Relay Topology.png|05-Freme Relay Topology.png]]

## Freme Relay address Mapping

![[Assets/Note/Research/Frame Relay 介紹/06-Freme Relay address Mapping.png|06-Freme Relay address Mapping.png]]

- 在MA網路環境中通常IP→mac address Mappeing，但是在Frame Relay是IP→DLCI

## 反向ARP和LMI的操作

![[Assets/Note/Research/Frame Relay 介紹/07-反向ARP和LMI的操作.png|07-反向ARP和LMI的操作.png]]

## Frame Relay設定

### physics Topology

![[Assets/Note/Research/Frame Relay 介紹/08-physics Topology.png|08-physics Topology.png]]

### logic Topology

![[Assets/Note/Research/Frame Relay 介紹/09-logic Topology.png|09-logic Topology.png]]

Tips : Frame Relay可以用Router設備來做模擬，Frame Relay不需要使用IP

### 設定

- 在DCE上
    
    ```Plain
    FR1(config)#frame-realay switching
    FR1(config)#interface Serial0/1
    FR1(config-if)#no ip address
    FR1(config-if)#encapsulation frame-relay
    FR1(config-if)#clock rate 64000
    FR1(config-if)#frame-relay intf-type dce
    FR1(config-if)#frame-relay route 102 interface Serial0/2 201
    FR1(config-if)#frame-relay route 103 interface Serial0/3 301
    !
    FR1(config)#interface Serial0/2
    FR1(config-if)#no ip address
    FR1(config-if)#encapsulation frame-relay
    FR1(config-if)#clock rate 64000
    FR1(config-if)#frame-relay intf-type dce
    FR1(config-if)#frame-relay route 201 interface Serial0/1 102
    !
    FR1(config)#interface Serial0/3
    FR1(config-if)#no ip address
    FR1(config-if)#encapsulation frame-relay
    FR1(config-if)#clock rate 64000
    FR1(config-if)#frame-relay intf-type dce
    FR1(config-if)#frame-relay route 301 interface Serial0/2 103
    ```
    
    ```Plain
    FR1#sh frame-relay route
    
    Input Intf      Input Dlci      Output Intf     Output Dlci     Status
    Serial0/1       102             Serial0/2       201             active
    Serial0/1       103             Serial0/3       301             active
    Serial0/2       201             Serial0/1       102             active
    Serial0/3       301             Serial0/1       103             active
    ```
    
- 在DTE上
    - 自動設定
        
        ```Plain
        R1(config)#interface Serial0/0
        R1(config-if)#ip address 10.1.123.1 255.255.0
        R1(config-if)#encapsulation frame-relay
        ```
        
        ```Plain
        R1#sh frame-relay map
        
        Serial0/0 (up): ip 10.1.123.2 dlci 102(0x66,0x1860), dynamic,
                      broadcast,
                      CISCO, status defined, active
        ```
        
    - 手動設定(建議)
        
        ```Plain
        R1(config)#interface Serial0/0
        R1(config-if)#ip address 10.1.123.1 255.255.0
        R1(config-if)#encapsulation frame-relay
        R1(config-if)#no frame-relay inverse-arp  //關閉inverse-arp
        R1(config-if)#frame-relay map ip 10.1.123.2 102 broadcast
        R1(config-if)#frame-relay map ip 10.1.123.3 103 broadcast
        
        Broadcast關鍵字是可以選的，加上這個關鍵字，則該條PVC具有 "廣播"的能力，當然
        ，所謂的Frame Relay環境的廣播，指的是向所有的PVC都發送一份封包的複製，實現，
        類似廣播的功用
        ```
        
        ```Plain
        R1#sh frame-relay map
        
        Serial0/0 (up): ip 10.1.123.2 dlci 102(0x66,0x1860), static,
                      broadcast,
                      CISCO, status defined, active
        Serial0/0 (up): ip 10.1.123.3 dlci 103(0x67,0x1870), static,
                      broadcast,
                      CISCO, status defined, active
        ```
        
        ```Plain
        R1#sh int s0/0
        
        Serial0/0 is up, line protocol is up
          Hardware is M4T
          Internet address is 10.1.123.1/24
          MTU 1500 bytes, BW 1544 Kbit/sec, DLY 20000 usec,
             reliability 255/255, txload 1/255, rxload 1/255
          Encapsulation FRAME-RELAY, crc 16, loopback not set
          Keepalive set (10 sec)
          Restart-Delay is 0 secs
          LMI enq sent  113, LMI stat recvd 114, LMI upd recvd 0, DTE LMI up
          LMI enq recvd 0, LMI stat sent  0, LMI upd sent  0
          LMI DLCI 1023  LMI type is CISCO  frame relay DTE
          FR SVC disabled, LAPF state down
          Broadcast queue 0/64, broadcasts sent/dropped 2/0, interface broadcasts 0
          Last input 00:00:00, output 00:00:00, output hang never
          Last clearing of "show interface" counters 00:19:00
          Input queue: 0/75/0/0 (size/max/drops/flushes); Total output drops: 0
          Queueing strategy: fifo
          Output queue: 0/40 (size/max)
          5 minute input rate 0 bits/sec, 0 packets/sec
          5 minute output rate 0 bits/sec, 0 packets/sec
             126 packets input, 2942 bytes, 0 no buffer
             Received 0 broadcasts (0 IP multicasts)
             0 runts, 0 giants, 0 throttles
             0 input errors, 0 CRC, 0 frame, 0 overrun, 0 ignored, 0 abort
             128 packets output, 2617 bytes, 0 underruns
             0 output errors, 0 collisions, 1 interface resets
             0 unknown protocol drops
             0 output buffer failures, 0 output buffers swapped out
             1 carrier transitions     DCD=up  DSR=up  DTR=up  RTS=up  CTS=up
        ```
        

### 設定問題

- 完成上設定後，R1和R2之間，R1和R3之間通信就沒有問題了

- R2與R3之間如何通信呢?
    
    因為沒有對方的DLCI所以在第二層的時候無法互相通訊，所以以下指令就可以解決
    
    ```Plain
    R2(config-if)#frame-relay map ip 10.1.123.3 201 broadcast
    ```
    
    ```Plain
    R3(config-if)#frame-relay map ip 10.1.123.2 301 broadcast
    ```
    
- R1、R2、R3都無法ping通自己為什麼
    
    因為沒有自己的DLCI所以ping不通，所以再把自己的DLCI加上就可以了
    
    ```Plain
    R1(config-if)#frame-relay map ip 10.1.123.1 102 broadcast
    ```
    
    ```Plain
    R2(config-if)#frame-relay map ip 10.1.123.2 201 broadcast
    ```
    
    ```Plain
    R3(config-if)#frame-relay map ip 10.1.123.3 301 broadcast
    ```
    

### Frame Relay環境中動態路由協定問題

- Frema Relay，不支持廣播，但是可以”模擬”廣播的操作，作法即是通過所有PVC發送一個複製封包
- 在建立PVC時，通過invers-arp自動建立mapping，預設就開啟上述的特性，如果是手動配置mapping比須加上”broadcast關鍵字”
