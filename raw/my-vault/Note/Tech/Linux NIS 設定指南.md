概念:集中管理Linux Server上的帳號密碼

執行種類:Master,Slave,Client

## Topology

![[Assets/Note/Tech/Linux NIS 設定指南/01-Topology.png|01-Topology.png]]

## Master設定方法:

```Shell
apt install nis
vim /etc/default/nis
```

![[Assets/Note/Tech/Linux NIS 設定指南/02-Master設定方法.png|02-Master設定方法.png]]

```Shell
vim /etc/ypserv.securenets
```

![[Assets/Note/Tech/Linux NIS 設定指南/03-Master設定方法.png|03-Master設定方法.png]]

```Shell
vim /etc/hosts
```

![[Assets/Note/Tech/Linux NIS 設定指南/04-Master設定方法.png|04-Master設定方法.png]]

```Shell
vi /etc/defaultdomain 建立新的檔案預設沒有
```

![[Assets/Note/Tech/Linux NIS 設定指南/05-Master設定方法.png|05-Master設定方法.png]]

```Shell
systemctl restart rpcbind ypserv yppasswdd ypxfrd
systemctl enable rpcbind ypserv yppasswdd ypxfrd
/usr/lib/yp/ypinit -m 加入要得NIS Server加入完按 ctrl + D 在按下 y
```

![[Assets/Note/Tech/Linux NIS 設定指南/06-Master設定方法.png|06-Master設定方法.png]]

![[Assets/Note/Tech/Linux NIS 設定指南/07-Master設定方法.png|07-Master設定方法.png]]

  

## Slave設定方法:

```Shell
先在hosts file加入Master的ip和domain
vim /etc/hosts
```

![[Assets/Note/Tech/Linux NIS 設定指南/08-Slave設定方法.png|08-Slave設定方法.png]]

```Shell
vim /etc/defaultdomain
```

![[Assets/Note/Tech/Linux NIS 設定指南/09-Slave設定方法.png|09-Slave設定方法.png]]

在Master上

```Shell
/usr/lib/yp/ypinit -m 
```

![[Assets/Note/Tech/Linux NIS 設定指南/10-Slave設定方法.png|10-Slave設定方法.png]]

回到Slave

```Shell
	vim /etc/ypserv.securenets
```

![[Assets/Note/Tech/Linux NIS 設定指南/11-Slave設定方法.png|11-Slave設定方法.png]]

```Shell
vim /etc/default/nis
```

![[Assets/Note/Tech/Linux NIS 設定指南/12-Slave設定方法.png|12-Slave設定方法.png]]

```Shell
systemctl restart ypserv
systemctl enable ypserv
/usr/lib/yp/ypinit -s NISM.test.com 同步NIS master
```

## Client設定方法:

```Shell
apt install nis
vim /etc/yp.conf 從 NISM 和 NISS 同步
```

![[Assets/Note/Tech/Linux NIS 設定指南/13-Client設定方法.png|13-Client設定方法.png]]

```Shell
vim /etc/nsswitch.conf
```

![[Assets/Note/Tech/Linux NIS 設定指南/14-Client設定方法.png|14-Client設定方法.png]]

```Shell
vi /etc/defaultdomain
```

![[Assets/Note/Tech/Linux NIS 設定指南/15-Client設定方法.png|15-Client設定方法.png]]

```Shell
vim /etc/pam.d/common-session
```

![[Assets/Note/Tech/Linux NIS 設定指南/16-Client設定方法.png|16-Client設定方法.png]]

```Shell
systemctl restart rpcbind nscd ypbind
systemctl enable rpcbind ypbind
```