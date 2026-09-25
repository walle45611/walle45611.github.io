---
Type:
  - Linux Basic
OS:
  - Fedora
---
### change NIC Name

- command
    
    ```PowerShell
    $ vim /etc/udev/rules.d/70-persistent-ipoib.rules
    在最下面加上
    ACTION=="add",SUBSYSTEM=="net",ATTR{address}=="MAC address",DRIVERS=="?*",NAME="eth0"
    ```
    

### change basic network config file

- command 
    
    ```PowerShell
    $ vim /etc/sysconfig/network-script/ifcfg-["nic name"]
    
    DEVICE="eth0"               <==網路卡代號，必須要 ifcfg-eth0 相對應
    HWADDR="08:00:27:71:85:BD"  <==就是網路卡位址，若只有一張網卡，可省略此項目
    ONBOOT="yes"                <==是否預設啟動此介面的意思
    BOOTPROTO=none              <==取得IP的方式，其實關鍵字只有dhcp，手動可輸入none
    IPADDR=192.168.1.100        <==就是 IP 啊
    NETMASK=255.255.255.0       <==就是子網路遮罩
    GATEWAY=192.168.1.254       <==就是預設路由
    # 重點是上面這幾個設定項目，底下的則可以省略的囉！
    NETWORK=192.168.1.0         <==就是該網段的第一個 IP，可省略
    BROADCAST=192.168.1.255     <==就是廣播位址囉，可省略
    ```