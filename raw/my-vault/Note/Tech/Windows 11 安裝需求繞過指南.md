---
Type:
  - windows Basic
---
## 概述

Windows 11 對硬體有較嚴格的要求，包括 TPM 2.0、Secure Boot 和網路連線等。在某些情況下（如測試環境、舊硬體或實驗室環境），可能需要繞過這些限制來進行安裝。

> [!warning] 重要聲明 這些方法僅供教育和測試目的使用。繞過安全功能可能會降低系統安全性，請在生產環境中謹慎使用。

## 繞過網路連線要求

### 方法一：OOBE 繞過指令

在 Windows 11 安裝過程中，當系統要求連接網路時：

```bash
oobe\bypassnro
```

#### 使用步驟

1. **到達網路設定畫面**：在 Windows 11 安裝過程中，當看到「讓我們為您連接到網路」畫面時
2. **開啟命令提示字元**：按下 `Shift + F10` 開啟命令提示字元
3. **執行繞過指令**：輸入 `oobe\bypassnro` 並按 Enter
4. **系統重新啟動**：系統會自動重新啟動
5. **選擇離線安裝**：重新啟動後會出現「我沒有網際網路連線」選項

### 方法二：中斷網路連線

1. **拔除網路線**：如果使用有線連接，直接拔除網路線
2. **關閉 Wi-Fi**：如果使用無線連接，在網路選擇畫面關閉 Wi-Fi
3. **繼續安裝**：系統會提供離線安裝選項

## 繞過 TPM 和 Secure Boot 要求

### 註冊表修改方法

當 Windows 11 安裝程式檢測到不符合硬體需求時：

#### 步驟 1：開啟註冊表編輯器

```powershell
# 在安裝畫面按 Shift + F10 開啟命令提示字元
regedit
```

#### 步驟 2：導航到 Setup 路徑

導航到以下註冊表路徑：

```powershell
HKEY_LOCAL_MACHINE\SYSTEM\Setup\
```

#### 步驟 3：建立 LabConfig 機碼

> [!important] 新增 LabConfig 機碼
> 
> 在 Setup 機碼下建立新的機碼：
> 
> ```powershell
> HKEY_LOCAL_MACHINE\SYSTEM\Setup\LabConfig\
> ```

#### 步驟 4：新增繞過值

在 LabConfig 機碼中新增以下 DWORD 值：

```powershell
# 繞過 TPM 檢查
BypassTPMCheck = 1 (DWORD)

# 繞過 Secure Boot 檢查  
BypassSecureBootCheck = 1 (DWORD)

# 可選：繞過 RAM 檢查
BypassRAMCheck = 1 (DWORD)

# 可選：繞過 CPU 檢查
BypassCPUCheck = 1 (DWORD)
```

#### 完整的註冊表項目

```registry
Windows Registry Editor Version 5.00

[HKEY_LOCAL_MACHINE\SYSTEM\Setup\LabConfig]
"BypassTPMCheck"=dword:00000001
"BypassSecureBootCheck"=dword:00000001
"BypassRAMCheck"=dword:00000001
"BypassCPUCheck"=dword:00000001
```

### PowerShell 自動化腳本

```powershell
# 自動建立 LabConfig 機碼和值
$registryPath = "HKLM:\SYSTEM\Setup\LabConfig"

# 檢查並建立 LabConfig 機碼
if (!(Test-Path $registryPath)) {
    New-Item -Path $registryPath -Force | Out-Null
}

# 新增繞過檢查的值
Set-ItemProperty -Path $registryPath -Name "BypassTPMCheck" -Value 1 -Type DWord
Set-ItemProperty -Path $registryPath -Name "BypassSecureBootCheck" -Value 1 -Type DWord
Set-ItemProperty -Path $registryPath -Name "BypassRAMCheck" -Value 1 -Type DWord
Set-ItemProperty -Path $registryPath -Name "BypassCPUCheck" -Value 1 -Type DWord

Write-Host "LabConfig 設定完成，可以繼續 Windows 11 安裝程序"
```

## 其他繞過方法

### 使用修改版 ISO

1. **下載工具**：使用 Rufus 等工具建立安裝媒體
2. **選擇選項**：在 Rufus 中選擇移除 TPM、Secure Boot 和 RAM 需求
3. **建立媒體**：建立修改後的安裝媒體

### 使用第三方工具

- **Windows 11 Installation Assistant**
- **MediaCreationTool.bat**
- **UUP dump**

## 完整安裝流程

### 準備階段

1. **備份重要資料**
2. **準備安裝媒體**
3. **確認硬體最低需求**（即使繞過檢查，仍建議符合基本需求）

### 安裝階段

1. **啟動安裝程式**
2. **遇到硬體檢查時**：
    - 按 `Shift + F10`
    - 執行 `regedit`
    - 新增 LabConfig 設定
3. **遇到網路要求時**：
    - 按 `Shift + F10`
    - 執行 `oobe\bypassnro`
4. **繼續正常安裝流程**

### 後續設定

```powershell
# 安裝完成後檢查系統狀態
Get-ComputerInfo | Select-Object WindowsProductName, WindowsVersion, TotalPhysicalMemory

# 檢查 TPM 狀態
Get-Tpm

# 檢查 Secure Boot 狀態
Confirm-SecureBootUEFI
```

## 注意事項和風險

### 安全性考量

> [!warning] 安全性風險
> 
> - 繞過 TPM 可能會降低加密安全性
> - 關閉 Secure Boot 可能增加惡意軟體風險
> - 建議僅在測試環境中使用

### 相容性問題

- 某些功能可能無法正常運作
- Windows Update 可能會受到影響
- 某些企業功能可能不可用

### 法律和政策考量

- 確保符合組織的 IT 政策
- 了解軟體授權條款
- 僅在合法範圍內使用

## 故障排除

### 常見問題

#### 1. 註冊表修改無效

**解決方案**：

- 確認路徑正確：`HKLM:\SYSTEM\Setup\LabConfig`
- 確認值的類型為 DWORD
- 重新啟動安裝程序

#### 2. 仍然要求網路連線

**解決方案**：

- 重新執行 `oobe\bypassnro`
- 確認系統已重新啟動
- 檢查是否有離線選項

#### 3. 安裝後系統不穩定

**解決方案**：

- 檢查硬體驅動程式
- 更新 BIOS/UEFI
- 考慮降級到 Windows 10

## 替代方案

### Windows 10 升級路徑

如果硬體確實不符合要求，考慮：

1. **繼續使用 Windows 10**
2. **升級硬體組件**
3. **使用 Linux 替代方案**
4. **虛擬化方案**

### 硬體升級建議

- **添加 TPM 模組**
- **啟用 UEFI Secure Boot**
- **升級到支援的 CPU**
- **增加 RAM 到 4GB 以上**

這些方法應該謹慎使用，並且主要用於測試和實驗目的。在生產環境中，建議使用符合官方要求的硬體來獲得最佳的安全性和穩定性。