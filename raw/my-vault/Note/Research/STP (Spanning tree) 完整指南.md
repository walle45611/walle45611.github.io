## 為什麼需要

因為switch備援路線會導致網路有回環產生進而產生出廣播風暴和Mac address泛洪的問題，所以要解決這個問題就有人提出使用Spanning tree這項演算法來解決此問題。

![[Assets/Note/Research/STP (Spanning tree) 完整指南/01-為什麼需要.png|01-為什麼需要.png]]

## Spanning-tree standards

- 802.1D
- PVST+ Cisco私有協定，沒個VLAN有自己的Spanning tree
- 802.1W RSTP
- 802.1S MSTP 2大型網路
- PVRST+ Cisco私有，對RSTP的增強版

|協定|標準|所有資源|收斂速度| |
|---|---|---|---|---|
|CST|802.1D|低|慢|所有VLAN|
|PVST+ 預設|Cisco|高|慢|單個VLAN|
|RSTP|802.1W|中|快|所有VLAN|
|RPVST+ 用更多的|Cisco|很高|快|單個VLAN|
|MST 用更多的|802.1S|中/高|快|VLAN列表|

---

# STP 802.1D

## BPDU (bridge protocol data unit)

- 利用BPDU傳送STP的各種訊息在switch之間來進行選舉。
- 組播地址因為是L2所以是mac address 0180:c200:0000
- 在廣播域選擇一個Root Switch且這台Switch的Port都是DR。
- 其他台的Switch利用Root Switch的來計算對於Root Switch開銷。
- 不是Root Switch的Switch選出一個RP且這個Port是朝向Root Switch，且會一直收到Root Switch的BPDU訊息來更新自己的STP。
- 每個Segment選出一個DP。
- 如果選舉出來有個Port沒有被選到就會是Block Port這樣回環就被打破了。

BPDU分成兩種類型一種是

- config BPDU再設定的時候會發佈
- TCN網路topology有改變時會發佈
- BPDU訊息內容
    
    ![[Assets/Note/Research/STP (Spanning tree) 完整指南/02-BPDU (bridge protocol data unit) - B.png|02-BPDU (bridge protocol data unit) - B.png]]
    
|Byte|意義|描述|
|---|---|---|
|2|協議|代表上層協議(BPDU)，必為0。|
|1|版本|(802.1D為0)。|
|1|message TYPE|Config BPDU為0，Topology change BPDU為80。|
|1|Flag|LSB最低有效為表示TC標示 ; MSB最高有效未表示TCA標示。|
|8|Root ID|Root Switch ID收斂後就是Root Switch保持不變。|
|4|Cost|到達Root Switch的cost。|
|8|bridger ID|發送BPDU的switch的ID。|
|2|Port ID|BPDU發送的bridger的Port ID(優先及(預設為128)+Port ID)。|
|2|Message age 訊息壽命|從Root SW發出的BPDU之後的秒數，每經過一個SW就減一，所以本質上是達到Root SW的跳數。|
|2|Max age 最大壽命|當一段時間未收到任何BPDU，生存騎到達Max age時，SW認為該Port故障。default 20s。|
|2|HELLO時間|Root SW連續發送BPDU之間的時間間隔。default 2s|
|2|轉發Delay|在listen和學習狀態所停留的時間間隔。default 15s|
    

---

## Bridge ID

![[Assets/Note/Research/STP (Spanning tree) 完整指南/03-Bridge ID.png|03-Bridge ID.png]]

- priority ID
    - 只能設定4096的倍數
- external system id
- **minimum** mac address

> [!important] bridge id : 16*4=64 bits 其中包括 48 bit mac address 和 12 bit external system id (vlan id) 和 4bit priority，priority最後要乘4096

![[Assets/Note/Research/STP (Spanning tree) 完整指南/04-Bridge ID.png|04-Bridge ID.png]]

---

## STP Cost

1. root bridge 發出的cost為0的BPDU，因為交換port就在root bridge
2. 當下一個接近的鄰居收到BPDU時，就會將BPDU到達的交換port之路徑成本與0相加。BPDU被接收時完成的
3. 此鄰居將以新累加值作為root path cost的BPDU發送出去
4. 下游的每個鄰居都會累加cost，並送出BPDU

