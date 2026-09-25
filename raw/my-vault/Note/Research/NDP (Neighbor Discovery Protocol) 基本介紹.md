- [[#用處]]
- [[#RS/RA]]
    - [[#使用NDP RS及RA探索路由器]]
    - [[#使用NDP RS及RA探索SLAAC定址資訊]]
- [[#NS/NA]]
    - [[#使用NDP NS及NA探索鄰居的鏈路地址]]
    - [[#使用NDP NS及NA探索重複位置]]

## 用處

- SLAAC : 使用SLAAC時，主機使用NDP訊息來獲得其位址的第一部分與首碼長度
- 路由器探索 : 主機使用NDP訊息像相同子網中可用的路由器獲取IPv6位址
- 重複位置偵測 : 無論主機的IPv6位址是經由人工設定或是自動獲取，該主機沒辦法知道位址是否重複，所以要透過DAD(Duplicate address Detectin)的程序使用NDP訊息
- 芳鄰MAC探索 : 再主機經過DAD程序，並適用IPv6地址後，位於LAN的主機需知道同一個子網路中其他主機的MAC位址，NDP取代了ARP在IPv6中。

## RS/RA

- ICMPv6 編號 RS 135、 RA 134
- 路由請求RS (Router Solicitation) : 此訊息會被發送到所有「IPv6的路由器(all-IPv6-Routers)」本地範圍的群播位址FF02::2。該訊息只能使用在本地鏈路中，要求所有的路由器提供自己的身分
- 路由通告 RA (Router Advertisement) : 該訊息由路由器發送，其中包括路由器的鏈路區域IPv6位址等許多資訊。當RA產生時，他會被發送到本地範圍內的群播地址FF02::1，表示「所有IPv6主機(all-IPv6-hosts)」

### 使用NDP RS及RA探索路由器

![[Assets/Note/Research/NDP (Neighbor Discovery Protocol) 基本介紹/01-使用NDP RS及RA探索路由器.png|01-使用NDP RS及RA探索路由器.png]]

ICMPv6取代了ICMP在IPv6中，IPv6包含ping指令所需要的Request和Echo Reply訊息。ICMPv6同樣包含所有的NDP訊息如下列兩種訊息。

### 使用NDP RS及RA探索SLAAC定址資訊

![[Assets/Note/Research/NDP (Neighbor Discovery Protocol) 基本介紹/02-使用NDP RS及RA探索SLAAC定址資訊.png|02-使用NDP RS及RA探索SLAAC定址資訊.png]]

RS/RA其實就是一種簡易的「查詢/回應」的通訊協定(或是「請求(solicitation)/通告(advertisement)」)

  

## NS/NA

NDP還定義了第二對請求通告訊息，(neighbor solicitation，NS)和(neighbor advertissement，NA)，基本上NS就像IPv4的ARP回應，列出所有該主機的MAC位址。

NA和NS傳輸過程和RS和RA都一樣，NS訊息要求，NA提供資訊，不同的地方RS/RA著重跟路由拿資料，NA/NS著重跟任何IPv6主機所持有的資訊。

- NS : 要求一台具有特定IPv6位址的主機回傳一個帶有mac位置的NA。NS訊息會被發送到與目標主機有關的請求節點的群播位址。
- NA : 該訊息回覆NS訊息，該訊息會被發送到所有IPv6的本地群播地址FF02::1

### 使用NDP NS及NA探索鄰居的鏈路地址

![[Assets/Note/Research/NDP (Neighbor Discovery Protocol) 基本介紹/03-使用NDP NS及NA探索鄰居的鏈路地址.png|03-使用NDP NS及NA探索鄰居的鏈路地址.png]]

### 使用NDP NS及NA探索重複位置

以免重複的位址，IPv6在使用單播位址前，利用DAD來確保練路上沒有相同的節點

![[Assets/Note/Research/NDP (Neighbor Discovery Protocol) 基本介紹/04-使用NDP NS及NA探索重複位置.png|04-使用NDP NS及NA探索重複位置.png]]

|功能|協定訊息|誰來探索資訊|誰來提供資訊|資訊內容|
|---|---|---|---|---|
|路由探索|RS/RA|任何IPv6主機|任何IPv6 Router|路由器鏈路區域IPv6位址|
|首碼/長度探索|RS/RA|任何IPv6主機|任何IPv6 Router|使用在本地鏈路上的首碼及首碼長度|
|芳鄰探索|NS/NA|任何IPv6主機|任何IPv6 主機|鄰居使用的鏈結層位址(MAC位址)|
|重複位址偵測|NS/NA|任何IPv6主機|任何IPv6 主機|單播位址是被使用，使用簡易的確認|

```Plain
R1#show ipv6 neighbors
```