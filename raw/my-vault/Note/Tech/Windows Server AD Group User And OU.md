## Object and Attribute

ADDS，有很多物件例如user、computer都是物件，於物件的內容就是屬性，物件就是屬性的結合體

## Active Directory Administrator Center

在成為DC的時候就會有這兩個工具自動產生，這兩個公具的功能大同小異，只是Administrative Center是比較新的功能在windows server 2012加入的，在升級成DC的時候只有在建立網域內的第1台DC會將該server的users和computer轉移到ADDS database裡面

![[Assets/Note/Tech/Windows Server AD Group User And OU/01-Active Directory Administrator Cente.png|01-Active Directory Administrator Cente.png]]

- administrative center
    
    這個在操作的時候底下會有powershell指令顯示出來幫助powershell的學習
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/02-Active Directory Administrator Cente.png|02-Active Directory Administrator Cente.png]]
    
    > [!important] win + r dsac
    
- AD users and computers
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/03-Active Directory Administrator Cente.png|03-Active Directory Administrator Cente.png]]
    
    > [!important] win + r dsa.msc
    
- 檢查工具有沒有被安裝
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/04-Active Directory Administrator Cente.png|04-Active Directory Administrator Cente.png]]
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/05-Active Directory Administrator Cente.png|05-Active Directory Administrator Cente.png]]
    
      
    

## Container and Organization Units

container 很像物件，但是可以在container裡面放物件，也可以放入其他的container，OU可以放其他物件之外，還有GPO的功能

- 新建一個OU
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/06-Container and Organization Units - 新.png|06-Container and Organization Units - 新.png]]
    
- ==*==代表一定要打
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/07-Container and Organization Units - =.png|07-Container and Organization Units - =.png]]
    

---

## AD User

- 在OU中新建一個使用者
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/08-AD User - 在OU中新建一個使用者.png|08-AD User - 在OU中新建一個使用者.png]]
    
- 新建使用者
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/09-AD User - 新建使用者.png|09-AD User - 新建使用者.png]]
    
    - UPN(User Principal Name) 登入可以利用信箱的格式登入網域。整個樹系，此名稱是唯一的。test@sayms.local
    - SAN(SamAccountName) sayms為netbios網域名稱相同，同一個網域內，此登入名稱必須是唯一的。sayms\test
- 防止被刪除
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/10-AD User - 防止被刪除.png|10-AD User - 防止被刪除.png]]
    
- 用戶到期
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/11-AD User - 用戶到期.png|11-AD User - 用戶到期.png]]
    
- 用戶登入時段
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/12-AD User - 用戶登入時段.png|12-AD User - 用戶登入時段.png]]
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/13-AD User - 用戶登入時段.png|13-AD User - 用戶登入時段.png]]
    
- 允許登入哪台電腦
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/14-AD User - 允許登入哪台電腦.png|14-AD User - 允許登入哪台電腦.png]]
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/15-AD User - 允許登入哪台電腦.png|15-AD User - 允許登入哪台電腦.png]]
    

---

## AD Group

### Group type

- 可以分為兩種網域群組
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/16-Group type - 可以分為兩種網域群組.png|16-Group type - 可以分為兩種網域群組.png]]
    
    - 安全性群組
        
        他可以用來被使定權限，例如檔案具有讀取的權限。也可以備用在安全無關的工作上，例如可以發送電子郵件給安全性群組
        
    - 發佈群組
        
        他被用在與安全設定無關的工作，但無法配置權限
        
    
    > [!important] 可以互相轉換身分
    

### Group Scope

![[Assets/Note/Tech/Windows Server AD Group User And OU/17-Group Scope.png|17-Group Scope.png]]

|特性\群組|Domain local|Global|Universal|
|---|---|---|---|
|可包含的成員|所有網域內的使用者、全域群組、萬用群組 : 相同網域內的本機群組|相同網域內的使用者與全域群組|所有網域內的使用者、全域群組、萬用群組|
|可以在哪一個網域內被設定使用權限|同一個網域|所有網域|所有網域|

- Domain local
    
    只要被用來指派網域內的權限，以便可以存取該網域內的資源
    
    - 其成員可以包含在一個網域內的使用者、global group、universal group;也可以包含網域內的domain local group但是無法包含其他網域內的網域本機群組
    - Domain local group only access self domain resource，無法存取其他網域內的資源，可以設定相同網域內的domain local group，但是無法設定其他網域的
- Global
    
    他主要用來組織使用者，也就是可以將多個即將被賦予相同權限]的只用者帳戶，加入到global群組內
    
    - 可以包含相同網域內使用者與全域群組
    - Global可以存取任何一個網域內的資源，也就是可以在任何一個網域內設定global group的權限。
- universal
    
    用在所有網域的權限設定，以便存取所有網域內的資源
    
    - universal group 具有”universal scope”特性，其他成員可以包還在樹戲中任何一個網域內的使用者、global group、unversal group。但是無法包含任何一個domain local group
    - universal group 可以存取任何一個網域內的資源，也就是說可以在任何一個網域內設定universal group的權限。

### 內建AD Domain Group

- Domain local
    
    這些群組本身被賦予一些權限，ADDS具備有管理的能力，只要將使者或群組加入到這些群組，也會有相同的權限，這些群組在Builtin
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/18-內建AD Domain Group - Domain local.png|18-內建AD Domain Group - Domain local.png]]
    
    ![[Assets/Note/Tech/Windows Server AD Group User And OU/19-內建AD Domain Group - Domain local.png|19-內建AD Domain Group - Domain local.png]]
    
    - Acount Operators
        
        可以讓使用者新增/刪除/修改使用者、群組和電腦帳戶，內建的OU除外
        
    - Administrators
        
        成員有管理員權限，對所有DC有最大的控制權，Administrator無法移除此群組，此群組包含Administrator、domain Admins、enterprise admins
        
    - backup Operators
        
        可以讓使用者使用windows server backup備份還原，也可以讓使用者將DC關機
        
    - Guests
        
        不能改變桌面登出後刪除，預設帳戶Guest和domain Guests的群組
        
    - Performance Monitor Users
        
        讓成員可以監控DC效能情況
        
    - Print Operators
        
        可以控制DC內的Printer，可以讓使用者將DC關機
        
    - Remote Desktop Users
        
        使其成員可以透過Remote Desktop來登入
        
    - Users
        
        基本的權限，但是不能修改系統設定，不能將DC關機，Domain Users預設全組
        
- Global
    - Domain Admins
        
        網域成員電腦會自動加入到Administrators內，Domain Admin群組內的每個成員，在網域內每台電腦都具備系統管理員的權限。預設使用者是Administrator
        
    - Domain Computers
        
        除了DC以外的電腦都會被加入到此群組內
        
    - Domain Controllers
        
        所有的DC會被加入到此群組內
        
    - Domain Users
        
        網域成員電腦會被自動將此群組加入到本機Users，因此Domain Users內的使用者享有Users的權限，登入本機的許可權。使用者預設的成員網域使用者administrator，而以後新增的網域使用者都隸屬於這group
        
    - Domain Guest
        
        會被加入到domain local guest群組內預設是guest帳號
        
- Universal
    
    - enterprise admins
        
        只存在樹系根網域，其成員有管理樹系內所有望域。預設使用者是該樹系根網域內的使用者administrator
        
    - schema Admins
        
        此群組只存在於樹系根網域，其成員具有管理schema的權限。預設administrator