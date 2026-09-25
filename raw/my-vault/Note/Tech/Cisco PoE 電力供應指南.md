
## Cisco 無線 AP 和 IP Phone 電力供應方式

Cisco 的無線 AP 或 IP Phone 必須要有電力供應，有以下三種方式：

1. **使用外接變壓器**
2. **插座供電**
3. **Ethernet PoE 電線**

### Inline Power 技術

Inline power 利用 Ethernet UTP 線路提供 DC 48V 電力，讓網路設備可以透過網路線同時接收數據和電力。

## PoE 協定規格

|方法|一般名稱|功率|
|---|---|---|
|Cisco inline power|ILP|7W|
|IEEE 802.3af|PoE|15.4W|
|IEEE 802.3at|PoE+|25.5W|
|Cisco Universal PoE|UPoE|60W|

## PoE 功率等級

|功率等級|48V DC 最大輸出功率|
|---|---|
|0 (default)|15.4W|
|1|4.0W|
|2|7.0W|
|3|15.4W|
|4 (802.3at)|最高 30W|

## 設定指令

### 啟用 PoE

```cisco
SW(config-if)#power inline {auto | static} [max milliwatts]
```

#### 參數說明

- **auto**：自動偵測並提供適當功率
- **static**：靜態分配功率
- **max milliwatts**：設定最大功率限制（以毫瓦為單位）

### 停用 PoE

```cisco
SW(config-if)#power inline never
```

### 查看 PoE 狀態

```cisco
SW#show power inline
```

## 實際應用注意事項

> [!important] 功率規劃 在規劃 PoE 部署時，需要考慮交換器的總功率預算和每個連接埠的功率需求

> [!warning] 相容性 確保 PoE 設備與交換器的 PoE 標準相容，避免功率不足或過載問題

### 常見設備功率需求

- **IP Phone**：通常需要 7-15W
- **無線 AP**：依型號不同，需要 15-60W
- **安全攝影機**：通常需要 12-25W
- **LED 照明**：依亮度需求，通常 10-30W