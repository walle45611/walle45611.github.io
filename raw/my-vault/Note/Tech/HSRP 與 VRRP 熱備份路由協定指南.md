---
blog: true
blog_title: "HSRP 與 VRRP 熱備份路由協定指南"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/hsrp-vrrp-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

# HSRP (Hot Standby Routing Protocol)

## HSRP概述

HSRP是一種Cisco的專屬協定，他讓多台路由使用一個IP Gateway。RFC2281

為了共用一個閘道 IP，路由器會加入同一個 HSRP group。其中一台是 active router，一台是 standby router，其餘路由器處於 listen 狀態。HSRP 會定期傳送 hello 封包，讓群組成員監控 active router。

- HSRPv1 使用組播位址 224.0.0.2；HSRPv2 使用 224.0.0.102。

## HSRP vlan 關係

HSRPv1 的 group number 範圍是 0–255；HSRPv2 是 0–4095。Group number 在各介面上分別使用，因此不同 VLAN 介面可重複使用相同編號。

## HSRP 選舉

### 狀態設定priority

HSRP priority 範圍為 0–255，預設值為 100；數值越高，成為 active 的優先權越高。priority 相同時，介面 IP 位址較高者優先。

```Plain
SW(config-if)#standby group priority priority
```

### HSRP 狀態 和 timer

1. Disabled
2. Init
3. Learn
4. Listen
5. Speak
6. Standby
7. Active

同一個 group 的路由器都會接收 Hello；其中一台為 active，另一台被選為 standby，其餘處於 listen。HSRPv1 預設 Hello 為 3 秒、hold time 為 10 秒；standby 在 hold time 內未收到 active 的 Hello 才會接手。

