## 概述

在眾多 Data Link Layer 協定中，使用最廣泛的莫過於 Ethernet。它簡單、易於 NIC 及 Driver 實現，初期 Ethernet NIC 相對於其他網卡，最初的速度從 10Mbps、1Gbps 到最後都可以達到更高速的網路。現在 Ethernet 已經成為最具有相容性與未來發展的資料鏈路層協定。

早期是由兩家美國公司設計的通訊方式，後來由 IEEE 802.3 規範化，但是兩個的 frame 內容又有點不一樣，因此 IEEE 802.3 所規範的 Ethernet 又被稱為 802.3 Ethernet。

## Ethernet II 格式

![[Assets/Note/Research/Ethernet 乙太網路協定指南/01-Ethernet II 格式.png]]

### IEEE 802.3 Ethernet

有一些 Layer 2 的協定會使用這個協定，如 CDP、VTP，但通常還是使用 Ethernet II。

![[Assets/Note/Research/Ethernet 乙太網路協定指南/02-IEEE 802.3 Ethernet.png]]

## 整體流程

![[Assets/Note/Research/Ethernet 乙太網路協定指南/03-整體流程.png]]

## 封裝與解封裝過程

### Encapsulation（封裝）

![[Assets/Note/Research/Ethernet 乙太網路協定指南/04-Encapsulation(封裝).png]]

#### 重要特性

- **Ethernet Frame 物理特性**：使其長度必須在 46~1500 bytes 之間
- **Ethernet Header**：有一個 16-bit 的 Ethernet Protocol Number（IP、ARP、PPPoE）
- **IP Header**：存入一個 8-bit 的值，稱為 IP Protocol Number
- **傳輸層**：TCP、UDP 都使用一個 16-bit 的 Port Number 表示不同的應用程式

#### 長度處理

- **不夠 46 bytes**：需要填充（Padding）
- **大於 1500 bytes**：需要分片（Fragmentation）

### Decapsulation（解封裝）

![[Assets/Note/Research/Ethernet 乙太網路協定指南/05-Decapsulation(解封裝).png]]

## Ethernet 連接方式

### 傳統 Ethernet 連接方式

基本上現在已經沒有使用這種方式：

![[Assets/Note/Research/Ethernet 乙太網路協定指南/06-傳統 Ethernet 連接方式.png]]

### 現代 Ethernet 連接方式

現在都使用 Switch：

![[Assets/Note/Research/Ethernet 乙太網路協定指南/07-現代 Ethernet 連接方式.png]]

## Ethernet 的分類

T 系列通常都是 100 公尺的傳輸距離：

![[Assets/Note/Research/Ethernet 乙太網路協定指南/01-Ethernet 的分類.png]]

## 共享型網路 vs 非共享型網路

### 共享型網路

從通信介質的使用方法上看，網路可以分成共享型和非共享型。共享型網路指由多個設備共享一個通信介質的一種網路。最早的 Ethernet 就是共享型。在這種連接方式下，網路設備使用同一個接收和發送的設備，因此都採用半雙工的通訊方式，並有必要對介質進行存取控制（Hub、無線 AP 就是具體的共享型網路）。

#### Contention（競爭）

Contention 是指爭奪資料傳輸權利的機制。

##### CSMA (Carrier Sense Multiple Access)

這種方式通常令網路的各個節點採用先搶先贏的方式佔用網路信道進行發送。如果多個站同時發送 frame，則會產生衝突現象，也因此會導致網路效能下降。

##### CSMA/CD (Carrier Sense Multiple Access with Collision Detection)

改良 CSMA 的一種方法，一旦發生衝突，則盡早釋放信道：

1. **載波偵測**：如果信道上沒有任何資料，都可以發送
2. **碰撞檢測**：檢查是否發生碰撞，一旦發生碰撞，放棄發送資料，同時立即釋放信道
3. **隨機退避**：放棄爭奪後，隨機延遲一段時間，再次爭奪發送封包

### 非共享型網路

其實就是可以使用全雙工的網路，也就是說可以同時發送資料和接收資料，最典型的就是 Switch。所以就可以知道共享型網路是很慢的，也就是說 AP 基本上數據都要乘以 0.6。

## MAC Address

### 基本特性

- **長度**：48-bit（6 bytes）
- **儲存位置**：在使用網卡的情況下，MAC 位址一般會被燒入到 ROM
- **唯一性**：MAC 位址是唯一的，全球都不會重複

![[Assets/Note/Research/Ethernet 乙太網路協定指南/02-基本特性.png]]

### Switch 的 MAC Address 處理

Switch 會根據 MAC Address Table 去做轉發：

參考：[[STP (Spanning tree) 完整指南]]

#### 學習過程

1. **學習來源**：從 Source MAC Address 可以知道主機連接的 Port
2. **未知轉發**：複製那些以「未知」MAC Address 標示的封包給所有 Port（Flooding）
3. **建立對應**：從 Source MAC Address 可以得知主機與 Port 相連的資訊
4. **精確轉發**：由於已經知道主機與某個 Port 相連，那麼發送給該主機的 Frame 只複製給該 Port

## Ethernet II 和 IEEE 802.3 封裝格式比較

![[Assets/Note/Research/Ethernet 乙太網路協定指南/03-Ethernet II 和 IEEE 802.3 封裝格式比較.gif]]

### 共同特性

- 兩種 Frame 格式都採用 48-bit（6 bytes）的 Source MAC Address 和 Destination MAC Address
- ARP 和 RARP 協定對 32-bit 的 IP Address 和 48-bit 的 MAC Address 進行對應

### 主要差異

#### 區別方式

802.3 定義的 Length 與 Ethernet 的 Type 完全不相同，這樣就可以區別兩種 Frame。

#### IEEE 802.3 特殊欄位

- **DSAP（Destination Service Access Point）**：目的地服務存取點，值設定為 0xAA
- **SSAP（Source Service Access Point）**：來源服務存取點，值設定為 0xAA
- **Control**：控制欄位，都設定為 3
- **Organization Code**：組織代碼，都是 0
- **Protocol Type**：最後兩個欄位跟 Ethernet II 一樣

#### 最小長度要求

- **IEEE 802.3**：規定至少要滿足 38 bytes
- **Ethernet II**：規定至少 46 bytes
- 如果不足最小長度，就需要填充 Padding

## 常用 Ethernet Protocol Number

|Protocol|Number|
|---|---|
|**IP**|0x0800|
|**ARP**|0x0806|
|**PPPoE**|0x8863、0x8864|

## 技術發展趨勢

### 速度演進

|技術|速度|年代|
|---|---|---|
|**早期 Ethernet**|10 Mbps|1980年代|
|**Fast Ethernet**|100 Mbps|1990年代|
|**Gigabit Ethernet**|1 Gbps|2000年代|
|**10 Gigabit Ethernet**|10 Gbps|2010年代|
|**更高速 Ethernet**|100+ Gbps|現在|

### 關鍵優勢

1. **簡單性**：協定設計簡單，易於實現
2. **相容性**：具有良好的向下相容性
3. **成本效益**：製造成本相對較低
4. **擴展性**：支援不同速度和媒體類型
5. **標準化**：有完整的 IEEE 標準支援

## 最佳實務建議

1. **選擇適當的 Frame 格式**：一般情況下使用 Ethernet II
2. **考慮網路拓撲**：現代網路建議使用 Switch 而非 Hub
3. **規劃 MAC Address**：了解 MAC Address 的學習和老化機制
4. **效能最佳化**：優先選擇全雙工和非共享型網路
5. **故障排除**：熟悉封裝和解封裝過程，有助於網路問題診斷