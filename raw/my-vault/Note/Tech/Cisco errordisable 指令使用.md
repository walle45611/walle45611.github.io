如果錯誤狀態被檢測到，switch port會進入errdisable狀態並關閉可以在全域模式下設定

- command
    
    ```Plain
    SW(config)#errdisable detect cause [all | cause-name]
    ```
    
- casue-name
    - all : 檢測每個原因
    - arp-inspection : 動態檢測ARP錯誤
    - bpduguard : 檢測STP portfast的switch port所接收到的BPDU
    - dhcp-rate-limit : 偵測DHCP窺探所發生的錯誤
    - dtp-flap :用來檢測trunking封裝變更的檢測
    - gbic-invalid : 偵測到無效的GBIC或是SFP mode
    - inline-power : 偵測到PoE發生的錯誤
    - link-flap : 偵測交換port反覆up down不穩定的flapping
    - loopback : 用介面的迴路檢測
    - pagp-flap : 偵測etherchannel狀態協議不一致
    - pppoe-ia-rate-limit : 偵測pppoe relay的速率限制所發生的錯誤
    - psecure-violatin : 偵測有關SW安全性觸發條件
    - psp : 偵測有關協定風暴保護的錯誤
    - security-violation : 偵測有關802.1X安全錯誤
- 錯誤狀態自動恢復
    
    errdisable錯誤發生時需要sh和no sh一次非常麻煩，所以可以設定errdisable的時間
    
    - command
        
        ```Plain
        switch(config)#errdisable recovery cause [all | cause-name]
        ```
        
        ```Plain
        Switch(config)#errdisable recovery interval seconds 
        ```
        
    - 範例
        - 當偵測到PoE錯誤停用，1 hr重新啟用
            
            ```Plain
            Switch(config)#errdiable recovery cause inline-power
            Switch(config)#errdiable recovery interval 3600
            ```