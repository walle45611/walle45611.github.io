
# Linux networking 常用指令

集中整理網路設定、連線診斷與封包分析。診斷範例以 Ubuntu／Debian 為主，適用於 Linux VM、Container 與一般主機；NetworkManager、舊版 RHEL／CentOS 設定與 macOS 差異另列。範例中的 `eth0`、IP、port 請依實際環境替換。

回到 [[DevOps Technology Overview]] · 基礎操作見 [[Linux 基本觀念和指令]]

## 安裝與用途速查

Linux 是否預裝取決於發行版與映像，精簡 Container 尤其可能缺少工具。先用 `command -v ss`、`command -v tcpdump` 等確認。

| 指令 | 主要用途 | Ubuntu／Debian 套件 | macOS |
|---|---|---|---|
| `ping` | ICMP 可達性與延遲 | `iputils-ping`，一般系統常有 | 通常內建 |
| `ip` | 介面、位址、路由、鄰居表 | `iproute2`，一般系統常有 | 無原生 Linux 版本 |
| `ss` | Socket、監聽 port、連線 | `iproute2`，一般系統常有 | 無原生 Linux 版本 |
| `nc` | TCP／UDP 連線測試 | `netcat-openbsd`，可能需安裝 | 通常內建，BSD 衍生版 |
| `socat` | 資料流連接、轉發、multicast | `socat`，通常需安裝 | `brew install socat` |
| `tcpdump` | 擷取與快速檢查封包 | `tcpdump`，可能需安裝 | 通常內建 |
| `tshark` | 協定解析、封包欄位擷取 | `tshark`，通常需安裝 | `brew install wireshark` |
| `dig` | DNS 查詢 | `dnsutils`，可能需安裝 | 通常內建 |
| `curl` | HTTP／HTTPS、API 測試 | `curl`，可能需安裝 | 通常內建 |
| `traceroute` | 路徑與各跳回應 | `traceroute`，可能需安裝 | 通常內建 |
| `mtr` | 持續觀察路徑、延遲與丟包 | `mtr-tiny`，通常需安裝 | `brew install mtr` |
| `arping` | 同一 L2 網路的 IPv4 ARP 測試 | 本筆記使用 `iputils-arping` | 可裝 `arping`，參數不同 |

```bash
sudo apt update
sudo apt install -y iproute2 iputils-ping netcat-openbsd socat \
  tcpdump tshark dnsutils curl traceroute mtr-tiny iputils-arping
```

macOS 已有 Homebrew 時，可補裝：

```bash
brew install socat wireshark mtr arping
tshark --version
```

Homebrew 的 `wireshark` formula 提供命令列工具；GUI 是另外的 cask。套件安裝完成不代表已有抓包權限，Linux 可能需要設定 `dumpcap` 權限，Container 也受 capabilities 限制。

## NetworkManager 與 IP 設定

本節集中管理主機 IP 與連線設定。`nmtui`、`nmcli` 屬於 NetworkManager；原筆記的 `net-tools` 並不提供這兩個工具。僅在由 NetworkManager 管理網路的主機使用。

發行版完整設定另見 [[Linux Fedora 設定 IP 指南]] 與 [[Linux Ubuntu or Debian Set hostname and IP address]]。

- command line change ip setting
    - nmtui
        
        ![[Assets/Note/Tech/Linux 基本觀念和指令/03-IP address - nmtui.png|03-IP address - nmtui.png]]
        
    - nmcli
        - 查看
            
            ```Shell
            nmcli device status
            ```
            
            ![[Assets/Note/Tech/Linux 基本觀念和指令/04-IP address - 查看.png|04-IP address - 查看.png]]
            
        - 更改
            
            ```Shell
            nmcli con mod eth0 ipv4.method manual ipv4.addresses 172.25.0.25/24 ipv4.gateway 172.25.0.254 ipv4.dns 172.25.0.254
            ```
            
        - 新增
            
            ```Shell
            nmcli con add con-name "連接名稱" ifname [介面名稱] type ethernet
            ```
            
    - 重啟
        
        ```Shell
        sudo systemctl restart NetworkManager
        ```
        
### 舊版 RHEL／CentOS 設定方式

以下保留原筆記的 network-scripts 與 udev 設定範例，使用前須確認發行版是否仍支援；不適用於所有 Ubuntu／Debian 主機。

- 設定ip位置
    
    ```Shell
    vim /etc/sysconfig/network-scripts/ifcfg-[NicName]
    ```
    
- 更改nic名稱
    
    ```Shell
    vim /etc/udev/rules.d/nic.rules
    
    SUBSYSTEM==”net”,ACTION==”add”,ATTR{type}==”1”,ATTR{address}==”mac address”,DIRVERR==”?*”,NAME=”eth0”
    ```
    


