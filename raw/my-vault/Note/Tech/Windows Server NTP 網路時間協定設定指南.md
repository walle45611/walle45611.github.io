## NTP Server 設定

### 檢查當前 NTP Server 設定

```PowerShell
# 取得 w32time registry 的參數
Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\w32time\TimeProviders\NtpServer"
```

**典型輸出結果：**

```
AllowNonstandardModeCombinations : 1
ChainDisable                     : 0
ChainEntryTimeout                : 16
ChainLoggingRate                 : 30
ChainMaxEntries                  : 128
ChainMaxHostEntries              : 4
DllName                          : C:\Windows\system32\w32time.dll
Enabled                          : 0      # 預設為停用狀態
EventLogFlags                    : 0
InputProvider                    : 0
RequireSecureTimeSyncRequests    : 0
```

### 啟用 NTP Server

```PowerShell
# 啟用 NTP Server 功能
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\w32time\TimeProviders\NtpServer" -Name "Enabled" -Value 1

# 設定時間同步宣告旗標
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\services\W32Time\Config" -Name "AnnounceFlags" -Value 5

# 重新啟動 Windows Time 服務
Restart-Service w32Time
```

#### AnnounceFlags 參數說明

|值|說明|
|---|---|
|0|不宣告時間服務|
|1|總是宣告時間服務|
|2|自動宣告時間服務|
|4|總是宣告可靠的時間服務|
|5|自動宣告可靠的時間服務（推薦）|

### 設定防火牆規則

```PowerShell
# 新增 NTP Server 防火牆規則
New-NetFirewallRule `
    -Name "NTP Server Port" `
    -DisplayName "NTP Server Port" `
    -Description 'Allow NTP Server Port' `
    -Profile Any `
    -Direction Inbound `
    -Action Allow `
    -Protocol UDP `
    -Program Any `
    -LocalAddress Any `
    -LocalPort 123
```

> [!info] NTP 通訊協定資訊 NTP (Network Time Protocol) 使用 UDP 123 連接埠進行時間同步通訊

---

## NTP Client 設定

### 註冊和重新啟動

```PowerShell
# 註冊 Windows Time 服務
w32tm /register

# 重新啟動電腦以套用設定
Restart-Computer
```

### 設定時間伺服器

```PowerShell
# 設定手動時間同步來源
w32tm /config /update /manualpeerlist:"ntp.skills39.com,0x1"
```

#### manualpeerlist 參數說明

格式：`"伺服器位址,旗標"`

常用旗標：

- `0x1`：使用對稱主動模式
- `0x8`：以用戶端模式運作
- `0x9`：使用對稱主動模式 + 用戶端模式

### 重新啟動時間服務

```PowerShell
# 停止 Windows Time 服務
Stop-Service w32time

# 啟動 Windows Time 服務
Start-Service w32time
```

### 驗證設定

```PowerShell
# 檢查時間同步對等端
w32tm /query /peers

# 檢查時間來源
w32tm /query /source

# 檢查完整設定
w32tm /query /configuration
```

### 手動同步時間

```PowerShell
# 強制立即同步時間
w32tm /resync /force
```

## 常用 NTP 伺服器

### 公用 NTP 伺服器

```PowerShell
# Google NTP
w32tm /config /update /manualpeerlist:"time.google.com,0x1"

# Cloudflare NTP
w32tm /config /update /manualpeerlist:"time.cloudflare.com,0x1"

# Microsoft NTP
w32tm /config /update /manualpeerlist:"time.windows.com,0x1"

# 台灣 NTP（國家時間與頻率標準實驗室）
w32tm /config /update /manualpeerlist:"tock.stdtime.gov.tw,0x1"
```

## 故障排除

### 檢查時間同步狀態

```PowerShell
# 檢查詳細狀態
w32tm /query /status /verbose

# 檢查時間差異
w32tm /stripchart /computer:your-ntp-server.com
```

### 重設 Windows Time 設定

```PowerShell
# 重設為預設設定
w32tm /unregister
w32tm /register
Restart-Service w32time
```

### 常見問題

#### 問題 1：時間同步失敗

**解決方案：**

1. 檢查防火牆設定
2. 確認網路連線
3. 驗證 NTP 伺服器位址

#### 問題 2：服務無法啟動

**解決方案：**

```PowerShell
# 檢查服務狀態
Get-Service w32time

# 重新註冊服務
w32tm /unregister
w32tm /register
```

> [!warning] 注意事項
> 
> - 修改 NTP 設定後，建議重新啟動電腦以確保所有設定生效
> - 在 Domain Controller 上修改 NTP 設定時要特別小心，可能影響整個網域的時間同步
> - 確保所選擇的 NTP 伺服器穩定可靠，避免時間漂移問題

## 最佳實務建議

1. **選擇可靠的時間來源**：使用知名的公用 NTP 伺服器或建立內部 NTP 階層
2. **設定多個時間來源**：提高時間同步的可靠性
3. **定期監控**：監控時間同步狀態，及時發現問題
4. **文檔記錄**：記錄所有 NTP 設定變更，便於維護管理