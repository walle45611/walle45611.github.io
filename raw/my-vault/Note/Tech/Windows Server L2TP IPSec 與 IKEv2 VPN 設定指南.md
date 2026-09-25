## L2TP/IPSec Pre-Shared Key Tunnel

### 網路拓撲

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/01-網路拓撲.png]]

### 設定步驟

#### 先決條件

須先建立 PPTP Site-to-Site VPN，Pre-shared Key 需設定一樣。

#### VPNS2 設定

1. **基本 VPN 連接設定**
    
    ![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/02-VPNS2 設定 - 基本 VPN 連接設定.png]]
    
2. **介面 Pre-shared Key 設定**
    
    介面也需要設定 Pre-shared Key：
    
    ![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/03-VPNS2 設定.png]]
    
3. **進階 Key 設定**
    
    點擊 Advanced 就可以設定 Key：
    
    ![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/04-VPNS2 設定 - 進階 Key 設定.png]]
    

> [!important] 重要提醒 建議重開機以確保設定穩定

## L2TP/Certificate IPSec Tunnel

### 網路拓撲

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/01-網路拓撲.png]]

### 大致步驟

1. **在 DC 上安裝服務**：安裝 Domain Controller、ADCS 和 DHCP（供內部使用）
2. **VPNS1 加入網域**：將 VPNS1 加入 DC 的網域
3. **建立 Site-to-Site VPN**：VPNS1 和 VPNS2 建立 tunnel
4. **RODC 加入網域**：將 VPNS2 內部的 RODC 加入 DC 網域（可以用 RODC 也可以變成子網域，DNS 指向 192.168.8.1）
5. **VPNS2 加入網域**：將 VPNS2 加入網域（DNS 指向 192.168.9.1）
6. **重新登入**：VPNS2 重新開機後先用本機的 Administrator 建立 VPN 連線再登入網域的 Administrator

### 詳細設定流程

#### 1. DC 服務安裝

在 DC 上執行：

```powershell
# 安裝 Active Directory Domain Services
Install-WindowsFeature AD-Domain-Services -IncludeManagementTools
```

升級為 DC 後再安裝：

```powershell
# 安裝 ADCS 憑證服務和 Web 註冊
Install-WindowsFeature ADCS-Cert-Authority, ADCS-Web-Enrollment -IncludeManagementTools
```

#### 2. VPNS1 加入網域

```powershell
# 將 VPNS1 加入網域
Add-Computer -DomainName skills39.com
```

#### 3. VPN 和路由服務安裝

在 VPNS1 和 VPNS2 上安裝 VPN 和 Routing：

```powershell
# 安裝 DirectAccess VPN 和 Routing 功能
Install-WindowsFeature DirectAccess-VPN, Routing -IncludeManagementTools
```

#### 4. 憑證申請設定

##### 在 VPNS1 上設定 NAT

在 VPNS1 上 NAT 出 DC 的 Web Server，讓 VPNS2 可以利用網頁申請憑證，也可以透過其他方式先在 VPNS1 申請再傳送到 VPNS2。

##### 在 VPNS2 上申請憑證

在 VPNS2 上面 Certificate Template 選擇 Administrator，因為 L2TP 的檢查方式不會檢查 DNS，只會檢查有沒有 CA 認證過的憑證。

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/05-在 VPNS2 上申請憑證.png]]

##### 憑證安裝

將憑證下載下來再安裝到 VPNS2，在 VPNS1 上也需要憑證，可以透過 MMC 或是 Web 的方式申請。

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/06-憑證安裝.png]]

#### 5. VPN 介面設定

新增撥入介面時，記得要改通道類型：

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/07-5. VPN 介面設定.png]]

### L2TP Site-to-Site Certificate Authority 總結

方法大致上跟一般的 L2TP Certificate 方式是一樣的，主要差異在於需要建立完整的 PKI 基礎架構來管理憑證。

## IKEv2 VPN 設定

### IKEv2 Pre-Shared Key IPSec Tunnel

#### 設定步驟

步驟大致跟 L2TP Pre-Shared Key 相同：

![[Assets/Note/Tech/Windows Server L2TP IPSec 與 IKEv2 VPN 設定指南/08-設定步驟.png]]

#### 基本設定流程

1. **建立 IKEv2 連接**
    
    - 選擇 IKEv2 作為通道類型
    - 設定目標伺服器 IP
    - 配置 Pre-shared Key
2. **安全性設定**
    
    - 設定加密演算法
    - 配置完整性驗證
    - 選擇 DH 群組
3. **連接測試**
    
    - 驗證連接狀態
    - 測試資料傳輸
    - 檢查日誌記錄

### IKEv2 Certificate IPSec Tunnel

#### 設定流程

1. **憑證準備**
    
    - 建立 CA 根憑證
    - 為每個端點產生憑證
    - 安裝憑證到適當的憑證存放區
2. **IKEv2 設定**
    
    ```powershell
    # 設定 IKEv2 連接參數
    Add-VpnConnection -Name "IKEv2-Site" `
                      -ServerAddress "vpn.example.com" `
                      -TunnelType IKEv2 `
                      -AuthenticationMethod MachineCertificate `
                      -EncryptionLevel Maximum
    ```
    
3. **憑證驗證設定**
    
    - 配置憑證驗證規則
    - 設定 CRL 檢查
    - 配置 OCSP 驗證（可選）

#### 進階設定選項

```powershell
# 設定進階 IKEv2 參數
Set-VpnConnectionIPsecConfiguration -ConnectionName "IKEv2-Site" `
                                   -AuthenticationTransformConstants SHA256128 `
                                   -CipherTransformConstants AES256 `
                                   -DHGroup Group14 `
                                   -IntegrityCheckMethod SHA256 `
                                   -PfsGroup Group14 `
                                   -EncryptionMethod AES256
```

## 比較表格

|特性|L2TP/IPSec PSK|L2TP/IPSec Cert|IKEv2 PSK|IKEv2 Cert|
|---|---|---|---|---|
|**安全性**|中等|高|高|最高|
|**設定複雜度**|簡單|複雜|中等|複雜|
|**效能**|中等|中等|高|高|
|**相容性**|佳|佳|有限|有限|
|**管理難度**|低|高|中等|高|

## 故障排除

### 常見問題

#### 1. 連接失敗

**檢查項目**：

- 防火牆設定
- Pre-shared Key 是否一致
- 網路連通性
- 服務狀態

#### 2. 憑證驗證失敗

**檢查項目**：

- 憑證是否有效
- 憑證鏈是否完整
- 時間同步
- CRL 可達性

#### 3. 效能問題

**最佳化建議**：

- 選擇適當的加密演算法
- 調整 MTU 大小
- 檢查網路延遲
- 監控 CPU 使用率

### 除錯指令

```powershell
# 檢查 VPN 連接狀態
Get-VpnConnection

# 檢查 IPSec 安全關聯
Get-VpnConnectionIPsecConfiguration

# 檢查路由表
route print

# 檢查憑證
certlm.msc

# 檢查事件日誌
Get-WinEvent -LogName "Application" | Where-Object {$_.ProviderName -like "*VPN*"}
```

## 最佳實務建議

1. **安全性**
    
    - 優先使用憑證驗證
    - 定期更新 Pre-shared Key
    - 啟用強式加密演算法
2. **效能**
    
    - 選擇適當的 DH 群組
    - 考慮硬體加速
    - 監控連接品質
3. **管理**
    
    - 建立完整的 PKI 架構
    - 定期備份設定
    - 建立監控機制
4. **相容性**
    
    - 測試不同用戶端
    - 確認防火牆相容性
    - 驗證 NAT 支援