> [!important] 強調是收到的時候是因為，當計算SPT時是用新的root path cost在BPDU進入時發生，而非離開的

|Link Speed|Cost IEEE規定的|
|---|---|
|10 Gb/s 以上|2|
|10 Gb/s|2|
|1 Gb/s|4|
|622 Mb/s|6|
|155 Mb/s|14|
|100 Mb/s|19|
|45 Mb/s|39|
|16 Mb/s|62|
|10 Mb/s|100|
|4 Mb/s|250|

---

## port id

- 只能更改128部分小數點部份是按照順序的

![[Assets/Note/Research/STP (Spanning tree) 完整指南/05-port id.png|05-port id.png]]

```Plain
SW1(config-if)#spanning-tree port-priority <0-192 64的倍數>
```

---

## 802.1D STP 狀態

- disable
    
    此狀態不屬於STP發展的一部分，也就是說沒辦法再show指令上觀察到disable相關的字眼
    
- Blocking
    
    當交換port經過初始狀態，便處於blocking狀態，使得橋接loop無法產生。blocking狀態中不能接收或傳送資料，並且不法爸mac位置新增到他的位置表中;只能接收BPDU，以便聆聽其他鄰居的狀態
    
- Listening 15s PC———SW PC開機
    
    Blocking轉換到listing，交換port還是不能發送或接收一般資料，但是可以收BPDU，使他可以主動餐與STP topology的程序中，由於交換器藉由發送BPDU通告此交換port，故此port可以成為PR或是DP。如果失去RP或DP的角色就會回到blocking
    
- learing 15s
    
    在listening狀態中所花的時間稱為forwarding delay。當經過此段時間，可以接收發送BPDU，此外交換機可以學習mac address，learing的這段時間內，交換機還是可以做自己的事，收集mac address，但是還是無法接收發送資料。
    
- forwarding
    
    可以開始發送接收資料，也可以接收發送BPDU。最終的狀態，除非是檢測到bridging loop，如有檢測到切換到blocking
    

|STP status|交換port具備的功能|交換port不具備的功能|持續時間|
|---|---|---|---|
|disable|N/A|發送資料或收到資料|N/A|
|blocking|接收BPDU|發送資料或是學習mac address|如果偵測到迴圈，則無期限封鎖|
|listening|發送接收BPDU|發送資料或是學習mac address|forwarding delay 15s|
|learing|發送接收BPDU，學習mac address|收或發送資料|forwarding delay 15s|
|forwarding|發送接收BPDU，學習mac address，收或發送資料||只要交換機處於up且沒有偵測到loop時間無期限|

---

## 802.1D STP Role

|Role|功能|
|---|---|
|RP (root port)|switch最靠近root bridge之switch port|
|DP (Designate port)|lan segment最靠近root bridge的switch port。該port沿著stp向下發送BPDU|
|Blocking port|不是DP也不是RP的switch port|
|alternate port|backup RP，也靠近root bridge，但會處於blocking port的狀態。藉由STP uplinkfast功能來識別這些替代port|
|Forwarding port|沒有偵測其他STP活動情況的交換port|

---

## 802.1D STP 選舉的整體流程

1. 先選出Root SW
    
    通常Root Bridge會放在distrlutein layer中因為這樣access layer and core layer可以更快地得知TCN的訊息
    
    STP Root bridge選舉操作
    
    透過BPDU傳送以下訊息來進行選舉，主要會看以下這兩個數值
    
    - priority是STP的優先權數越小的越先預設值32768
    - mac address是所有介面卡網卡的第一個mac address
    - 一開始所有的Switch都會發送BPDU訊息等到網路收斂完畢，也就是選出Root Switch就會只有Root Switch發送BPDU訊息
    - 從RP收到Root Switch的BPDU訊息之後從DP傳送出去這個動作稱為Relay
2. 在非Root bridge上選出一個RP
    
    STP RP選舉操作tie-breaking
    
    - 最低的root bridge id
    - 到Root SW的最低cost
    - sender最低的BID
    - sender最低的Port ID
