# 什麼是 Microsoft Entra？

- source: `raw/什麼是 Microsoft Entra？.md`
- source link: https://learn.microsoft.com/zh-tw/entra/fundamentals/what-is-entra
- original title: 什麼是 Microsoft Entra？
- author: kenwith
- published: 2026-04-23
- type: article

Microsoft Entra 是一系列身份與網路存取產品，旨在協助組織實施零信任 (Zero Trust) 安全策略。它涵蓋了從基礎身份管理到 AI 代理程式安全存取的完整生態系。詳見 [Microsoft Entra](./../concepts/microsoft-entra.md)。

## 核心產品分類

### 1. 建立零信任存取控制
- **Microsoft Entra ID**: 基礎雲端身份與存取管理服務。
- **Microsoft Entra Domain Services**: 為舊有應用提供受管理的網域服務 (LDAP, Kerberos/NTLM)。

### 2. 保護存取權限
- **私人存取 (Private Access)**: 保護私有應用，無需 VPN。
- **網際網路存取 (Internet Access)**: 規範 SaaS 與 Microsoft 365 的網頁內容存取。
- **ID 管理與保護**: 包含 **ID 管理 (Governance)** (自動化生命週期) 與 **ID 保護 (Protection)** (偵測身份風險)。
- **驗證識別碼 (Verified ID)**: 基於去中心化身份 (DID) 標準的憑證驗證。

### 3. 外部與工作負載身份
- **外部 ID (External ID)**: 管理客戶與合作夥伴的存取權限 (CIAM/B2B)。
- **工作負載 ID (Workload ID)**: 為應用程式、服務與容器提供身份。
- **Agent ID**: 針對 AI 代理程式 (AI Agents) 的身份與安全框架，提供受控的身份結構與稽核軌跡。

## 管理與開發工具
- **Microsoft Entra 系統管理中心**: 單一介面的網頁入口網站。
- **Microsoft Graph API**: 用於自動化管理工作。
- **Microsoft 身分識別平台**: 供開發者建立身份感知應用程式。

## 相關連結
- [Microsoft Entra 授權](https://learn.microsoft.com/zh-tw/entra/fundamentals/licensing)
- [身份與存取基礎](https://learn.microsoft.com/zh-tw/entra/fundamentals/identity-fundamental-concepts)
