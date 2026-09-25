# HSRP (Hot Standby Routing Protocol)

## HSRP概述

HSRP是一種Cisco的專屬協定，他讓多台路由使用一個IP Gateway。RFC2281

基本上，為了使用相同的閘道IP，都會被分配到同一個HSPR Grouop，其中一台是active HSRP router，另一台做standby HSRP router，其他路由器處於聆聽的狀態，HSRP 會定期發送hello封包以便知道active路由的存在

- 224.0.0.2

## HSRP vlan 關係

可以分配0~255任何的群組編號給HSRP Group，==如果再幾個VLAN介面上，設定HSRP Group，可以將群組編號設定為VLAN相同。==大多數只支持16個群組編號，所以有了另一種做法，讓妹個VLAN介面的群組編號都是相同(即1)，==因為HSRP群組只有在本身介面上有意義，完全不用擔心編號用完，也就是說vlan 11的HSRP 1跟vlan 10 HSRP 1是不一樣的==

## HSRP 選舉

### 狀態設定priority

HSRP priority 是根據群組設定的(0~255)來選舉，預設100，最高255，越高越有優先權成為active，如果priority相同或，HSRP介面的IP最大成為active路由器。

```Plain
SW(config-if)#standby group priority priority
```

### HSRP 狀態 和 timer

1. Disabled
2. Init
3. Listen
4. Speak
5. Standby
6. Active

只有standby優先權次高的路由器聽來active router的hello，預設==hello interval 3s==。如果再==Hold-Time (default 10s，或是Hello timer 3倍)==沒有收到hello就代表active router死了，stanby變成actvie。其他listen狀態的router會成為standby狀態router。

- 修改timer，如果修改一台router的timer，應在其他HSRP路由器也要修改
    
    ```Plain
    SW(config-if)#standby group timers [msec] hello [msec] holdtime
    ```
    
    - 預設是以秒為單位如果要以毫秒就必須加上msec
    - hello : 1~254s 或是 50~3000 msec
    - ==Hold-time至少為3倍hello==
- 開啟搶占功能
    
    ==如果active故障後，修復就算是priority比較高，也不會進入active狀態，也就是說最先啟動的路由器就是active狀態。==
    
    如果某台路由器priority比較高的時候可以搶佔active role
    
    ```Plain
    SW(config-if)#standby group preempt [delay [minimum seconds] [reload seconds]]
    ```
    
    - minimum : 強制路由器嘗試去帶優先權較低的active路由器以前的等待時間，從成為active role算起。
    - reload : 指定路由器重新啟動後等待秒數。如果路由協定需要完成收斂這個方法很方便。

### 重新選舉

HSRP可偵測鍊路是否故障，並重新選舉的機制，讓其他路由器有機會接管active role。HSRP追中特定的介面時，只要一發現介面失效，就會根據設定的優先權調降priority，如果每發生一次介面故障就調降一次

```Plain
Sw(config-if)#standby group track type mod/num [decrementvalue]
```

- decrementvalue預設情況為10，追蹤並不會引響HSRP介面的狀態而是特定介面的狀態會引響路由器本身的資格。
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
    SW(config-if)#standby ipv6 autoconfig
    ```
    

> [!important] 可能會很好奇閘道這個位址的mac address是什麼，所以HSRP定義了0000.0c07.acxx這個mac address，例如HSRP group 1 mac address 0000.0c07.ac01

  

## HSRP驗證

### 明文

HSRP封包發送時包含金鑰(8 bit)，這是一種簡單驗證的方法，cisco設備預設使用cisco字串做驗證

```Plain
SW(config-if)#standby group authentication string
```

### MD5

MD5訊息會隨著HSRP Group一起發送，跟路由協定驗證很像。

```Plain
SW(config-if)#standby group authentication md5 key-string [0|7] string
```

- 預設情況，金鑰字串做多為64 bit是以明文的方式指定的，就是使用參數0。輸入密碼後戶已加密的方式存在config裡面，參數可以設定為7
- key chain設定能夠定義多個key跟HSRP關聯起來
    
    ```Plain
    SW(config)#key chain chain-name
    SW(config-keychain)# key key-number
    SW(config-keychain-key)#key-string [0|7] string
    SW(config)#interface type mod/num
    Sw(config-if)#standby group authentication mdb5 key-chain chain-name
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

VRRP是由RFC2338所定義的，可取代HSRP，VRRP與HSRP類似，只是術語稍微不同。

- 224.0.018

## 比較VRRP 和 HSRP

| |VRRP|HSRP|
|---|---|---|
|主要|master route|active|
|備用|backup|standby|
|group id 範圍|0~255|1~16|
|priority 範圍|1~254|0~255|
|Mac address|000.5e00.01xx|0000.0c07.acxx|
|Hello|1s|3s|

## command

### basic

|任務|指令語法|
|---|---|
|指定priority (default 100)|SW(config-if)#**vrrp** group **priority** level|
|修改hello timer (default 1s)|SW(config-if)#**vrrp** group **timers advertise** [msec] interval|
|master路由器得知通告的時間間隔|SW(config-if)#**vrrp** group **timers learn**|
|停用接管 (預設為接管) 開啟搶占|SW(config-if)#**no vrrp** group **preempt**|
|對通告進行驗證|SW(config-if)#**vrry** group **authentication** string|
|virtual gateway IP|SW(config-if)#**vrrp** group **ip** ip-address [**secondary**]|
|追蹤|SW(config-if)#**vrrp** group **track** object-number [**decrement** priority]|

### Show

|查看內容|命令|
|---|---|
|查看介面上的VRRP|SW#show vrrp [brief]|
|查看role|SW#show vrrp brief|
|完整設定|SW#show vrrp|