3. 在每個segment上選出DP
    
    STP DR選舉操作tie-breaking
    
    - 最低的root bridge id
    - 到Root SW的最低cost
    - sender最低的BID
    - sender最低的Port ID
4. 非DP就是BLK Port

### 手動計算STP

|任務|描述|
|---|---|
|1. 辨識鏈路cost|對於交換器的每條鏈路，寫出每台交換器使用該鏈路的成本|
|2.辨識出root bridge|找出bridge id最小的交換器|
|3.選出RP 每台交換器只有一個|找出前往根橋接器之最佳路徑的交換port，其中根路徑成本最低|
|4.選出DP 一段區段只有一個|對於交換器之間的每條鏈路，找出鏈路的某一端為DP。該port的路徑成本最低;如果相同，會使用tie-breaking的條件。|
|5.辨識出BLK port|非RP或DP就是BLK port|

---

### 802.1D 選路範例

### case 1

![[Assets/Note/Research/STP (Spanning tree) 完整指南/06-case 1.png|06-case 1.png]]

### case 2

![[Assets/Note/Research/STP (Spanning tree) 完整指南/07-case 2.png|07-case 2.png]]

---

## 802.1D timer

- hello timer
    
    由root bridge發送BPDU的時間間隔。會決定所有非root bridge發送BPDU的間隔時間。802.1D預設是hello 2s
    
- forwarding delay timer
    
    進入Listening與Learning預設15s
    
- Max age timer
    
    在丟棄BPDU前，儲存該BPDU的時間間隔，每個交換port會保留其最佳BPDU。如果沒有收到BPDU，等待max age逾期然後丟棄，預設20s
    

|   |   |
|---|---|
|hello|組態BPDU的間隔時間|
|forwarding delay|進入forwarding狀態，所處listening和Learing狀態的時間而|
|max age|在沒有收到更新之前保存BPDU的最長時間；到期表示遇到DP或是root bridge壞掉了|

- command
    - hello timer 
        
        ```Plain
        sw(config)#spanning-tree [vlan vlan-id] hello-time seconds
        ```
        
    - forwading delay timer 
        
        ```Plain
        sw(config)#spanning-tree [vlan vlan-id] forwarding-time seconds
        ```
        
    - max age timer
        
        ```Plain
        sw(config)#spanning-tree [vlan vlan-id] max-age seconds
        ```
        
    - 自動調整
        
        ```Plain
        sw(config)#spanning-tree vlan vlan-id root {primary | secondary} [diameter diamter [hello-time hello-time]]
        ```
        

---

## 802.1D TC(Topology change)機制

![[Assets/Note/Research/STP (Spanning tree) 完整指南/08-802.1D TC(Topology change)機制.png|08-802.1D TC(Topology change)機制.png]]

|欄位描述|byte number|
|---|---|
|協定ID 0|2|
|版本必為0|1|
|配置或 TCN BPDU|1|

當交換器的port轉移到forwarding stata或是forwarding或learning轉到blocking狀態時，即topology change。換句話說交換port的狀態不是up 就是down，TCN BPDU只是發送拓樸變更消息不是攜帶有關資料。如果開啟portfast不會發送TCN BPDU Message。

交換機會隔hello時間發送TCN BPDU直到上游鄰居確認直到root bridge ack，當root bridge收到TCN BPDU會將自己的BPDU topology change flag設定，並將該BPDU傳到其他bridge。

### 直接拓樸變更

是可以偵測的一種類型例如說，少一條trunk線路斷掉，鏈路一端的switch就會立刻偵測，因此改變topology。

### 間接拓樸變更

也就是說不是沒有檢測鏈路損壞，但是BPDU packet無法送達，這時候因為沒有偵測topology發生的工具，所以就只能依靠max age timer等到期了就會自動發送新的BPDU，這時候就會有新的topology。

---

# RSTP 802.1W

- 802.1D STP能夠在大約1min內恢復連接
- 802.1W STP能夠快速收斂
- 切換模式指令
    
    ```Plain
    SW1(config)#spanning-tree mode rapid-pvst
    ```
    

---

## 802.1W 狀態

