---
Created: 2023-02-27T20:50
---
- [[#LDAP身世]]
- [[#LDAP優勢]]
- [[#LDAP 可以做甚麼]]
- [[#LDAP組件]]
    - [[#資料結構]]
    - [[#LDAP樹狀結構]]
    - [[#LDAP 的組成]]
- [[#LDAP查詢語法]]
- [[#LDAP跟帳號有什麼關西呢]]
    - [[#SSO]]
- [[#推薦影片]]

---

## LDAP身世

LDAP前輩是x.500 directory service，一個操作在應用層複雜的協定;因為會隨著載入的資料大小不同而導致封包大小不同。LDAP樣式多變，只有定義一個協定，所以輕量，也很乾淨

LDAP是一個以訊息為基礎，採用主從式架構，定義於RFC2251協定，並是一個非同步的協定架構，意味著用戶端可以發乎多筆請求，且server不必要照順序的方式回答slave可以以警及程度回答slave

## LDAP優勢

- LDAP 透過軟體之間的配合，讓帳號密碼能單純化
- 可以超逾區域網路的限制，遠端也能結合
- 本身有主從架構(Master \Slave)，可以利用多個子系統來分散存取，主從模式下也可以備用
- 沒有特別限定用途，也就是說可以限制帳號密碼，已可以用dns，也可以用RADIUS
- 具有跨平台的能力，可以在多個系統中使用
- Ldap 運行在tcp/ip

---

## LDAP 可以做甚麼

- 一般用於
    - 郵件server
    - samba帳號
    - 郵件查詢
- 比較特別的
    - FreeRadius witch LDAP
    - Squid認證
    - AP witch LDAP
    - DNS LDAP

---

## LDAP組件

### 資料結構

![[Assets/Note/Research/LDAP (Light-weight Directory Access Protocol)/01-資料結構.png]]

LDAP資料結構的三層概念是 : schema、Object Class、Attribute Type

- Schema
    - `schema`在LDAP很重要，字面上解釋就是Model或式Diagram，其實就是描述，`schema`就是一個對於LDAP的溝通語言，如果沒有schema在LDAP就沒辦法溝通，LDAP就是schema和資料庫的世界
    - `Object Class`的集合，透過集合相同性質的類別，描述現實中的個體的資訊。
- Object Class
    - `Attribute Type` 的集合，每種 `Object Class` 會定義有哪些必要、可選的 `Attribute Type`
    - `Object Class` 具有繼承的關係
- `Attribute Type`
    - 描述資料的內容
    - 已鍵值（Key-Value）的方式表示

### LDAP樹狀結構

![[Assets/Note/Research/LDAP (Light-weight Directory Access Protocol)/02-LDAP樹狀結構.png]]

使用上述的資料結構來組成 LDAP 的樹狀結構，在 LDAP 中稱呼樹的節點為 `Entry`  
，一個   
`Entry`只能使用一種 `Object Class` 來表達，而葉子就是 `Attribute Type`  
 了，下面這張圖表達了 LDAP 目錄樹的關聯階層  

- 最上面就是父節點 這只有一個，但是另一個樹可能有相同的名子的父根節點
- `pipeline`就是依存關西的建立，也就是連接線
- 可以無限層分層，名子可以重複在不同的父節點中
- 個個節點可以有資料也可以沒有資料
- 我覺得可以想像層成，發電廠，核電站→區域發電站→社區變電所→你家

### LDAP 的組成

![[Assets/Note/Research/LDAP (Light-weight Directory Access Protocol)/03-LDAP 的組成.png]]

|Attribute type|full name|illustrate (說明)|
|---|---|---|
|CN|Common Name|用戶名稱、單位名稱|
|DN|Distinguishecd Name|識別名稱，絕對位置|
|OU|Organiztional Unit Name|組織單位名稱|
|DC|Domain Componet|網域元件|

- 非關聯性的資料庫環境時，而每一筆資料透過`DN`來提示定義，就是`OU`可以分成兩個無相關的東西(People和Server這兩個無相關的東西)
- `baseDN`也就是這個LDAP系統最先要設定的東西`dc=company,dc=com`，這個就是`baseDN`
- 每個 `baseDN` 底下還有附屬組織單位，也就是`OU`，例如人事部門可以使用類似： 『 `dn: ou=People,dc=example,dc=com` 』之類的方式來命名。而該 DN 底下就會有多種屬性定義！這些屬性定義就牽涉到每一個 LDAP 的用途為何。
- 因為 LDAP 僅是一個『目錄服務』的提供者，但這些目錄底下資料的定義使用的功能為何，就是透過預先定義的 `Schema` 來定義

  

---

## LDAP查詢語法

> [!info] LDAP 查詢基礎  
> 作者: William Taylor 本文將探討輕量型目錄存取通訊協定 (LDAP) 查詢，這在疑難排解 Microsoft® Exchange Server 及其與其目錄的關係時可能很實用，但容易令人混淆。文中將提供關於 LDAP 查詢的基本資訊。 = (等於) 此一 LDAP 引數是指特定屬性必須等於 True 的特定值。比方說，假如您想要尋找名字為 John 的所有物件，您可使用： (givenName=John) 這會傳回所有名字為 John 的物件。包括括弧是要強調 LDAP 陳述式的開始和結束。 & (邏輯 AND) 當您有一個以上的條件，或希望序列中的所有條件皆為 True 時可使用此語法。比方說，假如您想要尋找所有名字為 John 且住在達拉斯的人，您可使用： (&(givenName=John)(l=Dallas)) 請注意每個引數都位於自己的一組括弧內。整個 LDAP 陳述式必須封裝在主要的括弧組中。& 運算子代表每個引數對於此篩選器必須為 True，以套用到剛剛提到的物件。 !  
> [https://docs.microsoft.com/zh-TW/previous-versions//dd159860(v=technet.10)?redirectedfrom=MSDN](https://docs.microsoft.com/zh-TW/previous-versions//dd159860(v=technet.10)?redirectedfrom=MSDN)  

---

## LDAP跟帳號有什麼關西呢

- 首先要了解linux的帳號認證機制PAM
- LDAP與PAM機制的橋樑nss_ldap
- LDAP需要事先規劃好，規劃樹怎麼長
- 為甚麼要使用LDAP管理帳號
    - LDAP可以跨網路，超脫區域網路連結的限制，跨國際
    - LDAP可以快速複製轉移
    - LDAP可以藉由單一帳號登入 (Sing Account Sign on)，變成完全的單一登入 (Single Sign On)(SSO)

### SSO

企業中的資訊息系統做到SSO，沒有很簡單，首先要基礎建設要有LDAP的建置(常見的產品有Novell和Microsoft AD)，規劃LDAP作為企業個資訊系統的驗證授權中心，各系統到LDAP做驗證和授權，當然也可以不用LDAP，用VB+database，也可以做成，幾乎跟LDAP一樣

---

## 推薦影片

> [!info] StudyAreaTw  
> Share your videos with friends, family, and the world  
> [https://www.youtube.com/user/StudyAreaTw/videos](https://www.youtube.com/user/StudyAreaTw/videos)  

> [!info] StudyAreaTw  
> Share your videos with friends, family, and the world  
> [https://www.youtube.com/user/StudyAreaTw/videos](https://www.youtube.com/user/StudyAreaTw/videos)