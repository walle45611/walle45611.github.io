---
blog: true
blog_title: "Cisco SNMP (Simple Network Management Protocol) 介紹和使用方式"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-snmp-guide/
blog_tag: OS & Networking
blog_topic: Networking
---


## SNMP概述

在傳統網路中是一個很常見的網管協定，可以監控網路設備也可以看自己想看的資料，如果出現了問題，會主動推送一個事件給管理員，是使用udp 161 port

---

## SNMP三大組建

|component| |
|---|---|
|snmp manager|管理控制台netowrk management station (NMS)|
|snmp agent|網路設備|
|Management Information Base (MIB)|是一種樹狀結構的database，每個末梢節點是可以被查詢的資料，每個節點都會有一個name和id都是唯一的|

---

## SNMP的行為

![[Assets/Note/Tech/Cisco SNMP (Simple Network Management Protocol) 介紹和使用方式/01-SNMP的行為.png|01-SNMP的行為.png]]

- trap : 從agent推送資料到NMS上
- inform : 事件訊息發送到SNMP manager，而manager要回應agent確認收到
- get : 從agent得到資料
    - 如何在MIB中查詢
        
        會發送一個(Object ID)OID，其中Oid裡面放的資料是一串節點ID，如何訪問樹例如 : 1-102-2-101這個就是在MIB中裡面的節點ID，這串OID會發送給agent，這樣就可以得到想得到的資料
        
- Get Next : 需要跟隨初始get請求的下一個或連續的值
- GetBulk（SNMPv2c/v3）：一次取得多個後續 MIB 變數，常用於讀取表格資料。
- set : 設定agent的資料

---

## SNMP version

- noAuthNoPriv : SNMP封包未驗證也沒有加密
- authNoPriv : SNMP有驗證沒加密
- authPriv : SNMP有驗證有加密
- community 可以分成兩類一種是read-only一種是read-write，其實就是兩組密碼，純明文傳輸的密碼。
- v1 和 v2基本是沒有安全性的，所以建議是切一段網段來做專門的網管網路
- SNMPv3 的驗證與加密演算法依平台和軟體版本而異。新部署應優先使用 authPriv，並在設備支援時採用 SHA-2 與 AES；避免使用 MD5 或 DES 等舊式演算法。

|security model|security level|Authentication Strategy|Encryption type|
|---|---|---|---|
|SNMPv1|不適用|Community string|none|
|SNMPv2c|不適用|Community string|none|
|SNMPv3|noAuthNoPriv|username|none|
|SNMPv3|authNoPriv|MD5、SHA 或平台支援的 SHA-2|none|
|SNMPv3|authPriv|SHA 或平台支援的 SHA-2|平台支援的 AES|

---

## SNMP設定

### SNMPv1

```Plain
SW(config)#access-list access-list-number permit ip-address [wildcard-mask]
SW(config)#snmp-server community community-string [ro | rw] [access-list-num]
SW(config)#snmp-server host host-address community-string [trap-type]
```

### SNMPv2c

```Plain
SW(config)#access-list access-list-number permit ip-address [wildcard-mask]
SW(config)#snmp-server community community-string [ro | rw] [access-list-num]
SW(config)#snmp-server host host-address [informs] version 2c community-string 
```

### SNMPv3

1. ACL允許存取或是察看MIB的IP位址
2. snmp-server view 命令可限制管理者可讀取的 OID 範圍。要允許或排除 OID 子樹，需明確指定 included 或 excluded；view 的讀寫權限再由 group 設定。
    
    ```Plain
    SW(config)#snmp-server view view-name oid-tree {included | excluded}
    ```
    
3. 使用snmp-server group命令設定group name，會針對隸屬於該群組的SNMPv3使用者設定安全等級原則。noauth、auth、priv，還沒有設定密碼的步驟
    
    ```Plain
    SW(config)#snmp-server group group-name v3 {noauth | auth | priv} [read read-view] [write write-view] [notify notify-view] [access access-list]
    ```
    
4. 指定snmp manager使用者名稱和密碼還有加密如果有snmp-server group priv這個選項
    
    ```Plain
    SW(config)#snmp-server user username group-name v3 auth {md5 | sha} auth-password priv {des | 3des | aes {128 | 192 | 256}} priv-password [access access-list]
    ```
    
5. 使用 snmp-server host 指定接收 trap 或 inform 的 SNMP manager
    
    ```Plain
    SW(config)#snmp-server host host-address [traps | informs] version 3 {noauth | auth | priv} username [notification-type]
    ```
    

- case
    
    使用 priv 安全等級，mymonitor 透過 SNMPv3 inform 傳送通知到 192.168.3.99。實際環境請使用唯一且隨機的密碼。
    
    ```Plain
    sw(config)#access-list 10 permit 192.168.3.99
    sw(config)#access-list 10 permit 192.168.100.3
    sw(config)#snmp-server group NetOps v3 priv
    sw(config)#snmp-server user mymonitor NetOps v3 auth sha auth-secret priv aes 128 priv-secret access 10
    sw(config)#snmp-server host 192.168.3.99 informs version 3 priv mymonitor
    ```

演算法和命令選項依平台及版本而異；正式環境優先使用 authPriv，並採用設備支援的 SHA-2/AES。參考：[Cisco SNMP security guidance](https://www.cisco.com/c/en/us/support/docs/ip/simple-network-management-protocol-snmp/20370-snmpsecurity-20370.html)。