![[Assets/Note/Research/STP (Spanning tree) 完整指南/09-802.1W 狀態.png|09-802.1W 狀態.png]]

- discarding : 將收到的data丟棄且無法學習到Mac address這種狀態合併了802.1D disable、listening、blocking，因為這三個狀態都無法傳送資料。況且listening根本多餘，因為RSTP能夠迅速的協商狀態變化，無須聆聽BPDU
- learning : 會丟棄data，可以學到mac address
- forwarding : 根據mac address來轉發data

---

## 802.1W interface Role

![[Assets/Note/Research/STP (Spanning tree) 完整指南/10-802.1W interface Role.png|10-802.1W interface Role.png]]

- RP : 在交換機中，由到達root bridge最低的cost的switch擔任此角色，但是root bridge沒有此角色root bridge只有DP跟802.1D一樣
- DP : 在每個lan segment，由到達root bridge的cost最低switch port功能與802.1D一樣發送BPDU
- AP : 由擁有前往root bridge的替代路徑，與RP採用的路徑不同。替代路徑成本高一些例如 : 如廣將access layer two uplink，其中一個將成為root port另一個成為alternate port。
- BP : 由連接到該segment的備援路徑，也就是說DP的backup，這個port有可能沒有

---

## 802.1W BPDU

RSTP使用的是802.1D的BPDU格式，以便向下兼容，RSTP使用message type這欄位。發送RSTP角色和狀態來辨識自己。RSTP的BPDU version被設定為2以便和802.1D做區別。無論是否有收到root bridge BPDU，所有switch port都每隔hello時間發送一次hello，網路中任何交換器都會主動維護Topology，==如果連續三次沒有收到BPDU==，==就會認為該鄰接switch出問題了==，就會把該前往該鄰接的資料刪除。==代表802.1W使用3個hello (6s)，802.1D使用max age(20s)==

如果有遇到802.1D就需要轉換封包格式，這時候會有一種「鎖住」的協定措施，以免再短時間收到802.1D和802.1W的BPDU，在轉換的過程中會有migration delay timer會鎖住協定類型，以免平凡切換STP類型。

---

## Link Type

- Edge Port :
    
    與一台主機相連就屬於edge port，習慣上是透過portfast來決定RSTP承襲了PortFast的概念，這類的switch port可以立即進入forwarding sstatus，如果再edge port上收到BPDU就會失去此資格。
    
    - command
        
        ```Plain
        SW(config-if)#spanning-tree portfast 
        ```
        
- Root Port :
    
    與Root Bridge最低cost就是屬於此type， 一般來說只能有一個RP但是會選出AP這個角色來當作備用，所以當RP出事了，備用port會立即變成forwarding status。
    
- Point-to-Point Port
    
    任何連接其他交換機並成為DP的switch port就屬於這個type。直接相連的switch可以利用handshake就能同意switch port的狀態，不需要等到timer到期。並以proposal和agreement的方式交換BPDU。一台交換機成為DP，如果另一台接受，將其同意訊息進行回應。
    
    p2p port以雙工的模式來決定，採用全雙工可以視為p2p port type，可以透過handshake快速交換訊息完成STP convergence，如果是半雙工就不屬於此類型，且只能使用傳統802.1D的收斂方式。
    
    - command
        
        正常來說是自動判別的但是可能某須原因需要半雙工，就可以使用此指令更改預設
        
        ```Plain
        Sw(config-if)#spanning-tree link-type point-to-point
        ```
        

---

## 802.1W Sync

1. 如果proposal的發送者擁有更好的BPDU，則交換器本身認為發送者有資格成為DP，因此自己的交換port必須成為新的root port
2. 交換機同意任何事情前，自己必須與topology進行同步
3. 所有非edge port進入discarding (blocking)狀態，防止bridge loop
4. 向發送者回覆同意BPDU，同意選出來新的DP這邊也告訴發送者，此交換機正在同步中
5. RP立即進入forwarding。發送者的switch port也能夠立即開始轉送
6. 對於目前處於discarding狀態的非edge port，提議訊息會被傳送到各自的鄰居
7. 期望從非edge port上收到同意訊息
8. not edge port 進入forwarding

