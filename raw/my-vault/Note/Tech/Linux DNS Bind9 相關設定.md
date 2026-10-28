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

## BIND9 的 DNSSEC：簽署與驗證是兩件事

**權威伺服器替自己管理的 zone 簽署資料；validating resolver 驗證查回來的資料。** `dnssec-policy` 管 zone 簽署，`dnssec-validation` 管 resolver 驗證，不能互相替代。

原理先看 [[DNS (Domain Name Service)#DNSSEC 的紀錄與信任鏈]]；DNSSEC／DoT／DoH 比較見 [[DNS (Domain Name Service)#DNSSEC、DoT、DoH：保護什麼？]]。

> [!note] 範例的確認範圍
> 以下範例以 ISC BIND 9.20.16 文件為參考，尚未實機驗證。先用 `named -v` 確認版本，再對照該版本 ARM。範例網域與位址需換成自己的環境。

### 權威端：自動簽署 zone

假設 unsigned zone 已存在，而且一般 DNS 查詢正常。以下設定是局部片段，併入既有設定，不要重複建立同名 zone。

```conf
zone "example.com" {
    type primary;
    file "/var/cache/bind/db.example.com";
    key-directory "/var/cache/bind/keys";
    dnssec-policy default;
    inline-signing yes;
};
```

- `dnssec-policy default;` 讓 BIND 管理金鑰與簽章；9.20.16 的內建 default 使用單一 ECDSAP256SHA256 CSK，金鑰 lifetime 無期限。不能把它解讀成一定定期換 key，升級也應重新確認 policy。
- `inline-signing yes;` 維護獨立 signed 版本；明寫可讓意圖清楚。相關預設與 dynamic update 要求依版本確認。
- `named` 執行帳號必須能寫入 key 目錄與 signed／journal 所在目錄；目錄需先建立，並符合發行版的 AppArmor／SELinux 限制。
- 本例維護原始 unsigned zone；修改資料時更新原始 zone 及 SOA serial，再 reload，讓 BIND 重簽，不直接改 `.signed` 或金鑰狀態檔。動態更新 zone 另依其更新流程處理。

```bash
named -v
named-checkconf
named-checkzone example.com /var/cache/bind/db.example.com
# 確認檢查通過後，在有管理權限的環境重讀設定
rndc reconfig
# 後續修改既有靜態來源 zone 時
rndc reload example.com
```

檢查權威端是否發布簽章；以下假設該權威服務在本機：

```bash
dig @127.0.0.1 example.com SOA +dnssec +norecurse
dig @127.0.0.1 example.com DNSKEY +dnssec +norecurse
```

查得到 DNSKEY／RRSIG，表示有相關資料，**尚不能證明公網信任鏈完整**。服務異常時核對實際 unit 名稱，查看 `bind9` 或 `named` 的狀態與日誌。

### 把 DS 交給父層

1. 先確認所有對外權威伺服器都已提供正確 signed zone。
2. 選取本 zone 正確的 KSK／CSK 公鑰，產生 DS；下面檔名僅示意，不能原樣執行。

```bash
dnssec-dsfromkey -2 /var/cache/bind/keys/Kexample.com.+013+12345.key
```

3. 依註冊商／父 zone 管理者的流程提交，讓 DS 發布在父 zone。DS 的 key tag、algorithm、digest type、digest 必須對應正確金鑰。
4. 查父層 DS，並經 validating resolver 或具正確 anchor 的驗證工具確認整條鏈。

**BIND 自動產 key、重簽與依政策管理生命週期，不代表註冊商會自動建立 DS。** CDS／CDNSKEY 可協助父層更新，但父層必須支援且有可信的導入流程；`parental-agents`／`checkds` 是檢查父層 DS 狀態，也不是直接替你修改父 zone。

### Resolver：啟用 DNSSEC validation

公網 resolver 的典型設定片段：

```conf
options {
    dnssec-validation auto;
};
```

`auto` 使用 BIND 提供、管理的公開 root trust anchor。這不會替自己的權威 zone 簽署資料。對外服務前，也要依服務對象設定 recursion／query ACL，避免把內部 resolver 意外開放到公網。

### dig、AD、CD 與 delv 的差別

以下 `192.0.2.53` 是文件示例，換成自己的 validating resolver；`example.com` 也換成要驗證的 zone。

```bash
dig @192.0.2.53 example.com A +dnssec
dig @192.0.2.53 example.com DS +dnssec
dig @192.0.2.53 example.com DNSKEY +dnssec
delv @192.0.2.53 example.com A
```

| 檢查 | 能確認什麼 |
|---|---|
| `dig +dnssec` | 設 DO bit 請求 DNSSEC 資料；dig 本身不因此做完整密碼學驗證 |
| 回應的 `ad` flag | 回覆的 resolver 宣稱資料通過驗證；仍須信任該 resolver 與取得此回覆的通道 |
| `delv` | 使用自身的驗證邏輯與 trust anchor 驗證；私有 root lab 需另外提供其 anchor |
| `dig +cdflag` | 請 resolver 停用該次查詢的 checking，協助定位驗證問題；不是修復方式 |

若普通查詢 SERVFAIL，但相同 resolver、相同查詢加 `+cdflag` 後可取得資料，應進一步查 DNSSEC 日誌、DS／DNSKEY、簽章有效期與系統時間。SERVFAIL 也可能有其他原因，不能只憑回應碼定案。

```bash
dig @192.0.2.53 example.com A +dnssec
dig @192.0.2.53 example.com A +dnssec +cdflag
```

常見檢查方向：

- 父層沒有 DS 且能證實是不安全委派：通常仍可解析，結果是 insecure；不等同 bogus。
- 父層已有 DS，但金鑰不匹配或簽章失效：可能成為 bogus，正常 checking 下回 SERVFAIL。
- 時鐘偏差、RRSIG 過期、錯誤的 key rollover 時序。
- DNSSEC 回應較大；檢查 UDP 路徑、EDNS、MTU 與 TCP 53 fallback 是否可用。

### 從 root 開始的私有 DNSSEC lab

私有 DNSSEC lab 可拆成四個角色：

| 角色 | 負責內容 |
|---|---|
| 私有 root 權威 | `.` zone，委派私有 TLD，保存 TLD 的 DS |
| 私有 TLD 權威 | 例如 `test.` zone，委派 `example.test.`，保存子 zone DS |
| 子 zone 權威 | `example.test.` zone，發布 DNSKEY 與帶 RRSIG 的資料 |
| Validating resolver | 使用私有 root hints 與私有 root trust anchor |

實驗使用隔離的 `.test` 階層，避免與公開網域混淆。

實作順序：

1. 先建立一般委派 NS、必要 glue 與各層 SOA／NS／A，確認 resolver 能沿私有階層查到答案。
2. 各層 zone 各自簽署，可用 `dnssec-policy` 自動維護。
3. 從 `example.test.` 的 KSK／CSK 產 DS，交給 `test.`；從 `test.` 的金鑰產 DS，交給私有 root。父 zone 更新後也要重簽。
4. Resolver 的 `zone "." { type hint; ... };` 指向私有 root hints。**Hints 負責找到 root，trust anchor 負責信任 root，兩者都需要。**
5. 以私有 root 公鑰設置 `trust-anchors`，並使用 `dnssec-validation yes;` 管理自訂 anchor；不要將同名靜態 root anchor 與 `auto` 的公開 root 自動驗證混用。
6. 測試成功的 secure 答案，再分別測試移除 DS、DS 不匹配、簽章過期等情況，對照 insecure 與 bogus。

手動簽署工具包括：`dnssec-keygen` 產 key、`dnssec-signzone` 簽 zone、`dnssec-dsfromkey` 產 DS。手動路徑還要 include 公鑰、載入 signed zone、安排重簽與換 key，不能只跑一次簽名指令就視為完成。本節是練習設計，尚未提供可直接部署的完整 lab 檔案。

### BIND 與 DoT／DoH 的關係

**DNSSEC 處理 DNS 資料驗證，DoT／DoH 處理加密傳輸，兩者分開設定。** DNSSEC 的 DNSKEY 不是 TLS 憑證；開 dnssec-policy 不會自動開啟 DoT／DoH。

## DoT／DoH：options、tls 與 http 放在哪裡？

**DoT／DoH 的 listen-on 寫在 options；tls 與 http 的命名區塊放在 named.conf 最外層，和 options 同層。** 檔案名稱不決定語法層級；Debian／Ubuntu 常把這些內容放在由 named.conf include 的 named.conf.options。

| 設定 | 所在層級 | 用途 |
|---|---|---|
| acl | 最外層 | 定義可使用服務的來源範圍 |
| tls | 最外層，不能塞在 options 裡 | 定義服務端憑證、私鑰等 TLS 連線設定 |
| http | 最外層，不能塞在 options 裡 | 定義 DoH endpoint，例如 /dns-query |
| listen-on／listen-on-v6 | options 裡 | 設定監聽位址、port，引用 tls／http 名稱 |
| dnssec-validation | options 等支援位置 | 驗證 resolver 查詢結果，不是開加密 listener |
| dnssec-policy | 簽署 zone 常直接寫在 zone 裡 | 管該 zone 的簽署，不是開 DoH 的設定 |

### 內網 resolver 的最小設定範例

沿用本篇 BIND 9.20.16 參考版本；未實機啟動。範例前提：本機持有 192.168.1.10，授權使用者在 192.168.1.0/24，服務名稱 dns.example.com 的 TLS 憑證、私鑰已備妥；憑證 SAN 要符合服務名稱，Client 也要信任簽發 CA。所有 IP、名稱與路徑均須換成實際值。

**以下是要併入既有設定的結構範例，不要直接覆蓋整份檔案或重複建立 options。** 保留既有 directory、forwarders、zone include 等需要的設定；這裡示範 IPv4，IPv6 需另外依既有政策配置 listen-on-v6。

```conf
acl "dns-clients" {
    127.0.0.1;
    192.168.1.0/24;
};

tls "dns-tls" {
    cert-file "/etc/bind/tls/fullchain.pem";
    key-file "/etc/bind/tls/privkey.pem";
};

http "dns-http" {
    endpoints { "/dns-query"; };
};

options {
    recursion yes;
    allow-query { dns-clients; };
    allow-recursion { dns-clients; };
    allow-query-cache { dns-clients; };
    dnssec-validation auto;

    listen-on port 53 { 127.0.0.1; 192.168.1.10; };
    listen-on port 853 tls "dns-tls" { 192.168.1.10; };
    listen-on port 443 tls "dns-tls" http "dns-http" { 192.168.1.10; };
};
```

| 入口 | 這個範例做什麼 |
|---|---|
| 一般 DNS 53 | 保留一般 DNS 服務；不是自動強迫 Client 使用加密 DNS |
| DoT TCP 853 | 直接引用 dns-tls，加密傳送 DNS |
| DoH TCP 443 | 同時引用 dns-tls、dns-http，以 HTTPS 接收 /dns-query |
| DNSSEC 驗證 | auto 使用公開 root trust anchor 驗證外部結果，不替自己的 zone 簽署 |

**是 named 自己提供 DoH 的 HTTPS endpoint，不會自動啟動 Lighttpd。** 本例不需要另外裝 Web Server；若已有網站占用同 IP 的 443，需另選位址／port，或設計反向代理入口。

- TLS 憑證與私鑰需讓 named 執行帳號能讀取，並符合 AppArmor／SELinux 等規則；不要把私鑰設成任意人可讀。
- 此處要求 Client 驗證伺服器憑證，不代表一定使用雙向 TLS。要求 Client 也出示憑證是另外的 mTLS 設定，與 query／recursion ACL 也不是同一層。
- HTTP/2／TLS 能力要看 BIND 版本與建置選項；一般 DoH 可使用 HTTP/3，不代表這份 BIND 範例就會提供 HTTP/3。
- 只設定 Client 的 nameserver IP 不會自動升級 DoH／DoT；OS／瀏覽器要明確配置相應傳輸與服務名稱。
- 上述是給 Client 連入的 listener，不表示 BIND 向所有上游／權威查詢也改成 DoT／DoH。

### 檢查與 Client 驗證

在 DNS Server 上確認版本與設定；檢查通過後再依環境重讀：

```bash
named -v
named -V
sudo named-checkconf
# 檢查通過、檔案與憑證可讀後才重讀
sudo rndc reconfig
# Linux 上確認實際監聽的 53／853／443 與程序
sudo ss -lntup
```

named-checkconf 不能代替實際 TLS 握手、埠占用與可達性測試；重讀後仍需查 named／bind9 日誌。防火牆只允許預期 Client 與必要入口；此例不是公開遞迴 resolver 的部署設定。

Client 測試需要支援相關參數的新版 dig。以下 /path/to/ca.pem 是 **Client 本機**信任的 CA 檔案，dns.example.com 必須符合伺服器憑證 SAN；不是把伺服器私鑰複製給 Client。

```bash
# 一般 DNS
dig @192.168.1.10 example.com A

# DoT：加上伺服器憑證與名稱驗證
dig @192.168.1.10 example.com A +tls \
  +tls-ca=/path/to/ca.pem +tls-hostname=dns.example.com

# DoH：HTTPS endpoint，加上同樣的 TLS 身分驗證
dig @192.168.1.10 example.com A +https=/dns-query \
  +tls-ca=/path/to/ca.pem +tls-hostname=dns.example.com
```

以上 CA 路徑需換成實際值；若使用受系統信任的公開 CA，可依 dig 版本使用不帶檔名的 +tls-ca。**只加 +tls／+https 不能當作已驗證伺服器憑證的證據**；BIND 9.20.16 的 dig 要用 +tls-ca 啟用該驗證。

| 現象 | 優先檢查 |
|---|---|
| 不認得 +https／+tls | Client dig 版本與建置，不是立刻判定 Server 沒開 |
| Connection refused／timeout | listener、IP、port 占用、防火牆與路由 |
| 憑證錯誤 | SAN、信任 CA、完整憑證鏈、有效期與系統時間 |
| REFUSED | query／recursion／cache ACL，是否允許實際 Client 來源 |
| SERVFAIL | 上游解析與 DNSSEC，接著查日誌；不只因為 HTTPS 就認定是 TLS 錯誤 |

比較與完整原理：[[DNS (Domain Name Service)#DNSSEC、DoT、DoH：保護什麼？]]、[[DNS (Domain Name Service)#DoT／DoH 的連線與防火牆：DNS Server 在哪裡？]]。

## 參考資料

- [ISC DNSSEC Guide](https://bind9.readthedocs.io/en/v9.20.16/dnssec-guide.html)：簽署、父層 DS、resolver 與排錯。
- [ISC Configuration Reference](https://bind9.readthedocs.io/en/v9.20.16/reference.html)：`dnssec-policy`、`inline-signing`、trust anchor、`listen-on`、`tls`、`http`。
- [ISC dig Manual](https://bind9.readthedocs.io/en/v9.20.16/manpages.html#dig-dns-lookup-utility)：`+tls`、`+https`、`+tls-ca`、`+tls-hostname`。
