## DHCPv4

### 可以使用的設備

- L3以上的設備

### 原理

- 用意 : 自動分配IP address，gateway，dns，網路中無盤系統提供引導加載，bootP是DHCP的前身所以在cisco上有的時候設定bootPC就是dhcp client，bootPS就是dhcp server

![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/01-原理.png|01-原理.png]]

### DHCPv4 Relay

可以使DHCP Server在不同往段分配IP給需要IP的網段

![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/02-DHCPv4 Relay.png|02-DHCPv4 Relay.png]]

### Automatic Private IP address (APIPA)

當windows client無法從DHCP server租用到IP會自動把IP變成169.254.0.0/16的IP位址，並使用這個P位址更其他主機溝通，會先廣播詢問其他台主機有沒有使用這個IP位址，會5min來尋找一次DHCP

## DHCPv6

DHCPv6在使用可以分為兩大類，有狀態DHCPv6或是無狀態的DHCPv6。有狀態DHCPv6類似DHCPv4。有狀態DHCPv6會記錄用戶狀態和租借資訊，無狀態DHCPv6不會記錄用戶資訊。

### stateful DHCPv6

### DHCPv6和DHCPv4不同的地方

- 取得Default gateway需要透過IPv6內建的NDP來取得
    
    ![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/03-DHCPv6和DHCPv4不同的地方.png|03-DHCPv6和DHCPv4不同的地方.png]]
    
- DHCPv6封包
    
    ![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/04-DHCPv6和DHCPv4不同的地方 - DHCPv6封包.png|04-DHCPv6和DHCPv4不同的地方 - DHCPv6封包.png]]
    
    DHCPv6是使用IPv6封包來更新訊息，而非IPv4，DHCPv4的:探索(Discover)、提供(offer)、要求(Request)、確認(acknowledgment)等等訊息(簡稱DORA)，DHCPv6所使用的四個訊息:請求(Solicit)、通告(advertise)、要求(Request)、回應(Reply)。
    
      
    

### DHCPv6和DHCPv4相似的地方

- 在LAN上的DHCP用戶端發戶的訊息只能在本地的LAN上傳送，目的是因為為了找到DHCP server
- DHCP server和Client如果在同一個LAN，他們可以直接交換訊息，無須路由器幫助
- 如果DHCP server和Client不再同一個LAN，就需要依靠路由器的幫助
- 如果路由器要將一條鏈路上的訊息轉送到子網路的DHCP server上，除了設定路由器為dhcp relay，還須知道DHCP server ipv6的address
- DHCP server組態中，列有每個子網路的位址scope，提供server分配位址
- DHCP server為client提供IP位址租借，租借時間內用戶都是有效的。
- DHCP server會記錄狀態資訊，特別是用戶端的識別碼 ( 通常按照MAC address ) 與目前租借給該用戶端的地址

  

### DHCPv6 relay

![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/05-DHCPv6 relay.png|05-DHCPv6 relay.png]]

1. A先向所有DHCP proxy發送群播FF02::1:2，因為是群播的所以僅限在此條鏈路中，不會向外發送
2. R1擔任DHCP proxy，R1聆聽DHCP訊息知道A要將訊息發送到FF02::1:2於是將封包目的地，變為右側DHCP server，R1把source address改成自己的IP address，這與DHCPv4 proxy不太一樣。
3. DHCP server接收到後傳回R1，R1在將目的地位置替換為A的。

### stateless DHCPv6

因為IT人員需要設定執行和管理DHCP server。要替server設定每個子網的IP，這些功能都沒有問題，但是需要依賴人規劃管理。

IPv6的SLAAC提供替代方法，可以動態分配IPv6 address，SLAAC不需要分配或是租借位址，也不需要IT人員設定每個子網的資料。

### 使用SLAAC建構IPv6位址

當使用SLAAC時，主機無須租借IPv6地址，也不用學習IPv6地址，取代為學習perfix部分的位址，後再補足IPv6位址的其餘部分。

1. 任何路由器利用NDP RS/RA學習到鏈路上的IPv6
2. 將介面ID接在步驟1學到的IPv6 perfix之後，組合成IPv6地址
3. 在使用位址前先用DAD檢查是否有相同IP

![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/06-使用SLAAC建構IPv6位址.png|06-使用SLAAC建構IPv6位址.png]]

### NDP及stateless DHCP with SLAAC

- 最後使用DHCPv6來提供DNS server
    
    ![[Assets/Note/Research/DHCP (Dynamic Host Configuration Protocol)/07-NDP及stateless DHCP with SLAAC.png|07-NDP及stateless DHCP with SLAAC.png]]
    
- stateless and stateful比較

|功能|stateful|stateless|
|---|---|---|
|record client ipv6 address status|yes|no|
|租借IPv6 address給client|yes|no|
|支援DNS server位址清單|yes|yes|
|搭配SLAAC使用|no|yes|