![[Assets/Note/Research/STP (Spanning tree) 完整指南/11-802.1W Sync.png|11-802.1W Sync.png]]

---

## 802.1W TC機制

![[Assets/Note/Research/STP (Spanning tree) 完整指南/12-802.1W TC機制.png|12-802.1W TC機制.png]]

- 直接通告給鄰居不用再經過root bridge

1. root bridge知道網路topology產生變更時，設定BPDU的TC
2. 此BPDU傳輸給網路中的所有bridge
3. bridge接收到了TC位置BPDU後，將bridge table老化時間300s降到forwarding delay 15s

---

## 802.1W設定

- 配置模式
    
    ```Plain
    SW1(config)#spanning-tree mode rapid-pvst
    ```
    
- 指定root
    
    - 32768 - 4096*2
    
    ```Plain
    Switch(config)#spanning-tree vlan 1 root primary
    ```
    
- 指定secondary root
    
    - 32768 - 4096
    
    ```Plain
    Switch(config)#spanning-tree vlan 1 root secondary
    ```
    
- 指定priority
    
    ```Plain
    SW(config)#spanning-tree vlan 1 priority priority
    ```
    

---

# MST 802.1S (**Multiple Spanning Tree Protocol**)

- 802.1Q : 所有VLAN採用同一個STP如果有500個vlan總共只有一個STP不會像傳統的會有500個STP，因此稱為共同擴展樹(CST)，運作在trunk navite vlan

## MST 概觀

RSTP的問題，RSTP都是Pre-VLAN Base也就是說STP的樹木就等於VLAN數目如果，有100 vlan就要處理100 STP這樣會耗費大量的RAM和CPU，所以MSTP提出了instance，同一組instance的vlan共用一組STP，舉例來說vlan 100到200，可以把vlan 100至 vlan 150用到inst的STP

## MST area

雖然MST和802.1Q and PVST+相容，但MST與他們都不同。如果switch使用MST，就必須知道鄰居使用的STP類型。透過將switch加入到MST area，使得area內部的switch可以使用相同的參數執行mst

## MST內部的STP

- CST
    
    MST能夠與其他STP相容，因此需要支援各種版本的STP，可以將企業網路想成一個CST topology，所有任何東西，包括vlan MST area都共用一個STP。CST只是維護not loop的topology，同時集合了各種可用的STP。
    
- IST
    
    每個MST內部需要有一個計算not loop的機制，只要在一個MST area執行IST(internal spanning tree)，便會在CST和MST區域交界處的鏈路及區域內部的交換器之間，建立一個沒有loop的topology。你可以將IST視為本地的CST，並以MST area為邊界
    

![[Assets/Note/Research/STP (Spanning tree) 完整指南/13-MST內部的STP - IST.png|13-MST內部的STP - IST.png]]

## MSTP設定

- 指定模式
    
    ```Plain
    switch(config)#spanning-tree mode mst
    ```
    
- 參數配置
    
    ```Plain
    switch(config)#spanning-tree mst configuration
    ```
    
    - name
        
        ```Plain
        switch(config-mst)#name school
        ```
        
    - Revision number版本號，每次設定應都要加1
        
        ```Plain
        switch(config-mst)#revision 1
        ```
        
    - vlan instance對照表
        
        ```Plain
        switch(config-mst)#instance inst-num vlan range
        ```
        
- 設定primary secondary rootsr和設定priority
    
    ```Plain
    Switch(config)#spanning-tree mst inst-num root [primary | secondary]
    ```
    
    ```Plain
    Switch(config)#spanning-tree mst inst-num priority <0-61440 4096的倍數> 
    ```
    
- 查看
    
    ```Plain
    SWitch(config-mst)#show pending
    ```
    
- 生效
    
    ```Plain
    SWitch(config-mst)#exit
    ```
    
- MST設定的一些指令
    
