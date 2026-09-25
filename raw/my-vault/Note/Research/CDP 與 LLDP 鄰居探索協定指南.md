## CDP (Cisco Discovery Protocol)

### 功能說明

CDP 可以提供支援某些功能的資訊，以及提供 IT 人員管理設備裝置的資訊。

### CDP 從鄰居 Cisco 設備探索的資料

- **裝置識別碼**：通常是主機名稱
- **位址清單**：網路及資料連結層位址
- **通訊埠識別碼**：鏈路另一端路由器或交換機上發送 CDP 通告的介面
- **相容性清單**：裝置屬於何種類型的資訊（例如路由器或交換器）
- **作業平台**：裝置的型號及執行的 OS 版本

### 檢查 CDP 鄰居指令

|指令|說明|
|---|---|
|`show cdp neighbors [type number]`|將每個鄰居或指定介面上找到的鄰居之相關資訊列成一行摘要|
|`show cdp neighbors detail`|列出一大組（約 15 行）的資訊，每個鄰居一組|
|`show cdp entry name`|與 `show cdp neighbors detail` 相同，只是指定特定鄰居|

### 驗證 CDP 運作指令

|指令|描述|
|---|---|
|`show cdp`|說明 CDP 是否全域啟用，並列出預設更新及保留計時器|
|`show cdp interface [type number]`|說明 CDP 是否在每個介面或所列的單一介面上啟用|
|`show cdp traffic`|列出發送及接收 CDP 通告次數之全域統計|

### CDP 預設狀態

預設情況下 CDP 是開啟的。

![[Assets/Note/Research/CDP 與 LLDP 鄰居探索協定指南/01-CDP 預設狀態.png]]

#### 查看 CDP 鄰居

```cisco
SW2#show cdp neighbors
```

![[Assets/Note/Research/CDP 與 LLDP 鄰居探索協定指南/02-查看 CDP 鄰居.png]]

#### 關閉 CDP

```cisco
SW(config)#no cdp run
```

### CDP 設定指令

```cisco
# 全域啟用 CDP
SW(config)#cdp run

# 全域關閉 CDP
SW(config)#no cdp run

# 在特定介面啟用 CDP
SW(config-if)#cdp enable

# 在特定介面關閉 CDP
SW(config-if)#no cdp enable

# 設定 CDP 更新間隔（秒）
SW(config)#cdp timer seconds

# 設定 CDP 保留時間（秒）
SW(config)#cdp holdtime seconds
```

## LLDP (Link Layer Discovery Protocol)

### 協定說明

LLDP 定義於 IEEE 標準 802.1AB，與 CDP 相仿，提供了與 CDP 相同功能的標準化協定。LLDP 作為 Ethernet 上層的第二層協定，不依靠第三層協定。

### 基本查看指令

```cisco
# 查看 LLDP 鄰居
SW#show lldp neighbors

# 查看特定鄰居詳細資訊
SW#show lldp entry R2

# 查看 LLDP 鄰居詳細資訊
SW#show lldp neighbors detail

# 查看 LLDP 統計資訊
SW#show lldp traffic

# 查看 LLDP 介面狀態
SW#show lldp interface
```

### LLDP 設定步驟

#### 1. 全域啟用 LLDP

```cisco
SW(config)#lldp run
```

#### 2. 在介面上啟用 LLDP

```cisco
SW(config-if)#lldp transmit    # 啟用傳送
SW(config-if)#lldp receive     # 啟用接收
```

### LLDP 完整設定指令

```cisco
# 全域啟用 LLDP
SW(config)#lldp run

# 全域關閉 LLDP
SW(config)#no lldp run

# 在介面上啟用 LLDP 傳送
SW(config-if)#lldp transmit

# 在介面上關閉 LLDP 傳送
SW(config-if)#no lldp transmit

# 在介面上啟用 LLDP 接收
SW(config-if)#lldp receive

# 在介面上關閉 LLDP 接收
SW(config-if)#no lldp receive

# 設定 LLDP 更新間隔（秒）
SW(config)#lldp timer seconds

# 設定 LLDP 保留時間（秒）
SW(config)#lldp holdtime seconds

# 設定 LLDP 重新初始化延遲（秒）
SW(config)#lldp reinit seconds
```

## CDP vs LLDP 比較

|特性|CDP|LLDP|
|---|---|---|
|**廠商**|Cisco 專有|IEEE 標準（802.1AB）|
|**相容性**|僅 Cisco 設備|多廠商設備|
|**預設狀態**|啟用|關閉|
|**設定複雜度**|簡單|需要額外設定|
|**功能**|豐富|標準化|

## 安全考量

> [!warning] 安全提醒 CDP 和 LLDP 都會洩露網路拓撲資訊，在面向公網的介面上建議關閉這些協定

### 安全設定建議

```cisco
# 在面向外網的介面關閉 CDP
Router(config-if)#no cdp enable

# 在面向外網的介面關閉 LLDP
Router(config-if)#no lldp transmit
Router(config-if)#no lldp receive
```

## 故障排除

### 常見問題與解決方案

#### 問題 1：看不到鄰居設備

**可能原因**：

- CDP/LLDP 未啟用
- 介面層級的協定被關閉
- 網路連線問題

**解決方案**：

```cisco
# 檢查全域狀態
SW#show cdp
SW#show lldp

# 檢查介面狀態
SW#show cdp interface
SW#show lldp interface
```

#### 問題 2：資訊不完整

**可能原因**：

- 計時器設定過短
- 網路不穩定

**解決方案**：

```cisco
# 調整計時器
SW(config)#cdp timer 90
SW(config)#cdp holdtime 270
```

## 最佳實務建議

1. **內部網路**：保持啟用以便於管理和故障排除
2. **外部介面**：關閉以提高安全性
3. **混合環境**：優先使用 LLDP 以支援多廠商設備
4. **監控**：定期檢查鄰居資訊以偵測未授權設備
5. **文檔記錄**：將探索到的拓撲資訊記錄在網路文檔中