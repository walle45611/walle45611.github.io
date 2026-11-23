---
blog: true
blog_title: "Cisco SPAN (Switch port Analysis) 簡介和使用方式"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-span-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

- [為什麼需要](#為什麼需要)
- [SPAN分類](#span分類)
- [Local SPAN](#local-span)
    - [Local SPAN設定](#local-span設定)
    - [Remote SPAN](#remote-span)
    - [show](#show)

## 為什麼需要

假設下層交換設備有問題，可能需要網路分析器來收集資料，可以拓透過端口鏡射也就是SPAN，也就是說可以直接把某個switch port反射到某個主機進行分析，分析某個switch port傳出的數據。

## SPAN分類

- local SPAN : SPAN來源與目的地均作路在本地交換機。來源或是多個port。
- remote SPAN : SPAN來源與目的地在不同的交換機上。鏡射資料流是金過特殊用途的vlan傳送到目的地交換機進行複製資料。

## Local SPAN

部分平台可用 Port-channel 作為 SPAN source；也可指定 VLAN 監控該 VLAN 的流量。支援的來源與目的組合依型號和版本而異。SPAN 複製封包會使用交換器資源；若目的埠頻寬不足，鏡像封包可能被丟棄，原始流量通常仍照常轉送。

若來源流量總量超過目的埠可承載的頻寬，目的埠可能無法收到全部鏡像封包。

可以支援的SPAN數量是由switch的型號而定的。

![[Assets/Note/Tech/Cisco SPAN (Switch port Analysis) 簡介和使用方式/01-Local SPAN.png|01-Local SPAN.png]]

### Local SPAN設定

1. 設定SPAN session，指定SPAN session data來源
    
    ```Plain
    SW(config)#monitor session session-number source
    {interface interface-id | vlan vlan-id} {rx | tx | both}
    ```
    
    - 如果需要多個SPAN來源編號可以重複使用
    - 設定VLAN時需要物理界面，不是SVI
    - 可以使用雙向的both，也可以只有接收rx，或是傳送tx
2. 設定SPAN目的地
    
    ```Plain
    SW(config)#monitor session session-number destination interface interface-id [encapsulation replicate]
    ```
    
    - 每個 session 可用的目的埠數量、目的埠能否共用，依平台與版本而異；許多 Catalyst 平台支援在同一 session 設定多個目的埠。
    - encapsulation replicate 用來保留來源封包的封裝形式，不是啟用 CDP、STP、VTP 等控制封包鏡像的開關；控制封包是否可作為來源需查平台文件。
3. 以上面的圖
    
    ```Plain
    SW(config)#monitor session 1 source interface g0/1 both 
    SW(config)#monitor session 1 destination interface g0/3 
    ```
    

### Remote SPAN

RSPAN需要設定特殊用途的VLAN成為RSPAN VLAN，如果VTP server上設定RSPAN VLAN，VTP會傳播給其他Switch，如果不是使用VTP就要確保每個switch都有設定到，RSPAN VLAN也會被VTP pruning引響

1. 設定RSPAN VLAN
    
    ```Plain
    SW(config)#vlan vlan-id
    SW(config)#remote-span
    ```
    
2. 設定source port and dest port
    
    ```Plain
    SW(config)#monitor session session-number source 
    {interface interface-id | vlan vlan-id} {rx | tx | both}
    ```
    
    ```Plain
    SW(config)#monitor session session-number destination remote vlan rspan-vlan-id
    ```
    

### show

```Plain
show run | in monitor
```

```Plain
show monitor 
```