|任務|指令|
|---|---|
|設定root bridge|Switch(config)#**spanning-tree mst** _instance-id_ **root** {**primary** \| **secondary**} [**diameter** _diameter_]|
|設定priority|Switch(config)#**spanning-tree mst** _instance-id_ **priority** _bridge-priority_|
|設定cost|Switch(config-if)#**spanning-tree mst** _instance-id_ **cost** _cost_|
|設定port priority|Switch(config-if)#**spanning-tree mst** _instance-id_ **port-priority** _port-priority_|
|設定STP timer|Switch(config)#**spanning-tree mst hello-time** _seconds_ Switch(config)#**spanning-tree mst forward-time** _seconds_ Switch(config)#**spanning-tree mst max-age** _seconds_|
    

---

# 保護STP的topology

## 防止非預期的BPDU

### portfast

![[Assets/Note/Research/STP (Spanning tree) 完整指南/14-portfast.png|14-portfast.png]]

- 能夠使得L2介面立即進入轉發狀態，在一秒內
- 連接用終端用戶的介面可以開啟
- cisco private
- command
    - 介面設定
        
        ```Plain
        Switch(config-if)#spanning-tree portfast
        ```
        
    - 全域設定 所有的access都開啟portfast
        
        ```Plain
        Switch(config-if)#spanning-tree portfast default
        ```
        
        ![[Assets/Note/Research/STP (Spanning tree) 完整指南/15-portfast.png|15-portfast.png]]
        
    - 介面禁用
        
        ```Plain
        Switch(config-if)#no spanning-tree portfast
        ```
        
    - 指定access啟用portfast
        
        ```Plain
        Switch(config-if)#switchport host
        ```
        

---

### RootGuard

- 防止搶佔掉root SW
- 啟用防護的介面不能成為RP，將成為DP
- 該介面收到更好的BPDU，根防護特性就會進入 root-inconsistent狀態
- root-inconsistent處於listen狀態
- 等到沒有收到更優BPDU，會自動恢復
- 使用的地方access layer Switch
- command
    
    ```Plain
    SW(config-if)#spanning-tree guard root
    ```
    

---

### BPDUguard

- 如果再Access layer Switch上接上一台switch且priority為0，會跟root SW搶佔root的位置，就會造成網路的不穩定，所以可以使用BPDUfilter或是BPDUguard
- 如果從portfast介面收到BPDU，就會把狀態設定為error disable，可以透過sh和no sh重新開啟介面
    
    ![[Assets/Note/Research/STP (Spanning tree) 完整指南/16-BPDUguard.png|16-BPDUguard.png]]
    
- command
    - 只有portfast才會生效
        
        ```Plain
        SW2(config)#spanning-tree portfast bpduguard default
        ```
        
        ```Plain
        SW2(config-if)#spanning-tree bpduguard enable
        ```
        
    - 自動恢復功能
        
        ```Plain
        SW2(config)#errdisable recovery casue bpduguard
        S鏈路
        ```
        

---

### BPDUfilter

- 過濾BPDU封包
- 很像passive interface
- 能夠限制交換機開啟了portfast端口發送不必要的BPDU
- 可以在全域開啟BPDUfilter，portfast要開啟，不然會沒有作用
- 介面開啟不發送BPDU也不接收，收到後丟棄
- BPDUfile < BPDU Guard
- BPDUGuard BPDUfile同時開啟BPDUfile有效
- command
    
    - 只有portfast才會生效
    
    ```Plain
    SW(config)#spanning-tree portfast bpdufilter default
    ```
    
    ```Plain
    SW(config-if)#spanning-tree portfast bpdufilter enable
    ```
    

---

## 防止BPDU意外遺失

### LoopGuard

- 如果switch的某個port處於blocking狀態，但是blocking要一直收到BPDU封包，但是出於某些原因沒辦法收到了，this port就會認為接入其他設備，也就會變成DP role，這樣就有可能會造成bridge loop，此功能就是會持續會記錄非DP role的BPDU活動情況;當收到BPDU正常工作，沒有收到時進入loop-inconsistent狀態
- 避免bridge loop，提高網路穩定性(主要是防止鏈路產生單向故障)
- 使用場景 : 都可以，因為一般網路線也會有RX TX、光纖、半雙工網路環境

![[Assets/Note/Research/STP (Spanning tree) 完整指南/17-LoopGuard.png|17-LoopGuard.png]]

