---
Type:
  - windows Server GPO
---
## 概述

啟用auditing功能可以讓系統管理員追蹤使用者存取電腦內的資源、追蹤電腦運作情況。啟用需要兩個步驟

- 啟用稽核原則 : Administrators Group裡面的成員才有權利啟用稽核原則
- 設定auditing的資源 : 具備**管理稽核安全性紀錄**權限的使用者才可以稽核資源，預設是Administrators才可以。可以將使用者權限指派來**管理稽核及安全性紀錄**權限給其他使用者

## 查看稽核

![[Assets/Note/Tech/Windows auditing/01-查看稽核.png|01-查看稽核.png]]

## auditing 設定

![[Assets/Note/Tech/Windows auditing/02-auditing 設定.png|02-auditing 設定.png]]

每個稽核事件都可以分成成功和失敗兩種，也就是說事件成功的發生，例如可以稽核使用者登入的動作或是使用者登入之敗的動作

- Audit account logon event
    
    audit是否發生了利用本機使用者來登入事件。例如此電腦打開這個policy，若此點腦上利用本機使用者帳號登入，則安全性紀錄檔內也會有紀錄
    
- Audit account management
    
    audit是否有新增、修改、刪除、啟用、停用、更改帳號名稱、與帳號有關的事件發生
    
- audit directory service access
    
    audit 使用者存取ADDS內的物件。您必須另外選擇audit的物件與使用者。此設定只對DC有用
    
- audit logon events
    
    audit 使用者是否發生使用者登入與登出的行為，不論是本機登入或是透過網域登入都會記錄
    
- audit object access
    
    audit是否有使用者存取檔案、資料夾或印表機。必須另外再選擇稽核的檔案、資料夾或印表機
    
- audit policy change
    
    稽核使用者權限只配原則、稽核原則或信任原則是否有異動
    
- audit privlege use
    
    稽核使用者是否使用了使用者權限只派原則內所賦予的權限，例如變更系統資料(系統不會稽核部分會產生大量紀錄的事件，因此會引想到系統下能，例如備份檔案或目錄、還原檔案及目錄等事件如果要稽核需要再regedit，啟用fullprivilegeauditing value，Computer\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Lsa
    
- audit process tracking
    
    稽核程式的結束執行
    
- audit system events
    
    重開機關機任何影響系統安全的事件都會記錄
    

## case

- 假設要記錄登入失敗和成功的使用者
    
    ![[Assets/Note/Tech/Windows auditing/03-case - 假設要記錄登入失敗和成功的使用者.png|03-case - 假設要記錄登入失敗和成功的使用者.png]]
    
    - 登入成功
        
        ![[Assets/Note/Tech/Windows auditing/04-case - 登入成功.png|04-case - 登入成功.png]]
        

  

- 稽核檔案的存取行為
    - GPO設定
        
        ![[Assets/Note/Tech/Windows auditing/05-case - GPO設定.png|05-case - GPO設定.png]]
        
    - 假設是對test.txt做稽核
        
        ![[Assets/Note/Tech/Windows auditing/06-case - 假設是對test.txt做稽核.png|06-case - 假設是對test.txt做稽核.png]]
        
    - 指定使用者
        
        ![[Assets/Note/Tech/Windows auditing/07-case - 指定使用者.png|07-case - 指定使用者.png]]
        
        ![[Assets/Note/Tech/Windows auditing/08-case - 指定使用者.png|08-case - 指定使用者.png]]
        
    - check
        
        ![[Assets/Note/Tech/Windows auditing/09-case - check.png|09-case - check.png]]
        
- 稽核ADDS存取行為
    
    ![[Assets/Note/Tech/Windows auditing/10-case - 稽核ADDS存取行為.png|10-case - 稽核ADDS存取行為.png]]
    
    ![[Assets/Note/Tech/Windows auditing/11-case - 稽核ADDS存取行為.png|11-case - 稽核ADDS存取行為.png]]
    
    ![[Assets/Note/Tech/Windows auditing/12-case - 稽核ADDS存取行為.png|12-case - 稽核ADDS存取行為.png]]
    
    ![[Assets/Note/Tech/Windows auditing/13-case - 稽核ADDS存取行為.png|13-case - 稽核ADDS存取行為.png]]
    
    ![[Assets/Note/Tech/Windows auditing/14-case - 稽核ADDS存取行為.png|14-case - 稽核ADDS存取行為.png]]
    
    ![[Assets/Note/Tech/Windows auditing/15-case.png|15-case.png]]
    
    ![[Assets/Note/Tech/Windows auditing/16-case.png|16-case.png]]