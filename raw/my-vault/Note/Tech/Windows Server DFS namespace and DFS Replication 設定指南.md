---
Type:
  - windows Server DFS
---
## 概述

> [!important] win +r dfsmgmt.msc

透過DFS將相同檔案同時儲存到網路上多台server

- 可以提高檔案存取率 : DFS來存取資料時，DFS會引黨用戶端從，最接近用戶端的server來存取檔案，讓用戶端快速存取到檔案
    
    DFS會提供用戶端一份server list，這些server內都有用戶端所需的資料，但是DFS會將最接近用戶端的server給client使用，例如跟用戶端同一個ADDS站台的server，放在清單的最前面。
    
- 提高檔案的高可用性 : 若位於伺服器清單最前面的sever故障還是可以透過其他server拿到想要的資料
- 提供伺服器負載平衡的功: 每一個用戶端所獲得清單中的伺服器排序可能都不同，因此他們所存取的server也可能不同。
- 必須是NTFS，必須位於同一個ADDS樹

## DFS 架構

DFS namespace : 可透過DFS namespace來將位於不同伺服器內的共用資料夾結合在一起，並以一個虛擬資料夾的樹狀結構給用戶端

- 網域命名型 : 將namespace的設定資料存到ADDS database。如果有多台namespace的server會具備命名空間的容錯性。
- 獨立命名空間 : 他將命名空間的設定資料夾存到一台命名空間server的registry。命名空間僅支持命名空間，不具有容錯性 ㄙ
- 組成
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/01-DFS 架構 - 組成-2.png]]
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/01-DFS 架構 - 組成.png|01-DFS 架構 - 組成.png]]
    

## DFS replication

兩個目標對應的共用資料夾，期內提供給用戶端的檔案需要同步也就是相同，此同步動作可由DFS複寫來自動執行，DFS複寫使用一個**遠端差異壓縮(Remote Differential Compressions，RDC)**的壓縮演算法，他能夠偵測檔案異動，因此複寫檔案只有在異動的區塊複寫，不是整個檔案

如果獨立命名空間的目標server未加入網域，其目標對應到的共用資料夾內的檔案需要手動同步。

### replication topology

![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/02-replication topology.png]]

- hub and spoke : 中間一台server會互聯其他台server
- full mesh : 建立所有伺服器的邏輯連線

## case

![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/02-case.png|02-case.png]]

- Server1 是DC，安裝DFS namespace
- Server2 Server3 ，加入DC安裝DFS replication

![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/03-case.png|03-case.png]]

1. 安裝
    
    ```PowerShell
    Install-WindowsFeatrue fs-dfs-namespace -IncludeManagementTools
    Install-WindowsFeatrue fs-dfs-replication -IncludeManagementTools
    ```
    
2. Server2 Server3 創建共用資料夾，C:\Pictures證明有複寫
    - Server 2 C:\Pictures
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/04-case.png|04-case.png]]
        
    - Server3 C:\Pictures
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/05-case.png|05-case.png]]
        

  

1. create new namespace
    1. 指定server1 當作namespace
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/06-case.png|06-case.png]]
        
    2. 設定namespace 名稱
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/07-case - 設定namespace 名稱.png|07-case - 設定namespace 名稱.png]]
        
    3. 選擇網域模式
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/08-case - 選擇網域模式.png|08-case - 選擇網域模式.png]]
        
    4. create
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/09-case - create.png|09-case - create.png]]
        
2. create folder
    1. 創建
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/10-case - 創建.png|10-case - 創建.png]]
        
    2. 命名新增目的檔案
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/11-case - 命名新增目的檔案.png|11-case - 命名新增目的檔案.png]]
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/12-case - 命名新增目的檔案.png|12-case - 命名新增目的檔案.png]]
        
    3. 是否要複寫，案否等等設定
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/13-case - 是否要複寫,案否等等設定.png|13-case - 是否要複寫,案否等等設定.png]]
        
3. 設定複寫
    1. 設定複寫，按下去
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/14-case - 設定複寫,按下去.png|14-case - 設定複寫,按下去.png]]
        
    2. Next
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/15-case - Next.png|15-case - Next.png]]
        
    3. ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/16-case - Next.png|16-case - Next.png]]
        
    4. 選擇從哪裡回復至
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/17-case - 選擇從哪裡回復至.png|17-case - 選擇從哪裡回復至.png]]
        
    5. 選擇架構
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/18-case - 選擇架構.png|18-case - 選擇架構.png]]
        
    6. 複寫頻寬
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/19-case - 複寫頻寬.png|19-case - 複寫頻寬.png]]
        
4. Server2 Server 3 上面，可能複寫會很久喔
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/20-case.png|20-case.png]]
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/21-case.png|21-case.png]]
    
5. 設定複寫topology和排程設定
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/22-case.png|22-case.png]]
    
