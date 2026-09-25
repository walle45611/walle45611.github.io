---
Created: 2023-02-27T20:50
---
## 加密技術概述

### 非對稱加密（不是共用同一把密鑰）

非對稱加密通常不會用來加密資料，因為效能太慢。

> [!important] 主要用途 **通常都使用在身分驗證上面**

#### 優點

- **安全性高**：公私鑰分離，安全性佳
- **不怕被劫持**：即使公鑰被截取也無法解密
- **不用預先知道公鑰位置**：可透過證書機制分發

#### 缺點

- **速度極慢**：比對稱加密慢約 1000 倍

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/01-缺點.png]]

### 對稱加密（共用同一把密鑰）

#### 優點

- **速度快**：加解密效率高
- **安全性佳**：適當的密鑰長度下安全可靠
- **緊湊**：資料傳輸效率高

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/02-優點 - 緊湊 資料傳輸效率高.png]]

### 目前網路使用的加密流程（非對稱加密+對稱加密）

現代網路通訊結合兩種加密技術的優點：

1. 使用非對稱加密進行身分驗證和密鑰交換
2. 使用對稱加密進行實際資料傳輸

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/03-目前網路使用的加密流程(非對稱加密+對稱加密) - 使用對稱加密進行實際.png]] ![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/04-目前網路使用的加密流程(非對稱加密+對稱加密) - 使用對稱加密進行實際.png]]

---

## 加密算法種類

### 對稱加密種類

|算法|密鑰長度|安全性|備註|
|---|---|---|---|
|**DES**|56 bit|低|Data Encryption Standard，已不推薦使用|
|**3DES**|168 bit|中|Triple DES，逐漸被淘汰|
|**AES**|128/192/256 bit|高|Advanced Encryption Standard|
|**Blowfish**|可變|中高|開源算法|

#### AES 變體

- **AES-128**：基本安全等級
- **AES-192**：高安全等級
- **AES-256**：最高安全等級

### 非對稱加密種類

|算法|密鑰長度|主要用途|備註|
|---|---|---|---|
|**RSA**|2048 bit|加密與數位簽名|最常用的非對稱加密|
|**DSA**|可變|僅數位簽名|Digital Signature Algorithm|
|**ElGamal**|可變|加密與簽名|需要授權費用|

---

## 金鑰交換協定

### DH 算法（Diffie-Hellman）

Diffie-Hellman 算法允許雙方在不安全的通道上建立共享密鑰。

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/01-DH 算法(Diffie-Hellman).png]] ![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/05-DH 算法(Diffie-Hellman).png]]

#### 運作原理

在網際網路上不會知道 x、y 的實際數字，但透過數學計算卻能算出相同的結果，這就是 DH 算法的精妙之處。這樣就可以不用預先知道對方的密鑰就可以進行安全通訊。

### PFS（Perfect Forward Secrecy）

#### 特性

- **要求每次連線都重新產生密鑰**
- **新密鑰與舊密鑰無任何衍生關係**
- **預設不啟用**，需要手動設定

#### Cisco 設定指令

```cisco
crypto map WORD 10 set pfs group2
```

---

## 雜湊驗證

## HMAC（Hash Message Authentication Code）

### 用途

確定資料內容是否相同，檢測是否有被竄改。

### Hash 特性

|特性|說明|
|---|---|
|**固定輸出大小**|可以將任意大小的資料計算成固定大小的雜湊值|
|**敏感性**|資料若有任何改變，雜湊值都會產生巨大變化|
|**不可逆性**|無法從雜湊值反推原始資料|
|**唯一性**|理論上不可能有兩個不同文件產生相同雜湊值|

### Hash 流程

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/06-Hash 流程.png]]

### 流行的 Hash 種類

- **MD5**：已不安全，不建議使用
- **SHA**：安全雜湊算法系列
    - SHA-1：已不安全
    - SHA-256：目前推薦使用
    - SHA-512：更高安全性

> [!info] 術語說明 Hash value 也稱為 **Fingerprint**（指紋）

### 應用場景

#### 1. 路由驗證（只有 Hash 沒有加密）

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/07-1. 路由驗證(只有 Hash 沒有加密).png]]

#### 2. CHAP 驗證（PPP）

用於點對點協定的身分驗證

#### 3. 數位簽章

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/08-3. 數位簽章.png]]

#### 4. IPSec

用於 VPN 通道的完整性驗證

#### Cisco 設定範例

```cisco
Router#verify /md5 system:running-config
```

---

## SSL/TLS 協定

## SSL（Secure Socket Layer）/ TLS（Transport Layer Security）

SSL/TLS 是由 Netscape 公司開發的安全協定，在應用層和傳輸層之間創建一個安全層。在傳輸層以下的層級（包括傳輸層）都是以明文方式傳輸。

### 版本演進

|版本|狀態|備註|
|---|---|---|
|SSLv1|已廢棄|從未公開發布|
|SSLv2|已廢棄|有嚴重安全漏洞|
|SSLv3|已廢棄|有 POODLE 攻擊漏洞|
|**TLSv1.0**|逐漸淘汰|仍有部分應用使用|
|**TLSv1.2**|**廣泛使用**|目前主流版本|
|**TLSv1.3**|最新標準|效能和安全性最佳|

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/09-版本演進.png]]

### SSL/TLS 建立過程

#### Phase 1（握手階段）