## 連通性、介面與路由：ping、ip

| 指令 | 用途／常用參數 |
|---|---|
| `ping -c 4 192.168.1.1` | `-c` 指定次數 |
| `ping -n -c 4 -W 2 192.168.1.1` | `-n` 不反查名稱；Linux `-W` 回應等待秒數 |
| `ping -I eth0 192.168.1.1` | `-I` 指定來源介面或位址 |
| `ping -6 -c 4 ::1` | IPv6 測試 |
| `ping -M do -s 1472 -c 4 192.168.1.1` | Linux IPv4 禁止分片測試；1472 payload + 20 IP + 8 ICMP = 1500 bytes，未計額外封裝 |
| `ip -br addr` | `-br` 簡要列出介面 IP |
| `ip -br link` | 簡要列出介面與 link 狀態 |
| `ip -s link show dev eth0` | `-s` 顯示傳送／接收統計 |
| `ip route` | IPv4 路由表 |
| `ip -6 route` | IPv6 路由表 |
| `ip route get 1.1.1.1` | 查詢此目的地會使用的路由、介面與來源 IP |
| `ip neigh` | ARP／NDP 鄰居表 |
| `ip netns list` | 已命名的 network namespace |
| `sudo ip netns exec ns1 ip addr` | 在既有 `ns1` namespace 中查看位址 |

以下會修改網路設定，通常不會持久保存；遠端主機修改時可能影響目前連線。

```bash
sudo ip link set dev eth0 up
sudo ip link set dev tun0 mtu 1492
```

`ping` 失敗可能只是 ICMP 被擋；成功也不能證明 TCP port 或應用服務正常。

## Socket 與連線：ss

| 參數 | 意義 |
|---|---|
| `-t` / `-u` | TCP／UDP |
| `-l` | 只列監聽 Socket；UDP 常顯示 `UNCONN` |
| `-n` | 以數字顯示位址與 port |
| `-p` | 顯示 process／PID；查看其他使用者的資訊可能需要 root |
| `-a` | 同時包含監聽與非監聽 Socket |
| `-s` | Socket 統計摘要 |

| 指令 | 用途 |
|---|---|
| `sudo ss -tulnp` | TCP／UDP 監聽 port 與對應程序 |
| `ss -tan` | 全部 TCP Socket |
| `sudo ss -tanp` | 全部 TCP Socket 與程序 |
| `ss -tan state established` | 已建立的 TCP 連線 |
| `sudo ss -ltnp 'sport = :8080'` | 查本機 TCP 8080 是否監聽 |
| `ss -tn 'dport = :443'` | 查遠端 port 為 443 的 TCP Socket |
| `ss -s` | 連線統計 |

`sport` 是本機端 port，`dport` 是對端 port。服務若只綁定 `127.0.0.1`，其他主機不能直接連入。

## TCP／UDP 測試：nc

以下使用 Ubuntu／Debian 的 **netcat-openbsd** 語法。BusyBox、traditional netcat、Ncat 與 macOS 的選項有差異，先看 `nc -h` 或 `man nc`。

| 參數 | 意義 |
|---|---|
| `-l` | 監聽模式 |
| `-k` | 搭配 `-l`，連線結束後繼續接受新連線 |
| `-u` | 使用 UDP；預設 TCP |
| `-z` | 探測 port，不進行一般資料傳輸 |
| `-v` | 顯示較詳細資訊 |
| `-n` | 不解析名稱；搭配數字 IP |
| `-w 3` | 連線／閒置逾時秒數；OpenBSD 版本不適用監聽等待 |
| `-p 50000` | 指定來源 port；本版監聽 port 放在位置參數，不用 `-p` |

| 指令 | 用途 |
|---|---|
| `nc -zvn -w 3 192.168.1.10 443` | 測試 TCP 443 是否可建立連線 |
| `nc -zv -w 3 192.168.1.10 20-25` | 測試指定 port 範圍 |
| `nc -l 8080` | TCP server，等待一個連線 |
| `nc -lk 8080` | TCP server，持續接受後續連線 |
| `nc 192.168.1.10 8080` | TCP client，輸入文字測試 |
| `nc -u -l 5000` | UDP 接收端 |
| `nc -u 192.168.1.10 5000` | UDP 傳送端，輸入文字測試 |

UDP 沒有 TCP 握手，`nc -zu` 顯示成功不能證明服務存在。應確認接收端真的收到資料，或搭配抓包與應用層回應。

## 資料流與轉發：socat

基本結構是 `socat [options] ADDRESS1 ADDRESS2`，預設雙向傳輸；`-` 代表 stdin／stdout。單向接收可用 `-u`，資料由第一個 address 流向第二個。