- 修改timer，如果修改一台router的timer，應在其他HSRP路由器也要修改
    
    ```Plain
    SW(config-if)#standby group timers [msec] hellotime [msec] holdtime
    ```
    
    - 預設是以秒為單位如果要以毫秒就必須加上msec
    - hello : 1–254 秒；使用毫秒時為 15–999 毫秒。hold time 的毫秒範圍為 50–3000，且需符合平台的最小 hold time 規則。
    - Cisco 的預設 hold time 是 10 秒、hello 是 3 秒，並非嚴格的 3 倍限制。計時器應依網路延遲與收斂需求設定，且各設備需相容。
    - 命令範圍依平台而異；參考 [Cisco HSRP command reference](https://www.cisco.com/c/en/us/td/docs/routers/sdwan/command/iosxe/qualified-cli-command-reference-guide/m-hsrp-commands.html)。
- 開啟搶占功能
    
    HSRP 預設不搶占；active 故障後恢復時，即使 priority 較高，預設也不會自動取回 active。要讓較高 priority 的設備接手，需設定 preempt。
    
    如果某台路由器priority比較高的時候可以搶佔active role
    
    ```Plain
    SW(config-if)#standby group preempt [delay [minimum seconds] [reload seconds]]
    ```
    
    - minimum：介面啟用後，至少等待指定秒數才嘗試搶占 active。
    - reload：路由器重新載入後的搶占等待秒數，可留時間讓路由協定先收斂。

### 重新選舉

HSRP可偵測鍊路是否故障，並重新選舉的機制，讓其他路由器有機會接管active role。HSRP追中特定的介面時，只要一發現介面失效，就會根據設定的優先權調降priority，如果每發生一次介面故障就調降一次

```Plain
Sw(config-if)#standby group track type mod/num [decrementvalue]
```

- decrement value 預設為 10；追蹤會調整 HSRP priority，不會直接改變 HSRP 介面的狀態。當本機 priority 下降到低於另一台路由器，且對方已啟用 preempt，對方才會接手。參考：[Cisco HSRP tracking guidance](https://www.cisco.com/c/en/us/support/docs/ip/hot-standby-router-protocol-hsrp/13780-6.html)。
- 調降後誰可以成為active
    - 另一台路由器的HSRP優先權現在變得比較高了
    - 路由器在自己HSRP組態中設定了接管permit

## HSRP Gateway address

每台路由器都有一個共用的位置就是virtual Gateway，由HSRP確保位址的可用性，該位址稱為**HSRP位址或是standby位址，用戶端使用這個位址做default Gateway**

```Plain
SW(config-if)#standby group ip address [secondary]
```

- IP address介面上使用HSRP secondary可以設定次要閘道
- 如果要讓HSRP支援IPv6 
    
    ```Plain
    SW(config-if)#standby version 2
    SW(config-if)#standby group ipv6 autoconfig
    ```
    

> [!important] HSRPv1 的虛擬 MAC 格式為 0000.0c07.acxx；HSRPv2 為 0000.0c9f.fxxx。例如 HSRPv1 group 1 使用 0000.0c07.ac01。

  

## HSRP驗證

### 明文

HSRP 的文字驗證字串會以未加密方式放入封包，最多 8 個字元；傳統 IOS 預設字串為 cisco。不要把這項功能當成加密或強式身分驗證。

```Plain
SW(config-if)#standby group authentication string
```

### MD5

MD5訊息會隨著HSRP Group一起發送，跟路由協定驗證很像。

```Plain
SW(config-if)#standby group authentication md5 key-string [0|7] string
```

- key-string 使用參數 0 時以明文輸入；參數 7 只會做可逆混淆，不是加密。金鑰鏈可用來輪替多組金鑰。
- key chain設定能夠定義多個key跟HSRP關聯起來
    
    ```Plain
    SW(config)#key chain chain-name
    SW(config-keychain)# key key-number
    SW(config-keychain-key)#key-string [0|7] string
    SW(config)#interface type mod/num
    Sw(config-if)#standby group authentication md5 key-chain chain-name
    ```
    
## HSRP load balance
假設有vlan 10 20 30 40，如果可以選擇把vlan 10 20分到SW1，vlan 30 40分到SW2上，這樣就可以實現一個簡單的負載平衡了，也就是說SW1的group 10 priority 200設定為，SW2的group 10 priority 100，group 10裡面放了vlan 10 20，反之可以使用一樣的方法處理vlan 30 40

## SHOW

```Plain
show standby vlan 50 brief
```

---

# VRRP (Virtual Router Redundancy Protocol)

## VRRP 概述

VRRP 是標準化的第一跳備援協定。現行 VRRPv3 規格為 [RFC 9568](https://www.rfc-editor.org/rfc/rfc9568.html)，已取代 RFC 5798；它與 Cisco 專有的 HSRP 類似，但封包格式與互通方式不同。

- IPv4 組播位址為 224.0.0.18。

## 比較VRRP 和 HSRP

| |VRRP|HSRP|
|---|---|---|
|主要|master route|active|
|備用|backup|standby|
|group id 範圍|1~255|HSRPv1：0~255；HSRPv2：0~4095|
|priority 範圍|1~254（255 為位址擁有者，0 表示放棄）|0~255|
|Mac address|0000.5e00.01xx|HSRPv1：0000.0c07.acxx；HSRPv2：0000.0c9f.fxxx|
|Hello|1s|3s|

## command

### basic

|任務|指令語法|
|---|---|
|指定priority (default 100)|SW(config-if)#**vrrp** group **priority** level|
|修改hello timer (default 1s)|SW(config-if)#**vrrp** group **timers advertise** [msec] interval|
|master路由器得知通告的時間間隔|SW(config-if)#**vrrp** group **timers learn**|
|停用搶占（VRRP 預設啟用）|SW(config-if)#**no vrrp** group **preempt**|
|virtual gateway IP|SW(config-if)#**vrrp** group **ip** ip-address [**secondary**]|
|追蹤|SW(config-if)#**vrrp** group **track** object-number [**decrement** priority]|

### Show

|查看內容|命令|
|---|---|
|查看介面上的VRRP|SW#show vrrp [brief]|
|查看role|SW#show vrrp brief|
|完整設定|SW#show vrrp|

VRRPv3 不定義協定層驗證；不要把舊版或特定廠商的 authentication 命令當成 VRRPv3 通用設定。VRRPv3 預設允許較高 priority 的 backup 搶占。
