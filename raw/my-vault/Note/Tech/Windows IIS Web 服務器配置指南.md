## 流程概述

1. 申請 DNS
2. 管理區域內的 DNS
3. 架設 IIS Web 服務

## DNS 配置

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/01-DNS 配置.png]]

## IIS 安裝

### 安裝 IIS Web 服務器

```PowerShell
Install-WindowsFeature web-server -IncludeManagementTools
```

### 測試預設網站

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/02-測試預設網站.png]]

## 網站配置

### 預設網站位置

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/03-預設網站位置.png]]

#### 使用共享資料夾

- 如果要使用共享資料夾，需要使用 UNC 路徑（`\\computer name\folder name`）
- 在網站上需要有權存取共用資料夾的帳號名稱

#### 指定連線身分

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/04-指定連線身分.png]]

#### 設定指定使用者

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/05-設定指定使用者.png]]

### 網頁預設檔案配置

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/06-網頁預設檔案配置.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/07-網頁預設檔案配置.png]]

> [!warning] 權限問題 如果沒有網頁檔案或是沒有權限，就會出現 403 拒絕存取錯誤

## HTTP 重新導向

### 安裝 HTTP 重新導向功能

```PowerShell
Install-WindowsFeature Web-Http-Redirect -IncludeManagementTools
```

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/08-安裝 HTTP 重新導向功能.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/09-安裝 HTTP 重新導向功能.png]]

### 重新導向選項

#### Redirect all request to exact destination

- 預設選項
- 例如：從 `http://www.sayms.local/default.htm` 導向到 `http://www.sayiis.local/default.htm`

#### Only redirect requests to content in this directory

- 不包含子目錄
- 由目的地網站決定要跳轉到哪個頁面

## 目錄管理

### Physical Directory 與 Virtual Directory

Virtual Directory 可以用來存取其他磁碟機內的資料夾或共用資料夾，每個 Virtual Directory 都有一個 alias。

#### Physical Directory

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/10-Physical Directory.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/11-Physical Directory.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/12-Physical Directory.png]]

#### Virtual Directory

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/13-Virtual Directory.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/14-Virtual Directory.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/15-Virtual Directory.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/16-Virtual Directory.png]]

## 網站繫結（Binding）

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/17-網站繫結(Binding).png]]

### 繫結選項說明

#### Hostname

- Default Web Site 沒有設定主機名
- 一旦設定了就必須用 hostname 連線
- 例如：設定了 `www.sayms.local` 需使用 `http://www.sayms.local/` 來連接
- 不可以使用 IP 位址連線

#### IP Address

- 電腦如果設定多個 IP 位址，可以每一個都設定 IP
- 例如：設定 `192.168.1.1`，連接到 `192.168.8.1` 的要求都會被送到 Default Web Site

#### TCP Port Number

- 預設是 80 port
- 可以更改 port，如果更改連線時就必須加 port
- 例如：`http://www.sayms.local:8080/`

## Web 安全性

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/18-Web 安全性.png]]

### 驗證方式（Authentication）

IIS 網站預設允許所有使用者連接，也可以要求必須輸入帳號與密碼。主要驗證方法有：匿名驗證、基本驗證、Digest Authentication、Windows Authentication。

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/19-驗證方式(Authentication).png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/20-驗證方式(Authentication).png]]

### 驗證方法使用順序

用戶端瀏覽器會按以下順序嘗試驗證：

1. **匿名驗證**
2. **基本驗證**
3. **摘要式驗證**
4. **Windows 驗證**

#### 匿名驗證

- 內建會有一個 IUSR 的特殊群組
- 當匿名連線網站時，使用 IUSR 來代表這個使用者
- 使用者的權限與 IUSR 的權限相同

#### 基本驗證（Basic Authentication）

- 傳送帳號密碼，但沒有加密
- 可以使用 SSL 保障密碼安全

#### 摘要驗證（Digest Authentication）

- 比基本驗證更安全
- 帳號與密碼經 MD5 演算法處理
- 處理後產生的 hash 傳送給 Web Site
- 就算攔截到也無法得知密碼
- 需要停用匿名驗證和 Windows 驗證

#### Windows 驗證

- 帳號密碼傳送會進行 hash 處理，確保安全性
- 適用於內部網路，會自動使用目前登入的帳號密碼
- 如果登入失敗會要求輸入帳號

### IP 位址限制

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/21-IP 位址限制.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/22-IP 位址限制.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/23-IP 位址限制.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/24-IP 位址限制.png]]

### 透過 NTFS 權限增加網頁安全性

可以透過檔案系統權限來管理誰可以訪問網站內容。

參考：[[Windows 檔案權限管理指南]]

## 遠端管理 IIS

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/25-遠端管理 IIS.png]]

### 安裝管理服務

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/26-安裝管理服務.png]]

### 建立 IIS 管理員帳號

IIS 電腦可以指定遠端管理的 IIS 網站使用者：

- 可以使用網域或內建使用者當作管理員
- 也可以在 IIS 內另外建立 IIS 管理員
- 如果要使用 IIS 管理員建立的方式，需要使用 Windows Authentication

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/27-建立 IIS 管理員帳號.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/28-建立 IIS 管理員帳號.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/29-建立 IIS 管理員帳號.png]]

### 委派功能

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/30-委派功能.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/31-委派功能.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/32-委派功能.png]]

### 啟用遠端連線

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/33-啟用遠端連線.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/34-啟用遠端連線.png]]

### 遠端用戶端設定

#### 先決條件

- 要先安裝 IIS

#### 遠端連線步驟

![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/35-遠端連線步驟.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/36-遠端連線步驟.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/37-遠端連線步驟.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/38-遠端連線步驟.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/39-遠端連線步驟.png]] ![[Assets/Note/Tech/Windows IIS Web 服務器配置指南/40-遠端連線步驟.png]]

> [!important] 重要資訊 IIS 遠端用戶端是透過 TCP 8172 連接，安裝管理服務 role 就會自動開放此連接埠