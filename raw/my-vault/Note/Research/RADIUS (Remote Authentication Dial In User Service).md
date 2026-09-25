---
Created: 2023-02-27T20:50
---
- [[#概述]]
- [[#組成元件]]
- [[#可以使用驗證的Protocol]]

## 概述

RADIUS定義於IETF中的RFC2865，允許network Authtication server(NAS)對用戶進行**驗證、授權、計費(AAA)**。RADIUS server具體來說是透過LDAP進行驗證，RADIUS可以集中放置從儲存在LDAP 伺服器中，並由RADIUS server進行驗證用戶資訊，而減少管理的開銷，有可以使用過程安全更簡單。

## 組成元件

RADIUS是一種Client/Server的行定。RADIUS的client通常是，Router，Switch或是無線設備，如果NAS收到用戶的請求，就會傳送到指定的RADIUS，並將用戶的設定資訊傳給NAS，然後NAS接受或拒絕。

AAA，在RADIUS中驗證和授權是結合在一起的。如果發現用戶名，且密碼正確，RADUIS會回傳一個Access-Accept的回應，其中包括一些參數，以暴政對該用戶的存取。這些參數是在RADIUD中的設定，包括access type、protocol type、用戶指定該戶的IP address及u6ek7ACL會要NAS上應用的static route，另外還有一些數值

RADIUS計費允許session的開始和結束發送資料，表明在session期間使用的可能於安全或計費，time packet number，byte這些資訊會被收費

## 可以使用驗證的Protocol

RADIUS可以使用很多不同的用戶機制，除了LDAP還有PAP與PPP使用或是CHAP，還有Linux本地的資料庫(/etc/passwd)