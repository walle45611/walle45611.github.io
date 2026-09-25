---
Type:
  - Linux Web
OS:
  - Debian
---
### 安裝nginx

```Shell
apt install nginx
```

### 啟動nginx

```Shell
$nginx 啟動
$nginx -s stop 快速關閉
$nginx -s quit 關閉，先做完在關閉之前的訪問再關閉
$nginx -s reload 重新加載配置
```

### nginx主要文件

```Shell
/etc/nginx
├── conf.d
├── fastcgi.conf
├── fastcgi_params
├── koi-utf
├── koi-win
├── mime.types
├── modules-available  
├── modules-enabled   
│   ├── 50-mod-http-geoip.conf -> /usr/share/nginx/modules-available/mod-http-geoip.conf
│   ├── 50-mod-http-image-filter.conf -> /usr/share/nginx/modules-available/mod-http-image-filter.conf
│   ├── 50-mod-http-xslt-filter.conf -> /usr/share/nginx/modules-available/mod-http-xslt-filter.conf
│   ├── 50-mod-mail.conf -> /usr/share/nginx/modules-available/mod-mail.conf
│   ├── 50-mod-stream.conf -> /usr/share/nginx/modules-available/mod-stream.conf
│   └── 70-mod-stream-geoip.conf -> /usr/share/nginx/modules-available/mod-stream-geoip.conf
├── nginx.conf
├── proxy_params
├── scgi_params
├── sites-available 通常會將vhost的實體檔案放在這裡，在使用ln指令連結到sites-enabled
│   └── default
├── sites-enabled
│   └── default -> /etc/nginx/sites-available/default
├── snippets
│   ├── fastcgi-php.conf
│   └── snakeoil.conf
├── uwsgi_params
└── win-utf

6 directories, 20 files
```

### 網頁預設放的位置

```Shell
/var/www/
└── html
    └── index.nginx-debian.html

1 directory, 1 file
```

### log訊息

```Shell
$vim /etc/nginx/nginx.conf
access_log /var/log/nginx/access.log 每個人訪問都會被記錄在此
error_log /var/log/nginx/error.log 系統錯誤放到這裡或是訪問錯誤
```

### nginx處理請求流程

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/01-nginx處理請求流程.png|01-nginx處理請求流程.png]]

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/02-nginx處理請求流程.png|02-nginx處理請求流程.png]]

### mine.types

- mine.types是server傳送給用戶端對於檔案的處理方式要顯示還是下載之類的。
- example:今天有個exe檔在server上用戶端請求這個檔案server回應的時候不是查看附檔名而是去查看mine.types裡面的屬性去做相對應的選擇。

```Shell
vim /etc/nginx/mine.types
```

---

## sendfile

這是有沒有啟用sendfile的時候的topology，這樣看起來是nginx先複製一份html文件和test.mp4到nginx的內存裡面再發送給網卡緩存這樣，這樣多複製了一份檔案所以這樣比較慢。

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/03-sendfile.png|03-sendfile.png]]

這是有啟用sendfile的時候的topology，這樣看來啟動sendfile的好處是不用再經過nginx的調用，是直接從網卡緩從直接存取test.mp4這樣會比較快。

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/04-sendfile.png|04-sendfile.png]]

---

VHost (Virtual Host)

可以一台Web server上運行多個VHost

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/05-sendfile.png|05-sendfile.png]]

```Shell
server {
       listen 80;
       listen [::]:80;

       server_name example.com;

       root /var/www/example.com;
       index index.html;

			 \#http://test.com/test/test1 url
			 \#/test/test1 uri
       location / {
               try_files $uri $uri/ =404;
       }

}
```

實際

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/06-sendfile.png|06-sendfile.png]]

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/07-sendfile.png|07-sendfile.png]]

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/08-sendfile.png|08-sendfile.png]]

---

## everse proxy and load balancer

### **forward** proxy

用戶端主動向外網訪問但是訪問不到所以需要正向代理伺服器 以CLT角度來看

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/09-forward proxy.png|09-forward proxy.png]]

  

### reverse proxy

以Server交度來看是Server需要代理給CLT，反著代理了，所以也稱為反向代理，其實兩個的本質都是相同的，但是角度不同所以名稱不同

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/10-reverse proxy.png|10-reverse proxy.png]]

### load balancer

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/11-load balancer.png|11-load balancer.png]]

- weight
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/12-load balancer - weight.png|12-load balancer - weight.png]]
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/13-load balancer - weight.png|13-load balancer - weight.png]]
    
    這個參數可以應用在Applacation Server兩台硬體配置不同的時候，來調整權重值讓輪尋更好，以上面的圖來說Weight8的機器每10次就會有8次輪到，Weight2的機器就會每10次就輪到2次，這個算法不是說太準確，但是可以這樣推估。
    
- Down
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/14-load balancer - Down.png|14-load balancer - Down.png]]
    
    可以暫時的把某台Server不參予load balance
    
- backup
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/15-load balancer - backup.png|15-load balancer - backup.png]]
    
    等到所有load balacne的機器都不能跑的時候這台才會啟動
    

### 附載平衡的策略(很少用通常都要寫腳本，因為這些方法都有缺陷)

因為輪旬的用法會有一些缺陷所以需要更多的策略來輔助輪巡附載平衡，例如以下情況等於說我登入了兩次，這樣會讓使用者崩潰。

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/16-附載平衡的策略(很少用通常都要寫腳本,因為這些方法都有缺陷).png|16-附載平衡的策略(很少用通常都要寫腳本,因為這些方法都有缺陷).png]]

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/17-附載平衡的策略(很少用通常都要寫腳本,因為這些方法都有缺陷).png|17-附載平衡的策略(很少用通常都要寫腳本,因為這些方法都有缺陷).png]]

- ip_hash
    
    裡用使用者的IP來算出一個Hash vlaue然後放到同一台Application Server上，但是這個方法很少用。
    
- least_conn
    
    將流量分配給最少流量的機器，但是這個方法很少用。
    
- url_hash
    
    需要裝插件，根據用戶url的定向轉發請求，這個在訪問固定資源的時候比較常用到，不再統一Server上。
    
- fair
    
    需要裝插件，根據後端Server響應時間來轉發請求
    

## 實作

- topology
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/18-實作 - topology.png|18-實作 - topology.png]]
    
- Nginx1
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/19-實作 - Nginx1.png|19-實作 - Nginx1.png]]
    
- Nginx 2
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/20-實作 - Nginx 2.png|20-實作 - Nginx 2.png]]
    
- Nginx3
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/21-實作 - Nginx3.png|21-實作 - Nginx3.png]]
    
- CLT測試
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/22-實作 - CLT測試.png|22-實作 - CLT測試.png]]
    
    ![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/23-實作 - CLT測試.png|23-實作 - CLT測試.png]]
    

### https

沿用上面的topology

![[Assets/Note/Tech/Linux Nginx Web Server 設定指南/24-https.png|24-https.png]]

直接在DC上請求certificate然後再用ssh的方式送到Nginx1上

```Shell
openssl pkcs12 -in [filename.pfx] -clcerts -nokeys -out [certificatename.crt]
openssl pkcs12 -in [filename.pfx] -nocerts -out [certificatename.key]
openssl rsa -in tmp.key -out tmp.key.unsecure
```

### 使用systemctl啟動nginx

```Shell
$systemctl enable nginx 在開機的時候自動開啟nginx
$systemctl restart nginx
$systemctl stop nginx
$systemctl start nginx
```

---