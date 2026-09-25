## 概觀

在很多的時候要做冗餘的設計，也就是多拉幾條線，但是這樣STP會把路徑blocking掉造成花費更多的錢買更多高級的線材就只是為了冗餘(Redundancy)的設計，這樣有點浪費了，所以有了ehterchannel這項技術的時候就可以將多條線邏輯成一條線路並將所有線材的STP status從blocking轉到forwarding狀態，這樣不只線材損壞時有多的備用路徑，還可以有更大的BW。

## EtherChannel限制

一般來說，必須是屬於通一個VLAN或是trunk的交換port必須楚瑜在trunking mode，且必須要有相同的navtive vlan。每個交換port在aggregate的時候speed和duplex需要一樣才能設定成功。

## EtherChannel流量分配

在EtherChannel中的資料以特定的方式在鏈路之間的分配，而是以hash演算法的結果，但附載並不需要等量的負載均衡，hash演算法可以採用source ip，或是destination ip，source ip and destination ip，source mac address、destination address組合或是TCP/UDP通訊編號。

若hash僅使用一個位置或通訊port，則交換器使用hash vlaue一個以上的LSB當作索引，並以索引每個訊框轉送至連結鏈路，使用兩個位址或port number計算hash則switch或TCP/UDP port number一個以上的LSB做XOR運算，將運算結果作為引索，並據此將每個data轉送到對應的鏈路

EtherChannel需要一個位元作為index。引索為0，則選擇鏈路0若為1則使用鏈路1。

間單來說就是幾個線路做成一個channel，如果是兩個做成一個channel表示只需要一位做xor，如果是4條線代表需要2位做xor，但是如果有其他設備干擾的話，最後1位的0與1來平均分配

|二進位| |
|---|---|
|位址 1: …..xxxxxxx0 位址 2: …..xxxxxxx0|…..xxxxxxx0 : 使用鏈路0|
|位址 1: …..xxxxxxx0 位址 2: …..xxxxxxx1|…..xxxxxxx1 : 使用鏈路1|
|位址 1: …..xxxxxxx1 位址 2: …..xxxxxxx0|…..xxxxxxx1 : 使用鏈路1|
|位址 1: …..xxxxxxx1 位址 2: …..xxxxxxx1|…..xxxxxxx0 : 使用鏈路0|

## EtherChannel load-balance設定

|method vlaue|hash input|hash運算|交換型號|
|---|---|---|---|
|src-ip|來源IP|位元|all|
|dst-ip|目的地IP|位元|all|
|src-dst-ip|來源與目的地IP|xor|all|
|src-mac|來源mac address|位元|all|
|dst-mac|目的地Mac addresss|位元|all|
|src-dst-mac|來源與目的地MAC|xor|all|
|src-port|來源port編號|位元|4500、6500|
|dst-port|目的port number|位元|4500、6500|
|src-dst-port|來源與目的port|xor|4500、6500|

- 設定方式
    
    ```Plain
    Switch(config)#port-channel load-balance methob
    ```
    
- 查看方式
    
    ```Plain
    SW#show etherchannel load-balance
    ```
    

---

## Etherchannel protocol

如果設定完成後，會有Po這個新的邏輯介面，這個接面做設定會動態的給實體介面。

| |被動，直到要求建立|主動建立通道|是否有協商封包|使否是公開標準|
|---|---|---|---|---|
|LACP|passive|active|yes|yes|
|PAgP|auto|desirable|yes|no cisco 私有|
|on|N/A|N/A|no||

### PAgP

| |desirable|auto|
|---|---|---|
|Desirable|V|V|
|Auto|V|X|

- command
    
    ```Plain
    Switch(config)#interface type member/module/number
    Switch(config-if)#channel-protocol pagp
    Switch(config-if)#channel-protocol number mode {no | {{auto | desirable} [non了
    ```
    
- example
    
    這樣代表了 g1/0/1-4做成pagp的通道，且模式是desirable 且不聆聽等待silent夥伴
    
    ```Plain
    SW(config)#port-channel load-balance src-dst-port
    SW(config)#interface range gig 1/0/1-4
    SW(config-if)#channel-protocol pagp
    SW(config-if)#channel-group 1 mode desirable non-silent
    ```
    

### LACP IEEE 802.3ad

| |Active|Passive|
|---|---|---|
|Active|V|V|
|Passive|V|X|

priority預設是32768最高可以到65535，priority越低用高優先權，最多支持16條線其中8條會hot standby，

- command
    
    ```Plain
    SW(config)#lacp system-priority priority
    SW(config)#interface type member/modul/number
    SW(config-if)#channel-protocol lacp
    SW(confgi-if)#channel-group number mode {on | passive | active}
    SW(config-if)#lacp prot-priority priority
    ```
    
- 範例
    
    ```Plain
    SW(config)#lacp system-priority 100
    SW(config)#interface range gig 1/0/1-4 ，gig 2/0/1-4
    SW(config-if)#channel-protocol lacp
    SW(config-if)#channel-group 1 mode active
    SW(config-if)#lacp port-priority 100
    SW(config-if)#exit
    ```
    

---

## EtherChannel error

假設有兩台switch和Switch a and Switch b，兩台都設定了etherChannel，但是插錯兩個介面，可能會產生bridge loop，因為STP設定不同調，為了降低錯誤預設系統中有開etherchannel guard

```Plain
sw(config)# [no] spanning-tree etherchannel guard missconfig
```

可以使用show interface status err-disable查看當前有沒有錯誤的port，如果發現可以使用，sh no sh重啟介面

---

## show etherchannel

|顯示功能| |
|---|---|
|目前etherchannel每個成員port狀態|show etherchannel summary|
|目前etherchannel每個成員port狀態|show etherchannel port|
|etherchannel變更的時間戳記|show etherchannel port-channel|
|使用哪種hash load balance|show etherchannel load-balance|
|每個switch port使用的load balance索引|show etherchannel port-channel|
|每個switch port上 etherChannel neighbor|show { lacp \| pagp } neighbor|
|LACP系統ID|show lacp sys-id|