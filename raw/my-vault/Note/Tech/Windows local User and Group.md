每台電腦中都會有一個SAM在使用者登入前，都需要正確的帳號密碼，而這個使用者就是建立在SAM中

## 內建預設的本機帳戶

- administrator : 擁有最高的權限，可以管理整台電腦的工作，此帳號無法刪除，為了安全起見建議改名
- Guest : 臨時使用的帳號，無法刪除，預設disable

## 本機內建群組

- Administrators : 群組內最高管理員的權限，執行電腦任何工作，Administrator無法移除這個群組
- backup Operators : 此群組使用者可以透過windows server backup工具還原後備份電腦
- Guest : 這個群組的使用者無法改變桌面的工作環境，系統會建立一個臨時的工作環境，登出後會刪除
- Network Configuration Operators : 此群組的使用者可以改變 IP ，但是不能安裝驅動，也不可以執行network server
- Remote Desktop Users : 可以使用遠端桌面
- Users : 可以執行應用程式等等，但是不能共用資料夾，不能關機。預設新增使用者會加入到這個群組

## 特殊群組

- Everyone : 所有只用者都屬於這個群組。如果guest啟用就會自動加入
- authenticatoin : 有效使用帳戶登入此電腦的使用者
- Interactive : 凡是在本機登入的使用者，都屬於這個群組
- Network : 凡是透過網路登入都是這個群組

## SID (security identifier)

系統內部是使用，這個ID去查詢的，所以就可以有很多相同姓名的使用者，但是SID不同，每創建一個新的使用者，都會獲得一個SID