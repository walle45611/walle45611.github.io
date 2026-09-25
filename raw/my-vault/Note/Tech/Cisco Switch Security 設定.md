- [[#Port Protect]]

---

## Port Protect

在某些情況呢，server不需搬動的，可以使用mac位址的對應去鎖定port，如果是移動式server可以使用switch動態的去學習mac address，將其加入對應port的列表。

- 啟用port-secuirty 功能
    
    ```Plain
    sw(config-if)#switchport port-security
    ```
    
- 指定一組允許的mac address，可以手動設定位址，或是根據交換機的port動態學習到位址，以下指令是指定最大mac address的數目
    
    ```Plain
    SW(config=-if)#switchport port-security maximum max-addr
    ```
    
    - 預設情況只與許一組，1~1024。
    - 預設上每個使用port-secutity可動態的學到MAC address，並期望這些port可以出現在該介面。當主機發送data到switch介面上時就會主動的去學習，主動學習的數目不超過設定的maximum，如果相連的主機沒有動靜，mac address可能會到期的關係，而從ARP表中刪除，預設情況不刪除
- 如果交換機重開機不想讓學習到的位址發生改變
    
    ```Plain
    SW(config-if)#switchport port-securtyi mac-address sticky
    ```
    
    - 預設是重新學
- 也可以定義多個mac address這些都能夠透過該port存取網路
    
    ```Plain
    SW(config-if)#switchport port-security mac-address mac-addr
    ```
    
    - 如果設定mac addr的數目沒有超過設定的最大值，其餘的會用動態的方式
    - example
        
        ```Plain
        SW(config-if)#switchport port-security mac-address 0006.5b02.a841
        ```
        
- 最後設定mac位址錯誤時的反應
    
    ```Plain
    SW(config-if)#switchport port-security violatin {shutdown | restricat | protect}
    ```
    
    - 超過最大值
    - 位置非靜態定義的mac
    - shutdown : 進入Errdisable
    - Restric : this port status up，但是會丟棄封包，會通知Log訊息，SNMP trap
    - Protect : 狀態跟Restric 一樣只是不會記錄相關事情
    
    > [!important] 如果Restric 或是Protect 被啟動，需要清空mac address,讓特定主機使用該port，如果要清除可以使用Switch#
    > 
    > **clear port-security {all | conifgured | dynamic | sticky }** {**address** mac-addr | **interface** g0/1}
    
- example
    
    ```Plain
    int g0/1
    	switchport access vlan 991
    	switchport mode access
    	switchport port-security
    	switchport port-security violation retrict
    	spanning-tree portfast
    ```
    
- show
    - 顯示防護狀態
        
        ```Plain
        show port-security interface g1/0/11
        ```
        
    - Errdisable 摘要訊息
        
        ```Plain
        show int status err-disable
        ```
        
    - 顯示全部的
        
        ```Plain
        show port-security
        ```