## 概述

本指南說明如何使用 OpenSSL 產生憑證並在 Windows Web 伺服器（IIS）中使用，包括建立 Root CA、申請憑證、簽署憑證，以及在 Windows 系統中匯入和設定憑證。

## 憑證建立流程

### 1. 產生 Root CA

建立自簽的根憑證授權單位，作為內部憑證的簽署機構：

```bash
# 產生 Root CA 私鑰和自簽憑證（有效期 30 年）
openssl req -x509 -nodes -new -sha256 -days 10950 -newkey rsa:4096 \
    -keyout RootCA.key -out RootCA.pem \
    -subj "/O=NIHS/CN=Root-CA"

# 轉換為 CRT 格式（Windows 相容）
openssl x509 -outform pem -in RootCA.pem -out RootCA.crt

# 建立 PFX 格式（包含私鑰，用於匯入 Windows）
openssl pkcs12 -export -in RootCA.pem -inkey RootCA.key -out RootCA.pfx
```

#### Root CA 憑證資訊

產生的 Root CA 憑證包含以下資訊：

- **組織 (O)**：NIHS
- **一般名稱 (CN)**：Root-CA
- **金鑰長度**：4096 位元 RSA
- **雜湊演算法**：SHA-256
- **有效期**：30 年（10950 天）

![[Assets/Note/Tech/Linux OpenSSL With Windows Web 憑證設定指南/01-Root CA 憑證資訊.png]]

### 2. 申請網站憑證

為網站建立憑證申請，包括 Subject Alternative Name (SAN) 設定：

#### 建立 SAN 設定檔

```bash
# 建立 SAN 設定檔
vim san.txt
```

在 `san.txt` 中加入以下內容：

```ini
subjectAltName = @alt_names
[alt_names]
DNS.1 = www.skills39.com
DNS.2 = skills39.com
DNS.3 = *.skills39.com
```

#### 產生憑證申請 (CSR)

```bash
# 產生網站憑證的私鑰和憑證申請
openssl req -new -nodes -newkey rsa:4096 \
    -keyout www.skills39.com.key -out www.skills39.com.csr \
    -subj "/C=TW/ST=Taipei/L=Taiwan/O=NIHS/CN=*.skills39.com"
```

#### 憑證申請參數說明

|參數|值|說明|
|---|---|---|
|C|TW|國家代碼（台灣）|
|ST|Taipei|州/省（台北）|
|L|Taiwan|城市/地區（台灣）|
|O|NIHS|組織名稱|
|CN|*.skills39.com|通用名稱（萬用字元憑證）|

### 3. 簽署憑證

使用 Root CA 簽署網站憑證：

```bash
# 使用 Root CA 簽署憑證（有效期 1 年）
openssl x509 -req -sha256 -days 365 -in www.skills39.com.csr \
    -CA RootCA.pem -CAkey RootCA.key -CAcreateserial \
    -extfile san.txt -out www.skills39.com.crt
```

#### 網站憑證資訊

簽署後的網站憑證包含：

- **主體**：`*.skills39.com`
- **SAN**：支援多個域名
- **簽發者**：Root-CA
- **有效期**：1 年

![[Assets/Note/Tech/Linux OpenSSL With Windows Web 憑證設定指南/02-網站憑證資訊 - 有效期 1 年.png]]

### 4. 建立 PFX 格式憑證

將網站憑證和私鑰打包成 PFX 格式，以便匯入 Windows：

```bash
# 建立包含憑證鏈的 PFX 檔案
openssl pkcs12 -export -out www.skills39.com.pfx \
    -inkey www.skills39.com.key -in www.skills39.com.crt \
    -certfile RootCA.pem -name "www.skills39.com"
```

## Windows 憑證匯入設定

### 1. 匯入 Root CA 憑證

#### 使用憑證管理員 (certlm.msc)

1. **開啟憑證管理員**：
    
    ```cmd
    certlm.msc
    ```
    
2. **導航到受信任的根憑證授權單位**：
    
    - 展開「受信任的根憑證授權單位」
    - 右鍵點選「憑證」→「所有工作」→「匯入」
3. **匯入 Root CA**：
    
    - 選擇 `RootCA.crt` 或 `RootCA.pfx`
    - 完成匯入精靈

#### 使用 PowerShell

```powershell
# 匯入 Root CA 到受信任的根憑證授權單位
Import-Certificate -FilePath "RootCA.crt" -CertStoreLocation Cert:\LocalMachine\Root

# 或匯入 PFX 格式
Import-PfxCertificate -FilePath "RootCA.pfx" -CertStoreLocation Cert:\LocalMachine\Root
```

