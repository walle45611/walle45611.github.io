---
Type:
  - windows Server DNS
---
### 建立primary zone

![[Assets/Note/Tech/Windows Server DNS 設定指南/01-建立primary zone.png|01-建立primary zone.png]]

- primary zone
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/02-建立primary zone.png|02-建立primary zone.png]]
    
- 建立primary zone
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/03-建立primary zone.png|03-建立primary zone.png]]
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/04-建立primary zone.png|04-建立primary zone.png]]
    
- Domain name
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/05-建立primary zone - Domain name.png|05-建立primary zone - Domain name.png]]
    
- 先不請用動態更新
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/06-建立primary zone - 先不請用動態更新.png|06-建立primary zone - 先不請用動態更新.png]]
    
- 下一步
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/07-建立primary zone - 下一步.png|07-建立primary zone - 下一步.png]]
    
- 設定好的樣子
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/08-建立primary zone - 設定好的樣子.png|08-建立primary zone - 設定好的樣子.png]]
    
- 設定A record
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/09-建立primary zone - 設定A record.png|09-建立primary zone - 設定A record.png]]
    
- 會有A紀錄
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/10-建立primary zone - 會有A紀錄.png|10-建立primary zone - 會有A紀錄.png]]
    
- 查詢結果
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/11-建立primary zone - 查詢結果.png|11-建立primary zone - 查詢結果.png]]
    

---

### secondary zone

![[Assets/Note/Tech/Windows Server DNS 設定指南/12-secondary zone.png|12-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/13-secondary zone.png|13-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/14-secondary zone.png|14-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/15-secondary zone.png|15-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/16-secondary zone.png|16-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/17-secondary zone.png|17-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/18-secondary zone.png|18-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/19-secondary zone.png|19-secondary zone.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/20-secondary zone.png|20-secondary zone.png]]

---

## reverse

![[Assets/Note/Tech/Windows Server DNS 設定指南/21-reverse.png|21-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/22-reverse.png|22-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/23-reverse.png|23-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/24-reverse.png|24-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/25-reverse.png|25-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/26-reverse.png|26-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/27-reverse.png|27-reverse.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/28-reverse.png|28-reverse.png]]

- 結果
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/29-reverse - 結果.png|29-reverse - 結果.png]]
    

---

## subdomain 和 delegatin domain

- 建立子網域，然後將記錄輸入到此子望路內，這些紀錄是儲存在這台DNS Server
- 也可以將子網域的紀錄委派到給其他DNS server來管理，也就是此子網域內的紀錄是除存被委派的DNS伺服器
- 建立子網域
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/30-subdomain 和 delegatin domain - 建立子網域.png|30-subdomain 和 delegatin domain - 建立子網域.png]]
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/31-subdomain 和 delegatin domain - 建立子網域.png|31-subdomain 和 delegatin domain - 建立子網域.png]]
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/32-subdomain 和 delegatin domain - 建立子網域.png|32-subdomain 和 delegatin domain - 建立子網域.png]]
    
- delegatin domain
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/33-subdomain 和 delegatin domain.png|33-subdomain 和 delegatin domain.png]]
    
    - DNS3，建立primary dns
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/34-subdomain 和 delegatin domain.png|34-subdomain 和 delegatin domain.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/35-subdomain 和 delegatin domain.png|35-subdomain 和 delegatin domain.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/36-subdomain 和 delegatin domain.png|36-subdomain 和 delegatin domain.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/37-subdomain 和 delegatin domain.png|37-subdomain 和 delegatin domain.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/38-subdomain 和 delegatin domain.png|38-subdomain 和 delegatin domain.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/39-subdomain 和 delegatin domain.png|39-subdomain 和 delegatin domain.png]]
        
    - DNS1
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/40-subdomain 和 delegatin domain - DNS1.png|40-subdomain 和 delegatin domain - DNS1.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/41-subdomain 和 delegatin domain - DNS1.png|41-subdomain 和 delegatin domain - DNS1.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/42-subdomain 和 delegatin domain - DNS1.png|42-subdomain 和 delegatin domain - DNS1.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/43-subdomain 和 delegatin domain - DNS1.png|43-subdomain 和 delegatin domain - DNS1.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/44-subdomain 和 delegatin domain - DNS1.png|44-subdomain 和 delegatin domain - DNS1.png]]
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/45-subdomain 和 delegatin domain.png|45-subdomain 和 delegatin domain.png]]
        

---

## SOA

![[Assets/Note/Tech/Windows Server DNS 設定指南/46-SOA.png|46-SOA.png]]

