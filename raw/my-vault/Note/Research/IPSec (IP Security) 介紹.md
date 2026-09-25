### 先備知識

### IPSec框架

- 加密 DES/3DES/AES...
- 驗證 MD5/SHA-1... 完整性
- 封裝協議 ESP/AH... 在網路層如何封裝的形式
- 模式 Transport/Tunnel
- 密鑰有效 3600/1800
- 保護網路層以下的層不保護IP本身

## IPSec組成

![[Assets/Note/Research/IPSec (IP Security) 介紹/01-IPSec組成.png|01-IPSec組成.png]]

- 安全協議: AH ESP 協商加密方法例如是用Des加密用和那種Hash算法
- 密鑰管理:ISAKMP IKE SKEME
- 算法:用於加密和身分驗證

---

### IPSec Header AH/ESP/AH-ESP

AH 不能穿過Nat，但ESP卻可以，所以ESP最常用，AH-ESP太慢了沒有必要使用

- AH : 提供hash、身分驗證、保護IP header，不提供data加密
- ESP : 提供data加密，hash，身分驗證，不保護IP header
- Transport Mode
    
    - 保留原始IP header
    - 效率高
    - 無法再internet傳輸
    
    ![[Assets/Note/Research/IPSec (IP Security) 介紹/02-IPSec Header AH ESP AH-ESP - 無法再inte.png|02-IPSec Header AH ESP AH-ESP - 無法再inte.png]]
    
- tunnel Mode
    
    - 保護IP header
    - 重寫IP header，可以在internet傳輸
    - 效率低
    
    ![[Assets/Note/Research/IPSec (IP Security) 介紹/03-IPSec Header AH ESP AH-ESP - 效率低.png|03-IPSec Header AH ESP AH-ESP - 效率低.png]]
    

### SA

協商內容協商後的內容就是(安全關聯SA)

![[Assets/Note/Research/IPSec (IP Security) 介紹/04-SA.png|04-SA.png]]

---

## IPSec種類模式(傳輸層面的)

### transport

通常用於兩台主機之間

![[Assets/Note/Research/IPSec (IP Security) 介紹/05-transport.png|05-transport.png]]

完整的IPSec封包

![[Assets/Note/Research/IPSec (IP Security) 介紹/06-transport.png|06-transport.png]]

傳送出去的IPSec封包

![[Assets/Note/Research/IPSec (IP Security) 介紹/07-transport.png|07-transport.png]]

### tunnel

通常用於多台主機的兩個站台

![[Assets/Note/Research/IPSec (IP Security) 介紹/08-tunnel.png|08-tunnel.png]]

原始的IP封包

![[Assets/Note/Research/IPSec (IP Security) 介紹/09-tunnel.png|09-tunnel.png]]

這是完整的IPSec封包的樣子，要新增IP Header和IPSec Header

![[Assets/Note/Research/IPSec (IP Security) 介紹/10-tunnel.png|10-tunnel.png]]

經過加密點之後ESP加密後再internet的樣子是下面這張圖

![[Assets/Note/Research/IPSec (IP Security) 介紹/11-tunnel.png|11-tunnel.png]]

---

## IPSec控制層面

### IKE (internet key Exchange)

![[Assets/Note/Research/IPSec (IP Security) 介紹/12-IKE (internet key Exchange).png|12-IKE (internet key Exchange).png]]

- 凡是在internet交換密鑰的就屬於這個
- IKE主要負責建立和維護IKE SAs和IPSec SAs。
- 對雙方進行驗證
- 交換公共密鑰，產生密鑰資源，管理密鑰。
- 協商參數(封裝，加密，驗證....)
- 大部分的VPN都是用主模式只有EzVPN是用主動模式

### IPSec模式

![[Assets/Note/Research/IPSec (IP Security) 介紹/13-IPSec模式.png|13-IPSec模式.png]]

