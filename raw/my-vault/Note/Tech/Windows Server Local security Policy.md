---
Type:
  - windows Server GPO
---
## Password Policy

![[Assets/Note/Tech/Windows Server Local security Policy/01-Password Policy.png|01-Password Policy.png]]

- enforce password history
    
    用來設定是否要保存使用者曾經使用過的舊密碼
    
    - 1-24 : 表示要保存舊的密碼紀錄，如果設定5，使用者的新密碼不可以跟前5次的舊密碼相同
    - 0 (default) : 表示不保存密碼歷史紀錄，因此密碼可以重複使用
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/02-Password Policy.png|02-Password Policy.png]]
    
- Maximumm password age
    
    default 42 day 再登入時，密碼到期，系統會要求使用者更改密碼，0表示沒有限制
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/03-Password Policy.png|03-Password Policy.png]]
    
- Minimum password age
    
    期限未到前，使用者不能變更密碼，default 0 表示隨時能變更密碼
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/04-Password Policy.png|04-Password Policy.png]]
    
- Minimum password length
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/05-Password Policy.png|05-Password Policy.png]]
    
    用來設定使用者的密碼最少要幾個字，default 0表示可以不用密碼
    
- Minimum password length audit
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/06-Password Policy.png|06-Password Policy.png]]
    
    當使用者變更密碼時，小於此設定，系統會有log event，管理員可以查看此紀錄，搜尋時的來源Directory-Services-SAM、事件識別碼16978
    
- Password must meet complexity requirements
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/07-Password Policy.png|07-Password Policy.png]]
    
    使用者必須滿足以下條件才可以設定密碼(預設值)
    
    - 不可以有帳戶名稱或是全名
    - 長度必須6個字元
    - 至少包含A-Z、a-z、0-9、非字母數字($、\#、%)等4組字元中的3組
- Relax minimum password length limit
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/08-Password Policy.png|08-Password Policy.png]]
    
    若未定義或停用此原則，則mimimum password length處的設定最大為14字元;如果啟用這個功能，就可以超過14位可到128，
    
- store passwords uning reversible encryption
    
    ![[Assets/Note/Tech/Windows Server Local security Policy/09-Password Policy.png|09-Password Policy.png]]
    
    若app需要讀取user password，以便驗證使用者身分，就可以啟用此公能。不過因為沒有加密不安全，謹慎使用
    

## 帳號鎖定

![[Assets/Note/Tech/Windows Server Local security Policy/10-帳號鎖定.png|10-帳號鎖定.png]]

- Account lockout duration
    
    鎖定帳戶的期限，期限過了就是解除，0min代表永久鎖定
    
- Account lockout threshold
    
    可以讓使用者登入失敗多次，就將該使用者鎖定。在未解鎖前都不能嘗試登入
    
- Reset account lockout counter after
    
    鎖定計時器，是用來記錄登入失敗的次數。若使用者前一次登入失敗，已經經過了此處的設定時間的話就會清空
    

## 使用者權限指派

![[Assets/Note/Tech/Windows Server Local security Policy/11-使用者權限指派.png|11-使用者權限指派.png]]

## 安全性選項

![[Assets/Note/Tech/Windows Server Local security Policy/12-安全性選項.png|12-安全性選項.png]]

- interactive logon : Do not require CTRL + ALT + DEL