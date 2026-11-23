---
blog: true
blog_title: "Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-ip-sla-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

## 為什麼需要IP SLA

- 使用ping-echo檢測鏈路的是否連通，或是支持多種協定TCP，UDP檢測鏈路
- 可搭配 track、PBR、HSRP 等功能，把探測狀態接到路由或備援決策；需另行設定對應功能
- 早期是RTR來檢測，Cisco IOS也可以用rtr指令另設定RTR
- 可用 reaction 設定在超過門檻時採取動作，例如產生 SNMP trap；可用 track object 將 operation 狀態提供給 HSRP 或路由功能
- 收集網路VoIP品質

---

## 支援協定

- DHCP
- TCP
- ICMP
- UDP
- DNS
- RTP (VoIP)
- HTTP
- FTP

可用 operation 與參數會依 IOS/IOS XE 平台和版本而異。

---

## command

### Basic command

|命令|描述|
|---|---|
|ip sla operation-number|進入 IP SLA operation 設定模式。|
|icmp-echo destination-ip [source-ip source-ip-address \| source-interface interface-name]|指定 ICMP Echo operation 的目的地與來源。|
|frequency seconds|設定 operation 的執行間隔，於 operation 設定模式輸入。|
|ip sla schedule operation-number [life {forever \| seconds}] [start-time {now \| pending \| hh:mm:ss ...}]|在全域設定模式安排 operation。life 是 operation 執行的期限，不是測試間隔。|
|ip sla responder|在支援的 Cisco IOS 設備上啟用 responder；只有特定 operation（例如 UDP jitter）需要目的設備啟用 responder。|

### 驗證MD5

```Plain
SW(config)#key chain chaine-name
SW(config-keychain)#key key-number
SW(config-keychain-key)#key-string string
SW(config-keychain-key)#exit
SW(config-keychain)#exit
SW(config)#ip sla key-chain chain-name
```

### 設定步驟

1. 在來源交換機設定新的IP SLA
    
    ```Plain
    SW(config)#ip sla operation-number
    ```
    