### 2. 匯入網站憑證

![[Assets/Note/Tech/Linux OpenSSL With Windows Web 憑證設定指南/03-2. 匯入網站憑證.png]]

> [!important] 重要提醒 記得要使用 PFX 格式匯入，因為 PFX 包含私鑰

#### IIS 憑證匯入步驟

1. **開啟 IIS 管理員**
2. **選擇伺服器憑證**
3. **匯入憑證**：
    - 點選「匯入」
    - 選擇 `www.skills39.com.pfx`
    - 輸入密碼（如果有設定）
    - 選擇憑證存放區：「個人」

![[Assets/Note/Tech/Linux OpenSSL With Windows Web 憑證設定指南/04-IIS 憑證匯入步驟 - 選擇憑證存放區 「個人」.png]]

#### 使用 PowerShell 匯入

```powershell
# 匯入網站憑證到個人憑證存放區
Import-PfxCertificate -FilePath "www.skills39.com.pfx" -CertStoreLocation Cert:\LocalMachine\My

# 檢視匯入的憑證
Get-ChildItem -Path Cert:\LocalMachine\My | Where-Object {$_.Subject -like "*skills39.com*"}
```

## IIS 網站 SSL 設定

### 1. 設定 HTTPS 繫結

1. **選擇網站**：在 IIS 管理員中選擇要設定的網站
2. **編輯繫結**：右鍵點選網站 → 「編輯繫結」
3. **新增 HTTPS 繫結**：
    - 類型：`https`
    - IP 位址：`所有未指派` 或特定 IP
    - 連接埠：`443`
    - 主機名稱：`www.skills39.com`
    - SSL 憑證：選擇匯入的憑證

### 2. 設定 SSL 設定

在 IIS 管理員中設定 SSL 選項：

```powershell
# 使用 PowerShell 設定 SSL 繫結
New-WebBinding -Name "Default Web Site" -IP "*" -Port 443 -Protocol https
```

### 3. 強制 HTTPS 重新導向

```xml
<!-- 在 web.config 中加入 HTTPS 重新導向 -->
<system.webServer>
  <rewrite>
    <rules>
      <rule name="Redirect to HTTPS" stopProcessing="true">
        <match url="(.*)" />
        <conditions>
          <add input="{HTTPS}" pattern="off" ignoreCase="true" />
        </conditions>
        <action type="Redirect" url="https://{HTTP_HOST}/{R:1}" redirectType="Permanent" />
      </rule>
    </rules>
  </rewrite>
</system.webServer>
```

## 驗證憑證設定

### 1. 檢查憑證狀態

```powershell
# 檢查憑證到期時間
Get-ChildItem -Path Cert:\LocalMachine\My | 
    Where-Object {$_.Subject -like "*skills39.com*"} | 
    Select-Object Subject, NotAfter, Thumbprint

# 檢查憑證鏈
certlm.msc
```

### 2. 測試 HTTPS 連線

```powershell
# 使用 PowerShell 測試 HTTPS 連線
Invoke-WebRequest -Uri "https://www.skills39.com" -UseBasicParsing

# 檢查 SSL 憑證詳細資訊
$cert = Invoke-WebRequest -Uri "https://www.skills39.com" -UseBasicParsing
$cert.BaseResponse.ServicePoint.Certificate
```

### 3. 瀏覽器測試

在瀏覽器中訪問 `https://www.skills39.com`，檢查：

- 🔒 綠色鎖圖示
- 憑證詳細資訊
- 沒有憑證錯誤警告

## 常見問題排除

### 1. 憑證不受信任

**問題**：瀏覽器顯示「憑證不受信任」

**解決方案**：

- 確認 Root CA 已匯入到「受信任的根憑證授權單位」
- 檢查憑證鏈是否完整
- 重新啟動瀏覽器或清除憑證快取

### 2. 主機名稱不匹配

**問題**：憑證主機名稱與網站不匹配

**解決方案**：

- 檢查憑證的 CN 和 SAN 設定
- 確認 IIS 繫結的主機名稱正確
- 使用萬用字元憑證 (`*.skills39.com`)

### 3. 私鑰無法存取

**問題**：IIS 無法存取憑證私鑰

**解決方案**：

```powershell
# 設定私鑰權限給 IIS
$cert = Get-ChildItem -Path Cert:\LocalMachine\My | Where-Object {$_.Subject -like "*skills39.com*"}
$keyPath = "$env:ProgramData\Microsoft\Crypto\RSA\MachineKeys\" + $cert.PrivateKey.CspKeyContainerInfo.UniqueKeyContainerName
icacls $keyPath /grant "IIS_IUSRS:R"
```
