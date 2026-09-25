## 概述

HSRP、VRRP 和 GLBP 提供 Gateway 位址的高可用性機制。當主要路由器故障時，備援路由器會自動接替角色。

然而，若是直接連接故障的路由器可能會導致封包無法轉送，尤其當交換機或路由器的處理器故障時。部分 Cisco Catalyst 系列（如 4500R、6500、6800）具備備援監控模組，可在發生故障時接手，確保不中斷的運作。

---

## 備援交換監控模組（Supervisor Module Redundancy）

- 支援機型：Catalyst 4500R / 6500 / 6800
    
- 機箱內安裝兩個 supervisor module
    
- 首個啟動者為 `active`，另一個為 `standby`
    
- 若 active 故障，standby 模組會接手其職責
    

### 三種備援模式：

#### 1. RPR (Route Processor Redundancy)

- standby 模組未完全初始化
    
- active 故障時，standby 需重新啟動所有模組
    
- 切換時間最久
    

#### 2. RPR+ (Route Processor Redundancy Plus)

- standby 已開機，但未啟用 L2/L3 功能
    
- 切換較快，但不如 SSO
    

#### 3. SSO (Stateful Switchover)

- standby 完整啟動且與 active 同步 L2 狀態
    
- 切換時幾乎無中斷，最佳選擇
    

---

## 設定備援模式

### 切換時間比較

|備援模式|故障切換時間|
|---|---|
|RPR|> 2 分鐘|
|RPR+|> 30 秒|
|SSO|< 1 秒|

### 檢查狀態

```Plain
SW# show redundancy states
```

### 設定命令

```Plain
SW(config)# redundancy
SW(config-red)# mode {rpr | rpr-plus | sso}
```

> [!important] 若設定 RPR+，主備兩者需為相同 IOS 版本

### 同步 startup config / bootvar 等設定

```Plain
SW(config)# redundancy
SW(config-red)# main-cpu
SW(config-red-mc)# auto-sync {startup-config | config-register | bootvar}
```

也可使用簡化指令：

```Plain
auto-sync standard
```

---

## NSF（Non-Stop Forwarding，不間斷轉送）

SSO 模式下，可啟用 NSF 以快速同步路由資訊。其運作流程為：

- RIB (Routing Information Base) → FIB (Forwarding Information Base)
    
- FIB 下載至所有支援 CEF 的硬體中
    

### 常見協定 NSF 設定指令

|協定|設定指令|
|---|---|
|BGP|`router bgp <asn>` → `bgp graceful-restart`|
|EIGRP|`router eigrp <asn>` → `nsf`|
|OSPF|`router ospf <process-id>` → `nsf`|
|IS-IS|`router isis` → `nsf [cisco|

> [!info] NSF 提供快速路由表恢復功能，有助於減少網路中斷時間