- serial number 發生異動，serial number增加，secondary server和master server，可以根據雙方的serial number是否新紀錄，以便透過zone transfer，複寫新紀錄到secondary server。
- Primary server : 主要的server。
- Responsible persion : 用點取代了@，這邊可以使用mail型式填寫負責人。
- Refresh interval : secondary 每隔此時間，就會詢問master server，如果有新紀錄就會zone transfer 。
- Retry interval : 如果zone transfer失敗，間隔這段時間再重傳一次
- Expires after : 如果查詢失敗都沒有成功過，這個時間到了就不會查詢了
- TTL (Time to Live) : 這個就是DNS server紀錄提供給查詢者後，查詢者可以將記錄存到cache，但是這個紀錄只會保存一段時間，如果過了TTL就會刪除
    - 查詢 cache
        
        ![[Assets/Note/Tech/Windows Server DNS 設定指南/47-SOA - 查詢 cache.png|47-SOA - 查詢 cache.png]]
        
    - 用戶端clear cache
        
        ```PowerShell
        ipconfig /flusdns
        ```
        
    - 用戶端查看cache
        
        ```PowerShell
        ipconfig /displaydns
        ```
        

---

## 新增DNS server

![[Assets/Note/Tech/Windows Server DNS 設定指南/48-新增DNS server.png|48-新增DNS server.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/49-新增DNS server.png|49-新增DNS server.png]]

![[Assets/Note/Tech/Windows Server DNS 設定指南/50-新增DNS server.png]]

---

## 動態更新

DNS具備動態更新的能力，若DNS用戶端的主機、IP位址有一動的話，這些異動資訊會傳送到DNS Server，自動更新zone內的資訊

- Server設定
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/51-動態更新 - Server設定.png]]
    
- client設定
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/52-動態更新 - client設定.png]]
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/53-動態更新 - client設定.png]]
    

## Root hint

DNS用戶端對DNS serer提出查詢要求，若server沒有所需的記錄的話，則server會代替用戶端向位於root hints內的DNS查詢或是forwarer

![[Assets/Note/Tech/Windows Server DNS 設定指南/54-Root hint.png]]

## Forwarder

如果查詢不再本地的server就會轉向Root hints，如果有多台DNS server出於安全的考量只讓一台server對外並讓其他內部DNS server將查詢要求委託給這一台DNS server來負責，也就是說這一台DNS是其他內部DNS server的Forwarder

![[Assets/Note/Tech/Windows Server DNS 設定指南/55-Forwarder.png]]

- 下面有個勾勾代表如果沒有可用的轉寄站，則使用Root hints，如果不想讓此server對外查詢，可以取消勾選，此台DNS稱為forward-only server
- conditional forwarders
    
    - 將查詢domain name，轉寄到某台server IP
    
    ![[Assets/Note/Tech/Windows Server DNS 設定指南/56-Forwarder.png]]
    

## 監控DNS

![[Assets/Note/Tech/Windows Server DNS 設定指南/57-監控DNS.png]]

## DNSSEC

  

  

  

  

  

  

  

---

## Powershell

DNS Geo-Location Awareness

例如不同地點的用戶端會查到www.contoso.com的不同A紀錄

- 設定方式
    1. 建立DNS用戶端網段(Client Subnet)
        
        ```PowerShell
        Add-DnsServerClientSubnet -Name "AmericaSubnet" -IPv4Subnet 192.0.0.0/24,182.0.0.0/24
        Add-dnsServerClientSubnet -name "EuropeSubunet" -IPv4Subnet 141.1.0.0/24,151,1.0.0/24
        ```
        
    2. 建立DNS區域範圍 (Zone Scope)
        
        ```PowerShell
        add-DnsServerZoneScope -zoneName "contosogiftservices.com" -Name "DublinZoneScope"
        add-DnsServerZoneScope -zoneName "contosogiftservices.com" -Name "AmsterdamZoneScope"
        ```
        
    3. 在區域範圍中加入DNS紀錄
        
        ```PowerShell
        add-DnsServerResourceRecord -zoneName "contosogiftservices.com" -A -Name "www" -IPv4Address "151.1.0.1" -ZoneScope "DublinZoneScope"
        add-DnsServerResourceRecord -zoneName "contosogiftservices.com" -A -Name "www" -IPv4Address "141.1.0.1" -ZoneScope "AmsterdamZoneScope"
        ```
        
    4. 建立DNS原則
        
        client subnet
        
        ```PowerShell
        add-DnsServersQueryResolutionPolicy -Name "AmsterdamZoneScope" -Action ALLOW -ClientSubnet "eq,americaSubnet" -ZoneScope "SeatlleZoneScope,2;ChicagoZonescope,1;TexasZoneScope,1" -ZoneName "contosogiftservices.com" -ProcessingOrder 1
        ```
        
        fqdn
        
        ```PowerShell
        add-dnsServerqueryResolutionPolicy -name "WorldWidePolicy" -Action ALLOW -FQDN "eq,*.contoso.com" -ZoneScope "SeattleZoneScope,1;ChicagoZoneScope,1;TexasZoneScope,1;DublinZoneScope,1;AmsterdamZoneScope,1" -ZoneName "contosogiftservices.com" -Processingorder 3
        ```