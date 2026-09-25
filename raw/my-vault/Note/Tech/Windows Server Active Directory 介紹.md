## 概述

何謂directory，紀錄一些資料以目錄的形式，通訊錄、或是檔案目錄。

如果資料很多就會有很多個目錄，如果把這些目錄整理好，讓使用者好找到這些資訊，提供directory service就是讓使用者可以更快的尋找directory裡面的資料。

AD網域內的directory database常常被儲存使用者帳號、電腦帳號、應表機與共用資料夾，而提供目錄服務的元件就是ADDS (AD Domain Services)，負責管理資料庫。

## Namesapce

namespace是一塊被定義好的bounded area，在此區域內，可以利用名稱查詢找到此名稱的有關的資訊]，例如電話簿，我們利用姓名找到電話、地址生日資料等等。例如NTFS也是一個namespace，可以找檔案也可以修改日期文件內容。

ADDS也是可以提供很多物件的訊息，namespace也是採用dns架構，因此DNS格是用來命名，例如ADDS domain name test.com。

## DC (Domain Controller)

ADDS的目錄資料室儲存在DC裡面一個網域可以有很多台DC，每台地位都是相同的(沒有設定的情況)，他各自有一份相同的ADDS資料庫，當在DC中建立帳戶會在ADDS中的資料庫新增一個帳號，之後會被複寫到

- 建立多個DC的好處
    - 改善登入效率
    - 容錯

## 安裝ADDS

```PowerShell
Install-WindowFeature ad-domain-services -IncludemanagementTools
```

- 可以在windows admin center看到
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/01-安裝ADDS.png|01-安裝ADDS.png]]
    
- 安裝新增樹系
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/02-安裝ADDS - 安裝新增樹系.png|02-安裝ADDS - 安裝新增樹系.png]]
    
- 打上樹系的密碼
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/03-安裝ADDS - 打上樹系的密碼.png|03-安裝ADDS - 打上樹系的密碼.png]]
    
    - DNS可以不用勾選的原因是因為可以把DNS單獨設計在別邊，但是通常比賽會把ADDS和DNS放在一起
- 下一步
- 下一步
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/04-安裝ADDS - 下一步.png|04-安裝ADDS - 下一步.png]]
    
- 下一步
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/05-安裝ADDS - 下一步.png|05-安裝ADDS - 下一步.png]]
    
    - AD就是利用這些資料夾進行同步的
    - database 就釋放adds database
    - log file 紀錄adds 便度的消息
    - SYSVOL gpo之類的訊息
- 安裝完後reboot
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/06-安裝ADDS - 安裝完後reboot.png|06-安裝ADDS - 安裝完後reboot.png]]
    
- 檢查
    - 可以檢查有沒有這台的A record
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/07-安裝ADDS.png|07-安裝ADDS.png]]
        
    - 和Srv
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/08-安裝ADDS - 和Srv.png|08-安裝ADDS - 和Srv.png]]
        

### 加入網域

- Windows Server
    - 設定DNS
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/09-加入網域 - 設定DNS.png|09-加入網域 - 設定DNS.png]]
        
    - 打上網域名稱，輸入網域管理員張帳號密碼
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/10-加入網域.png|10-加入網域.png]]
        
    - 輸入網域密碼
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/11-加入網域 - 輸入網域密碼.png|11-加入網域 - 輸入網域密碼.png]]
        
    - 選擇複寫對象可以使用media，也可以從其他DC上同步
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/12-加入網域.png|12-加入網域.png]]
        
    - 安裝重啟
        
        ![[Assets/Note/Tech/Windows Server Active Directory 介紹/13-加入網域 - 安裝重啟.png|13-加入網域 - 安裝重啟.png]]
        
- Windows 或是單純加入網域不參予DC
    
    ```PowerShell
    add-computer -domainname sayms.local
    ```
    
- 使用網域使用者登入
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/14-加入網域 - 使用網域使用者登入.png|14-加入網域 - 使用網域使用者登入.png]]
    

### 脫離網域

需要enterprise Admins、Domain admins群組的成員或是Administrator才可以脫離網域

```PowerShell
add-computer -workgroupname WORKGROUP
```

### 移除ADDS

需要Enterprise admins成員才可以刪除，如果這台是GC的話，請先轉移到其他台GC上，但是應該不會有這個問題，因為預設全部DC都是GC

- 移除
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/15-移除ADDS.png|15-移除ADDS.png]]
    
- 降級
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/16-移除ADDS - 降級.png|16-移除ADDS - 降級.png]]
    
- 如果無發刪除可以勾選強制刪除
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/17-移除ADDS - 如果無發刪除可以勾選強制刪除.png|17-移除ADDS - 如果無發刪除可以勾選強制刪除.png]]
    
- Process with removal
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/18-移除ADDS.png|18-移除ADDS.png]]
    
- 輸入網域帳密碼
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/19-移除ADDS - 輸入網域帳密碼.png|19-移除ADDS - 輸入網域帳密碼.png]]
    
- 等待降級
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/20-移除ADDS - 等待降級.png|20-移除ADDS - 等待降級.png]]
    
- 好了重開機
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/21-移除ADDS - 好了重開機.png|21-移除ADDS - 好了重開機.png]]
    

> [!important] 雖然不是DC了，但是AD網域服務的元件還在，如果要重新架設成DC需要參考以下步驟

- 把ADDS勾勾取消
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/22-移除ADDS - 把ADDS勾勾取消.png|22-移除ADDS - 把ADDS勾勾取消.png]]
    
- 把DNS也可以勾勾取消
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/23-移除ADDS - 把DNS也可以勾勾取消.png|23-移除ADDS - 把DNS也可以勾勾取消.png]]
    
- 確定移除
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/24-移除ADDS - 確定移除.png|24-移除ADDS - 確定移除.png]]
    
- 移除完後重開機
    
    ![[Assets/Note/Tech/Windows Server Active Directory 介紹/25-移除ADDS - 移除完後重開機.png|25-移除ADDS - 移除完後重開機.png]]
    

  

---

## 提高樹系功能等級

- Windows Server 2019 、 2022都沒有提高樹系的功能所以預設都在2016

![[Assets/Note/Tech/Windows Server Active Directory 介紹/26-提高樹系功能等級.png|26-提高樹系功能等級.png]]

## AD資源回收桶

- ad 回收桶啟用後，就無法停用，也沒辦法降級樹系ㄈ

---

  

  

![[Assets/Note/Tech/Windows Server Active Directory 介紹/27-AD資源回收桶.png|27-AD資源回收桶.png]]