2. 設定運作類型
    
    ```Plain
    SW(config-ip-sla)#test-type parameters.............
    ```
    
    - test-type：dhcp、dns、ethernet、ftp、http、icmp-echo、mpls、path-echo、path-jitter、slm、tcp-connect、udp-echo、udp-jitter；實際支援類型依平台和版本而異。
    - 以icmp-echo為例
        
        ```Plain
        SW(config-ip-sla)#icmp-echo destination-ip-addr [source-ip-addr]
        ```
        
    - udp-jitter
        
        ```Plain
        SW(config-ip-sla)#udp-jitter destination-ip-addr dest-udp-port [source-ip source-ip-addr] [source-port src-udp-port] [num-packets number-of-packets] [interval interpacket-interval]
        ```

        - `num-packets` 是每次 operation 的封包數；`interval` 是封包間隔，單位為毫秒。
        
    - udp-jitter VoIP
        
        ```Plain
        SW(config-ip-sla)#udp-jitter destination-ip-addr dest-udp-port codec {g711alaw | g711ulaw | g729a}
        ```

        codec 模式的預設值依編碼而定；此處列出的 codecs 在 Cisco IOS command reference 中預設為每次 1000 個封包、間隔 20 ms。參考：[Cisco UDP jitter codec defaults](https://www.cisco.com/c/en/us/td/docs/ios/ipsla/command/reference/sla_book/sla_05.html)。
        
3. 設定IP SLA 頻率
    
    ```Plain
    SW(config-ip-sla)#frequency seconds
    ```
    
    - 預設以60s為間隔
    - timeout 是等待回應的最長時間；frequency 則是兩次 operation 開始的間隔。
4. 安排測試
    
    ```Plain
    SW(config)#ip sla schedule sla-ops-number [life {forever | seconds}] [start-time {pending | now | hh:mm[:ss]}]
    ```
    
    - 不指定 life 時，operation 預設執行 3600 秒後停止；設定 life forever 才會持續執行。frequency 決定每次測試的間隔。

### SHOW

- 可以看到ip sla的設定
    
    ```Plain
    show ip sla configuration
    ```
    
- 顯示最近的測試統計
    
    ```Plain
    show ip sla statistics [operation-number]
    ```

一般 ICMP Echo operation 不需要在目的端啟用 IP SLA responder；UDP jitter 等特定 operation 才需要。參考：[Cisco IP SLA scheduling](https://www.cisco.com/c/en/us/td/docs/ios/ipsla/command/reference/sla_book/sla_02.html)、[Cisco UDP jitter operation](https://www.cisco.com/c/en/us/td/docs/routers/ios-xe/ip-addressing/ip-addressing/m_sla_udp_jitter-xe.html)。
    

---

## case

### case R1檢測S1

![[Assets/Note/Tech/Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定/01-case R1檢測S1.png|01-case R1檢測S1.png]]

```Plain
R1(config)#ip sla 11
R1(config-ip-sla)#icmp-echo 10.1.3.1 source-ip 10.1.1.254
R1(config-ip-sla)#frequency 60
R1(config-ip-sla)#exit
R1(config)#ip sla schedule 11 start-time now life forever
#route map
R1(config)#access-list 101 permit ip host 10.1.1.254 host 10.1.3.1
R1(config)#ip local policy route-map PC2-over-low-route
R1(config)#end
```

- show ip sla configuration
    
    ```Plain
    R1#show ip sla configuration
    IP SLAs Infrastructure Engine-III
    Entry number: 11
    Owner:
    Tag:
    Operation timeout (milliseconds): 5000
    Type of operation to perform: icmp-echo
    Target address/Source address: 10.1.3.1/10.1.1.254
    Type Of Service parameter: 0x0
    Request size (ARR data portion): 28
    Verify data: No
    Vrf Name:
    Schedule:
       Operation frequency (seconds): 60  (not considered if randomly scheduled)
       Next Scheduled Start Time: Start Time already passed
       Group Scheduled : FALSE
       Randomly Scheduled : FALSE
       Life (seconds): Forever
       Entry Ageout (seconds): never
       Recurring (Starting Everyday): FALSE
       Status of entry (SNMP RowStatus): Active
    Threshold (milliseconds): 5000
    Distribution Statistics:
       Number of statistic hours kept: 2
       Number of statistic distribution buckets kept: 1
       Statistic distribution interval (milliseconds): 20
    Enhanced History:
    History Statistics:
       Number of history Lives kept: 0
       Number of history Buckets kept: 15
       History Filter Type: None
    ```
    
- show ip sla statistics 11
    
    ```Plain
    R1#show ip sla statistics 11
    IPSLAs Latest Operation Statistics
    
    IPSLA operation id: 11
            Latest RTT: 14 milliseconds
    Latest operation start time: 07:44:34 UTC Tue Jun 21 2022
    Latest operation return code: OK
    Number of successes: 13
    Number of failures: 0
    Operation time to live: Forever
    ```
    
    - 已經經過了13次的間隔，都成功，延遲時間RTT(round trip time)

---

### track SLA

- 可以追蹤HSRP
    
    SWA和SWB設定HSRP，共用gateway 192.168.1.1。SWA比SWB較高的優先權，所以是現在用的閘道，每5就去ping一次192.168.70.1的上層路由器如果路由器沒有反應就，減去30優先權。
    
    ```Plain
    SWA(config)#ip sla 10
    SWA(config-ip-sla)#icmp-echo 192.168.70.1
    SWA(config-ip-sla)#frequency 5
    SWA(config)#ip sla schedule 10 life forever start-time now
    SWA(config)#track 1 ip sla 10 reachability
    SWA(config)#int vlan 10
    SWA(config-if)#ip addr 192.168.1.3 255.255.255.0
    SWA(config-if)#standby 1 priority 120
    SWA(config-if)#standby 1 track 1 decrement 30
    SWA(config-if)#standby 1 preempt
    SWA(config-if)#no sh
    ```
    
- 路由追蹤
    
    可以追蹤路徑假設R4設定ACL不允許R1 S1/1介面訪問或是某種原因讓，主要鏈路失效S1/1，就會將路由表自動刪除到背後，當這條鏈路回復時自己會在跑出來
    
    ```Plain
    R1(config)#track 2 ip sla 11 state
    R1(config-track)#delay up 90 down 90
    R1(config-track)#exit
    R1(config)#ip route 10.1.234.0 255.255.255.0 s1/1 track 2
    ```
    
    - 鏈路狀態OK時
        
        ![[Assets/Note/Tech/Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定/02-track SLA - 鏈路狀態OK時.png|02-track SLA - 鏈路狀態OK時.png]]
        
        ![[Assets/Note/Tech/Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定/03-track SLA - 鏈路狀態OK時.png|03-track SLA - 鏈路狀態OK時.png]]
        
    - 鏈路不OK時
        
        ![[Assets/Note/Tech/Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定/04-track SLA - 鏈路不OK時.png|04-track SLA - 鏈路不OK時.png]]
        
        ![[Assets/Note/Tech/Cisco IP SLA (IP Service-Level Agreement) 使用方式和設定/05-track SLA - 鏈路不OK時.png|05-track SLA - 鏈路不OK時.png]]