- 主模式(==6 Message==)Main Mode
    
    - 基於UDP 500 port
    - IKE Phase 1 (IKE SA)協商ISAKMP政策產生通道
        - 第一步驟協商確認
            
            這個階段就是確認雙方的policy是否相同如果policy 10不同就會換policy 20進行比對
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/14-IPSec模式 - 第一步驟協商確認.png|14-IPSec模式 - 第一步驟協商確認.png]]
            
            1. A將所有IPSec的策略傳給B
            2. B將所有IPSec的策略傳給A
        - 第二步驟非對稱密鑰協商形成DH
            
            這個步驟主要是產生DH密鑰
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/15-IPSec模式 - 第二步驟非對稱密鑰協商形成DH.png|15-IPSec模式 - 第二步驟非對稱密鑰協商形成DH.png]]
            
            3. 使用DH演算法算出三把鑰**匙**
            
            4. 使用DH演算法算出三把鑰**匙**
            
        - 第三步驟就是在做驗證，驗證的方法可以是PSK或是certificate，並且使用第一和第二步驟協商後的策略進行加密演算，並且在第五第六個封包中使用DH中的第一把鑰**匙進行加密**
            
            5. 互相匹配驗證
            
            6. 互相匹配驗證
            
            為什麼需要在5和6上面進行加密呢?因為不行讓自己的PSK和certificate在internet逛大街
            
            ==此時已經創建完畢ISAMP tunnel，並且DH鑰====**匙已經使用了一把**==
            
    - IKE Phase 2 (IPSec SA)協商IPsec政策並創建安全封包通道
        
        ![[Assets/Note/Research/IPSec (IP Security) 介紹/16-IPSec模式.png|16-IPSec模式.png]]
        
        1. interesting Traffic (ACL)
        2. P2 SA
            1. Mode (Tunnel/Transport)
            2. IPsec Timeout(Default 3600s)
            3. Encapsulated Protocol (ESP/AH)
            4. Encryption mechanism (DES/3DES/AES)
            5. Hashing mechanism (SHA/MD5)
        
        第一步驟對”實際的感興趣流的加密策略的加密協商” 簡單來說就是對感興趣流的加密策略進行加密，==在前面產生的DH鑰====**匙中的第二把用在第1個封包和第2個封包中**==**，第3個只是用來去確認用的。**最後對感興趣流進行最後的加密也就是用==DH鑰====**匙中的第三把進行加解密**==
        
        因為Phase 2是實體傳送資料的tunnel所以必須全程使用加密包括加密策略協商
        
        ![[Assets/Note/Research/IPSec (IP Security) 介紹/17-IPSec模式.png|17-IPSec模式.png]]
        
    
      
    
- 主動模式(==3 Message==，EzVPN比較常用)
    - IKE Phase 1
        
        ![[Assets/Note/Research/IPSec (IP Security) 介紹/18-IPSec模式 - IKE Phase 1.png|18-IPSec模式 - IKE Phase 1.png]]
        
    - IKE Phase 2
        
        跟上面一樣因為Phase 2都是快速模式
        
- IPSec certificate認證在Phase 1的第五個封包，第六個包變成R2被R1驗證
    
    ![[Assets/Note/Research/IPSec (IP Security) 介紹/19-IPSec模式.png|19-IPSec模式.png]]
    

---

## IPSec設定

### IPSec tunnel

![[Assets/Note/Research/IPSec (IP Security) 介紹/20-IPSec tunnel.png|20-IPSec tunnel.png]]

  

- 設定ipsec要先設定的
    
    ```Plain
    R1
    interface Ethernet0/0
     ip address 12.1.1.1 255.255.255.0
     ip nat outside
     ip virtual-reassembly in
    !
    interface Ethernet0/1
     ip address 192.168.1.254 255.255.255.0
     ip nat inside
     ip virtual-reassembly in
    !
    ip nat inside source list 100 interface Ethernet0/0 overload
    ip route 0.0.0.0 0.0.0.0 12.1.1.2
    !
    access-list 100 deny   ip 192.168.1.0 0.0.0.255 192.168.2.0 0.0.0.255
    access-list 100 permit ip any any
    !
    ```
    
    ```Plain
    R2
    interface Loopback0
     ip address 8.8.8.8 255.255.255.255 模擬internet的主機
    !
    interface Ethernet0/0
     ip address 23.1.1.2 255.255.255.0
    !
    interface Ethernet0/1
     ip address 12.1.1.2 255.255.255.0
    ```
    
    ```Plain
    R3
    interface Ethernet0/0
     ip address 23.1.1.3 255.255.255.0
     ip nat outside
     ip virtual-reassembly in
    !
    interface Ethernet0/1
     ip address 192.168.2.254 255.255.255.0
     ip nat inside
     ip virtual-reassembly in
    !
    ip nat source list 100 interface Ethernet0/0 overload
    ip nat inside source list 100 interface Ethernet0/0 overload
    ip route 0.0.0.0 0.0.0.0 23.1.1.2
    !
    access-list 100 deny   ip 192.168.2.0 0.0.0.255 192.168.1.0 0.0.0.255
    access-list 100 permit ip any any
    ```
    
