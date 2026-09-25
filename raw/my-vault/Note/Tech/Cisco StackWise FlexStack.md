## 概述

在傳統設備呢，都是獨立一台交換機，如果希望把交換機犯在同一個地方，就可以設定stackwise，就是讓很多台實體的Switch變成一個邏輯的switch

---

## StackWise組成

![[Assets/Note/Tech/Cisco StackWise FlexStack/01-StackWise組成.jpg]]

- Stack Member : stack中的每台都是stack memver
- Master : 從stack member中選一台當作master，控制其他台設備
- Switch : Number
- Stack Priority : 用於選舉，越大越優 範圍從1-15

### 個個組成在做什麼

- **Master做什麼**
    - **對堆疊組而言**
        1. 同步設備配置。
        2. 監控並同步各個member狀態信息。
        3. StackWise技術應用的可路由設備中，下發routing-table、forwarding-table、QoS策略等三層信息。
    - **對管理員而言**
        1. 提供CLI窗口，無論接到哪臺stack member中，實際上都是對master進行操作。
        2. 提供telnet、ssh等遠程登錄。
    - **對網絡其它設備而言**
        
        1.其它網絡設備在控制層面，實際上直接與master進行交互。
        
- **Slave做什麼**

接受master的管理，履行數據轉發層面的工作。

---

## **StackWise與FlexStack的比較**

FlexStack技術中，packet沿堆疊鏈路傳輸是hop-by-hop這種形式的，每臺設備收到packet時，都要端口上將packet加入到傳輸隊列，且該隊列受到QoS的影響。Packet在FlexStack中，傳輸到目標設備後就不再繼續轉發。當有某條stack鏈路故障時，故障2端的設備都只有half的堆疊帶寬，但是其它設備堆疊帶寬依然是full。

StackWise技術中，所有stack member構成的ring可以被看成是一個整體，一個環形軌道。packet一旦進入軌道，將沿軌道繞行，不受制於QoS影響，packet沿途被需要的設備複製取下，並繼續傳遞。通常情況下， packet傳遞迴起始設備後纔會被移除。當有某條stack鏈路故障時，整個stack帶寬降到half。

---

## 部署stack

### IOS限制

- **IOS版本必須一致**
    
    IOS版本號影響stack protocol的版本，stack protocol是stack members之間交互的協議，stack的部署要求stack protocol版本必須一致。
    

### StackWise連接方法

StackWise堆疊線的連接要求構成一個ring topology，此時交換機間通過堆疊線傳輸的速率能夠達到最大。而一旦其中有一條堆疊線故障，閉環被打破，在StackWise技術中傳輸速率降爲full值的一半。

- 2台設備連接
    
    ![[Assets/Note/Tech/Cisco StackWise FlexStack/02-StackWise連接方法 - 2台設備連接.jpg]]
    
    ![[Assets/Note/Tech/Cisco StackWise FlexStack/03-StackWise連接方法 - 2台設備連接.jpg]]
    
- 3台設備連接
    
    ![[Assets/Note/Tech/Cisco StackWise FlexStack/04-StackWise連接方法 - 3台設備連接.jpg]]
    
- 7台設備連接
    
    ![[Assets/Note/Tech/Cisco StackWise FlexStack/05-StackWise連接方法 - 7台設備連接.jpg]]
    

### 部署流程

- plan A : 設定各個設備的priority，誘發master競選，確保高priority勝出。
    1. 設定各設備的priority : 將欲指定爲master的設備配置較高的優先級，其它設備配置較低優先級。
    2. 保存配置關機
    3. 各設備逐臺開啓 : 各設備逐臺開啓
    4. 較驗
- plan B : 適用於需要給升級IOS的情況
    1. 設定各設備priority，保存配置關機 此時不連接堆疊線纜
    2. 先升級欲成爲master設備的IOS，重啓，先升級完並重啓後，使得該設備由於沒有檢測到其它member，直接成爲master。
    3. 升級第二臺設備並重啓
    4. 連接堆疊線纜，將第二臺設備加入堆疊組，在第二臺設備重啓過程中連接堆疊線纜，使得該設備由於檢測到“堆疊組內已有master”而自動成爲slave。
    5. 後續設備相同步驟
    6. 校驗

### 設定

1. **Stack Provision**
    
    - 術語
        - Provisionedconfiguration
            
            由管理員offline添加的配置，這個操作是在期望成爲master的設備上進行的，該設備加入其它stack後如果無法成爲master，provisionedconfiguration便不會被應用。\
            
        - Provisionedswitch
            
            加入stack並接受provisionedconfiguration的交換機爲provisioned switch。
            
    
    1. 創建provision組成員
        
        ```Plain
        Switch(config)#switch <number> provision <switch-type>
        ```
        
    2. 檢查
        
        ```Plain
        Switch#show switch           //此時在CurrentState欄可看到狀態爲“Provisioned”
        Switch#show ip interbr
        ```
        
2. **Stack Member Numbers**
    
    - 概述
    
    未進行配置時，設備默認number爲1，交換機加入到stack組中時，如果出現number與組內設備重複的情況，將放棄原有number，自動選取可用number中最小的
    
    - **Master & stack membernumber**
        1. Master與stack member number沒有絕對的關係，但是通常情況下master的number爲1。
        2. 當slave通過競選成爲master時，它原有的stack member number不會改變。
    - **修改stack member number**
        
        ```Plain
        Switch(config)# switch<current-number> renumber <new-number>
        Switch# reload slot<current-number>
        Switch#show switch
        ```
        
3. **Stack Priority**
    - 概述
        
        默認優先級爲1，可配置優先級從1-15，越大越優。
        
        優先級的修改主要用以影響master選舉，立即生效，但修改的效果將在下一次master election中體現。
        
    - 設定
        
        ```Plain
        Switch(config)#switch<number> priority <pri-value>
        Switch#show switch
        ```
        
4. 保存
    
    ```Plain
    Switch#copyrunning-config startup-config
    ```
    
5. show
    - 查堆疊端口情況
        
        ```Plain
        Switch#show switch neighbors
        ```
        
    - 爲每臺設備端口對端連接的設備number
        
        ```Plain
        Switch#show switchstack-ports
        ```
        
    - 檢查stack頻寬
        
        ```Plain
        Switch#show switchstack-ring speed
        ```
        
    - 檢查Stack Protocol版本
        
        ```Plain
        Switch#show platformstack manager all
        ```