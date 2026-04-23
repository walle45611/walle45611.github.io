---
title: "什麼是 Microsoft Entra？"
source: "https://learn.microsoft.com/zh-tw/entra/fundamentals/what-is-entra"
author:
  - "[[kenwith]]"
published:
created: 2026-04-23
description: "Microsoft Entra 產品家族介紹，包含入門連結。"
tags:
  - "clippings"
---
Microsoft Entra 是一系列身份與網路存取產品，協助組織實施 [零信任](https://learn.microsoft.com/zh-tw/security/zero-trust/zero-trust-overview) 安全策略。 使用 Microsoft Entra 來驗證身份、驗證存取條件、檢查權限、加密連線通道，並監控整個環境中的入侵情況。 Microsoft Entra 也整合 [Security Copilot](https://learn.microsoft.com/zh-tw/entra/security-copilot/security-copilot-in-entra) ，協助調查身份風險並利用 AI 排除存取問題。

## Microsoft Entra 產品系列

Microsoft Entra 產品家族涵蓋身份、存取、治理與安全。 它涵蓋員工、客戶、合作夥伴、工作負載及 AI 代理在任何雲端環境中的安全端對端存取。

### 建立 零信任 存取控制

#### Microsoft Entra ID

[Microsoft Entra ID](https://learn.microsoft.com/zh-tw/entra/fundamentals/what-is-entra) 是Microsoft Entra的基礎產物。 它是一項基於雲端的身份與存取管理服務，為使用者、裝置、應用程式和資源提供認證、政策執行與保護。 每個新的Microsoft Entra目錄都會包含一個初始網域名稱，例如 `contoso.onmicrosoft.com` 。 你也可以新增你組織的自訂網域名稱。

如果你是 **Microsoft 365、Azure或Dynamics CRM Online訂閱者** ，你已經在使用Microsoft Entra ID——每個租戶自動成為Microsoft Entra租戶。 你可以立即開始管理整合雲端應用程式的存取權限。

#### Microsoft Entra Domain Services

[Microsoft Entra Domain Services](https://learn.microsoft.com/zh-tw/entra/identity/domain-services/overview) 提供受管理網域服務，如群組政策、LDAP 及 Kerberos/NTLM 認證。 它是為無法使用現代認證方法的雲端舊應用設計的。

> **Scenario：** 擁有需要 Kerberos 認證服務的組織，可以建立一個受管理網域，Microsoft 負責部署與維護核心服務元件。

### 保護員工的存取

#### Microsoft Entra 私人存取

[Microsoft Entra 私人存取](https://learn.microsoft.com/zh-tw/entra/global-secure-access/overview-what-is-global-secure-access#microsoft-entra-private-access) 保護所有私有應用程式與資源的存取權限，包括企業網路與多雲環境。 遠端使用者可以從任何裝置和網路連接內部資源，無需 VPN。

**例如** ，員工可以在家或咖啡廳工作時，安全地存取企業網路印表機。

#### Microsoft Entra 網際網路存取

[Microsoft Entra 網際網路存取](https://learn.microsoft.com/zh-tw/entra/global-secure-access/overview-what-is-global-secure-access#microsoft-entra-internet-access) 確保存取所有網路資源，包括 SaaS 應用程式及Microsoft 365應用程式與資源。

> **情境：** 啟用網頁內容過濾功能，根據內容類別和網域名稱來規範網站存取。

#### Microsoft Entra ID 管理

[Microsoft Entra ID 控管](https://learn.microsoft.com/zh-tw/entra/id-governance/identity-governance-overview) 透過自動化存取請求、指派與審查，簡化身份與權限管理。 它還有助於透過身分生命週期管理來保護關鍵資產。

**例如** ，管理員可以自動將使用者帳號、群組和授權分配給新員工，並在員工離職時移除這些分配。

#### Microsoft Entra ID Protection

[Microsoft Entra ID Protection](https://learn.microsoft.com/zh-tw/entra/id-protection/overview-identity-protection) 偵測並回報基於身份的風險。 管理員可利用風險 [基礎條件存取政策](https://learn.microsoft.com/zh-tw/entra/id-protection/concept-identity-protection-policies) 等工具調查並自動修復風險。

> **情境：** 建立基於風險的條件存取政策，當登入風險等級為中高時，要求多重驗證。

#### Microsoft Entra 驗證識別碼

[Microsoft Entra 驗證識別碼](https://learn.microsoft.com/zh-tw/entra/verified-id/decentralized-identifier-overview) 是一項基於開放 [去中心化身份（DID）標準](https://learn.microsoft.com/zh-tw/entra/verified-id/verifiable-credentials-standards) 的憑證驗證服務。 組織可以向使用者發放可驗證的憑證——證明資訊真實性的數位簽章——使用者將憑證儲存在個人裝置中，並在需要時出示。

**例如** ，剛畢業的大學生可以請大學向其DID發放數位畢業證書，然後出示給潛在雇主，雇主能獨立核實發證人、發放時間及狀態。

### 保護客戶和合作夥伴的存取

#### Microsoft Entra 外部 ID

[Microsoft Entra 外部 ID](https://learn.microsoft.com/zh-tw/entra/external-id/external-identities-overview) 讓外部身份能安全存取商業資源與消費者應用程式。 它提供安全的方法，用於與商業夥伴及訪客在內部應用程式上的協作，並用於管理面向消費者的應用程式中的客戶身份與存取管理（CIAM）。

> **情境：** 為客戶設置自助註冊，使用一次性密碼或 Google 或 Facebook 的社群帳號登入網頁應用程式。

### 在任何雲端中安全存取

#### Microsoft Entra 工作負載 ID

[Microsoft Entra 工作負載 ID](https://learn.microsoft.com/zh-tw/entra/workload-id/workload-identities-overview) 是用於工作負載身份——需要驗證與授權政策的應用程式、服務與容器——的身份與存取管理解決方案。 它讓組織能利用自適應政策和自訂安全屬性來保護資源的存取。

**例如** ，GitHub Actions需要工作負載身份來存取Azure訂閱，以自動化、客製化及執行軟體開發工作流程。

### AI 代理程式的安全存取

#### Microsoft Entra Agent ID（Microsoft 進入代理 ID）

[Microsoft Entra Agent ID](https://learn.microsoft.com/zh-tw/entra/agent-id/what-is-microsoft-entra-agent-id) 是一個身份與安全框架，將Microsoft Entra能力延伸至 AI 代理。 隨著組織部署輔助性、自主且類使用者的代理，代理ID提供專門建構的身份結構，用於企業規模的認證、授權、治理及保護這些非人類身份。

> **情境：** 組織部署 AI 代理，代表使用者存取企業資料。 代理ID為每個代理提供受控的身份，強制執行最低權限存取，並維護代理行為的稽核軌跡。

## 準備您的環境

在部署 Microsoft Entra 之前，請依照安全最佳實務與標準配置您的基礎架構與流程。 以下文章提供架構、部署及營運指導：

- [架構](https://learn.microsoft.com/zh-tw/entra/architecture/architecture)
- [部署計畫](https://learn.microsoft.com/zh-tw/entra/architecture/deployment-plans)
- [作業參考](https://learn.microsoft.com/zh-tw/entra/architecture/ops-guide-intro)
- [作業指南](https://learn.microsoft.com/zh-tw/entra/architecture/security-operations-introduction)
- [建議的安全配置](https://learn.microsoft.com/zh-tw/entra/fundamentals/configure-security)

### 授權 Microsoft Entra 功能

Microsoft Entra 的功能有多種授權方式。 這些授權包括 Microsoft Entra ID 免費版、Microsoft Entra ID P1、Microsoft Entra ID P2、Microsoft Entra 套件、Microsoft Entra 外部 ID，Microsoft Entra 工作負載 ID、Microsoft Entra ID 控管 以及其他獨立產品。 Microsoft Entra也是像 Microsoft 365 和 Enterprise Mobility + Security 這類授權的一部分。 欲了解更多關於授權及可用選項的資訊，請參閱文章 [Microsoft Entra licensing](https://learn.microsoft.com/zh-tw/entra/fundamentals/licensing) 或 [Microsoft Entra 價格頁面](https://www.microsoft.com/security/business/microsoft-entra-pricing) 。

## 使用 Microsoft Entra 管理與開發

管理員可使用 Microsoft Entra 系統管理中心 及 Microsoft 圖形 API 來管理身份與網路存取資源。 開發者可以使用 [Microsoft 身分識別平台](#microsoft-identity-platform) 來建立身份感知應用程式。

### Microsoft Entra 管理中心

[Microsoft Entra 系統管理中心](https://entra.microsoft.com/) 是一個基於網頁的入口網站，可從單一介面配置和管理Microsoft Entra產品。

欲了解更多，請參閱 [概覽Microsoft Entra 系統管理中心](https://learn.microsoft.com/zh-tw/entra/fundamentals/entra-admin-center) 。

### Microsoft 圖形 API

[Microsoft 圖形 API](https://learn.microsoft.com/zh-tw/graph/api/overview) 自動化管理管理工作，如授權部署與使用者生命週期管理。

欲了解更多，請參閱 [使用 Microsoft Graph 管理 Microsoft Entra](https://learn.microsoft.com/zh-tw/graph/api/resources/identity-network-access-overview) 。

### 微軟識別平台

[Microsoft 身分識別平台](https://learn.microsoft.com/zh-tw/entra/identity-platform/v2-overview) 讓開發者能利用開源函式庫與符合標準的認證服務，為網頁、桌面及行動應用程式建立認證體驗。

若要開始開發，請參閱 [用戶入門](https://learn.microsoft.com/zh-tw/entra/identity-platform/v2-overview#getting-started) 。

## 下一步

- [Microsoft Entra 授權](https://learn.microsoft.com/zh-tw/entra/fundamentals/licensing) — 所有Microsoft Entra產品的詳細授權資訊。
- [身份與存取基礎](https://learn.microsoft.com/zh-tw/entra/fundamentals/identity-fundamental-concepts) — 理解核心身份概念。
- 報名參加 [免費30天Microsoft Entra ID P1或P2試用](https://azure.microsoft.com/trial/get-started-active-directory/) 。
- [比較Active Directory和Microsoft Entra ID](https://learn.microsoft.com/zh-tw/entra/fundamentals/compare) 。
- 從 [Microsoft Entra ID 開發者版](https://learn.microsoft.com/zh-tw/entra/identity-platform/) 開始著手。
- 在 [Microsoft 身分識別平台 詞彙表](https://learn.microsoft.com/zh-tw/entra/identity-platform/developer-glossary#tenant) 中尋找定義。

**注意：** 作者透過 AI 的協助創作了這篇文章。 [深入了解](https://learn.microsoft.com/principles-for-ai-generated-content)

---

## 其他資源

訓練

學習路徑

[Microsoft Entra 簡介 - Training](https://learn.microsoft.com/zh-tw/training/paths/describe-capabilities-of-microsoft-identity-access/?source=recommendations)

SC-900：Microsoft Entra 簡介

認證

[Microsoft 認證：身分識別與存取管理員助理 - Certifications](https://learn.microsoft.com/zh-tw/credentials/certifications/identity-and-access-administrator/?source=recommendations)

示範 Microsoft Entra ID 的功能，以現代化身分識別解決方案、實作混合式解決方案，以及實作身分識別治理。