---
blog: true
blog_title: "Cisco DHCP 設定"
blog_date: '2026-11-23'
blog_url: https://blog.walle4561.com/articles/posts/cisco-dhcp-guide/
blog_tag: OS & Networking
blog_topic: Networking
---

- 可以先看這個
    

- [DHCP](#dhcp)
    - [DHCP綁定mac address](#dhcp綁定mac-address)
    - [DHCP relay](#dhcp-relay)
- [DHCPv6](#dhcpv6)
    - [stateless dhcpv6](#stateless-dhcpv6)
    - [stateful dhcpv6](#stateful-dhcpv6)
    - [DHCPv6 Relay](#dhcpv6-relay)

## DHCP

- command
    
    ```Plain
    SW(config)#ip dhcp excluded-address start-ip end-ip
    SW(config)#ip dhcp pool pool-name
    SW(dhcp-config)#network ip-address subnet-mask
    SW(dhcp-config)#default-router ip-address [ip-address2]
    SW(dhcp-config)#lease {days [hours [minutes]] | infinite}
    SW(dhcp-config)#exit
    ```
    
    - **excluded-address** 保留位址部分發的位址
    - **lease 代表租約時間**
- show
    
    這樣就可以看到用戶
    
    ```Plain
    SW#show ip dhcp binding
    ```
    
- clear
    
    ```Plain
    SW#clear ip dhcp binding {* | ip-address}
    ```
    

### DHCP綁定mac address

- 可以使用client-identifier或是hardware-address這兩個指令來指定
    
    ```Plain
    SW(dhcp-config)#host 192.168.1.99 255.255.255.0
    SW(dhcp-config)#client-identifier 0100.50b6.5bc0.b5
    ```
    

### DHCP relay

- command
    
    ```Plain
    SW(config-if)#ip helper-address x.x.x.x
    ```
    

---

## DHCPv6

### stateless dhcpv6

```Plain
R1(config)#ipv6 dhcp pool dhcp-pool
R1(config-dhcpv6)#dns-server 2001:1234:5678::2
R1(config-dhcpv6)#domain-name lij.local
R1(config)#interface e0/0
R1(config-if)#ipv6 address 2001:1234:5678::1/64
R1(config-if)#ipv6 nd other-config-flag
R1(config-if)#ipv6 dhcp server DHCP-Pool
```

- case
    
    ![[Assets/Note/Tech/Cisco DHCP 設定/01-stateless dhcpv6 - case.png|01-stateless dhcpv6 - case.png]]
    
    ```Plain
    ! config R1
    ipv6 local pool P1 2001:1234::/64 64
    ipv6 dhcp pool P2
    	prefix-delegation pool P1
    !
    interface f1/0
    	ipv6 address 2001::7/64
    	ipv6 dhcp server P2
    ```
    
    ```Plain
    ! config R2
    interface f1/0
    	ipv6 address autoconfig default 
    		ipv6 enable
    	ipv6 dhcp client pd P2
    !
    interface f0/0
    	ipv6 address P2 ::1234/64
    	ipv6 enable
    ```
    

### stateful dhcpv6

```Plain
R1(config)#ipv6 dhcp pool DHCPPOOL
R1(config-dhcpv6)#address prefix 2001:1234:5678:9abc::/64 lifetime
R1(config-dhcpv6)#dns-server 2001:1234:5678:9abc::1
R1(config-dhcpv6)#domain-name lij.local
R1(config-dhcpv6)#exit
R1(config)int e0/0
R1(config-if)#ipv6 address 2001:1234:5678:9abc::2/64
R1(config-if)#ipv6 enable
```

  

### DHCPv6 Relay

```Plain
SW(config-if)#ipv6 dhcp relay destination ipv6-address
```