| 參數／address 選項 | 意義 |
|---|---|
| `-d -d` | 顯示更多診斷訊息 |
| `-v` / `-x` | 將傳輸內容以文字／十六進位形式記錄到 stderr |
| `-u` | 單向傳輸；與 `nc -u` 的 UDP 意義不同 |
| `TCP4-LISTEN:8080` | IPv4 TCP 監聽 |
| `TCP4:host:port` | IPv4 TCP 連線 |
| `UDP4-RECV:5000` | 接收 IPv4 UDP datagram |
| `UDP4-SENDTO:host:port` | 傳送 IPv4 UDP datagram |
| `reuseaddr` | 設定 `SO_REUSEADDR`，方便重新綁定 |
| `fork` | 支援此選項的 address 為連線／請求建立子程序 |
| `bind=127.0.0.1` | 指定本機綁定位址 |
| `ip-add-membership=群組:本機IP` | 加入 IPv4 multicast group |

| 情境 | 指令 |
|---|---|
| TCP server | `socat -d -d TCP4-LISTEN:8080,reuseaddr -` |
| TCP client | `socat - TCP4:192.168.1.10:8080` |
| UDP 接收 | `socat -u UDP4-RECV:5000,reuseaddr -` |
| UDP 傳送 | `socat -u - UDP4-SENDTO:192.168.1.10:5000` |
| 本機 TCP 轉發至遠端 80 | `socat TCP4-LISTEN:8080,bind=127.0.0.1,reuseaddr,fork TCP4:192.168.1.10:80` |
| 加入 multicast 並接收 | `socat -u UDP4-RECV:5000,reuseaddr,ip-add-membership=239.1.1.1:10.2.2.10 -` |
| 指定介面 IP 傳送 multicast | `socat -u - UDP4-SENDTO:239.1.1.1:5000,ip-multicast-if=10.2.2.10,ip-multicast-ttl=1` |

Multicast 範例中的 `10.2.2.10` 必須是該主機實際介面的 IP。TCP 轉發使用 `fork` 服務多個連線；互動 stdin 範例不加 `fork`，避免多個子程序共用終端輸入。

## 抓包與分析：tcpdump、tshark

| 比較 | tcpdump | tshark |
|---|---|---|
| 適合情境 | 快速抓包、查看位址／port／TCP flags | 詳細協定分析、抽取欄位 |
| 指定介面 | `-i eth0` | `-i eth0` |
| 列出可抓包介面 | `-D` | `-D` |
| 停用名稱解析 | `-nn` | `-n` |
| 封包數量 | `-c 100` | `-c 100` |
| 寫入／讀取封包檔 | `-w` / `-r` | `-w` / `-r` |
| Capture filter | 指令尾端 BPF expression | `-f`，BPF expression |
| Display filter | 不支援 Wireshark 語法 | `-Y`，Wireshark 語法 |
| 更詳細輸出 | `-v`、`-vv`、`-vvv` | `-V` |
| 檔案格式 | 一般輸出 pcap | 預設 pcapng；`-F pcap` 指定 pcap |

**Capture filter** 在擷取時縮小保留範圍；**display filter** 在解析時選取符合條件的封包。`port 53` 與 `dns` 並不完全等價：前者比對 port，後者比對解析出的協定。

| tcpdump 常用指令 | 用途 |
|---|---|
| `sudo tcpdump -i eth0 -nn -c 100` | 抓 100 個封包 |
| `sudo tcpdump -i any -nn 'tcp port 443'` | Linux 跨介面觀察 TCP 443 |
| `sudo tcpdump -i eth0 -nn 'host 192.168.1.10 and port 53'` | 指定主機的 DNS port 流量 |
| `sudo tcpdump -i eth0 -nn 'arp or icmp or icmp6'` | ARP 與 ICMP／ICMPv6 |
| `sudo tcpdump -i eth0 -nn -s 0 -w capture.pcap` | 保存封包；`-s 0` 使用工具的完整擷取長度設定 |
| `tcpdump -nn -r capture.pcap 'tcp port 443'` | 離線篩選與讀取 |
| `sudo tcpdump -i eth0 -nn -A 'tcp port 80'` | `-A` 顯示 ASCII payload |
| `sudo tcpdump -i eth0 -nn -X 'udp port 53'` | `-X` 顯示十六進位與 ASCII |

```bash
# 擷取符合 BPF 條件的封包，60 秒後停止
tshark -i eth0 -f 'port 53' -a duration:60 -w dns.pcapng

# 離線使用 display filter
tshark -r dns.pcapng -Y 'dns'

# 只輸出指定欄位
tshark -r dns.pcapng -Y 'dns.flags.response == 0' \
  -T fields -e ip.src -e ipv6.src -e dns.qry.name

# 明確指定 pcap 格式
tshark -i eth0 -c 100 -F pcap -w capture.pcap
```

