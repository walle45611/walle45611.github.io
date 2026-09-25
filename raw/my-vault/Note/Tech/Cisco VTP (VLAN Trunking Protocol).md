
![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/01-Cisco VTP (VLAN Trunking Protocol).png]]
## VTP working mode

1. Server mode ==(default)== :
    
    可以修改VLAN、創建、發送或轉發通告訊息，同步VLAN，將VLAN保存配置在NVRAM中
    
2. client mode
    
    不能創建、更改和刪除VLAN，轉發通告，同步VLAN配置，不會將VLAN配置保存在NVRAM中
    
3. transparent
    
    只能在本地交換機中創建，刪除和修改VLAN，轉發通告消息，不同步VLAN配置，將VLAN配置保存在NVRAM中
    
4. off
    
    不參與任何VTP活動。
    

---

## VTP version

- VTP版本1 : 不支持令牌環網路，透明模式檢測版本和域名
- VTP版本2 : 支持令牌環網路，支持TLV，透明模式不檢測版本
    
    |VTPv2的功能|說明|
    |---|---|
    |相依版本的transparent mode|不會檢查version number會立刻forwarding|
    |一致檢查|檢查從CLI或SNMP輸入的VTP及VLAN參數，避免傳播到其他switch|
    |支援Token Ring|可通告Token Ring交換及vlan|
    
- VTP版本3 : 支持擴展VLAN，支持創建和通告pVLAN，增強服務器認證，能與version 1 2互動，能夠基於介配置VTP
    - secondary server多了mode : 跟client功能一樣只是會將VLAN配置保存在NVRAM中，==默認是這個模式==
    - 可以將密碼加密
        
        ```Plain
        SW(config)#vtp password ccnp hidden
        ```
        

---

## VTP advertisement

### 摘要通告 (summary advertisement)

VTP server每隔300秒就會送出一次摘要通告，或是資料庫變動都會發送，包括VTP version、domain name、time tips、MD5 hasd code，摘要通告之後就是一個以上含有更多特定VLAN組態的子集通告

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/01-摘要通告 (summary advertisement).png|01-摘要通告 (summary advertisement).png]]

### 子集通告 (subset advertisement)

VTP server vlan 發生改變時才會發送subset advertisement。這些通告列出已執行的特定變更，vlan name change 、add vlan number、remove vlan、other…。

- subset advertisement
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/02-子集通告 (subset advertisement).png|02-子集通告 (subset advertisement).png]]
    
- VTP VLAN資訊欄位
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/03-子集通告 (subset advertisement) - VTP VL.png|03-子集通告 (subset advertisement) - VTP VL.png]]
    

  

### 用戶端要求

VTP client 本身沒有VLAN資訊所以向server提出要求

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/04-用戶端要求.png|04-用戶端要求.png]]

---

## VTP sync

無論交換器何時收到的VTP通告，如果reversion number比較高，都會被定義為先的VTP訊息，即使是最新的本vlan也會，複寫在nvram裡面的vlan database，如果要使reversion number重置開關機是行不通的

- 將VTP mode從原本的模式切換到transparent，再改回server
- 將VTP domain改成不存在的，再改成原本的

---

## VTP pruning

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/05-VTP pruning.png|05-VTP pruning.png]]

```Plain
SW(config)#vtp pruning
```

```Plain
SW(config)#int e0/0
SW(config-if)#switchport trunk pruning vlan {{{add | except | remove} vlan-list} | none}
```

## VTP刪除

- 因為vlan的資了會存在nvram裡面所以，他就算使用**erase startup-config**也清不掉可使用以下指令刪除，如果把舊有的設備沒有清除這個在同步VTP是一件非常危險的事情
    
    ```Plain
    SW#delete vlan.dat
    ```
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/06-VTP刪除.png|06-VTP刪除.png]]
    

## VTP的訊息

```Plain
SW1#sh vtp status
VTP Version capable             : 1 to 3
VTP version running             : 3
VTP Domain Name                 : ccna
VTP Pruning Mode                : Disabled
VTP Traps Generation            : Disabled
Device ID                       : aabb.cc80.1000

Feature VLAN:
--------------
VTP Operating Mode                : Server
Number of existing VLANs          : 7
Number of existing extended VLANs : 0
Maximum VLANs supported locally   : 4096
Configuration Revision            : 1   修訂編號
Primary ID                        : aabb.cc80.1000
Primary Description               : SW1
MD5 digest                        : 0x1B 0x0B 0x8A 0x94 0x55 0x2D 0x98 0x41
                                    0x98 0x90 0x46 0x93 0x89 0xBA 0x2B 0x9C


Feature MST:
--------------
VTP Operating Mode                : Transparent


Feature UNKNOWN:
--------------
VTP Operating Mode                : Transparent
```

