- 基本概念
    - Transaction（交易）：區塊鏈上的一筆操作，包含多個輸入（Inputs）與多個輸出（Outputs）。
    - UTXO：尚未被消費的交易輸出，每個 UTXO 代表一筆可以再次使用的「數量單位」。
    - 帳本狀態：節點維護所有 UTXO 集合，當輸出被下一筆交易使用後即從集合移除。
        
- 模型特性比較
    - 餘額管理：UTXO 模型將所有屬於同一地址的 UTXO 相加；帳戶模型則直接更新帳戶餘額。
    - 隱私性：UTXO 模型較佳（多筆 UTXO 不易追蹤整體餘額）；帳戶模型較差。
    - 並行驗證：UTXO 可同時驗證不同 UTXO；帳戶模型需鎖定帳戶，並行較差。
    - 實作複雜度：UTXO 需挑選與管理 UTXO；帳戶模型直接讀寫帳戶狀態。
        
- 鎖定與解鎖腳本
    - 鎖定腳本（scriptPubKey / Locking Script）：附加在交易輸出上，規定消費此 UTXO 的條件：
        ```text
        OP_DUP                # 複製堆疊頂部資料（公鑰）
        OP_HASH160            # 對公鑰做 SHA256 + RIPEMD160
        <PubKeyHash>          # 公鑰雜湊
        OP_EQUALVERIFY        # 驗證雜湊是否相等
        OP_CHECKSIG           # 驗證簽章合法
        ```
    - 解鎖腳本（scriptSig / Unlocking Script）：附加在交易輸入上，提供滿足鎖定條件的證明：
        ```text
        <Signature>           # 私鑰簽名結果
        <PublicKey>           # 公鑰，用於驗證簽章
        ```
        
- 典型流程（Alice → Bob）
    - 挖礦獎勵：礦工產生一筆鎖定給 Alice 的 UTXO#A。
    - Alice 發送交易：
        - Inputs：指向 UTXO#A，並附上 Alice 的解鎖腳本（scriptSig）。
        - Outputs：
            - 1.0000 BTC 鎖定給 Bob。
            - 找零金額鎖定給 Alice。
    - 節點驗證：
        - 將解鎖腳本與鎖定腳本串接執行，透過 OP_CHECKSIG 驗證簽章。
        - 驗證通過後，移除 UTXO#A，新增兩個新的 UTXO（Bob 的與 Alice 的找零）。
    - Bob 花用：Bob 在後續交易中使用他的解鎖腳本，解鎖鎖定給他的 UTXO。
        
- 範例交易
    - 範例 1：單一 UTXO 足額付款，含找零
        - 初始 UTXO：2.000 BTC → Alice
        - Outputs：
            - 1.300 BTC → Bob
            - 0.699 BTC → Alice (找零)
            - 0.001 BTC → 手續費 (礦工)
                
    - 範例 2：多 UTXO 聚合付款，無找零
        - 初始 UTXO：0.400 + 0.750 + 0.600 = 1.750 BTC → Carol
        - Outputs：
            - 1.700 BTC → Dave
            - 0.050 BTC → 手續費 (礦工)
                
- 小結
    - UTXO 模型以「可消費的輸出」為單位，不直接更新帳戶餘額。
    - 透過新增/移除 UTXO 管理餘額，具有防重放、隱私與並行驗證優勢。
    - 交易結構複雜度與錢包實作難度較高。