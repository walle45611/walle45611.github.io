---
Created: 2023-02-27T20:50
---
>[!info] win + r virtmgmt.msc

## 安裝

```PowerShell
Install-WindowsFeature Hyper-V -IncludeManagementTools -Restart
```

- 安裝後
    
    ![[Assets/Note/Tech/Windows Hyper-V/01-安裝.png|01-安裝.png]]
    

## Create Virtual Switch and Virtual Machine

### virtual switch

- 建立
    
    ![[Assets/Note/Tech/Windows Hyper-V/02-virtual switch - 建立.png|02-virtual switch - 建立.png]]
    
- 調整名稱和外部的網卡
    
    ![[Assets/Note/Tech/Windows Hyper-V/03-virtual switch - 調整名稱和外部的網卡.png|03-virtual switch - 調整名稱和外部的網卡.png]]
    
- 建完後會多了一個網卡
    
    ![[Assets/Note/Tech/Windows Hyper-V/04-virtual switch - 建完後會多了一個網卡.png|04-virtual switch - 建完後會多了一個網卡.png]]
    
- 原本的網卡上
    
    ![[Assets/Note/Tech/Windows Hyper-V/05-virtual switch - 原本的網卡上.png|05-virtual switch - 原本的網卡上.png]]
    

### 新建虛擬機

- 按new然後按virtualMachine
    
    ![[Assets/Note/Tech/Windows Hyper-V/06-新建虛擬機.png|06-新建虛擬機.png]]
    
- 第一頁next
- 名子
    
    ![[Assets/Note/Tech/Windows Hyper-V/07-新建虛擬機 - 名子.png|07-新建虛擬機 - 名子.png]]
    
- 選擇版本
    
    ![[Assets/Note/Tech/Windows Hyper-V/08-新建虛擬機 - 選擇版本.png|08-新建虛擬機 - 選擇版本.png]]
    
    - 可以兼容第一代版本的，也可以選擇比較多功能的第二代，我是選第二代
- 設定ram
    
    ![[Assets/Note/Tech/Windows Hyper-V/09-新建虛擬機 - 設定ram.png|09-新建虛擬機 - 設定ram.png]]
    
    - 可以打勾使用動態記憶體，因為這樣比較省
- 選擇連接的vSwitch
    
    ![[Assets/Note/Tech/Windows Hyper-V/10-新建虛擬機 - 選擇連接的vSwitch.png|10-新建虛擬機 - 選擇連接的vSwitch.png]]
    
- 硬碟設定，可以使用動態硬碟常用的
    
    ![[Assets/Note/Tech/Windows Hyper-V/11-新建虛擬機 - 硬碟設定,可以使用動態硬碟常用的.png|11-新建虛擬機 - 硬碟設定,可以使用動態硬碟常用的.png]]
    
- 放入ISO
    
    ![[Assets/Note/Tech/Windows Hyper-V/12-新建虛擬機 - 放入ISO.png|12-新建虛擬機 - 放入ISO.png]]
    
- 後面的步驟就是安裝windows server
- 快捷鍵
    
    > [!important] 快捷鍵 ctrl + alt + end
    

---

### 差異盤建立虛擬機

![[Assets/Note/Tech/Windows Hyper-V/13-差異盤建立虛擬機.png|13-差異盤建立虛擬機.png]]

- 新增虛擬差硬碟
    
    ![[Assets/Note/Tech/Windows Hyper-V/14-差異盤建立虛擬機 - 新增虛擬差硬碟.png|14-差異盤建立虛擬機 - 新增虛擬差硬碟.png]]
    
- 第一步跳過
- 第二步使用VHDX
- 選擇differencing
    
    ![[Assets/Note/Tech/Windows Hyper-V/15-差異盤建立虛擬機 - 選擇differencing.png|15-差異盤建立虛擬機 - 選擇differencing.png]]
    
- 命名
    
    ![[Assets/Note/Tech/Windows Hyper-V/16-差異盤建立虛擬機 - 命名.png|16-差異盤建立虛擬機 - 命名.png]]
    
- 以誰為母碟
    
    ![[Assets/Note/Tech/Windows Hyper-V/17-差異盤建立虛擬機 - 以誰為母碟.png|17-差異盤建立虛擬機 - 以誰為母碟.png]]
    
- 按完成
- 接著建立一個虛擬機
- 與前面一樣步驟，在選硬碟的時候要選已經建立好的
    
    ![[Assets/Note/Tech/Windows Hyper-V/18-差異盤建立虛擬機.png|18-差異盤建立虛擬機.png]]
    
- 就會發現是已經安裝好的windows server
    
    ![[Assets/Note/Tech/Windows Hyper-V/19-差異盤建立虛擬機.png|19-差異盤建立虛擬機.png]]
    
- 如果要建立多台只需要把差異硬碟複製給其他機器

---

## 網卡模式

![[Assets/Note/Tech/Windows Hyper-V/20-網卡模式.png|20-網卡模式.png]]

- 其實就有點像是vmware 裡面 host only，bridge，lan segment