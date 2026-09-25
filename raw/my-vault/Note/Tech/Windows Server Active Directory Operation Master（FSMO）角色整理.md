---
Type:
  - windows Server AD
---
## 一、AD 架構中 FSMO 分布

### 🔍 查詢指令

```powershell
Get-ADForest         # 查詢 Forest 層級 FSMO
Get-ADDomain         # 查詢 Domain 層級 FSMO
```

![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/01-🔍 查詢指令.png]]

---

## 二、Forest 層級 FSMO（樹系）

### 1. Schema Master

- 處理 **AD 架構結構定義的所有變更**（如物件屬性、類別等）
    
- 必須在 Schema Master 上安裝 `schmmgmt.dll` 才能啟用 GUI 操作：
    

```powershell
regsvr32 schmmgmt.dll
```

### 2. Domain Naming Master

- 處理 **新增或移除網域 / 應用目錄分割** 的操作
    
- 當建立樹系中新的網域名稱時，此角色會參與控制是否允許
    

---

## 三、Domain 層級 FSMO（網域）

### 3. RID Master（Relative ID）

- 每個 AD 物件都會有 SID，RID 是唯一識別碼的一部分
    
- RID Master 管理並分配 RID Pool 給其他 DC 使用
    

![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/02-3. RID Master(Relative ID).png]]

### 4. Infrastructure Master

- 處理目前網域與其他網域物件參考的更新（例如重新命名群組）
    
- **不應與 GC (Global Catalog) 設於同一台主機**，否則異動偵測會失效（因比對資料來源相同）
    
- 除非環境中只有單一網域，才可例外
    

![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/03-4. Infrastructure Master - 除非環境中只有單一.png]]

### 5. PDC Emulator（模擬主網域控制站）

- 模擬 NT PDC 功能，**同步密碼、時鐘、處理 GP 變更與密碼變更請求**
    
- 是時間同步來源與 GPO 預設應用對象
    
- 是唯一可立即接收密碼變更複寫的 DC
    

![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/04-5. PDC Emulator(模擬主網域控制站).png]]

#### 🛠️ 手動複寫操作

- Site and Services → 手動選擇複寫來源 DC
    

![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/05-🛠️ 手動複寫操作.png]]  
![[Assets/Note/Tech/Windows Server Active Directory Operation Master（FSMO）角色整理/06-🛠️ 手動複寫操作.png]]

#### 🔁 轉移 FSMO 角色（PDC 範例）

```powershell
Move-ADDirectoryServerOperationMasterRole -Identity "DC2" -OperationMasterRole PDCEmulator
```

#### 📋 多角色同時轉移

```powershell
Move-ADDirectoryServerOperationMasterRole -Identity "DC2" -OperationMasterRole 0,1,2,3,4
```

|FSMO 角色名稱|代碼|
|---|---|
|PDC Emulator|0|
|RID Master|1|
|Schema Master|2|
|Infrastructure Master|3|
|Domain Naming Master|4|

---

## 四、故障排除與奪取（Seize）

若 FSMO 擁有者損毀無法修復，需將角色強制奪取給其他主機：

```powershell
Move-ADDirectoryServerOperationMasterRole -Identity "DC2" -OperationMasterRole PDCEmulator -Force
```

### 各角色故障影響與建議：

#### 🔹 Schema Master

- 使用者不會直接感知
    
- 若需安裝 Exchange 等架構擴充時才會出現問題
    
- 可延後處理
    

#### 🔹 Domain Naming Master

- 不影響登入與使用，但無法新增或刪除網域
    
- 修好可繼續使用；損壞過久建議 Seize
    

#### 🔹 RID Master

- 使用者無感，但無法建立新帳號、群組等物件
    
- 每台 DC 有緩存 RID Pool，暫時不會出錯，RID 用完才有感
    
- 建議監控並及早 Seize
    

#### 🔹 PDC Emulator

- 直接影響使用者登入與密碼變更功能
    
- 若壞掉，使用者改密碼後無法登入 → 需立刻修復或轉移
    

#### 🔹 Infrastructure Master

- 使用者不直接受影響
    
- 影響管理員異動跨網域物件、群組成員變更等
    
- 若損壞太久仍未修復建議 Seize，否則可能造成複寫不一致