- ipsec
    - 可以照著這個policy設定，在不熟的時候
        
        ```Plain
        !
        R1#show crypto isakmp policy
        Protection suite of priority 10
                encryption algorithm:   AES - Advanced Encryption Standard (128 bit keys).
                hash algorithm:         Message Digest 5
                authentication method:  Pre-Shared Key
                Diffie-Hellman group:   #2 (1024 bit)
                lifetime:               3600 seconds, no volume limit
        ```
        
    - 開始設定
        
        1. IKE 第一階段 (isakmp)
            - interssting traffic
                
                ```Plain
                R(config)#access-list 101 permit ip 192.168.1.0 0.0.0.255 192.168.2.0 0.0.0.255
                ```
                
            - IKEPhase 1參數基本訊息
                
                ```Plain
                R1
                crypto isakmp policy 10
                 encr aes
                 hash md5
                 authentication pre-share
                 group 2
                 lifetime 3600
                ```
                
        2. IKE第二階段參數(IPSec)
            
            ```Plain
            R1(config)#crypto ipsec transform-set SET esp-aes esp-md5-hmac
            R1(cfg-crypto-trans)#mode tunnel
            ```
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/21-IPSec tunnel - IKE第二階段參數(IPSec).png|21-IPSec tunnel - IKE第二階段參數(IPSec).png]]
            
        3. 設定pre-share key和對方的public ip
            
            ```Plain
            R1(config)#crypto isakmp key 6 CCIE address 23.1.1.3
            ```
            
            ```Plain
            R1#sho crypto isakmp key
            ```
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/22-IPSec tunnel.png|22-IPSec tunnel.png]]
            
            ```Plain
            R1(config)#crypto isakmp key 0 CCIE address 23.1.1.3
            ```
            
            ```Plain
            R1#sho crypto isakmp key
            ```
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/23-IPSec tunnel.png|23-IPSec tunnel.png]]
            
        4. 定義Crypto MAP
            
            ```Plain
            R1(config)#crypto map MAP 10 ipsec-isakmp
            % NOTE: This new crypto map will remain disabled until a peer
                    and a valid access list have been configured. 簡單來說就是要設定感興趣流，和peer
            
            R1(config-crypto-map)#set peer 23.1.1.3
            R1(config-crypto-map)#set transform-set SET
            R1(config-crypto-map)#match address 101
            ```
            
        
        - R3可以用這指令複製出來然後改參數複製進去
            
            ![[Assets/Note/Research/IPSec (IP Security) 介紹/24-IPSec tunnel.png|24-IPSec tunnel.png]]
            
            ```Plain
            R3
            !
            crypto isakmp policy 10
             encr aes
             hash md5
             authentication pre-share
             group 2
             lifetime 3600
            crypto isakmp key 6 CCIE address 12.1.1.1
            access-list 101 per ip 192.168.2.0 0.0.0.255 192.168.1.0 0.0.0.255
            crypto ipsec transform-set SET esp-aes esp-md5-hmac
             mode tunnel
            crypto map MAP 10 ipsec-isakmp
             set peer 12.1.1.1
             set transform-set SET
             match address 101
            ```
            
        
        1. 最後出介面要調用
            
            ```Plain
            R1(config-if)crypton map MAP
            ```
            
            ```Plain
            R3(config-if)crypton map MAP
            ```
            
        
        - 查看
            
            - 會發現IP沒有被保護，但是pyload被包起來了
                
                ![[Assets/Note/Research/IPSec (IP Security) 介紹/25-IPSec tunnel.png|25-IPSec tunnel.png]]
                
            - 發送SPI，和接收會不一樣
                
                ![[Assets/Note/Research/IPSec (IP Security) 介紹/26-IPSec tunnel - 發送SPI,和接收會不一樣.png|26-IPSec tunnel - 發送SPI,和接收會不一樣.png]]
                
            - 會發現沒有東西很正常，因為沒有流量經過，所以需要ping一下
                
                ![[Assets/Note/Research/IPSec (IP Security) 介紹/27-IPSec tunnel.png|27-IPSec tunnel.png]]
                
            - ping
                
                ![[Assets/Note/Research/IPSec (IP Security) 介紹/28-IPSec tunnel - ping.png|28-IPSec tunnel - ping.png]]
                
                ![[Assets/Note/Research/IPSec (IP Security) 介紹/29-IPSec tunnel - ping.png|29-IPSec tunnel - ping.png]]
                
            
            - 查看ipsec sa
                
                ```Plain
                R1#sho crypto ipsec sa
                
                interface: Ethernet0/0
                    Crypto map tag: MAP, local addr 12.1.1.1
                
                   protected vrf: (none)
                   local  ident (addr/mask/prot/port): (192.168.1.0/255.255.255.0/0/0)
                   remote ident (addr/mask/prot/port): (192.168.2.0/255.255.255.0/0/0)
                   current_peer 23.1.1.3 port 500
                     PERMIT, flags={origin_is_acl,}
                    #pkts encaps: 9, #pkts encrypt: 9, #pkts digest: 9
                    #pkts decaps: 9, #pkts decrypt: 9, #pkts verify: 9
                    #pkts compressed: 0, #pkts decompressed: 0
                    #pkts not compressed: 0, #pkts compr. failed: 0
                    #pkts not decompressed: 0, #pkts decompress failed: 0
                    #send errors 0, #recv errors 0
                
                     local crypto endpt.: 12.1.1.1, remote crypto endpt.: 23.1.1.3
                     plaintext mtu 1438, path mtu 1500, ip mtu 1500, ip mtu idb Ethernet0/0
                     current outbound spi: 0x56FD4627(1459439143)
                     PFS (Y/N): N, DH group: none
                
                     inbound esp sas:
                      spi: 0xF77F2D36(4152306998)
                        transform: esp-aes esp-md5-hmac ,
                        in use settings ={Tunnel, }
                        conn id: 1, flow_id: SW:1, sibling_flags 80004040, crypto map: MAP
                        sa timing: remaining key lifetime (k/sec): (4273372/3078)
                        IV size: 16 bytes
                        replay detection support: Y
                        ecn bit support: Y status: off
                        Status: ACTIVE(ACTIVE)
                
                     inbound ah sas:
                
                     inbound pcp sas:
                
                     outbound esp sas:
                      spi: 0x56FD4627(1459439143)
                        transform: esp-aes esp-md5-hmac ,
                        in use settings ={Tunnel, }
                        conn id: 2, flow_id: SW:2, sibling_flags 80004040, crypto map: MAP
                        sa timing: remaining key lifetime (k/sec): (4273372/3078)
                        IV size: 16 bytes
                        replay detection support: Y
                        ecn bit support: Y status: off
                        Status: ACTIVE(ACTIVE)
                
                     outbound ah sas:
                
                     outbound pcp sas:
                ```
                
                ```Plain
                R3#sho cry ipse sa
                
                interface: Ethernet0/0
                    Crypto map tag: MAP, local addr 23.1.1.3
                
                   protected vrf: (none)
                   local  ident (addr/mask/prot/port): (192.168.2.0/255.255.255.0/0/0)
                   remote ident (addr/mask/prot/port): (192.168.1.0/255.255.255.0/0/0)
                   current_peer 12.1.1.1 port 500
                     PERMIT, flags={origin_is_acl,}
                    #pkts encaps: 9, #pkts encrypt: 9, #pkts digest: 9
                    #pkts decaps: 9, #pkts decrypt: 9, #pkts verify: 9
                    #pkts compressed: 0, #pkts decompressed: 0
                    #pkts not compressed: 0, #pkts compr. failed: 0
                    #pkts not decompressed: 0, #pkts decompress failed: 0
                    #send errors 0, #recv errors 0
                
                     local crypto endpt.: 23.1.1.3, remote crypto endpt.: 12.1.1.1
                     plaintext mtu 1438, path mtu 1500, ip mtu 1500, ip mtu idb Ethernet0/0
                     current outbound spi: 0xF77F2D36(4152306998)
                     PFS (Y/N): N, DH group: none
                
                     inbound esp sas:
                      spi: 0x56FD4627(1459439143)
                        transform: esp-aes esp-md5-hmac ,
                        in use settings ={Tunnel, }
                        conn id: 1, flow_id: SW:1, sibling_flags 80000040, crypto map: MAP
                        sa timing: remaining key lifetime (k/sec): (4197698/2984)
                        IV size: 16 bytes
                        replay detection support: Y
                        ecn bit support: Y status: off
                        Status: ACTIVE(ACTIVE)
                
                     inbound ah sas:
                
                     inbound pcp sas:
                
                     outbound esp sas:
                      spi: 0xF77F2D36(4152306998)
                        transform: esp-aes esp-md5-hmac ,
                        in use settings ={Tunnel, }
                        conn id: 2, flow_id: SW:2, sibling_flags 80000040, crypto map: MAP
                        sa timing: remaining key lifetime (k/sec): (4197698/2984)
                        IV size: 16 bytes
                        replay detection support: Y
                        ecn bit support: Y status: off
                        Status: ACTIVE(ACTIVE)
                
                     outbound ah sas:
                
                     outbound pcp sas:
                ```
                
        - TS (排錯)
            
            先查看 isakmp sa停在那個狀態，如果沒有active代表錯誤，如果有狀態，但是是active就代表ipsec sa錯誤，這時候SPI都是空的，怎麼TS呢，就是比對兩邊的config
            
            ```Plain
            R1#sho cry isakmp sa
            IPv4 Crypto ISAKMP SA
            dst             src             state          conn-id status
            23.1.1.3        12.1.1.1        QM_IDLE           1001 ACTIVE
            
            IPv6 Crypto ISAKMP SA
            ```
            

### GRE over ipsec

![[Assets/Note/Research/IPSec (IP Security) 介紹/20-IPSec tunnel.png|20-IPSec tunnel.png]]