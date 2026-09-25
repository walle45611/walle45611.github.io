
![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/01-Cisco 路由重分配 (route redistribute).png]]

## 位什麼需要route redistribute

- 網路中使用多個IP路由協定
- 多個廠商的路由環境
- 網路合併 ( 同一協議或是不同協議 )
- 從舊的路由協定過渡到新的路由協定
- 路由測略 ( 可靠性，冗餘性，分流模型等等 )

## 重分配需要考慮的問題

- 路由回饋
    
    - 因為AD值影響路由路徑導致產生了次優路徑
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/01-重分配需要考慮的問題.png|01-重分配需要考慮的問題.png]]
    
- 路由訊息不兼容 ( metric 訊息不一樣 )
- 收斂時間問題

## 如何選擇最佳路由

- metric
- AD (管理距離)
    
    |路由來源|AD|
    |---|---|
    |connected|0|
    |static|1|
    |EIGRP summary route|5|
    |外部BGP|20|
    |EIGRP|90|
    |OSPF|110|
    |ISIS|115|
    |RIPv2、RIPv1|120|
    |外部EIGRP|170|
    |內部BGP|200|
    |未知|255|
    

## 種子度量值 (default metric)

- 在自治區內部正常遞增，除了OSPF E2路徑

|將route redistribut到該協議|默認種子度量值|
|---|---|
|RIP|0，視為無窮大|
|IGRP/EIGRP|0，視為無窮大|
|OSPF|BGP為1，==其他路由為20==，OSPF之間的度量值不變|
|IS-IS|0|
|BGP|BGP度量值被設定為IGP度量值|

---

## redistrbute type

### redistrbute直連路由

- 所有直連接口，物理狀態up protocol up的情況才會重發布
- 當我不想要在R1上連接switch的介面上設定路由，有可能是因為設定動態路由協定會發送hello太佔網路頻寬或是其他考慮不設定靜態路由，就可以使用這個方法
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/02-redistrbute直連路由.png|02-redistrbute直連路由.png]]
    
- 這樣connected ，當L3SW redistrbute所有connected 介面時會把所有路由注入到自己的routing table中

![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/03-redistrbute直連路由.png|03-redistrbute直連路由.png]]

- command
    
    ```Plain
    R1(config)#router ospf 1
    R1(config)#redistrbute connected ....
    ```
    

### redistrbute動態路由

