## l2tp/ipsec pre-share-key

安裝

```Shell
$apt install xl2tp strongswan
```

設定ipsec strongswan

```Shell
$vim /etc/ipsec.conf

設定內容加在最下面

config setup

conn L2TP-PK
	keyexchange=ikev1
	authby=secret
	leftprooprot=17/1701
	leftfirewall=no	
	rightprotoport=17/%any
	type=transport	
	auto=add
```

設定l2tp xl2tp

```Shell
$vim /etc/xl2tpd/xl2tpd.conf
[global]
port = 1701
ipsec saref=no

在最下面加上
[lns default]
ip range=192.168.1.1-192.168.1.20
local ip = 192.168.1.254
length bit =yes
refuse pap=yes
refuse authentication=no
name=l2tp
pppoptfile=/etc/ppp/options.l2tpd.lns
```

設定ppp

```Shell
$cp /etc/ppp/options /etc/ppp/options.l2tpd.lns
$vim /etc/ppp/options.l2tpd.lns
require-mschap-v2
name l2tpd
```

---

## l2tp/ipsec certificate ikev2