1. **協商加密算法**：雙方協商使用的加密套件
2. **驗證伺服器**：客戶端驗證伺服器證書
3. **建立密鑰**：建立用於加密和 MAC 的密鑰

> [!info] 類比說明 類似於 IPSec VPN 中 IKE 的作用

#### Phase 2（安全資料傳輸階段）

在已建立的 SSL 連線中進行安全的資料傳輸

> [!info] 類比說明 類似於 IPSec VPN 中 ESP 的作用

---

## CA 證書頒發機構

## CA（Certificate Authority）

> [!warning] 重要提醒 時間同步非常重要，必須確保所有系統時間同步

### 第一步驟：CA 建立

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/10-第一步驟 CA 建立.png]]

### 第二步驟：證書產生流程

簡單來說，UserA 將自己的 Public Key 和個人 ID 傳送給 CA：

1. **CA 接收申請**：收到 Public Key 和 ID 資訊
2. **產生簽章**：CA 將這兩項資訊進行 Hash，再用自己的私鑰加密產生簽章
3. **製作證書**：用明文方式將以下資訊製作成證書：
    - ID（身分識別）
    - UserA 的 Public Key
    - CA 簽章
    - 有效期限

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/11-第二步驟 證書產生流程 - 有效期限.png]]

### 第三步驟：證書驗證流程

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/12-第三步驟 證書驗證流程.png]] ![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/13-第三步驟 證書驗證流程.png]]

### 多用戶證書交換

假設 UserC 也跟 CA 申請了證書：

1. **安全交換**：可透過不信任網路安全交換被 CA 簽名的公鑰
2. **信任驗證**：收到的公鑰都經過 CA 驗證，而 CA 的公鑰在每個個體本地必須是受信任的

#### 證書驗證範例

假設在 UserC 上對 UserA 的 Public Key 進行驗證：

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/14-證書驗證範例.png]]

---

## 證書申請標準

### File-based PKCS#10（最常用）

|特性|說明|
|---|---|
|**用途**|離線證書申請|
|**定義**|證書請求格式標準|
|**請求內容**|DN（個人資訊）、Public Key、其他屬性|

#### 請求包括內容：

- **DN（Distinguished Name）**：個人資訊
- **Public Key**：申請者的公開金鑰
- **其他屬性**：擴展資訊

### SCEP（Simple Certificate Enrollment Protocol）

|特性|說明|
|---|---|
|**用途**|**線上申請證書**|
|**設計者**|主要由 Cisco 設計|
|**標準地位**|VPN 設備 PKI 證書申請的工業標準|
|**傳輸協定**|**HTTP 傳輸**|
|**優勢**|為 VPN 設備提供簡單的申請方式|

### Web-based（Browser-to-CA）

|特性|說明|
|---|---|
|**常用平台**|Windows Server 常用|
|**介面**|Web 瀏覽器介面|
|**適用場景**|終端使用者證書申請|

---

## 證書吊銷機制

### CRL（Certificate Revocation List）

#### 特性

- **類似通緝公告**：列出已吊銷的證書
- **Time-stamped**：有明確的有效期限
- **CA-signed**：由 CA 簽名確保完整性

#### 運作機制

- **週期性輪詢**：客戶端週期性輪詢 CRL 儲存位置（HTTP/LDAP/FTP）
- **更新機制**：管理員吊銷證書後，新的 CRL 會被頒發
- **更新延遲**：CRL 不會立即更新，必須等舊的 CRL 過期才能獲取新的 CRL

![[Assets/Note/Research/PKI (Public key infrastructure)公鑰基礎設施/15-運作機制.png]]

### OCSP（Online Certificate Status Protocol）

|特性|說明|
|---|---|
|**用途**|線上查詢證書吊銷狀態|
|**即時性**|提供即時證書狀態查詢|
|**支援狀況**|IOS CA 暫時不支援|

---

## 證書格式與檔案類型

### 證書格式

#### X.509

X.509 是最常用的證書格式標準，包含：

|欄位|內容|
|---|---|
|**公鑰及有效期限**|證書持有者的公開金鑰和使用期限|
|**證書的合法擁有者**|身分識別資訊|
|**證書使用方式**|允許的用途和限制|
|**CA 資訊**|頒發機構的詳細資訊|
|**CA 簽章**|確保證書完整性的數位簽章|

#### PKCS#12

- **用途**：個人資訊交換格式
- **內容**：私鑰和證書的組合包
- **安全性**：通常有密碼保護

### 各類檔案副檔名

|副檔名|全名|說明|
|---|---|---|
|**CSR**|Certificate Signing Request|向 CA 遞交的證書申請檔案|
|**CRT**|Certificate|證書檔案|
|**PEM**|Privacy-Enhanced Mail|Linux 系統常用的證書編碼格式|
|**DER**|Distinguished Encoding Rules|Windows、Java 常用的證書編碼格式|

#### 編碼格式比較

|格式|特性|常用平台|
|---|---|---|
|**PEM**|Base64 編碼，文字格式|Linux、Apache|
|**DER**|二進位格式|Windows、Java|

## 總結

現代網路安全架構採用多層次的加密和認證機制：

1. **混合加密**：結合對稱和非對稱加密的優點
2. **完整性驗證**：使用雜湊算法確保資料未被竄改
3. **身分認證**：透過數位證書和 CA 體系確保通訊對象身分
4. **前向安全性**：透過 PFS 確保長期安全性

選擇適當的加密技術和證書管理策略，是建立安全網路環境的關鍵。