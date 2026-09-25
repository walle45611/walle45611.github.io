# Containerlab 安裝與設定指南

以 Apple Silicon Mac 搭配 OrbStack 的 Debian ARM64 Linux machine 建立網路實驗環境，先驗證 Containerlab，再加入 Cisco IOL Router 與 Switch。

來源：[Mac安裝網路模擬器](chatgpt-conversation://6aaf0ebe-f2b0-83ee-96dc-a5b133d99eed)；整理日期：2026-09-20。

## 環境架構

```text
macOS（Apple Silicon）
└── OrbStack
    └── Debian ARM64：containerlab
        ├── Docker Engine
        ├── Containerlab（clab）
        └── Lab nodes
            ├── Alpine Linux
            ├── Cisco IOL Router
            └── Cisco IOL-L2 Switch
```

Containerlab 透過 YAML 定義節點與連線，再建立容器及實驗網路。它需要 Linux 的 network namespace、veth、netlink 等功能，因此本流程在 Debian 裡執行 Containerlab 與 Docker。這是 [Containerlab macOS 官方指南](https://containerlab.dev/macos/)列出的方式。

OrbStack 自己提供的 Docker 環境，與 Debian machine 內安裝的 Docker Engine 是不同環境。以下 build、deploy、docker exec 都在同一台 Debian 裡操作，才能使用同一組映像與容器。

## 1. 建立並進入 Debian machine

先安裝並開啟 [OrbStack](https://orbstack.dev/)，在 Linux machines 建立 Debian，Apple Silicon 使用 ARM64，machine 名稱設為 `containerlab`，與原對話後續的實際提示符一致。

在 **Mac 終端機**直接進入指定 machine：

```bash
orb -m containerlab
```

在 **Debian 終端機**確認系統與架構：

```bash
uname -m
cat /etc/os-release
```

ARM64 預期顯示 `aarch64`。Debian 版本以實際輸出為準，不把原對話中的版本描述當成目前安裝版本。OrbStack 的 machine 操作可參考[官方文件](https://docs.orbstack.dev/machines/)。

更新套件並安裝操作、建置與除錯工具：

```bash
sudo apt update
sudo apt upgrade -y
sudo apt install -y curl ca-certificates wget git make vim tcpdump iproute2 iputils-ping traceroute dnsutils jq
```

## 2. 安裝 Docker 與 Containerlab

在 **Debian** 執行官方 quick setup：

```bash
curl -sL https://containerlab.dev/setup | sudo -E bash -s "all"
```

此腳本會安裝 Docker CE、Docker Compose、Containerlab 與 gh。完成後離開再重新進入 Debian，使群組權限生效；也可以先執行：

```bash
newgrp docker
```

驗證：

```bash
docker version
docker run --rm hello-world
containerlab version
clab version
```

`hello-world` 應顯示 `Hello from Docker!`；`clab` 是 Containerlab 的短指令名稱。

官方安裝頁的 quick setup 已測試清單仍列 Debian 11、12，因此較新 Debian 若安裝失敗，應依錯誤檢查 Docker 與 Containerlab 的個別安裝，不直接假設需要降級。參考 [Containerlab Installation](https://containerlab.dev/install/)。

## 3. 建立最小測試 Lab

在 Debian 建立目錄與拓撲檔：

```bash
mkdir -p ~/labs/test
cd ~/labs/test
vim lab.clab.yml
```

`lab.clab.yml`：

```yaml
name: first-lab

topology:
  nodes:
    pc1:
      kind: linux
      image: alpine:latest
    pc2:
      kind: linux
      image: alpine:latest

  links:
    - endpoints: ["pc1:eth1", "pc2:eth1"]
```

部署並查看狀態：

```bash
clab deploy -t lab.clab.yml
clab inspect -t lab.clab.yml
docker ps
```

預期產生 `clab-first-lab-pc1`、`clab-first-lab-pc2`。

從 Debian 設定兩台 PC 的實驗介面並測試：

```bash
docker exec clab-first-lab-pc1 ip addr add 10.0.0.1/24 dev eth1
docker exec clab-first-lab-pc1 ip link set eth1 up
docker exec clab-first-lab-pc2 ip addr add 10.0.0.2/24 dev eth1
docker exec clab-first-lab-pc2 ip link set eth1 up
docker exec clab-first-lab-pc2 ping -c 3 10.0.0.1
```

兩台 PC 位於同一個網段，這個測試不需要 default gateway。

```text
pc1 eth1                         pc2 eth1
10.0.0.1/24 ───── Lab link ───── 10.0.0.2/24
```

進入 Alpine 互動 shell：

```bash
docker exec -it clab-first-lab-pc1 sh
ip addr
ip route
exit
```

| 介面 | 用途 |
| --- | --- |
| `eth0` | Containerlab 自動建立的管理網路；原對話為 `172.20.20.0/24` |
| `eth1` | YAML `links` 定義的實驗連線 |

在 Debian 執行 `ip addr` 看到的是主機的介面、Docker bridge 與 veth；透過 `docker exec` 才是在查看 PC 容器內的介面。

## 4. 建置 Cisco IOL 映像

IOL 是在 Linux 執行的 Cisco binary；IOL-L2 用於交換器。Apple Silicon 上的 x86_64 IOL 需要轉譯支援，不能把它視為原生 ARM64 程式。Containerlab 官方 macOS 文件列有透過 Rosetta 執行 Cisco IOL 的案例。

`vrnetlab/cisco_iol` 不是保證能從 Docker Hub 直接拉取的公開 Cisco 映像；需要自行準備有使用授權的 IOL 檔，再用 Containerlab 相容的 vrnetlab 建置。參考 [Cisco IOL kind](https://containerlab.dev/manual/kinds/cisco_iol/)。

先把原始檔放進 **Debian** 的 `~/images/`：

```text
~/images/
├── x86_64_crb_linux-adventerprisek9-ms.iol
└── x86_64_crb_linux_l2-adventerprisek9-ms.iol
```

第一個是 L3 Router，第二個是 L2 Switch。原對話使用 `17.15.1` 標籤；下列命令保留該次版本，不代表最新版本。其他來源的檔案應先確認實際版本，不能只改檔名就視為升級。

確認 ARM64 Debian 可以執行 amd64 容器：

```bash
docker run --rm --platform linux/amd64 alpine:latest uname -m
```

預期為 `x86_64`。這只能確認基本轉譯可用，IOL 是否能正常啟動仍須在部署後驗證。

取得建置工具：

```bash
cd ~
git clone https://github.com/srl-labs/vrnetlab.git
cd ~/vrnetlab/cisco/iol
```

保留原始檔，複製成 Makefile 使用的名稱：

```bash
cp ~/images/x86_64_crb_linux-adventerprisek9-ms.iol cisco_iol-17.15.1.bin
cp ~/images/x86_64_crb_linux_l2-adventerprisek9-ms.iol cisco_iol-L2-17.15.1.bin
DOCKER_PLATFORM=linux/amd64 make docker-image
docker images vrnetlab/cisco_iol
```

應看到 Router 的 `17.15.1` 與 Switch 的 `L2-17.15.1`。檔名與版本標籤的對應可參考 [vrnetlab IOL Makefile](https://github.com/srl-labs/vrnetlab/blob/master/cisco/iol/Makefile)；新版 CML 若提供 `.tar.gz`，應依該版本 README 的流程處理。

## 5. 部署 Cisco Router 與 Switch

沿用原對話的單臂路由 Lab，先確認設備能啟動並登入。

```bash
mkdir -p ~/labs/router-on-stick
cd ~/labs/router-on-stick
vim lab.clab.yml
```

```yaml
name: router-on-stick

topology:
  nodes:
    r1:
      kind: cisco_iol
      image: vrnetlab/cisco_iol:17.15.1
    core-sw:
      kind: cisco_iol
      type: L2
      image: vrnetlab/cisco_iol:L2-17.15.1
    access-sw:
      kind: cisco_iol
      type: L2
      image: vrnetlab/cisco_iol:L2-17.15.1
    pc10:
      kind: linux
      image: alpine:latest
    pc20:
      kind: linux
      image: alpine:latest

  links:
    - endpoints: ["r1:e0/1", "core-sw:e0/1"]
    - endpoints: ["core-sw:e0/2", "access-sw:e0/1"]
    - endpoints: ["access-sw:e0/2", "pc10:eth1"]
    - endpoints: ["access-sw:e0/3", "pc20:eth1"]
```

```bash
clab deploy -t lab.clab.yml
clab inspect -t lab.clab.yml
ssh admin@clab-router-on-stick-r1
```

預設設定的帳號／密碼為 `admin / admin`；若套用自訂 startup-config，依自訂帳密登入。名稱無法解析時，用 `clab inspect` 顯示的管理 IP 連線。

進入 Cisco CLI 後確認：

```text
sh version
sh ip int br
```

`Ethernet0/0` 為管理介面；`Ethernet0/1` 起用於實驗連線。資料介面尚未配置時顯示 `administratively down` 並不代表 Containerlab 安裝失敗。

此 YAML 只建立節點與線路；VLAN、trunk、Router subinterface 與 PC IP 尚須另行設定，部署成功不等於 VLAN 間已能互通。

## 6. 查看拓撲與管理 Lab

在 Lab 目錄執行：

```bash
clab graph -t lab.clab.yml
```

保持此程序運行，從 Mac 瀏覽器開啟 `http://<Debian-IP>:50080`。Debian IP 可由 VM 內的 `ip -4 addr show eth0` 查看；原對話的 `192.168.139.233` 只是當時地址。`graph` 提供拓撲視圖，設備設定仍由 CLI 完成。參考 [graph 官方文件](https://containerlab.dev/cmd/graph/)。

常用操作均在對應 Lab 目錄執行：

```bash
clab inspect -t lab.clab.yml
clab save -t lab.clab.yml
clab destroy -t lab.clab.yml
clab deploy -t lab.clab.yml
```

`save` 用於支援設定匯出的節點；`destroy` 移除 Lab 容器與連線，並非刪除原始 YAML。Alpine 內手動執行的 IP 設定不會因此自動寫回 YAML；需要重現時，應放進 topology 的 `exec` 或初始化腳本。不要把重新 deploy 當成會自動恢復所有手動設定。

## 常見問題

| 現象 | 原因與處理 |
| --- | --- |
| IOL `pull access denied` | 先在 Debian 內完成映像建置，確認 YAML tag 與 `docker images` 一致。 |
| Mac 看得到 image，Debian 看不到 | 使用了不同 Docker Engine；在實際執行 Containerlab 的 Debian 內建置或匯入。 |
| SSH 到 Alpine 顯示 `Connection refused` | 此 Lab 的 Alpine 預設沒有 sshd，使用 `docker exec -it <容器名稱> sh`。 |
| `ip route add default` 顯示 `File exists` | 先用 `ip route` 查看既有管理網路 default route；只測其他 Lab 網段時，可加入特定目的網段路由。 |
| IOL 容器 running，但 SSH 尚未可用 | 等待設備初始化，搭配 `docker logs <容器名稱>` 檢查，最後以登入及 `sh version` 驗證。 |

例如單臂路由已完成後，PC10 可保留管理 default route，只加入前往 VLAN 20 的路由：

```bash
ip route add 192.168.20.0/24 via 192.168.10.1 dev eth1
```

PC20 也需要相對應的回程路由。這是 Host 路由設定，與 Router 之間使用 OSPF 交換路由是不同問題。

原對話已有兩台 Alpine 容器運行、實驗介面建立，以及成功進入 Cisco R1 CLI 的輸出；本筆記是流程整理，沒有重新執行安裝，也不將未提供結果的連通測試視為已通過。
