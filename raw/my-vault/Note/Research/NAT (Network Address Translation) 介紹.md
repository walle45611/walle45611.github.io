NAT容許私有地只轉換成外部網路位址(public ip)，這樣可以隱藏IP位址，也可以讓IPv4位址大大的足夠使用。

![[Assets/Note/Research/NAT (Network Address Translation) 介紹/01-NAT (Network Address Translation) 介紹.png|01-NAT (Network Address Translation) 介紹.png]]

|NAT IP|定義|
|---|---|
|內部本地|有關內部設備的private IP address|
|內部全域|有關內部設備public IP address|
|外部本地|有關外部設備的private IP address (在目的地位只用到NAT時可以看到)|
|外部全域|有關外部設備的公共IP|

|NAT IP|NAT IP address type|
|---|---|
|內部本地|10.1.1.1|
|內部本地|10.1.1.2|
|內部全域|198.51.100.3|
|內部全域|198.51.100.4|
|外部本地|N/A|
|外部全域|203.0.113.2|

## DNAT (dynamic NAT)

動態的指定內部位址從一組可用的位址裡自動指派內部全域位址 NAT Pool。

1. 設定ACL 設定內部的範圍
2. 在全域**ip nat pool** _pool_name strat_ip ending_ip_ **netmask** _mask_ 設定nat集群
3. 以**ip nat inside**指定內部介面
4. 以**ip nat outside**指定外部介面
5. 使用i**p nat inside source list** _acl_ **pool** _nat_pool_將acl和nat pool關聯起來

![[Assets/Note/Research/NAT (Network Address Translation) 介紹/02-DNAT (dynamic NAT).png|02-DNAT (dynamic NAT).png]]

```Plain
R1(config)#access-list 1 permit 10.1.1.0 0.0.0.255
R1(config)#ip nat pool ISP-POOL 192.51.100.3 198.51.100.14 netmask 255.255.255.240
R1(config)#ip nat inside source list 1 pool ISP-POOL
R1(config)#int e0/0
R1(config-if)#ip nat inside
R1(config)#int e0/1
R1(config-if)#ip nat outside
```

## SNAT (static NAT)

有些時候我們可能去靜態設定給網路內部特定的設備內部全域位址，公司的某某server之類的，是透過固定的IP address不是透過動態的去挑選位置。

1. **ip nat inside source static** _inside_local_address inside_global_address_ 命令產生內部本地位址與內部全域位址
2. 以**ip nat inside**指定內部介面
3. 以**ip nat outside**指定外部介面

```Plain
R1(config)#ㄙ
R1(config)#ip nat inside source static 10.1.1.2 192.51.100.4
R1(config)#int e0/0
R1(config-if)#ip nat inside
R1(config)#int e0/1
R1(config-if)#ip nat outside
```

## PAT (Port address translation)

使用port號對應的方式可以讓內部的機器去訪問外部時使用一個port訪問，這樣就可以節省外部IP的使用，也可以減少公司的開銷，因為DNAT會須要購買大量的IP，所以會需要很多的錢錢，使用PAT就可以得到相同的效果但是又省錢，但是缺點就是負載會比較大。

1. 設定ACL 設定內部的範圍
2. 在全域**ip nat pool** _pool_name strat_ip ending_ip_ **netmask** _mask_ 設定nat集群
3. 以**ip nat inside**指定內部介面
4. 以**ip nat outside**指定外部介面
5. 使用i**p nat inside source list** _acl_ **interface** _outside_interface_ **overload**

```Plain
R1(config)#access-list 1 permit 10.1.1.0 0.0.0.255
R1(config)#ip nat inside source list 1 interface e0/1 overload
R1(config)#int e0/0
R1(config-if)#ip nat inside
R1(config)#int e0/1
R1(config-if)#ip nat outside
```

## NVI (NAT Virtual interface，NVI)

傳統NAT是先做轉址，在做路由策略，但是NVI是先做初始路由策略，然後進行轉址，再進行最後使用外部IP外址的路由策略

```Plain
R1(config)#access-list 1 permit 10.1.1.0 0.0.0.255
R1(config)#int e0/0
R1(config-if)#ip nat enable
R1(config)#int e0/1
R1(config-if)#ip nat enable
R1(config)#ip nat inside source list 1 interface e0/1 overload
```

> [!important] NVI不支援SNAT