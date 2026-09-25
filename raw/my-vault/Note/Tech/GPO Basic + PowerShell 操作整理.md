## 概述（Group Policy Object - GPO）

- 可以使用 GPO 讓在網域內的電腦受到一些限制或自動安裝軟體等功能
    
- 下層 GPO 會繼承來自上層的 GPO 設定
    
- 若 GPO 設定有衝突，以下層（自己）為優先
    
- 隸屬於網域的電腦都會受到網域安全性原則影響
    
- 若本機與網域安全性原則設定衝突，優先使用 **網域設定**，本機設定僅在網域設定未定義時才會生效
    
- 網域安全設定有異動時，須套用後才會生效。
    

### GPO 套用時機

- 本機原則異動時
    
- 本機電腦重新啟動時
    
- 網域控制站 (DC)：每 5 分鐘套用一次
    
- 非 DC：預設每 90~120 分鐘套用（每 16 小時強制套用一次）
    
- 手動強制套用：
    

```powershell
gpupdate /force
```

---

## GPO 設定方式

### 1. 開啟 GPO 編輯工具

- 本機 GPO 編輯器：
    
    > [!important] Win + R → `gpedit.msc`
    
- AD 網域 GPO 管理：
    
    > [!important] Win + R → `gpmc.msc`
    

### 2. 找到需要設定的 GPO

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/01-2. 找到需要設定的 GPO.png]]

- `Default Domain Policy`：已套用至整個網域（如：sayms.local），影響所有使用者與電腦
    
- `Default Domain Controller Policy`：預設套用於 DC，影響 DC 上所有使用者與電腦
    

### 3. 停止 GPO 繼承 / 強制 GPO 套用

- OU 停止繼承：
    

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/02-3. 停止 GPO 繼承 強制 GPO 套用 - OU 停止繼承.png]]

- 強制某 GPO 套用（Enforced）：
    

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/03-3. 停止 GPO 繼承 強制 GPO 套用.png]]

### 4. GPO 例外排除（讓 OU 某台電腦不受影響）

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/04-4. GPO 例外排除(讓 OU 某台電腦不受影響).png]]  
![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/05-4. GPO 例外排除(讓 OU 某台電腦不受影響).png]]  
![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/06-4. GPO 例外排除(讓 OU 某台電腦不受影響).png]]

---

## GPO 實用設定範例

### 允許使用者登入 DC

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/07-允許使用者登入 DC.png]]  
![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/08-允許使用者登入 DC.png]]  
![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/09-允許使用者登入 DC.png]]

> - 一定要包含 `Administrators` 群組
>     
> - 指定使用者也必須加入 `Administrators` 群組
>     

### 拒絕登入本機

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/10-拒絕登入本機.png]]

### 禁止使用控制台和電腦設定

![[Assets/Note/Tech/GPO Basic + PowerShell 操作整理/11-禁止使用控制台和電腦設定.png]]

---

## 喜好設定 vs 原則設定

> [!important]
> 
> - **喜好設定**：非強制性，使用者可以變更設定值
>     
> - **原則設定**：強制性，一經套用無法變更
>     
> - 僅**網域使用者**可使用喜好設定
>     

---

## PowerShell 操作 GPO

### 查詢目前 GPO

```powershell
Get-GPO -All -Server DC
```

### 建立新的 GPO 並連結到指定 OU

```powershell
New-GPO -Name "test"
New-GPLink -Name "test" -Target "OU=PROD,DC=Skills39,DC=com"
```

### 使用 Starter GPO 建立新 GPO

```powershell
New-StarterGPO -Name "ITB-starterGPO"
New-GPO -Name "First GPO" -StarterGPOName "ITB-starterGPO"
```

---

如需進一步操作 GPO 授權、登入權限控制、登入時間限制、自動排程等，建議搭配 AD 使用者與電腦 (ADUC) 與 GPMC 詳細管理。