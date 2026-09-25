---
Type:
  - windows Server GPO
---
### APPLocker

- 只能支援企業版用戶端
    
> [!info] 使用 AppLocker (Windows) - Windows Security  
> 適用於：Windows 10 / Windows 11 / Windows Server 2016 及以上版本  
> 僅支援 **企業版 (Enterprise)** 作業系統
> 
> 使用條件如下：
> 
> - 一部執行支援作業系統的電腦建立規則（可為網域控制站）
>     
> - 至少一部安裝 GPMC 或 RSAT 的設備來載入規則
>     
> - 執行支援作業系統的電腦來**強制執行**規則
>     
> 
> 可搭配「軟體限制原則（SRP）」使用，但存在限制。
> 
> 🔗 [官方文件連結](https://docs.microsoft.com/zh-tw/windows/security/threat-protection/windows-defender-application-control/applocker/requirements-to-use-applocker)


![[Assets/Note/Tech/Windows AppLocker/01-APPLocker.png|01-APPLocker.png]]

applocker 可以用來封鎖一些應用程式或是一些指令檔，可以分為五大類

- Eexctable Rule: .exe .com程式
- Windows Installer Rules : .msi .msp .mst
- Script : .ps1 .bat .cmd .vbs .js
- Packaged app Rules: .appx
- DLL規則 : .dll .ocx

  

- 創建GPO並連結到此
    
    ![[Assets/Note/Tech/Windows AppLocker/02-APPLocker - 創建GPO並連結到此.png|02-APPLocker - 創建GPO並連結到此.png]]
    
- 右鍵testGPO然後按edit
    
    ![[Assets/Note/Tech/Windows AppLocker/03-APPLocker - 右鍵testGPO然後按edit.png|03-APPLocker - 右鍵testGPO然後按edit.png]]
    
- 建立預設，建立規則後，在規則內的執行檔都會被封鎖，這些預設會允許一般使用者執行ProgramFile與Windows資料夾內的所有程式、允許管理員執行所有程式
    
    ![[Assets/Note/Tech/Windows AppLocker/04-APPLocker.png|04-APPLocker.png]]
    
    ![[Assets/Note/Tech/Windows AppLocker/05-APPLocker.png|05-APPLocker.png]]
    
- 建立新規則
    
    ![[Assets/Note/Tech/Windows AppLocker/06-APPLocker - 建立新規則.png|06-APPLocker - 建立新規則.png]]
    
    ![[Assets/Note/Tech/Windows AppLocker/07-APPLocker - 建立新規則.png|07-APPLocker - 建立新規則.png]]
    
    ![[Assets/Note/Tech/Windows AppLocker/08-APPLocker - 建立新規則.png|08-APPLocker - 建立新規則.png]]
    
    ![[Assets/Note/Tech/Windows AppLocker/09-APPLocker - 建立新規則.png|09-APPLocker - 建立新規則.png]]
    
    ![[Assets/Note/Tech/Windows AppLocker/10-APPLocker - 建立新規則.png|10-APPLocker - 建立新規則.png]]
    
- 一旦建立規則後，凡是未表列在規則內的執行檔都會被封鎖，內建的appx也會被封鎖，如果要解除封鎖，則需要再以Packaged app Rules處來開放以Packaged app Rules，我們只需要透過建立預設規則來開放，預設會開放所有以簽署的封裝應用程式

> [!important] 可以不須再Windows installer規則與Scritp Rules建立預設規則，因為沒有受到影響

- 需要啟動application Identiy service 才可以使用Applocker功能，使用GPO自動套用到使用者電腦
    
    ![[Assets/Note/Tech/Windows AppLocker/11-APPLocker.png|11-APPLocker.png]]
    
- win 10 enterprise
    
    ![[Assets/Note/Tech/Windows AppLocker/12-APPLocker.png|12-APPLocker.png]]
    
- win 10 pro
    
    ![[Assets/Note/Tech/Windows AppLocker/13-APPLocker - win 10 pro.png|13-APPLocker - win 10 pro.png]]