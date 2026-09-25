---
Type:
  - windows Basic
---
## NTFS和ReFS權限種類

### 基本的檔案權限種類

- 讀取 : 可以讀取檔案內容、檢視檔案屬性與權限
- 寫入 : 可以修改檔案內容、在檔案後面增加與改變檔案屬性，使用者需要讀取的權限才可以使用
- 讀取和執行 : 除了擁有讀取，還具有執行程式的能力
- 修改 : 除了擁有讀取寫入執行，還可以刪除檔案
- 完全控制 : 擁有前述所有權限，再加上變更權限與取得權限的特殊權限

### 基本的資料夾權限

- 讀取 : 可以檢視資料夾內的檔案與子資料夾名稱、檢視資料夾屬性與權限
- 寫入 : 他可以在資料夾內新增檔案與子資料夾、改變資料夾屬性
- 列出資料夾內容 : 擁有讀取權限之外，還具備周遊資料夾的特殊權限，也就是可以進出資料夾
- 讀取和執行 : 他與列出資料夾相同，不過列出資料夾權限只能被資料夾繼承，而讀取執行則會同時被資料夾和檔案繼承
- 修改 : 除了擁有讀取寫入執行，還可以刪除檔案
- 完全控制 : 擁有前述所有權限，再加上變更權限與取得權限的特殊權限

---

## 使用者的有效權限

### 權限是可以被繼承的

下層的資料夾會繼承上層資料夾的權限，例如UserA對甲資料夾有讀取的權限，則A對甲資料夾內的資料有讀取的權限

也可以設定不讓資料繼承資料夾的權限，或是可以單一個資料需要繼承

設定子資料夾或檔案權限時，可以讓子資料夾不要繼承父資料夾的權限

### 權限有累加姓

如果使用者同時隸屬於多個群組，且使用者與這些群組分爺對某個檔案擁有個別的權限設定，則該使用者對此檔案的最後有效權是這些權限的總合

|使用者群組|權限|
|---|---|
|UersA|寫入|
|Group A|讀取|
|Group B|讀取和執行|
|最後權限|`寫入 + 讀取 + 執行`|

### 拒絕優先權限比較高

如果其中一個設定為拒絕的話，則使用者就不會擁有存取權。

|使用者群組|權限|
|---|---|
|UersA|寫入|
|Group A|讀取被拒絕|
|Group B|讀取和執行|
|最後權限|讀取權限為拒絕|

> [!important] 繼承的權限，其優先權比直接設定的權限還要低

---

## 權限設定

### 繼承關係

![[Assets/Note/Tech/Windows 檔案權限管理指南/01-繼承關係.png|01-繼承關係.png]]

- 在C:\Test
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/02-繼承關係 - 在C Test.png|02-繼承關係 - 在C Test.png]]
    

### 新增使用者

![[Assets/Note/Tech/Windows 檔案權限管理指南/03-新增使用者.png|03-新增使用者.png]]

![[Assets/Note/Tech/Windows 檔案權限管理指南/04-新增使用者.png|04-新增使用者.png]]

### 取消繼承

- User被繼承了
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/05-取消繼承 - User被繼承了.png|05-取消繼承 - User被繼承了.png]]
    
- 停止繼承
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/06-取消繼承 - 停止繼承.png|06-取消繼承 - 停止繼承.png]]
    
- after
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/07-取消繼承 - after.png|07-取消繼承 - after.png]]
    

### 特殊權限指派

前面都是基本的權限，還有特殊權限可以更精細的指派

![[Assets/Note/Tech/Windows 檔案權限管理指南/08-特殊權限指派.png|08-特殊權限指派.png]]

![[Assets/Note/Tech/Windows 檔案權限管理指南/09-特殊權限指派.png|09-特殊權限指派.png]]

- Traverse folder/execute file
    
    Traverse讓使用者即使在沒有權限存取資料夾的情況，能可以切換到該資料傑內。此設定僅有資料夾可以。在GPO未被賦予skip traverse folder才有效。execute file可讓只用者執行程式，此適用於檔案不適用於資料夾
    
- list folder /read data
    
    list folder讓使用者可以檢視資料夾內的檔案和資料夾只有在資料夾有用，read data讓使用者可以檢視資料，只有在非資料夾內有用
    
- read attributes
    
    他讓使用者可以檢視資料夾或檔案屬性 隱藏唯讀
    
- read extended attributes
    
    他讓使用者可以檢視資料或檔案的括稱屬性。擴充屬性使app定義的
    
