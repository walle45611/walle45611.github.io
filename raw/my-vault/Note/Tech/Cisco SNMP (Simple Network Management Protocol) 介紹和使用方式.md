
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
- Get bulk : 需要MIB變數值的整個資料表
- set : 設定agent的資料

---

## SNMP version

- noAuthNoPriv : SNMP封包未驗證也沒有加密
- authNoPriv : SNMP有驗證沒加密
- authPriv : SNMP有驗證有加密
- community 可以分成兩類一種是read-only一種是read-write，其實就是兩組密碼，純明文傳輸的密碼。
- v1 和 v2基本是沒有安全性的，所以建議是切一段網段來做專門的網管網路
- v3 支持所有的安全認證hash encryption都有

|security model|security level|Authentication Strategy|Encryption type|
|---|---|---|---|
|SNMPv1|no AuthNoPriv|Community string|none|
|SNMPv2|no AuthNoPriv|Community string|none|
|SNMPv3|no AuthNoPriv|username|none|
|SNMPv3|authNoPriv|MD5 or SHA|none|
|SNMPv3|authPriv|MD5 or SHA|CBC-DES(DES-56)|

---

## SNMP設定

### SNMPv1

```Plain
SW(config)#access-list access-list-number permit ip-addr
SW(config)#snmp-server community community-string [ro | rw] [access-list-num]
SW(config)#snmp-server host host-address community-string [trap-type]
```

### SNMPv2c

```Plain
SW(config)#access-list access-list-number permit ip-addr
SW(config)#snmp-server community community-string [ro | rw] [access-list-num]
SW(config)#snmp-server host host-address [informs] version 2c community-string 
```

### SNMPv3

1. ACL允許存取或是察看MIB的IP位址
2. snmp-server view命令址定使用者的特定檢視。只有位在名為oid-tree的OID之下的MIB變數才能被使用者全組看到，ifadminStatus包含了介面管理狀態，ifDescr包含介面說明等等。可以重複的使用snmp-server view命令增加而外的oid，沒設定預設是所有內容都可以看到
    
    ```Plain
    SW(config)#snmp view view-name oid-tree
    ```
    
3. 使用snmp-server group命令設定group name，會針對隸屬於該群組的SNMPv3使用者設定安全等級原則。noauth、auth、priv，還沒有設定密碼的步驟
    
    ```Plain
    SW(config)#snmp-server group group-name v3 {noauth | auth |priv} [read read-view] [write write-view] [notify notify-view] [acl acl-num]
    ```
    
4. 指定snmp manager使用者名稱和密碼還有加密如果有snmp-server group priv這個選項
    
    ```Plain
    SW(config)#snmp-server user username group-name v3 auth {md5 | sha} auth-password priv {des | 3des | aes {128 | 192 |256} priv-password [access-list-num]}
    ```
    
5. 使用snmp-server host指定接收trap inform的SNMP manager
    
    ```Plain
    SW(config)#snmp-server host host-address [informs] version 3 {noauth | auth | priv} username [trap-type]
    ```
    

- case
    
    使用priv安全等級所有都有，mymonitory 透過snmpv3 inform發送警示訊息到192.168.3.99
    
    ```Plain
    sw(config)#access-list 10 permit 192.168.3.99
    sw(config)#access-list 10 permit 192.168.100.3
    sw(config)#snmp-server group NetOps v3 priv
    sw(config)#snmp-server user mymonitory NetOps v3 auth sha hello prive aes 128 hello 10
    sw(config)#snmp-server host 192.168.3.99 informs versin 3 prive mymonitory 
    ```