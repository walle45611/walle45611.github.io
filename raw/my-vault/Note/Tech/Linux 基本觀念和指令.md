---
Type:
  - Linux Basic
OS:
  - Debian
---
# Linux 基本觀念和指令

整理檔案與目錄、使用者與權限、服務與日誌、套件、SELinux 及排程。網路設定與診斷見 [[Linux networking 常用指令]]。

回到 [[DevOps Technology Overview]]。

## Linux 目錄分層

- 常見的目錄
    - bin sbin 存放可執行文件
    - dev 存放各種硬體設備
    - home 存放普通用戶的主目錄
    - mnt 管理員手動掛載一些外部設備的目錄
    - media 自動識別掛載的設備目錄
    - proc memory data
    - tmp 臨時文件
    - var 存放變化的數據 系統日誌 郵箱
    - boot 存放系統啟動文件
    - etc 存放各種系統配置文件
    - opt 第三方軟體的資源或安裝目錄
    - usr 存放雨用戶相關的各種數據
    - root 管理員帳號的主目錄

---

## Folder command

- ls （list）用來列出目錄下有哪些目錄，列出目錄有哪些文件屬性
    - -l (long), 長格式列出對象的詳細訊息
    - -h (human), 顯示更容易懂的容量單位
    - -d (directory), 只看目錄本身的訊息
- pwd 當前在哪個目錄下
- cd 改目錄 change directory
- ~ 家目錄
- su (super user) 切換到另一個用戶 建議加上 “-” 模擬登入過程
    - 管理員切換到其他用戶不用密碼
    - 普通用戶切換到其他帳號需要密碼
- mkdir (make directory) 創建新目錄
    - -p 如果要創建的目錄 沒有父目錄他會自動創建
- touch 創建指定名稱的文件
- cat 閱讀短文件 直接顯示整個文件
- less 用來用讀文件，顯示文件的第一屏內容通過PgUp PgDn q退出
- cp 用來複製目錄
    - -r 複製目錄
- rm 用來刪除目錄
    - -r 會遞迴刪除目錄和所有子檔案
    - -f 強制刪除
- mv 改變目錄名稱或是檔案位置

---

## vim (visual editor IMproved)

## 服務管理與日誌：systemctl、journalctl

適用於使用 systemd 的主機，將 `<服務名稱>` 換成實際 unit 名稱。

| 指令 | 用途 |
|---|---|
| `systemctl` | 列出載入中的 units 與狀態 |
| `systemctl status <服務名稱>` | 查看服務狀態 |
| `sudo systemctl start <服務名稱>` | 啟動服務 |
| `sudo systemctl stop <服務名稱>` | 停止服務 |
| `sudo systemctl restart <服務名稱>` | 重新啟動服務 |
| `sudo systemctl enable <服務名稱>` | 設定開機啟動 |
| `sudo systemctl disable <服務名稱>` | 取消開機啟動 |
| `journalctl -u <服務名稱> -n 50 --no-pager` | 查看指定服務最近 50 筆日誌 |
| `journalctl -xe` | 查看 journal 尾端並附加可用的說明 |

`enable`／`disable` 本身不會立即啟停服務；加上 `--now` 才會同時執行。

---

## firewall

- 防火牆的狀態的控制systemctl disable firewalld --now

---

## SELinux

- mode
    - Enforcing
    - Permissive
    - Disabled
- default selinux policy 
    
    ```Shell
    getschool -a
    setsebool -P policy_name=[on | off]
    ```
    
- change mode
    - setenforce [0 | 1]
    - vim /etc/selinux/config
        
        ![[Assets/Note/Tech/Linux 基本觀念和指令/01-SELinux.png|01-SELinux.png]]
        
- getenforce
    
    ![[Assets/Note/Tech/Linux 基本觀念和指令/02-SELinux - getenforce.png|02-SELinux - getenforce.png]]
    
- open port
    - 開啟http port
        
        ```Shell
        semanage port -a -t http_port_ -p 82
        ```
        

### selinux偵錯

- setroubleshoot
    
    ```Shell
    yum -y install setroubleshoot-server
    grep setrouble /var/log/message
    sealert -l
    ```
    
