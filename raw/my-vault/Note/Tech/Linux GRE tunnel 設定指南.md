---
Type:
  - Linux P2P
OS:
  - Debian
---
## topology

![[Assets/Note/Tech/Linux GRE tunnel 設定指南/01-topology.png|01-topology.png]]

## GRE Tunnle

打開轉發功能

```Shell
sudo vim /etc/sysctl.conf
net.ipv4.ip_forward = 1
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.all.send_redirects = 0
```

暫存  
VPNS1:  

```Shell
ip tunnel add netb mode gre remote 20.0.0.1 local 10.0.0.1 (生成tunnel)
ip link set gre1 up (啟用vti)
ip addr add 50.50.50.1 dev gre1
ip route add 192.168.2.0/24 dev gre1
```

VPNS2:

```Shell
ip tunnel add neta mode gre remote 10.0.0.1 local 20.0.0.1 (生成tunnel)
ip link set gre1 up (啟用vti)
ip addr add 50.50.50.2 dev gre1
ip route add 192.168.1.0/24 dev gre1
```

永久  
VPNS1:  

```Shell
vim /etc/network/interface
auto gre1
iface gre1 inet static
address 50.50.50.1/24
pre-up ip tunnel add gre1 mode gre remote 20.0.0.1 local 10.0.0.1
post-down ip tunnel del gre1
up /sbin/ip route add 192.168.2.0/24 dev gre1
```

R2:

```Shell
vim /etc/network/interface
auto gre1
iface gre1 inet static
address 50.50.50.2/24
pre-up ip tunnel add gre1 mode gre remote 10.0.0.1 local 20.0.0.1
post-down ip tunnel del gre1
up /sbin/ip route add 192.168.1.0/24 dev gre1
```