- create files /write data
    
    create files適用於資料夾，讓使用者可以在資料夾建立檔;write data適用於檔案讓使用者可以修改覆蓋檔案
    
- write attributes
    
    讓使用者可以修改檔案屬性 唯讀隱藏之類的屬性
    
- write extended attributes
    
    讓使用者可以修改檔案的擴充屬性
    
- delete subfolders and files
    
    讓使用者可以刪除資料夾內的子資料夾或是檔案，沒有刪除權限也可以刪除
    
- delete
    
    讓使用者可以刪除此資料夾或檔案
    
- read permissions
    
    使用者可以檢視資料夾或是檔案的權限設定
    
- change permissions
    
    可以讓使用者改變資料夾或檔案設定
    
- take ownership
    
    使使用者可以奪取資料或盪案的擁有權，不論他對目前資料夾或檔案有何種權限，仍會有具備變更資料夾或檔案的能力
    

### 檢查使用者有哪些權限

![[Assets/Note/Tech/Windows 檔案權限管理指南/10-檢查使用者有哪些權限.png|10-檢查使用者有哪些權限.png]]

## 檔案與資料夾擁有者

NTFS與ReFS磁碟內每個檔案都需要有owner，建立資料或資料夾的使用者預設就是資料夾的擁有者，擁有者可以改變權限，不論目前有沒有權限存取此檔案，使用者可以奪取擁有權，變成擁有者需要有些條件才能奪取

- 具有取得檔案或其他物件的擁有權的使用者，預設只有administrators group才擁有此權限
- 對該檔案或資料夾擁有取得擁有權的特殊權限
- 具備還原檔案及目錄權限的使用者

![[Assets/Note/Tech/Windows 檔案權限管理指南/11-檔案與資料夾擁有者 - 具備還原檔案及目錄權限的使用者.png|11-檔案與資料夾擁有者 - 具備還原檔案及目錄權限的使用者.png]]

## file copy OR move 後的權限變化

![[Assets/Note/Tech/Windows 檔案權限管理指南/12-file copy OR move 後的權限變化.png|12-file copy OR move 後的權限變化.png]]

- 將檔案複製或搬移的使用者都會變成owner，FAT、FAT32、exFAT2都沒有權限設定功能，所以從ntfs複製或移動權限就會消失
- 如果要搬移資要的話要先有修改的權限，同時也必須對目的地資料夾有寫入的權限，因此系統在搬移時會先複製到目的地的資料夾，再刪除來源資料夾

## 檔案壓縮

系統支援ntfs壓縮與壓縮的zipped資料夾兩種不同的壓縮方式，ntfs壓縮只又在ntfs硬碟中支援，使用者在讀取壓縮檔案時，系統會讀取檔案、自動解壓縮後內容提供給使用者或應用程式然而儲存在硬碟內的檔案還是處於壓縮狀態，存回時會被自動壓縮

- NTFS壓縮
    - 1
        
        ![[Assets/Note/Tech/Windows 檔案權限管理指南/13-檔案壓縮 - 1.png|13-檔案壓縮 - 1.png]]
        
    - 2
        
        ![[Assets/Note/Tech/Windows 檔案權限管理指南/14-檔案壓縮 - 2.png|14-檔案壓縮 - 2.png]]
        
        - apply changes to this folder only
            
            以後在此新增的檔案、子資料夾，子資料都會被壓縮、但不會引想到現在有的資料
            
        - apply chages to folder，subfolder and files
            
            不但以後在此資料夾內新增的檔案，子資料夾與子資料夾內的檔案都會被壓縮，同時會將已經存在於此的資料夾內的檔案都會被自動壓縮，同時將以存在的檔案都壓縮
            

### 搬移或複製檔案壓縮屬性的變化

![[Assets/Note/Tech/Windows 檔案權限管理指南/15-搬移或複製檔案壓縮屬性的變化.png|15-搬移或複製檔案壓縮屬性的變化.png]]

## 加密檔案系統 (Encrypting file system，EFS)

efs提供加密功能，只有當初加密的使用者才可以讀取，檔案加密後，只有當初將其加密的使用者或被授權的使用者能夠讀取，因此可以增加檔案的安全性。只有NTFS磁碟內的檔案、資料夾才可以被加密，移動到非NTFS，新檔案就會被解密

加密與和壓縮無法共存

- 設定
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/16-加密檔案系統 (Encrypting file system,EFS).png|16-加密檔案系統 (Encrypting file system,EFS).png]]
    
- 確定
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/17-加密檔案系統 (Encrypting file system,EFS).png|17-加密檔案系統 (Encrypting file system,EFS).png]]
    

