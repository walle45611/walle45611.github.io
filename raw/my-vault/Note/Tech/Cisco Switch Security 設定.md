---
blog: true
blog_title: "Cisco Switch Security 設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-switch-security/
blog_tag: OS & Networking
blog_topic: Networking
---

- [Port Protect](#port-protect)

---

## Port Protect

在某些情況呢，server不需搬動的，可以使用mac位址的對應去鎖定port，如果是移動式server可以使用switch動態的去學習mac address，將其加入對應port的列表。

- 啟用 port-security 功能
    
    ```Plain
    sw(config-if)#switchport port-security
    ```
    
- 指定一組允許的mac address，可以手動設定位址，或是根據交換機的port動態學習到位址，以下指令是指定最大mac address的數目
    
    ```Plain
    SW(config-if)#switchport port-security maximum max-addr
    ```
    
    - 預設每個介面最多允許一個 secure MAC address；支援的最大值依平台而異。
    - 啟用後，交換器會學習 secure MAC address。這些位址存在 secure MAC address table，不是 ARP table；預設不會因閒置而老化。若要啟用老化，需另行設定 port-security aging。
- 如果交換機重開機不想讓學習到的位址發生改變
    
    ```Plain
    SW(config-if)#switchport port-security mac-address sticky
    ```
    
    - Sticky 學到的位址會寫入 running-config；要在重新開機後保留，必須另存 startup-config。
- 也可以定義多個mac address這些都能夠透過該port存取網路
    
    ```Plain
    SW(config-if)#switchport port-security mac-address mac-addr
    ```
    
    - 如果設定mac addr的數目沒有超過設定的最大值，其餘的會用動態的方式
    - example
        
        ```Plain
        SW(config-if)#switchport port-security mac-address 0006.5b02.a841
        ```
        
- 最後設定mac位址錯誤時的反應
    
    ```Plain
    SW(config-if)#switchport port-security violation {shutdown | restrict | protect}
    ```
    
    - 超過最大值
    - 位置非靜態定義的mac
    - shutdown : 進入Errdisable
    - Restrict：介面保持 up，丟棄來自未授權 MAC 的封包，並增加違規計數、送出 syslog 與 SNMP trap。
    - Protect：介面保持 up，丟棄來自未授權 MAC 的封包，但不記錄違規或增加計數。
    - Shutdown：介面進入 err-disabled 狀態。
    
    > [!important] 若 secure MAC address 已達上限，需移除不再使用的 secure 位址或提高上限，才會學習新的位址。清除動態 secure 位址的範例：`Switch#clear port-security dynamic interface g0/1`。此命令的可用選項依平台而異。
    
- example
    
    ```Plain
    int g0/1
        switchport access vlan 991
        switchport mode access
        switchport port-security
        switchport port-security violation restrict
        spanning-tree portfast
    ```
    
- show
    - 顯示防護狀態
        
        ```Plain
        show port-security interface g1/0/11
        ```
        
    - Errdisable 摘要訊息
        
        ```Plain
        show interfaces status err-disabled
        ```
        
    - 顯示全部的
        
        ```Plain
        show port-security
        ```
