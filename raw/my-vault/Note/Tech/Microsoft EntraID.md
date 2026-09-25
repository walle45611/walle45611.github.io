![[Assets/Note/Tech/Microsoft EntraID/Microsoft Entra ID plans comparison.png]]
https://learn.microsoft.com/zh-tw/entra/fundamentals/licensing

| 功能                            | 先想到什麼   | 對應層級 |
| ----------------------------- | ------- | ---- |
| 基本 SSO / MFA                  | 基礎功能    | Free |
| Conditional Access            | 條件式存取   | P1   |
| Risk-based Conditional Access | 風險式保護   | P2   |
| PIM                           | 高權限臨時升權 | P2   |

### **1. Revoke session (撤銷工作階段)**

- **主要用途**：當帳戶疑似**遭入侵 (Compromised)** 時使用的安全性操作。
- **執行效果**：此操作會立即**中斷所有當前已通過驗證的活動連線**。
- **帳戶狀態**：帳戶本身**依然存在**。通常在執行撤銷後，管理員會接著**停用 (Disable/Block)** 該帳戶，以防止攻擊者在連線被切斷後重新驗證登入。

### **2. Delete (刪除帳戶)**

- **主要用途**：當使用者不再是組織的一部分（如離職）或不再需要該帳戶時，將其從環境中移除。
- **執行效果**：帳戶會從使用者清單中移除，並進入**「已刪除使用者」**的暫存狀態。
- **保留與還原期**：
    - 被刪除的帳戶預設會保留 **30 天**。
    - 在這 30 天內，如果發現是誤刪或使用者回歸，管理員可以將其**還原 (Restore)**。
    - 一旦**超過 30 天**，該帳戶將會被**永久刪除**且無法再復原。

### **重點對比總結**

|特性|Revoke session (撤銷工作階段)|Delete (刪除帳戶)|
|:--|:--|:--|
|**核心目的**|強制將使用者踢下線，防止即時的惡意操作。|移除身分識別，管理使用者生命週期。|
|**帳戶存續**|帳戶保留在系統中，僅存取權暫時中斷。|帳戶被移往回收站，等待永久移除。|
|**後續操作**|通常需搭配「停用帳戶」或「重設密碼」。|30 天內可還原，之後則徹底消失。|

![[Assets/Note/Tech/Microsoft EntraID/Microsoft Entra ID tenant root group hierarchy.png]]
