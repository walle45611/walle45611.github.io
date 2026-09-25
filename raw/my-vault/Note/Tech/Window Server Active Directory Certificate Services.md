---
Type:
  - windows Server ADCS
---
- 可以先看

- [[#種類概述]]
- [[#安裝]]
- [[#申請證書]]

---

## 種類概述

- 企業CA
    
    企業CA又分為企業CA與企業次CA，他需要Active Directory Domain Services，可以將企業CA安裝到DC或成員server。發放憑證的對象僅限網域使用者，當網域使用者來申請certificate，企業CA會從Active Directory來得知該使用者帳戶資訊，並據以決定是否有權利來申請憑證。
    
    企業次級CA須向其父系CA(獨立根CA或是企業CA)取得憑證後才正常運作。企業次級CA也可以發放給下層使用
    
- 獨立CA
    
    獨立根與獨立次級CA，他不需要Active Directory網域。他可以是獨立server、成員server或DC。無論是否使用網域使用者，都可以向獨立CA申請。
    
    獨立次級CA象棋父系CA(企業根CA或獨立根CA)取得憑證才會正常運作。也可以發放給下層使用
    

## 安裝

1. install 
    
    ```PowerShell
    Install-WindowsFeature adcs-cert-authority -IncludeManagementTools
    Install-WindowsFeature adcs-web-enrollment -IncludeManagementTools
    ```
    

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/01-安裝 - install.png|01-安裝 - install.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/02-安裝.png|02-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/03-安裝.png|03-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/04-安裝.png|04-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/05-安裝.png|05-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/06-安裝.png|06-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/07-安裝.png|07-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/08-安裝.png|08-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/09-安裝.png|09-安裝.png]]

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/10-安裝.png|10-安裝.png]]

---

## 申請證書

```PowerShell
win+r mmc
```

```PowerShell
certlm.msc
```

![[Assets/Note/Tech/Window Server Active Directory Certificate Services/11-申請證書.png|11-申請證書.png]]