## GLBP (Gateway Load Balancing Protocol) 概述

也是Cisco私有的協定，用來克服現有的備援路由協定的缺點，其中有些概念跟HSRP/VRRP相同，且行為更為動態和完整，但術語有些不同。

為了提供虛擬路由器，得到多台交換器(路由器)，分配到同一個GLBP中。群組中所有的路由器都能參與負載平衡，分攤轉送部分的資料流，並非單獨讓active路由器代表虛擬路由來轉送資料。

- 組播位址 224.0.0.102
- UDP 3222

## GLBP 原理

==這種做發優點，用戶端無須指定特定的閘道，所有用戶端的default gateway 都是相同的，這個default gateway就是virtual router IP address ; 原理就是使用回送給用戶端的ARP回應中的虛擬路由mac address，來提供完整的load balance。當用戶端查詢virtual router IP address的ARP請求時，GLBP回傳一個ARP回應，其中包含從群組中選擇的路由器virtual mac address 如此所有用戶端都使用相同的gateway，但是提供的mac address不一樣==

- 要啟用GLBP，必須使用下面的指令
    
    ```Plain
    SW(config-if)#glbp group ip [ip-address [secondary]]
    ```
    
    - 可以不用設定ip address，因為其他的會從其人身上學到，但是AVG一定要設定
    - IPv6
        
        ```Plain
        Sw(config-if)#glbp group ipv6 autoconfigura
        ```
        

### AVG (active virtual gateway)

該路由器的優先權或是IP都是最大的。AVG負責回應所有有關於virtual router address的ARP請求，回送的MAC address取決於設定的load balance演散法，群組中某台路由器所支援的virtual mac addreess會回送給client

AVG會分配必要的MAC address GLBP群的每台路由器，每個群組最多可以用4 個mac address。

==AVG分配到mac address的路由器稱為AVF，負責轉送virtual Mac address上接收的資料，AVG也會分配次角色，群組中路由器都是備援或是AVF，以防止AVG不見了==

- 指定GLBP優先權
    
    ```Plain
    SW(config-if)#glbp group priority level
    ```
    
- 接管
    
    像HSRP一樣active出現故障才有可能被接管，如果有路由器priority比AVG更高，GLBP允許路由器成為AVG
    
    ```Plain
    SW(config-if)#glbp group preempt [delay minimum seconds]
    ```
    
- hello timer & hold timer
    
    GLBP必須互相監控，以便其他路由器出現故障，其他路由器能承擔他們的職責。為此AVG定期向其他GLBP鄰居發送hello
    
    ```Plain
    SW(config-if)#glbp group timers [msec] hellotime [msec] holdtime
    ```
    
    - hello預設是3s。如果holdtime預設是10s
    - holdtime必須是hellotime的三倍建議
    - AVG會通告自己的timer，所以不用每台都設定

### AVF(active virtual forwarder)

參與GLBP每台路由器可成為AVF，AVG分配MAC address給他。如果AVF故障也就是說AVG沒有收到Hello的回覆訊息，AVG就會判定AVF故障，AVG就會把AVF的角色轉給其他人。

> [!important] virtual mac address : 0007b4xx.xxyy ， xx.xx代表16位元相當0加上10位元的GLBP Group ID ; yy則是virtual forwarder的ID

- 被授予新的AVF Role的角色可能會有一個問題
    
    被授予新的AVF Role的角色可能會有一個問題，就是同時有兩個mac address，AVG維護的 timer可以解決這個問題。
    
    ```Plain
    SW(config-if)#glbp group timers redirect redirect timeout
    ```
    
    - 一種是redirect timer
        
        用來決定AVG何時停止在ARP中回應中使用舊的virtual mac address。符合舊位址的AVF將繼續扮演用戶端的gateway role
        
        - 預設600s (10min)
        - 設定範圍 0 ~ 3600s
    - timeout timer
        
        到後期將所有GLBP鄰居都刪除舊有的MAC address並使用自己的VF(virtual Forwarding)。假設AVG出現故障的AVF不會恢復，因此必須收回分配給他的mac address。此時仍會使用舊的MAC address的Client，必須在他們的ARP cache中跟新項目，已獲得新的virtual mac address
        
        - default 14400s (4 hr)
        - 700~64800s (18 hr)
- weighting function
    
    weighting function 決定使用那台路由器可以成為有Virtual Mac address的AVF。**每台路由器的都有一個權重(1~254)**，如果介面down，則可以使用設定降低權重 ; GLBP使用臨界值來決定路由器能否成為AVF，如果權重低於臨界值就必須放棄AVF，如果高於則可以繼續擔任 。
    
    - 權重預設100。要動態調整權重GLBP就必須知道哪接介面需要如何調整。追中介面是第一步
        
        ```Plain
        SW(config)#track object-number intreface type memeber/moudule/number
        {line-protocol | ip routing}
        ```
        
        - object-number任一數值 (1-500)
        - 觸發條件**line-protocol (介面protocol狀態up) | ip routing(啟動路由或是指定IP位置且處於UP狀態)**
    - 設定權重
        
        ```Plain
        SW(config-if)#glbp group weighting maximum [lower lower] [upper upper]
        ```
        
        - maximum 設定權重最大範圍預設100
        - **lower 下限值預設為1**
    - 最後整合這兩個讓GLBP知道使用那個介面和權重
        
        ```Plain
        SW(config-if)#glbp group weighting track  object-number [decrement value]
        ```
        
        - **decrement 當介面down，權重降低**value 預設10

