---
Created: 2023-02-27T20:50
---
## 專業名稱

![[Assets/Note/Research/DNS (Domain Name Service)/01-專業名稱.png]]

---

## DNS Server 伺服器類型說明

- **Primary Server**：
    
    - 在 DNS 上建立區域後，若可以直接新增、刪除、修改該區域內的紀錄，稱為 Primary Server。
        
    - 儲存該區域的 master copy（主副本資料）。
        
- **Secondary Server**：
    
    - 這類伺服器無法直接編輯區域紀錄，而是從其他伺服器複寫紀錄。
        
    - 複寫來的紀錄稱為 replica。
        
- **Master Server**：
    
    - 提供區域資料給 Secondary Server 的伺服器稱為 Master Server。
        
    - 這種將資料從 Master 傳送至 Secondary 的動作稱為 **Zone Transfer**（區域傳輸）。
        
- **Caching-only Server**：
    
    - 不負責任何區域權威，只提供查詢與快取。
        
    - 查詢結果暫時存放於本地端，供下一次快速回覆使用。
        

---

## DNS 查詢流程與類型

![[Assets/Note/Research/DNS (Domain Name Service)/02-DNS 查詢流程與類型.jpg]]

- **DNS 查詢的類型**：
    
    - **遞迴式查詢（Recursive）**：
        
        - Client 向 DNS Server 查詢，若 DNS Server 沒有答案會繼續向其他伺服器查詢並回覆 client。
            
        - 常見於 DNS client 與 DNS Server、Forwarder。
            
    - **反覆式查詢（Iterative）**：
        
        - DNS Server 間互相查詢時多為此模式。
            
        - 若無答案，只回覆下一層 DNS Server 的資訊，讓查詢者自行遞迴下去。
            
- **查詢來源分類**：
    
    - **授權回應（Authoritative Answer）**：
        
        - DNS Server 為該名稱空間的授權伺服器，可提供正確回應或回報查無資料。
            
    - **非授權回應（Non-authoritative Answer）**：
        
        - DNS Server 僅回應來自快取的資料（DNS cache），可能透過轉寄（forward）或向根提示查詢獲得資料。
            

---

## HOSTS File 本機解析優先

系統會優先檢查本機 `C:\Windows\System32\drivers\etc\hosts` 檔案是否有對應的 host 紀錄，若無再查詢 DNS。

![[Assets/Note/Research/DNS (Domain Name Service)/01-HOSTS File 本機解析優先.png|01-HOSTS File 本機解析優先.png]]

---

## DNS Resource Record（資源紀錄）類型

- **SOA（Start of Authority）**：
    
    - 每個 Zone File 中必須且僅能有一筆。
        
    - 放在檔案的最前面，記載該區域的名稱伺服器、版本號（serial）、更新頻率等資訊。
        
    - 用於區域間的同步判斷（例如 Secondary Server 會依 SOA 判斷是否需要更新）。
        
- **NS（Name Server）**：
    
    - 指定此區域使用哪些 DNS 伺服器。
        
    - 可包含多筆，通常會包含 Primary 與多個 Secondary 的位址。
        
- （可擴充：A、AAAA、MX、PTR、CNAME、TXT 等常見記錄類型）