TShark 範例假設已有擷取權限。即時擷取並搭配 `-w` 存檔時，使用 `-f`；不要把 `-Y` 當成此情境的存檔篩選。改副檔名不會改檔案格式。HTTPS payload 已加密，`-A`／`-X` 不會直接顯示解密後內容。

## DNS、HTTP 與路徑診斷

| 指令 | 用途／參數 |
|---|---|
| `dig example.com A` | 查 IPv4 位址；也可改查 `AAAA`、`MX`、`TXT`、`NS` |
| `dig @1.1.1.1 example.com A` | 指定 DNS server |
| `dig +short example.com` | 精簡結果 |
| `dig +tcp example.com` | 使用 TCP 查詢 DNS |
| `dig -x 192.168.1.10` | 反向 DNS 查詢 |
| `curl -I https://example.com` | 發送 HEAD 請求、查看 headers |
| `curl -v --connect-timeout 3 --max-time 10 https://example.com` | 詳細連線過程；連線逾時 3 秒、整體上限 10 秒 |
| `curl -L https://example.com` | 跟隨重新導向 |
| `curl -sS -o /dev/null -w '%{http_code}\n' https://example.com` | 只輸出 HTTP 狀態碼並保留錯誤訊息 |
| `traceroute -n 1.1.1.1` | 不反查名稱的路徑追蹤 |
| `sudo traceroute -T -p 443 -n 1.1.1.1` | Linux TCP 443 路徑追蹤 |
| `mtr -n -r -w -c 20 1.1.1.1` | 20 輪、數字位址、寬版報告 |
| `sudo arping -I eth0 -c 4 192.168.1.1` | iputils 版：指定介面，測同一 L2 的 IPv4 ARP 回應 |

中間 hop 不回覆或有丟包，不一定代表轉送失敗；需一起看後續 hop 與目的端。`arping` 不會經過一般 IP 路由器測到另一個子網。

## macOS 對照

| Linux | macOS 常用替代 |
|---|---|
| `ip addr` / `ip link` | `ifconfig` |
| `ip route` | `netstat -rn` |
| `ip route get 1.1.1.1` | `route -n get 1.1.1.1` |
| `ip neigh` | IPv4：`arp -a`；IPv6：`ndp -a` |
| `ss -ltnp` | `sudo lsof -nP -iTCP -sTCP:LISTEN` |
| `ss -uanp` | `sudo lsof -nP -iUDP` |

Linux `ping -W` 以秒計；macOS 的 `-W` 以毫秒計。Linux `iputils-arping` 的介面參數是 `-I`，Homebrew 的另一套 `arping` 通常使用 `-i`。不要直接跨系統照抄，先看該機 `man`／help。介面名稱也不同，可先用 `ifconfig` 或 `tcpdump -D` 查詢。

## 排錯順序

| 步驟 | 先回答的問題 | 常用工具 |
|---|---|---|
| 1 | 本機 IP、介面與路由正確嗎？ | `ip -br addr`、`ip -br link`、`ip route get` |
| 2 | 鄰居與目的地是否可達？ | `ip neigh`、`arping`、`ping` |
| 3 | 名稱能解析嗎？ | `dig` |
| 4 | 服務有監聽、遠端 port 能建立連線嗎？ | 服務端 `ss`，用戶端 `nc` |
| 5 | 應用層正常回應嗎？ | `curl` 或對應協定 client |
| 6 | 封包停在哪裡、回應內容是什麼？ | `tcpdump`、`tshark`，必要時兩端同時抓包 |

以上是排查線索，不必等到最後才抓包。服務狀態與日誌指令統一見 [[Linux 基本觀念和指令#服務管理與日誌：systemctl、journalctl|服務管理與日誌]]。

## 來源

- [原始對話：比較抓包工具](chatgpt-conversation://6ab2bf8b-3b8c-83e8-90d3-e7e681a370a6)
- [TShark 官方手冊](https://www.wireshark.org/docs/man-pages/tshark.html)：篩選、欄位輸出與檔案格式。
- [OpenBSD nc 手冊](https://man.openbsd.org/nc)：OpenBSD 系列選項；實際版本以本機手冊為準。
- [Homebrew wireshark formula](https://formulae.brew.sh/formula/wireshark)：macOS 命令列套件。
- [Debian iputils-arping 手冊](https://manpages.debian.org/bookworm/iputils-arping/arping.8.en.html)：ARP 與介面參數。
- 本機手冊：`man ip`、`man ss`、`man ping`、`man socat`、`man tcpdump`、`man dig`、`man curl`、`man mtr`。
