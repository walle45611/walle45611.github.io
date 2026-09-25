## 原理說明

> [!important] 必須先理解原理 必須先了解這個架構，不然會不知道自己在做什麼，而且這樣比較方便記住流程

## OpenSSL 架構

OpenSSL 由三個主要組件組成：

### 1. libcrypto - 加密庫

- 提供基本的加密演算法
- 支援對稱和非對稱加密
- 包含雜湊函數和數位簽章功能

### 2. libssl - TLS/SSL 工具

- 基於會話的通訊協定實作
- 實現了身分驗證功能
- 確保資料的機密性和完整性
- 提供完整的 TLS/SSL 函式庫

### 3. openssl - 多用途命令列工具

- 可以實現私有 CA (Certificate Authority)
- 包含眾多子命令
- 支援各種憑證操作和格式轉換

## CA (Certificate Authority) 建立

### 產生私有 Root CA

#### 一次性產生 Root CA 憑證和私鑰

```bash
# 產生 Root CA 私鑰和自簽憑證
openssl req -x509 -nodes -new -sha256 -days 10950 -newkey rsa:4096 \
    -keyout RootCA.key -out RootCA.pem \
    -subj "/O=NIHS/CN=Root-CA"
```

#### 轉換憑證格式

```bash
# 轉換為 CRT 格式
openssl x509 -outform pem -in RootCA.pem -out RootCA.crt

# 轉換為 PFX 格式（包含私鑰和憑證）
openssl pkcs12 -export -in linux_cert+ca.pem -inkey privatekey.key -out output.pfx
```

#### 參數說明

|參數|說明|
|---|---|
|`-x509`|產生自簽憑證|
|`-nodes`|私鑰不加密|
|`-new`|產生新的憑證請求|
|`-sha256`|使用 SHA-256 雜湊演算法|
|`-days 10950`|憑證有效期（約 30 年）|
|`-newkey rsa:4096`|產生 4096 位元 RSA 私鑰|
|`-subj`|指定主體資訊|

## 憑證申請流程

### 1. 建立 SAN 設定檔

Subject Alternative Name (SAN) 設定檔用於指定多個域名：

```bash
# 建立 SAN 設定檔
vim san.txt
```

```ini
subjectAltName = @alt_names
[alt_names]
DNS.1 = *.skills39.com
DNS.2 = skills39.com
DNS.3 = www.skills39.com
IP.1 = 192.168.1.100
```

### 2. 產生憑證申請 (CSR)

```bash
# 產生私鑰和憑證申請
openssl req -new -nodes -newkey rsa:4096 \
    -keyout certificate.key -out certificate.csr \
    -subj "/C=TW/ST=Taipei/L=Taiwan/O=NIHS/CN=*.skills39.com"
```

### 3. 簽署憑證

```bash
# 使用 Root CA 簽署憑證
openssl x509 -req -sha256 -days 365 -in certificate.csr \
    -CA RootCA.pem -CAkey RootCA.key -CAcreateserial \
    -extfile san.txt -out certificate.crt
```

## CSR (Certificate Signing Request) 管理

### 方法一：一條指令完成

```bash
openssl req -new -sha256 -key test.key -out test.csr \
    -subj "/CN=yourdomain.name" -reqexts SAN \
    -config <(cat /etc/ssl/openssl.cnf <(printf "\n[SAN]\nsubjectAltName=DNS:www.yourdomain.com,IP:192.168.10.1"))
```

### 方法二：分步驟操作

#### 1. 產生私鑰

```bash
# 產生 2048 位元 RSA 私鑰
openssl genrsa -out test.key 2048

# 產生加密的私鑰
openssl genrsa -aes256 -out test.key 2048
```

#### 2. 撰寫 OpenSSL 設定檔

建立自定義設定檔可以方便添加需要的參數：

```bash
vim server.cnf
```

```ini
[req]
distinguished_name = dn
req_extensions = req_ext
prompt = no

[dn]
C=TW
ST=Taiwan
L=Taipei
O=NIHS
OU=IT
CN=*.test.com

[req_ext]
subjectAltName = @alt_names

[alt_names]
DNS.1=*.test.com
DNS.2=test.com
IP.1=192.168.1.254
```

#### 3. 產生 CSR

```bash
# 使用設定檔產生 CSR
openssl req -new -key server.key -out server.csr -config server.cnf
```

## 信任 Root CA

### Linux 系統信任設定

