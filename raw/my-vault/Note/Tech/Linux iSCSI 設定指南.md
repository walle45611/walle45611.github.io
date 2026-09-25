---
Type:
  - iscsi
OS:
  - Debian
---
# iscsi

![[Assets/Note/Tech/Linux iSCSI 設定指南/01-iscsi.png]]

iSCSI是一種協議，該協議允許您在本地計算機上使用遠程存儲設備。使用iSCSI，您可以將存儲設備（如硬盤驅動器）連接到本地計算機而無需實際將它們連接到本地計算機。以下是在Debian上安裝iSCSI的方法：

## target

- 在Debian上安裝iSCSI目標，你可以使用 `tgt` 軟體包。你可以使用以下命令安裝它：
    
    ```Plain
    sudo apt-get install tgt lvm2
    ```
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/01-target.png|01-target.png]]
    
- vgs
    
    ```Shell
    vagcreate tecmint_iscsi /dev/sd{a,c}
    vgs
    ```
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/02-target - vgs.png|02-target - vgs.png]]
    
- lvcreate
    
    ```Shell
    lvcreate -l 100%FREE tecmint_lun1 tecmint_iscsi
    lvs
    ```
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/03-target - lvcreate.png|03-target - lvcreate.png]]
    
- 安裝完成後，你可以通過編輯 `/etc/tgt/conf.d/` 配置文件來配置iSCSI目標。你需要為每個想要創建的iSCSI目標創建一個新的配置文件。
    - example:
        
        ```Shell
        <target iqn.2019-02.tecmint.cmo:un1>
        	backing-store /dev/mapper/tecmint_iscsi-tecmint_lun1
        	initator-address 192.168.56.102
        	incominguser tecmint-iscsi-user password
        	outgoinguser debian-iscsi-target secretpass
        </target>
        ```
        
- restart iscsi target
    
    ```Shell
    systemctl restart tgt
    ```
    
- tgtadm check
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/04-target - tgtadm check.png|04-target - tgtadm check.png]]
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/05-target - tgtadm check.png|05-target - tgtadm check.png]]
    

## Initiator

- 安裝iscsi initiator
    
    ```Shell
    apt install open-iscsi
    ```
    
- 準備初始資料
    
    ```Shell
    iscsiadm -m discovery -t st -p 192.168.56.101
    ```
    
- vim edit
    
    ![[Assets/Note/Tech/Linux iSCSI 設定指南/06-Initiator - vim edit.png|06-Initiator - vim edit.png]]
    
- change
    
    ```Shell
    node.session.auth.authmethod = CHAP                    \#Enable CHAP Authentication
    node.session.auth.username = tecmint-iscsi-user        \#Target to Initiator authentication
    node.session.auth.password = password                  \#Target to Initiator authentication
    node.session.auth.username_in = debian-iscsi-target    \#Initiator to Target authentication
    node.session.auth.password_in = secretpass             \#Initiator to Target authentication
    ```
    
- 如果按照這個指南進行，此時 'node.startup' 選項將設置為 '手動'，這可能不是期望的。如果管理員希望在系統啟動時自動連接 iSCSI 目標，請將 'manual' 改為 'automatic'，方法如下：
    
    ```Shell
    node.startup = automatic
    ```
    
- systemctl restart 
    
    ```Shell
    systemclt restart open-iscsi
    ```
    
- check
    - iscsiadm
        
        ```Shell
        iscsiadm -m session
        ```
        
        ![[Assets/Note/Tech/Linux iSCSI 設定指南/07-Initiator - iscsiadm.png|07-Initiator - iscsiadm.png]]
        
    - lsblk
        
        ```Shell
        lsblk
        ```
        
        ![[Assets/Note/Tech/Linux iSCSI 設定指南/08-Initiator - lsblk.png|08-Initiator - lsblk.png]]