- 當路由表內有該協議的路由訊息才會被重發布
- redistrbute 到 OSPF
    
    - command
        
        ```Plain
        redistribute protocol [process-id | as-number]
        [metric metric-value]
        [metrci-type type-value]
        [match {internal | external 1 | external 2| nssa-external}]
        [tag tag-value]
        [route-map map-tag]
        [subnets]
        ```
        
        |選項||
        |---|---|
        |protocol|路由資訊的來源。包括bgp、connected、eigrp、isis、mobile、ospf、static、rip|
        |_process-id、as-number_|表示要在router全域設定命令上，對重分配的路由協定指定PID或是ASN|
        |metric-type { 1 \| 2 }|定義重分配外部路徑權值類型 : 1 (E1) or 2 (E2)|
        |match|如果重分配來自另一個OSPF程序的路徑，此參數可以用來比對OSPF的內部路徑、外部路徑(是類型而定)，以及NSSA外部路徑，已允許可被重分配的路徑|
        |tag|表示要將指數值只配給重分配的路徑，tag可以跟route-map給出的tag去做比對|
        |route-map|表示要參數照到某個route-map並透過對比來過濾路徑|
        |subnet|重分配分級式(classful)網路的子網路。如果沒有設定此參數，則只有完整的分級式網路路徑才會被重分配 (只有OSPF有)|
        
    
    - type 5 LSA
        
        - 執行重分配是ASBR的工作，因他負責將LSA通告到OSPF網路中，對於這類路徑，ASBR會建立第五類型的LSA
            
            |type 5 LSA|描述|
            |---|---|
            |LSID (鏈路狀態 ID)|子網路編號|
            |Mask|子網遮罩|
            |Advertising Router|負責通告路徑的ASBR之RID|
            |Metric|由ASBR所設定的metric|
            |External Metric Type|外部路徑權值類型為1或2|
            
        
        - 多個ASBR在區域內，如何找到最佳的ASBR
            
            1. 找出列在type 5 LSA裡的通告路由器ASBR
            2. 根據區域內的LSDB(介面成本)，計算達到任何ASBR最低成本路徑
            3. 到達ASBR(步驟2所選出的ASBR)的最佳路徑來使用出境介面和下一站
            4. 外部路徑的metric與列在type 5 LSA裡面相同，因此權值沒有任何變化
            
            - example (可參考case 1 topology)
                
                ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/04-redistrbute動態路由.png|04-redistrbute動態路由.png]]
                
            
            1. R4透過type 5 LSA得知RID 1.1.1.1是ASBR
            2. R4查看自己區域0的LSDB1，並計算有可能到達1.1.1.1的區域0路徑
            3. R4到達RID 1.1.1.1的最佳路徑是經過s0/0/0介面，故R4使用這些資訊到達需要到達的子網
            4. 該路徑列出的metric為20，與type 5 LSA一樣
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/05-redistrbute動態路由.png|05-redistrbute動態路由.png]]
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/06-redistrbute動態路由.png|06-redistrbute動態路由.png]]
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/07-redistrbute動態路由.png|07-redistrbute動態路由.png]]
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/08-redistrbute動態路由.png|08-redistrbute動態路由.png]]
            
        - 不同區域間如何找到ASBR
            
            - ABR接收到ASBR通告的type 5 LSA，先查看type 5 LSA的ASBR RID，然後ABR建立type 4 LSA，其中會比較type 5 LSA的RID即到達該ASBR的成本，再來就是flooding給相鄰的區域。
            
            1. 透過路由器根據所屬區域的LSDB，計算到達ABR的成本
            2. 與ABR到達ASBR的成本相加(該成本type 4 LSA)
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/09-redistrbute動態路由.png|09-redistrbute動態路由.png]]
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/10-redistrbute動態路由.png|10-redistrbute動態路由.png]]
            
            - show ip ospf data asbr-summary
                
                ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/11-redistrbute動態路由.png|11-redistrbute動態路由.png]]
                
                ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/12-redistrbute動態路由.png|12-redistrbute動態路由.png]]
                
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/13-redistrbute動態路由.png|13-redistrbute動態路由.png]]
            
            ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/14-redistrbute動態路由.png|14-redistrbute動態路由.png]]
            
        
        - E2/E1 patht差別
            
            E2不會累加經過路由的cost(metric)，E1則會經過路由器累加cost(metric)，也就是說E1是一般OSPF路由器的算法，而E2只會固定20或是ASBR定義的metric
            
    - NSSA
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/15-redistrbute動態路由 - NSSA.png|15-redistrbute動態路由 - NSSA.png]]
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/16-redistrbute動態路由 - NSSA.png|16-redistrbute動態路由 - NSSA.png]]
        
        ```Plain
        ASBR(config-router)#area 1 nssa on-summary
        ASBR(config-router)#redistribute eigrp 1 subnets
        ```
        
        ```Plain
        ABR1(config-router)#area 1 nssa on-summary 
        ```
        
        ```Plain
        R4(config-router)#area 1 nssa
        ```
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/17-redistrbute動態路由.png|17-redistrbute動態路由.png]]
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/18-redistrbute動態路由.png|18-redistrbute動態路由.png]]
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/19-redistrbute動態路由.png|19-redistrbute動態路由.png]]
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/20-redistrbute動態路由.png|20-redistrbute動態路由.png]]
        

---

### redistrbute靜態路由

- 可以用在這種情況，可能某些路由沒有對應的動態路由協定時可以使用這種方式將static發布到OSPF區域內
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/21-redistrbute靜態路由.png|21-redistrbute靜態路由.png]]
    
- command
    
    ```Plain
    R1(config)#router ospf 1
    R1(config)#redistrbute static ....
    ```
    

  

### 單點重發布

- 單向重發布
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/22-單點重發布 - 單向重發布.png|22-單點重發布 - 單向重發布.png]]
    
- 雙向重發布
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/23-單點重發布 - 雙向重發布.png|23-單點重發布 - 雙向重發布.png]]
    

  

### 多點重發布