> [!info] 重要資訊 根據官方文件，副檔名必須為 `.crt`
> 
> 參考：[update-ca-certificates(8) — ca-certificates — Debian buster — Debian Manpages](https://manpages.debian.org/buster/ca-certificates/update-ca-certificates.8.en.html)

![[Assets/Note/Tech/Linux OpenSSL 憑證管理完整指南/01-Linux 系統信任設定.png]]

```bash
# 複製 Root CA 憑證到系統目錄
sudo cp rootCAFile.crt /usr/local/share/ca-certificates/

# 更新系統憑證庫
sudo update-ca-certificates
```

### Windows 系統信任設定

```powershell
# 匯入憑證到受信任的根憑證授權單位
Import-Certificate -FilePath "RootCA.crt" -CertStoreLocation Cert:\LocalMachine\Root

# 或使用 certlm.msc 圖形界面匯入
certlm.msc
```

### macOS 系統信任設定

```bash
# 添加到系統鑰匙圈
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain RootCA.crt
```

## 憑證格式轉換

### PFX 轉換為憑證和私鑰

```bash
# 從 PFX 提取憑證（不含私鑰）
openssl pkcs12 -in filename.pfx -clcerts -nokeys -out certificatename.crt

# 從 PFX 提取私鑰（不含憑證）
openssl pkcs12 -in filename.pfx -nocerts -out certificatename.key

# 移除私鑰密碼保護
openssl rsa -in certificatename.key -out certificatename_unencrypted.key
```

### 其他常用格式轉換

#### PEM 轉 DER

```bash
# 憑證格式轉換
openssl x509 -outform der -in certificate.pem -out certificate.der

# 私鑰格式轉換
openssl rsa -outform der -in privatekey.pem -out privatekey.der
```

#### DER 轉 PEM

```bash
# 憑證格式轉換
openssl x509 -inform der -in certificate.der -out certificate.pem

# 私鑰格式轉換
openssl rsa -inform der -in privatekey.der -out privatekey.pem
```

#### 建立 PFX 檔案

```bash
# 將憑證和私鑰打包成 PFX
openssl pkcs12 -export -out certificate.pfx -inkey privatekey.key -in certificate.crt

# 包含憑證鏈的 PFX
openssl pkcs12 -export -out certificate.pfx -inkey privatekey.key -in certificate.crt -certfile ca-chain.crt
```

## 憑證檢視和驗證

### 檢視憑證內容

```bash
# 檢視 PEM 格式憑證
openssl x509 -in certificate.crt -text -noout

# 檢視 CSR 內容
openssl req -in certificate.csr -text -noout

# 檢視 PFX 內容
openssl pkcs12 -in certificate.pfx -info -noout

# 檢視私鑰資訊
openssl rsa -in privatekey.key -text -noout
```

### 驗證憑證

```bash
# 驗證憑證和私鑰是否匹配
openssl x509 -noout -modulus -in certificate.crt | openssl md5
openssl rsa -noout -modulus -in privatekey.key | openssl md5

# 驗證憑證鏈
openssl verify -CAfile rootca.crt -untrusted intermediate.crt certificate.crt

# 檢查憑證到期時間
openssl x509 -in certificate.crt -noout -dates
```

## 實用腳本範例

### 批次產生多個憑證

```bash
#!/bin/bash
# 批次產生憑證腳本

DOMAINS=("example.com" "test.com" "demo.com")
CA_CERT="RootCA.pem"
CA_KEY="RootCA.key"

for domain in "${DOMAINS[@]}"; do
    echo "正在為 $domain 產生憑證..."
    
    # 產生私鑰
    openssl genrsa -out "${domain}.key" 2048
    
    # 產生 CSR
    openssl req -new -key "${domain}.key" -out "${domain}.csr" \
        -subj "/C=TW/ST=Taiwan/L=Taipei/O=NIHS/CN=${domain}"
    
    # 簽署憑證
    openssl x509 -req -in "${domain}.csr" -CA "$CA_CERT" -CAkey "$CA_KEY" \
        -CAcreateserial -out "${domain}.crt" -days 365 -sha256
    
    echo "$domain 憑證產生完成"
done
```

### 憑證到期檢查腳本

```bash
#!/bin/bash
# 檢查憑證到期時間

check_cert_expiry() {
    local cert_file=$1
    local expiry_date=$(openssl x509 -in "$cert_file" -noout -enddate | cut -d= -f2)
    local expiry_epoch=$(date -d "$expiry_date" +%s)
    local current_epoch=$(date +%s)
    local days_left=$(( (expiry_epoch - current_epoch) / 86400 ))
    
    echo "憑證: $cert_file"
    echo "到期日: $expiry_date"
    echo "剩餘天數: $days_left 天"
    
    if [ $days_left -lt 30 ]; then
        echo "⚠️  警告: 憑證即將在 30 天內到期!"
    fi
    echo "---"
}

# 檢查目錄中所有 .crt 檔案
for cert in *.crt; do
    if [ -f "$cert" ]; then
        check_cert_expiry "$cert"
    fi
done
```

## 常見問題和解決方案

### 1. 憑證主體名稱不匹配

**問題**：瀏覽器顯示「憑證名稱不匹配」錯誤

**解決方案**：

- 確保 CN 或 SAN 包含正確的域名
- 使用萬用字元憑證 (*.domain.com)
- 添加所有需要的域名到 SAN

### 2. 憑證鏈不完整

**問題**：憑證無法驗證

**解決方案**：

```bash
# 建立完整的憑證鏈
cat certificate.crt intermediate.crt rootca.crt > fullchain.crt
```

### 3. 私鑰權限問題

**問題**：私鑰檔案權限過於開放

**解決方案**：

```bash
# 設定適當的檔案權限
chmod 600 privatekey.key
chown root:root privatekey.key
```

## 最佳實務建議

1. **金鑰長度**：使用至少 2048 位元的 RSA 金鑰
2. **有效期限**：伺服器憑證建議 1-2 年，Root CA 可設定較長時間
3. **備份**：定期備份 CA 私鑰和憑證
4. **權限控制**：嚴格控制私鑰檔案的存取權限
5. **定期更新**：在憑證到期前及時更新
6. **使用 SAN**：現代瀏覽器要求使用 Subject Alternative Name
7. **安全儲存**：考慮使用 HSM (Hardware Security Module) 儲存根憑證私鑰