6. 停用複寫
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/23-case - 停用複寫.png|23-case - 停用複寫.png]]
    
7. 那些檔案不想被複寫
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/24-case - 那些檔案不想被複寫.png|24-case - 那些檔案不想被複寫.png]]
    
8. client test
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/25-case - client test.png|25-case - client test.png]]
    
9. 備援DFS namespace，新增一台Server4並安裝DFS namespace
    
    ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/26-case.png|26-case.png]]
    
10. 用戶轉借設定
    
    DC或是namespace service 會提供一個referrals給用戶端，此清單內包含的擁有此資源的目標伺服器，用戶端會嘗試從列於清單裝最前面的service來存取資源，若這台server無法提供服務就會向下一台目標services
    
    若此server需要暫停服務可以讓這台server不再referrals
    
    1. 先關閉
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/27-case - 先關閉.png|27-case - 先關閉.png]]
        
    2. 決定轉借清單優先順序
        
        ![[Assets/Note/Tech/Windows Server DFS namespace and DFS Replication 設定指南/28-case - 決定轉借清單優先順序.png|28-case - 決定轉借清單優先順序.png]]
        
        - 快取時間
            
            用戶端取得轉介清單後，會決定清單快取道用戶端電腦，以後用戶端需要此份清單時，可以直接存本地得知
            
        - 先後順序設定
            - 目標server與用戶端位於同一個ADDS站台
                
                此站台會被列在清單中最前面，如果有多台，會隨機排
                
            - 不同ADDS
                - 最低成本
                    
                    如果不同ADDS的話，連接站台成本最低的優先。cost相同隨機
                    
                - 隨機順序
                    
                    不論是位於哪一個ADDS都隨機
                    
                - 排除用戶端站台外的目標
                    
                    只要目標server與用戶端在不同的ADDS站台，就不將這些目標server列於轉借清單
                    

## 資料

- DFS namespace
    
    > [!info] DFS 命名空間概觀  
    > 適用于：Windows Server 2022、Windows Server 2019、Windows Server 2016、Windows Server 2012 R2、Windows Server 2012、Windows Server 2008 R2、Windows Server 2008 DFS (分散式檔案系統) 命名空間是 Windows Server 中的角色服務，可讓您將位於不同伺服器上的共用資料夾分組成一或多個邏輯結構化命名空間。 因此可以為使用者提供共用資料夾的虛擬檢視，其中單一路徑即可通向位於多部伺服器上的檔案，如下圖所示： 以下說明構成 DFS 命名空間的元素： 命名空間伺服器 - 命名空間伺服器會裝載一個命名空間。 命名空間伺服器可以是成員伺服器或網域控制站。 命名空間根目錄 - 命名空間根目錄是命名空間的起始點。 在上圖中，根目錄的名稱為 Public，命名空間路徑為 \\Contoso\Public。 這種類型的命名空間以網域名稱 (例如 Contoso) 開頭，而且中繼資料儲存在 Active Directory Domain Services (AD DS)，因此是網域型命名空間。 雖然上圖顯示的是單一命名空間伺服器，但網域型命名空間還是可以裝載於多部命名空間伺服器來提高命名空間的可用性。 資料夾 -  
    > [https://docs.microsoft.com/zh-tw/windows-server/storage/dfs-namespaces/dfs-overview](https://docs.microsoft.com/zh-tw/windows-server/storage/dfs-namespaces/dfs-overview)  
    
- DFS複寫
    
    > [!info] DFS 複寫概觀  
    > 適用于：Windows Server 2022、Windows Server 2019、Windows Server 2016、Windows Server 2012 R2、Windows Server 2012、Windows Server 2008 R2、Windows Server 2008 DFS 複寫是 Windows Server 中的角色服務，可讓您在多個伺服器與站台有效率地複寫資料夾 (包括 DFS 命名空間路徑參照的資料夾)。 DFS 複寫是多重主要複寫引擎，可供您透過有限頻寬的網路連線讓伺服器之間的資料夾保持同步。 它會取代檔案複寫服務 (FRS) 作為 DFS 命名空間的複寫引擎。 提示 請考慮使用 Azure 檔案同步來減少內部部署儲存體使用量。 Azure 檔案同步可以讓多個Windows檔案伺服器保持同步，而且每個伺服器只需要在內部部署保留快取，而資料的完整複本位於雲端中。 Azure 檔案同步也有雲端備份與整合式快照集的額外優點。 如需詳細資訊，請參閱 規劃Azure 檔案同步部署 。 Active Directory 網域服務 (AD DS) 使用  
    > [https://docs.microsoft.com/zh-tw/windows-server/storage/dfs-replication/dfsr-overview](https://docs.microsoft.com/zh-tw/windows-server/storage/dfs-replication/dfsr-overview)