## GRE原理 (Generic Routing Encapsulation)

==主要目的是為了要在IPv4 internal上傳送非IPv4的封包(example: IPv6)==

![[Assets/Note/Research/GRE 介紹/01-GRE原理 (Generic Routing Encapsulation.png|01-GRE原理 (Generic Routing Encapsulation.png]]

  

![[Assets/Note/Research/GRE 介紹/02-GRE原理 (Generic Routing Encapsulation.png|02-GRE原理 (Generic Routing Encapsulation.png]]

Transport IP Header:

1. SIP=(R1 public ip)
2. DIP=(R2 public ip)

如果R2收到GRE Header他就會拆解封包並取得 Passenger(IP) Packet的內容

## command

ping通就成功了

```Plain
R1(config)#int tunnel test
R1(config-if)#ip address 172.16.12.1 255.255.255.0
R1(config-if)#tunnel source e0/0
R1(config-if)#tunnel destination 10.0.0.24.2
```

```Plain
R2(config)#int tunnel test
R2(config-if)#ip address 172.17.12.1 255.255.255.0
R2(config-if)#tunnel sourcce e0/0
R2(config-if)#tunnel destination 10.0.14.1
```
