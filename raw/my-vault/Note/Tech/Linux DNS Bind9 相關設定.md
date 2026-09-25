# 為甚麼需要DNS

簡單來說你記不得ip位置所以需要DNS來輔助你記憶這是個最主要的原因，在Linux DNS server通常以bind9最為常見所以以下都是以bind9來做示範

## 安裝

```Shell
apt install bind9 bind9-doc bind9-utils
```

## 設定

### zone setting

- vim /etc/bind/named.conf.local
    
    ![[Assets/Note/Tech/Linux DNS Bind9 相關設定/01-zone setting.png|01-zone setting.png]]
    

### zone file setting

- 正解 /var/cache/bind/db.skills39.com
    
    ![[Assets/Note/Tech/Linux DNS Bind9 相關設定/02-zone file setting.png|02-zone file setting.png]]
    
- 反解 /var/cache/bind/db.1.168.192
    
    ![[Assets/Note/Tech/Linux DNS Bind9 相關設定/03-zone file setting.png|03-zone file setting.png]]