- EIGRP預設本身不會domain loop problem，在OSPF和RIP
    - EIGRP internal AD 90 < OSPF external 110 並不會覆蓋掉內部的routing table
    - OSPF internal AD 110 < EIGRP external 170 並不會覆蓋掉內部的routing table
    - EIGRP internal AD 90 < RIP external AD 120
    - RIP internal AD 120 < EIGRP external AD 170
    - 修改AD值
        
        |路由協定|命令|
        |---|---|
        |RIP|distance ad-value|
        |EIGRP|distance eigrp internal-ad external-ad|
        |OSPF|distance ospf { external ad-value} {intra-area ad-value} inter-area ad-value}|
        
- 迴圈問題 (domain loop problem)
    
    當R1要到達B網段，就會產生routing loop，因為R3 RIP路由跳數比較少，所以會選擇跳數最少的當作最優路徑，但是路由的順序是R1→R2→R3重複，所以產生loop了
    
    ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/24-多點重發布.png|24-多點重發布.png]]
    

## case

### case 1

![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/25-case 1.png|25-case 1.png]]

![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/26-case 1.png|26-case 1.png]]

- redistrbute 到 EIGRP
    - command
        
        ```Plain
        redistribute protocol 
        [pid | asn] 
        [metric bw delay reliability load mtu] 
        [match| internal | nssa-external]
        [external 1 | external 2] 
        [tag tag-value] 
        [route-map name]
        ```
        
        |選項||
        |---|---|
        |protocol|路由資訊的來源。包括bgp、connected、eigrp、isis、mobile、ospf、static、rip|
        |_process-id、as-number_|表示要在router全域設定命令上，對重分配的路由協定指定PID或是ASN|
        |metric-type { 1 \| 2 }|定義重分配外部路徑權值類型 : 1 (E1) or 2 (E2)|
        |match|如果重分配來自另一個OSPF程序的路徑，此參數可以用來比對OSPF的內部路徑、外部路徑(是類型而定)，以及NSSA外部路徑，已允許可被重分配的路徑|
        |tag|表示要將指數值只配給重分配的路徑，tag可以跟route-map給出的tag去做比對|
        |route-map|表示要參數照到某個route-map並透過對比來過濾路徑|
        
    - methob 1 指定某條
        
        ```Plain
        RD1(config)#router eigrp 1
        RD1(config-router)#redistrbute ospf 1 metric 1544 2000 255 1 1500 
        ```
        
    - methob 2 全部
        
        ```Plain
        RD1(config)#router eigrp 1
        RD1(config-router)#redistrbute ospf 1 
        RD1(config-router)#default-metric 1544 2000 255 1 1500 
        ```
        
    - 會發現R2路由表中有D EX代表了EIGRP外部路由
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/27-case 1.png|27-case 1.png]]
        
    - 是所設定的metric，metric盡量也真實的介面設定
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/28-case 1.png|28-case 1.png]]
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/29-case 1.png|29-case 1.png]]
        
- redistrbute 到 OSPF
    - methob
        
        ```Plain
        RD1(config)#router ospf 1
        RD1(config-router)#redistribute eigrp 1 subnets
        ```
        
    - 在OSPF的database中可以看到172.30.2.0和6.0，表示了RD1(RID 1.1.1.1)通告了這五個新的type 5 LSA
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/30-case 1.png|30-case 1.png]]
        
    - 在R4路由表中可以看到是E2類型的外部路由，表示外部類型2的LSA，metric 是 20
        
        ![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/31-case 1.png|31-case 1.png]]
        

---

### case 2

![[Assets/Note/Tech/Cisco 路由重分配 (route redistribute)/32-case 2.png|32-case 2.png]]

- 重分布路由進RIP
    - 不要選擇太大或太小的metric，要選擇對的跳數
    - Command
        
        ```Plain
        R1(config)#router rip
        R1(config-router)#redistribute ospf 5 metric
        ```
        
- 重分布路由進EIGRP
    - 默認子度量值為0，盡量把參數跟物理界面盡量相同
    - 普通模式
        
        ```Plain
        R1(config)#router eigrp 1
        R1(config-router)#redistribute rip metric <Bandwidth> <delay> <reliability> <負載> <MTU>
        ```
        
    - 命名模式
        
        ```Plain
        R1(config-router)#address-family ipv4 unicast autonomous-system 1
        R1(config-router-af)#topology base
        R1(config-router-af-topology)#redistribute rip metric <Bandwidth> <delay> <reliability> <負載> <MTU>預設0.0.0.0
        ```