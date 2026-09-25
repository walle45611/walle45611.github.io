---
Type:
  - windows Basic
---
## Network Location（網路位置）

Network Location 可以分成以下幾種類型：

- **Private Network**（私人網路）
- **Public Network**（公用網路）
- **Domain Authenticated Network**（網域驗證網路）

加入網域會預設變成 Domain Network，可以透過介面更改也可以使用 PowerShell 指令。

### PowerShell 指令

#### 查看網路連線設定檔

```PowerShell
Get-NetConnectionProfile
```

![[Assets/Note/Tech/網路位置與 Windows Defender 防火牆設定指南/01-查看網路連線設定檔.png]]

#### 設定網路連線設定檔

```PowerShell
Set-NetConnectionProfile -Name "網路名稱" -NetworkCategory {Public | Private | Domain | DomainAuthenticated}
```

##### 網路名稱參考

![[Assets/Note/Tech/網路位置與 Windows Defender 防火牆設定指南/02-網路名稱參考.png]]

## Windows Defender 防火牆

![[Assets/Note/Tech/網路位置與 Windows Defender 防火牆設定指南/03-Windows Defender 防火牆.png]]

> [!important] 快速開啟防火牆管理 使用 `Win + R` 開啟執行對話框，輸入 `wf.msc` 即可快速開啟進階安全性 Windows Defender 防火牆

### 網路類型與防火牆設定

**Private Network**、**Public Network** 和 **Domain Authenticated Network** 這幾種網路類型對於防火牆有不同的預設設定：

![[Assets/Note/Tech/網路位置與 Windows Defender 防火牆設定指南/04-網路類型與防火牆設定.png]]

#### 防火牆動作設定

- **Block (Default)**：封鎖所有連線，除非防火牆規則明確允許
- **Block All Connections**：全部封鎖，不論是否有明確允許的規則
- **Allow**：允許連線，但會封鎖有防火牆規則明確禁止的連線

### 新增防火牆規則

![[Assets/Note/Tech/網路位置與 Windows Defender 防火牆設定指南/05-新增防火牆規則.png]]

#### 新增方式

1. **圖形介面**：透過 Windows Defender 防火牆進階安全性管理介面
2. **PowerShell 指令**：使用 PowerShell Cmdlet 進行自動化管理

### PowerShell 防火牆管理

> [!info] Windows PowerShell 防火牆管理指南 使用 Windows PowerShell 進階安全性管理 Windows Defender 防火牆
> 
> Windows 10、Windows 11、Windows Server 2016 及更新版本都支援使用 Windows PowerShell 指南進行進階安全性管理，提供將 Windows Defender 防火牆管理自動化的基本腳本。它專為 IT 專業人員、系統管理員、IT 管理員，以及其他在 Windows 中使用及需要自動化 Windows Defender 防火牆管理的人員而設計。
> 
> 您可以使用 Windows PowerShell 來管理防火牆和 IPsec 部署。此物件導向腳本環境可讓您更輕鬆地管理原則及監視網路狀況，而非 netsh 中所允許的情況。Windows PowerShell 可讓您透過每個 Cmdlet 中的語法和參數，自行探索網路設定。
> 
> 本指南示範如何在 netsh 中執行一般工作，以及如何使用 Windows PowerShell 來完成這些工作。在未來版本的 Windows 中，Microsoft 可能會移除 Windows Defender 防火牆的 netsh 功能。如果您目前使用 netsh 來設定和管理 Windows Defender 防火牆，Microsoft 建議您轉換為 Windows PowerShell。
> 
> **參考連結：** https://docs.microsoft.com/zh-tw/windows/security/threat-protection/windows-firewall/windows-firewall-with-advanced-security-administration-with-windows-powershell

#### PowerShell 與 netsh 命令比較

本指南不會教導您 Windows Defender 防火牆的基本概念，您可以在 Windows Defender 防火牆文件中找到這些概念。它不會教導 Windows PowerShell 的基本概念，而且會假設您熟悉 Windows PowerShell 語言和 Windows PowerShell 的基本概念。

> [!tip] 建議 如果您目前使用 netsh 來設定和管理 Windows Defender 防火牆，Microsoft 建議您轉換為 Windows PowerShell，因為未來版本可能會移除 netsh 功能。