- 用於通知鄰接的Catalyst Switch目前的VTP domain和設定和Configuration Revision
- 每5 min會發送一次會總通告消息，配置改變的時候發送通告
- switch會比對封包的VTP domain，如果不同則忽略
- Configuration Revision ，代表說新增一次VLAN，或是一次新增多個就會增加1
    - 可以透過修改成transparent mode就會變成0
- VTP domain相同將對比Configuration Revision ，如果自己的Configuration Revision 比較大就忽略，如果比較小就通告
- MD5 digest 會計算vlan.dat的md5 value

---

### case1 VTP version 2

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/07-case1 VTP version 2.png|07-case1 VTP version 2.png]]

> [!important] SW1是server，SW2是Client，並且使用trunk將兩台SW串在一起

SW1

```Plain
SW1(config)#interface Ethernet0/0
SW1(config-if)#switchport trunk encapsulation dot1q
SW1(config-if)#switchport mode trunk
SW1(config)#vtp domain ccna
SW1(config)#vtp password ccna
```

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/08-case1 VTP version 2.png|08-case1 VTP version 2.png]]

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/09-case1 VTP version 2.png|09-case1 VTP version 2.png]]

SW2

```Plain
SW2(config)#interface Ethernet0/0
SW2(config-if)#switchport trunk encapsulation dot1q
SW2(config-if)#switchport mode trunk
SW2(config)#vtp mode client
SW2(config)#vtp domain ccna
SW2(config)#vtp password ccna
```

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/10-case1 VTP version 2.png|10-case1 VTP version 2.png]]

  

### case2 VTP version3

![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/07-case1 VTP version 2.png|07-case1 VTP version 2.png]]

  

SW1

- 更改版本
    
    ```Plain
    SW1(config)#vtp version 3
    ```
    
- 會看到新增VLAN時會錯誤
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/11-case2 VTP version3 - 會看到新增VLAN時會錯誤.png|11-case2 VTP version3 - 會看到新增VLAN時會錯誤.png]]
    
    ```Plain
    SW1#vtp primary vlan
    ```
    
- 將密碼加密MD5
    
    ```Plain
    SW1(config)#vtp password ccna hidden
    ```
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/12-case2 VTP version3 - 將密碼加密MD5.png|12-case2 VTP version3 - 將密碼加密MD5.png]]
    
- show
    
    ```Plain
    SW1#sh vtp status
    VTP Version capable             : 1 to 3
    VTP version running             : 3
    VTP Domain Name                 : ccna
    VTP Pruning Mode                : Disabled
    VTP Traps Generation            : Disabled
    Device ID                       : aabb.cc80.1000
    
    Feature VLAN:
    --------------
    VTP Operating Mode                : Primary Server
    Number of existing VLANs          : 7
    Number of existing extended VLANs : 0
    Maximum VLANs supported locally   : 4096
    Configuration Revision            : 1
    Primary ID                        : aabb.cc80.1000
    Primary Description               : SW1
    MD5 digest                        : 0x1B 0x0B 0x8A 0x94 0x55 0x2D 0x98 0x41
                                        0x98 0x90 0x46 0x93 0x89 0xBA 0x2B 0x9C
    
    
    Feature MST:
    --------------
    VTP Operating Mode                : Transparent
    
    
    Feature UNKNOWN:
    --------------
    VTP Operating Mode                : Transparent
    ```
    
- 建議將primary server更改完後避免primary server，防止後期動到vlan，方法為先改成transparent在改成server
    
    ```Plain
    SW1(config)#vtp mode transparent
    SW1(config)#vtp mode server
    ```
    
    ```Plain
    SW1#sh vtp status
    VTP Version capable             : 1 to 3
    VTP version running             : 3
    VTP Domain Name                 : ccna
    VTP Pruning Mode                : Disabled
    VTP Traps Generation            : Disabled
    Device ID                       : aabb.cc80.1000
    
    Feature VLAN:
    --------------
    VTP Operating Mode                : Server
    Number of existing VLANs          : 7
    Number of existing extended VLANs : 0
    Maximum VLANs supported locally   : 4096
    Configuration Revision            : 1
    Primary ID                        : aabb.cc80.1000
    Primary Description               : SW1
    MD5 digest                        : 0x1B 0x0B 0x8A 0x94 0x55 0x2D 0x98 0x41
                                        0x98 0x90 0x46 0x93 0x89 0xBA 0x2B 0x9C
    
    
    Feature MST:
    --------------
    VTP Operating Mode                : Transparent
    
    
    Feature UNKNOWN:
    --------------
    VTP Operating Mode                : Transparent
    ```
    

SW2

- 更改版本
    
    ```Plain
    SW2(config)#vtp version 3
    ```
    
- md5加密密碼
    
    ```Plain
    SW2(config)#vtp password ccna hidden
    ```
    
    ![[Assets/Note/Tech/Cisco VTP (VLAN Trunking Protocol)/13-case2 VTP version3 - md5加密密碼.png|13-case2 VTP version3 - md5加密密碼.png]]
    

---