## share file

### share file permission

> [!important] 共用權限駛隊透過網路來存取的使用者才有作用，本機登入不會有作用

|   |   |   |   |
|---|---|---|---|
|具備的能力\權限種類|Read|Write|full Control|
|檢視檔案名稱與子資料夾名稱 ; 檢視檔案內的資料夾 ; 執行的程式|V|V|V|
|新增刪除改變資料夾內容||V|V|
|變更權限|||V|

- 也是有拒絕優先
- 複製或是搬移保留共用狀態，但是複製的那一份新的資料夾不會被設定共用資料夾。搬移到其他磁碟就不是共用資料夾了
- NTFS權限搭配使用
    
    若資料夾位於NTFS，則使用者到底有沒有權限存取，需要看共享時的權限和NTFS權限
    
    網路使用者最後的權限，是共用與NTFS權限中最嚴格的設定。今過累加後對共用資料夾的權限有效共用權限為**讀取、**同時NTFS有效權限為full control，A就只會有**讀取**
    
    > [!important] 如果使用者使用本機燈入，則是看NTFS權限來決定
    

### share file setting

administrators有共享權力

- 設定
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/18-share file setting - 設定.png|18-share file setting - 設定.png]]
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/19-share file setting - 設定.png|19-share file setting - 設定.png]]
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/20-share file setting - 設定.png|20-share file setting - 設定.png]]
    
- 會自動啟動file and printer sharing
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/21-share file setting.png|21-share file setting.png]]
    

### 隱藏式共用資料夾

在共用名稱後加上$符號，別人就沒有辦法搜尋到

![[Assets/Note/Tech/Windows 檔案權限管理指南/22-隱藏式共用資料夾.png|22-隱藏式共用資料夾.png]]

### 存取網路共用資料夾

![[Assets/Note/Tech/Windows 檔案權限管理指南/23-存取網路共用資料夾.png|23-存取網路共用資料夾.png]]

![[Assets/Note/Tech/Windows 檔案權限管理指南/24-存取網路共用資料夾.png|24-存取網路共用資料夾.png]]

- 打開這項功能就可以在network那邊發現共用資料夾
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/25-存取網路共用資料夾.png|25-存取網路共用資料夾.png]]
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/26-存取網路共用資料夾.png|26-存取網路共用資料夾.png]]
    
    > [!important] 如果沒有可以去看看service.msc裡面的function discovery resource publication有沒有開啟，如果是DC需要開啟GPO
    

### 連線網路電腦的驗證機制

- 當在連接網路上的其他]電腦，必須提供帳號密碼，電腦會自動以目前使用的帳號密碼連接該網路的電腦，也就是會以當初登入電腦的帳號所登入。
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/27-連線網路電腦的驗證機制.png|27-連線網路電腦的驗證機制.png]]
    
- 如果電腦都加入網域就不用手動輸入密碼，系統會自動用此帳號密碼登入，以下範例
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/28-連線網路電腦的驗證機制.png|28-連線網路電腦的驗證機制.png]]
    
- 如果沒有加入網域，系統還是會自動連接
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/29-連線網路電腦的驗證機制.png|29-連線網路電腦的驗證機制.png]]
    
    - 該網路電腦內已經建立一個相同名稱的帳號
        - 密碼也相同，自動利用此帳號來成功連謝
        - 密碼不同，會要求重新連線
    - 沒有建立一個相同名稱的帳號
        - 會該網路電腦會啟用Guest帳號，則系統會利用guest連線
        - 如果停用guest會要求重新輸入帳號密碼
- 存放認證的地方
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/30-連線網路電腦的驗證機制 - 存放認證的地方.png|30-連線網路電腦的驗證機制 - 存放認證的地方.png]]
    

### 透過網路磁碟連線

- 新增
    
    ```PowerShell
    net use Z: \\ipaddr\Database
    ```
    
- 刪除
    
    ```PowerShell
    net use Z: /Delete
    ```
    

### shadow copies of shared folders

會自動指定的時間，將共用的資料夾內的當案複製到另一個處存空間備用，如果誤刪檔案可以透過shadow copies救回資料

- 設定
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/31-shadow copies of shared folders - 設定.png|31-shadow copies of shared folders - 設定.png]]
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/32-shadow copies of shared folders - 設定.png|32-shadow copies of shared folders - 設定.png]]
    
- 用戶端如何存取到
    
    ![[Assets/Note/Tech/Windows 檔案權限管理指南/33-shadow copies of shared folders - 用戶.png|33-shadow copies of shared folders - 用戶.png]]