## GLBP Load balance

AVG是以固定virtual router mac address分配給用戶端建立負載平衡。AVG需要分給這些mac address給AVF且每個Group最多只能4個 mac address，這些位址是依照順序發的

### 輪替式 (round robin)

代表收到新的virtual router address ARP請求後在回應中提供下一個可用的virtual MAC address，假設每個用戶送和接收資料相同，則資料負載就會被頻居的分配通過群組中參與AVF的所有路由器。這是預設的方法。

### 權重(Weighted)

代表GLBP群組的權重決定被==傳送到AVF的比例==。權重越高，包含路由器虛擬mac地址的ARP回應頻率就會越高，如果沒有配置追蹤介面則會使用最大權重直來設定AVF之間的相對比例

### 主機的依賴性 (Host-dependent)

為了得知虛擬路由器位址產生ARP請求的每個用戶端，希望收到相同的virtual Mac address回應。如果用戶端需要(依賴)固定閘道MAC address，則可以使用這個方法 ; 否則視使用的load balance，隨時間的不同，用戶端收到的路由器mac位置也會不同。

- 設定
    
    ```Plain
    SW(config-if)#glbp group load-balancing [round-robin | weighted | host-dependent ]
    ```
    

## CASE

我不知道為什麼GNS3 IOU不能用

![[Assets/Note/Tech/GLBP  熱備份路由協定指南/01-CASE.png|01-CASE.png]]

  

- SW
    
    ```Plain
    interface Ethernet0/0
     switchport access vlan 50
     switchport mode access
    !
    interface Ethernet0/1
     switchport access vlan 50
     switchport mode access
    !
    interface Ethernet0/2
     switchport access vlan 50
     switchport mode access
    !
    interface Ethernet0/3
     switchport access vlan 50
     switchport mode access
    ```
    
- SWA
    
    ```Plain
    interface Ethernet0/0
     switchport access vlan 50
     switchport mode access
    !
    interface Vlan50
     mac-address 0000.aaaa.aaaa
     ip address 192.168.1.10 255.255.255.0
     glbp 1 ip 192.168.1.1
     glbp 1 priority 200
     glbp 1 preempt
    ```
    
    ![[Assets/Note/Tech/GLBP  熱備份路由協定指南/02-CASE.png|02-CASE.png]]
    
    - SWA#sh glbp
        
        ```Plain
        
        Vlan50 - Group 1
          State is Active
            1 state change, last state change 00:05:51
          Virtual IP address is 192.168.1.1
          Hello time 3 sec, hold time 10 sec
            Next hello sent in 0.480 secs
          Redirect time 600 sec, forwarder timeout 14400 sec
          Preemption enabled, min delay 0 sec
          Active is local
          Standby is 192.168.1.11, priority 150 (expires in 7.968 sec)
          Priority 200 (configured)
          Weighting 100 (default 100), thresholds: lower 1, upper 100
          Load balancing: round-robin
          Group members:
            0000.aaaa.aaaa (192.168.1.10) local
            0000.bbbb.bbbb (192.168.1.11)
            0000.cccc.cccc (192.168.1.12)
          There are 3 forwarders (1 active)
          Forwarder 1
            State is Active
              1 state change, last state change 00:05:40
            MAC address is 0007.b400.0101 (default)
            Owner ID is 0000.aaaa.aaaa
            Redirection enabled
            Preemption enabled, min delay 30 sec
            Active is local, weighting 100
            Client selection count: 1
          Forwarder 2
            State is Listen
            MAC address is 0007.b400.0102 (learnt)
            Owner ID is 0000.cccc.cccc
            Redirection enabled, 599.744 sec remaining (maximum 600 sec)
            Time to live: 14399.744 sec (maximum 14400 sec)
            Preemption enabled, min delay 30 sec
            Active is 192.168.1.12 (primary), weighting 100 (expires in 10.112 sec)
          Forwarder 3
            State is Listen
            MAC address is 0007.b400.0103 (learnt)
            Owner ID is 0000.bbbb.bbbb
            Redirection enabled, 597.984 sec remaining (maximum 600 sec)
            Time to live: 14397.984 sec (maximum 14400 sec)
            Preemption enabled, min delay 30 sec
            Active is 192.168.1.11 (primary), weighting 100 (expires in 9.184 sec)
        ```
        
- SWB
    
    ```Plain
    interface Ethernet0/1
     switchport access vlan 50
     switchport mode access
    !
    interface Vlan50
     mac-address 0000.bbbb.bbbb
     ip address 192.168.1.11 255.255.255.0
     glbp 1 ip 192.168.1.1
     glbp 1 priority 150
     glbp 1 preempt
    ```
    
    ![[Assets/Note/Tech/GLBP  熱備份路由協定指南/03-CASE.png|03-CASE.png]]
    
- SWC
    
    ```Plain
    interface Ethernet0/2
     switchport access vlan 50
     switchport mode access
    !
    interface Vlan50
     mac-address 0000.cccc.cccc
     ip address 192.168.1.12 255.255.255.0
     glbp 1 ip 192.168.1.1
    ```
    
    ![[Assets/Note/Tech/GLBP  熱備份路由協定指南/04-CASE.png|04-CASE.png]]