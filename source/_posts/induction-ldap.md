---
title: 簡單認識 LDAP：讓網路認證更輕量
tags:
  - LDAP
categories:
  - IT 技術
  - LDAP
abbrlink: d737
date: 2024-08-30 00:00:00
---

## LDAP 的起源

你可能聽過 LDAP 這個詞，但它到底是什麼？讓我們從頭說起。LDAP 的前身是 X.500 Directory Service，這是一個在應用層運行的複雜協定。由於 X.500 的封包大小會隨著資料量的不同而變化，因此管理起來有些麻煩。為了解決這個問題，LDAP 應運而生，它是一個更輕量的版本，只有一個簡單的協定定義，因此在運作上更為靈活。

<!--more-->

![圖 1. : Blog Image](https://imgur.com/pZv36P7.png)

LDAP 是一個基於訊息的協定，採用主從式架構，定義於 [RFC2251](https://www.ietf.org/rfc/rfc2251.txt) 中。它的非同步特性意味著用戶端可以同時發出多個請求，而伺服器則可以根據重要性來回應，不需要按順序進行，這讓系統運行更加有效率。



## 為什麼選擇 LDAP？

LDAP 不僅讓帳號和密碼管理變得更加簡單，還具有很多其他優點：

- **超越區域網路的限制**：無論你在哪裡，都能通過 LDAP 進行遠端管理。
- **主從架構**：LDAP 支持多子系統分散存取，並且可以用於備份，這讓它在可靠性上更勝一籌。
- **多用途**：LDAP 不僅能用來管理帳號密碼，還能應用於 DNS、RADIUS 等多種情境。
- **跨平台**：無論是 Linux、Windows 還是其他系統，LDAP 都能輕鬆應用。
- **基於 TCP/IP**：LDAP 運行在標準的 TCP/IP 協定之上，這意味著它幾乎可以在任何網路環境中運行。

## LDAP 能做什麼？

LDAP 在很多領域都有應用，以下是一些常見的例子：

- **郵件伺服器**：LDAP 能管理用戶的郵件帳號和密碼，讓郵件系統更容易維護。
- **Samba 帳號**：在企業內部，LDAP 可以管理 Samba 服務器的帳號，方便跨平台共享文件。
- **郵件查詢**：使用 LDAP，你可以輕鬆查詢和管理大量的郵件帳號資訊。

一些比較特別的應用包括：

- **FreeRadius with LDAP**：用於網路認證，確保只有合法用戶能夠連接網路。
- **Squid 認證**：通過 LDAP，Squid 代理伺服器可以輕鬆管理用戶的上網權限。
- **AP with LDAP**：使用 LDAP 管理無線接入點的認證機制，保證網路安全。
- **DNS LDAP**：利用 LDAP 來管理 DNS 伺服器，簡化配置和管理。

## LDAP 的組件和結構

![圖 2. : LDAP 的組件和結構 ](https://imgur.com/diYeWxm.png)

### 資料結構

![圖 3. : LDAP 的資料結構](https://imgur.com/J91WtRS.png)

LDAP 的資料結構其實是由三個主要部分組成的：`Schema`、`Object Class` 和 `Attribute Type`。

- **Schema**：可以想像成 LDAP 的字典或溝通語言。沒有 Schema，LDAP 就像失去了方向，無法正確運作。
- **Object Class**：這是一組 `Attribute Type` 的集合，每一種 `Object Class` 會定義哪些 `Attribute Type` 是必要的，哪些是可選的。
- **Attribute Type**：用來描述資料的內容，以鍵值（Key-Value）對的形式表示。

### LDAP 的樹狀結構

![圖 4. : LDAP 的樹狀結構](https://imgur.com/24T9wDP.png)

LDAP 的結構就像是一棵樹，每個節點都是一個 `Entry`，每個 `Entry` 只能使用一種 `Object Class` 來表達。而這些 `Entry` 可以無限層級地分層，形成複雜的樹狀結構。

- **父節點**：整棵樹的根源，最頂端的節點。
- **Pipeline**：連接各個節點的關係線。
- **層級**：可以根據需求無限地分層，各節點可以有資料也可以沒有資料。

## LDAP 查詢與帳號管理

在了解 LDAP 之後，你可能會好奇它與帳號有什麼關係。LDAP 通常與 PAM 機制結合使用，透過 nss_ldap 這個橋樑來管理帳號。使用 LDAP 管理帳號的好處是，它可以跨網路進行管理，並且支持快速複製和轉移。此外，LDAP 還可以實現單一帳號登入（Single Sign On，SSO），這讓用戶只需一組帳號密碼就能訪問多個系統。

### SSO（單一登入系統）

要在企業中實現 SSO，首先需要搭建 LDAP 作為認證授權中心，所有系統都通過 LDAP 來驗證和授權。常見的 LDAP 產品有 Novell 和 Microsoft AD，當然也可以用其他技術來實現類似功能。

## 推薦連結

- [StudyAreaTw](https://www.youtube.com/user/StudyAreaTw/videos)
- [LDAP 基礎查詢語法](https://docs.microsoft.com/zh-TW/previous-versions//dd159860(v=technet.10)?redirectedfrom=MSDN)