1. 當SW2的TX出現問題後，SW3過渡到STP狀態，將產生環路
2. 開啟loopguard，BPDU max-age(20s)之後，Blocking將會進入到loop-inconsistent狀態

- command
    
    ```Plain
    SW(config)#spanning-tree loopguard default
    ```
    
    ```Plain
    SW(config-if)#spanning-tree guard loop
    ```
    

---

### UDLD (Unidirectional link detection)

- 能夠檢測並禁用單向鏈路
- 是一個二層協定、與一層機制協同工作
- 啟用UDLD，交換機會定期像鄰居發送UDLD封包，並且要求收到回應，否則判斷為單向鏈路，並且關閉該介面
- 模式
    - Normal mode ， 只能檢測光纖，未收到UDLD訊息時，介面會undeteermined狀態產生log，並不會引響流量轉發
    - aggressive mode ，檢測光纖一般網路線都可，未收到UDLD訊息，就會重新建立鄰居關係，嘗試連接8次都失敗，就會變成error-diable狀態
- command
    
    ```Plain
    sw(config)#udld enable
    sw(config)#udld aggressive
    ```
    
    ```Plain
    sw(config-if)#udld port
    sw(config-if)#udld port aggressive
    ```
    
    ```Plain
    SW#udld reset
    ```
    
- Loopguard和UDLD差別
    
| |LoopGuard|UDLD|
|---|---|---|
|檢測基於|基於VLAN|基於端口|
|自動恢復|是|具有err-disable超時性|
|防止由於設定問題所影響的DP介面不能發送BPDU所到至的故障|是|否|
|防止配線錯誤|否|是|
    

---

### uplinkfast

![[Assets/Note/Research/STP (Spanning tree) 完整指南/18-uplinkfast.png|18-uplinkfast.png]]

- 同常用在root SW和一般的交換機之間，當線路故障時就可以使用uplinkfast就會加快收斂速度，
    
    因為有隱藏alternate port，所以可以快速的切換RP port切換成有alternate port的switch port
    
- A會去檢測有沒有收root SW BPDU，如果沒有收到就使用uplinkfast切換到backup陷入
- 從blocking→forwarding≤5s
- 在A上開
- 啟用條件
    - 交換機至少要有一個介面處於blocking狀態
    - 鏈路失效發生在根端口
- 啟用後會更改的value
    - priority有32768→49152加了3*4096
    - cost增加到3000
    - priority與cost不變
- 設定
    
    ```Plain
    sw(config)#spanning-tree uplinkfast
    ```
    
    ```Plain
    sw(config)#spanning-tree uplinkfast max-update-rate <0-32000>
    max-update-rate代表每秒鐘發送的多播數據的數目，預設是每秒150個
    ```
    

---

### BackboneFast

![[Assets/Note/Research/STP (Spanning tree) 完整指南/19-BackboneFast.png|19-BackboneFast.png]]

- 用於檢測鏈路，使介面blocking快速到listening (跳過20s)
- 通過檢查次級的BPDU來知道發生故障了
- root SW和backup SW開啟
- command
    
    ```Plain
    SW(config)#spanning-tree backbonefast
    ```
    

---

## 保護STP的topology的指令總結

||global command|interface command|
|---|---|---|
|RootGuard||Switch(config-if)#spanning-tree guard root|
|BPDUGuard|Switch(config)#spanning-tree portfast bpduguard defalut|Switch(config-if)#spanning-tree bpduguard enable|
|loopGuard|Switch(config)#spanning-tree loopguard default|Switch(config-if)#spanning-tree guard loop|
|UDLD|Switch(config)#udld {enable \| aggressive \|message time seconds}|Switch(config)#udld {enable \| aggressive \|disable}|
|BPDUfilter|Switch(config)#spanning-tree bpdufilter default|Switch(config-if)#spanning-tree bpdufilter enable|

||command|
|---|---|
|列出被標記inconsistent的port|show spanning-tree inconsistentports|
|顯示BPDUGuard、BPDUfilter、loopGuard 的狀態|show spanning-tree summary|
|顯示一個或所有switch port udld狀態|show udld [type mode/num]|
|遭到udld童用的errdisable port|udld reset|