- 系統日誌的查看方式見 [[#服務管理與日誌：systemctl、journalctl]]。
    

### semanage 設定

- selinux 設定 port
    
    ```Shell
    semanage fcontext -l                    //列出所有預設的安全policy
    semanage fcontext -a -t                 //添加文檔的上下文policy
    semanage fcontext -d ...                //刪除某個上下文poliy
    semanage port -l                        //列出所有預設的port開放policy
    semanage port -a -t policy -p potocol port_number     //添加某個類別端口開放policy
    semanage port -d                        //刪除某個上下文policy
    ```
    

---

## 網路設定與診斷

IP、NetworkManager、連線測試與抓包統一整理於 [[Linux networking 常用指令]]；原本的 IP 設定內容已移至 [[Linux networking 常用指令#NetworkManager 與 IP 設定|NetworkManager 與 IP 設定]]。

---

## yum

- command
    
    ```Shell
    yum clean all                      // 清除cahce
    yum repolist                       // 列出可用的repo/source information
    yum list [軟體名稱..]              // 列出軟體的安裝情況
    yum info 軟體名稱..                // 查詢指定軟體的描述訊息
    yum -y install 軟體名稱..          // 安裝指定軟體
    yum -y reinstall 軟體名稱..        // 指定特定軟體重新安裝 (可以修復壞掉的或是丟失的文件) 
    yum -y remove 軟體名稱..           // 刪除指定的軟體
    yum -y updtae 軟體名稱..           // 升級指定的軟體
    yum search 關鍵字                  // 根據關鍵詞搜索相關的文件
    yum provides [軟體名稱..]          // 查詢哪一個軟體能提供xx文件
    ```
    
- repo
    
    ```Shell
    [repo_name]
    	name=描述
    	baseurl=repo_位置
    	enabled=1|0
    	gpgcheck=1|0
    	\#gpgkey=file://pki/rpm-gpg/PRM-GPG-KEY在gpgcheck=1的時候
    ```
    

---

## User

- useradd 新增使用者
    - -u uid
    - -g gid
    - -G groups
    - -d direcotry
    - -s login_shell
- usermod
    
      
    
- userdel 刪除使用者
    
    ```Shell
    userdel -r test
    ```
    
- passwd 
    
    ```Shell
    echo 1234 | passwd --stdin test
    ```
    
- change user
    
    ```Shell
    su
    su -
    ```
    

---

## Permissoins

- normal
    - chown
    - chmod
- acl
    
    ```Shell
    getfacl file_path
    setfacl -m user:user_name:permissions file_path
    setfacl -m group:group_name:permissions file_path
    setfacl -b file_path   //刪除所有acl
    ```
    
    - setfacl
        
        ```Shell
        setfacl -m u:test:--- /var/tmp/hosts //test沒有任何權限
        setfacl -m u:test1:rw /var/tmp/hosts //test1有讀寫權限
        ```
        

  

---

## tuned

設定目錄 : /etc/tuned、/usr/lib/tuned/優化的方案’

service name : tuned

- tuned-adm
    
    ```Shell
               
    tuned-adm list                  // 查看系統推薦的優化方案
    tuned-adm recommend             // 列出可用的優化方案
    tuned-adm profile <方案名稱>    // 切換為xx優化方案
    tuned-adm active               // 查看當前的活動方案
    tuned-adm off                  // 關閉優化
    ```
    

---

## crondtab

![[Assets/Note/Tech/Linux 基本觀念和指令/05-crondtab.png|05-crondtab.png]]

- 設定文件格式
    
    ```Shell
    * * * * *　分別代表了 分時日月週
    ```
    
- 時刻表示方式
    
    ```Shell
    *
    5-10
    5,7,9
    */5     // 每五分鐘
    1-15/3  // 假設放到日 每個月的1-15每三天
    ```
    
- install
    
    ```Shell
    dnf install crontabs
    ```
    
- 設定文件 不建議
    
    ![[Assets/Note/Tech/Linux 基本觀念和指令/06-crondtab - 設定文件 不建議.png|06-crondtab - 設定文件 不建議.png]]
    
- command tools
    - 目錄位置 [/var/spool/cron/username]
    - crontab -e [-u username]
    - crontab -l [-u username]
    - crontab -r [-u username]
- checklog
    - tall /var/log/cron
        
        ![[Assets/Note/Tech/Linux 基本觀念和指令/07-crondtab.png|07-crondtab.png]]