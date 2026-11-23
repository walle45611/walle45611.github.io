---
blog: true
blog_title: "Cisco errordisable 指令使用"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-errdisable-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

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
    - psecure-violation : 偵測到 port security 違規
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
            Switch(config)#errdisable recovery cause inline-power
            Switch(config)#errdisable recovery interval 3600
            ```

可用的偵測原因與恢復原因依交換器型號及 IOS/IOS XE 版本而異；請用裝置的 CLI 說明確認支援項目。port-security 的恢復原因名稱為 psecure-violation。
