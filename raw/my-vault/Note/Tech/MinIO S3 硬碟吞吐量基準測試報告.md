## 一、測試環境  
- **叢集規模**：24 節點分散式 MinIO 叢集  
- **硬體配置**：  
  - **服務端節點**（24 × h1.16xlarge）：  
    - CPU：64 vCPU  
    - RAM：256 GB  
    - 網路：25 Gbps  
    - 儲存：8 × 2 TB HDD (JBOD)  
  - **客戶端**（16 × c5d.18xlarge）：  
    - CPU：72 vCPU  
    - RAM：144 GB  
    - 網路：25 Gbps  
    - 儲存：2 × 900 GB HDD  
- **網路連線**：所有節點透過同一 25 Gbps 網路交換封包  
- **基準工具**：wasabi-tech S3-benchmark（關閉 MD5 計算以飽和網路）

## 二、Linux 內核調校  
- **檔案描述符**：`fs.file-max=4194303`  
- **Swap 使用**：`vm.swappiness=1`  
- **TCP Buffer**：  
  - `net.core.rmem_max=268435456`  
  - `net.core.wmem_max=268435456`  
- **並發連線與延遲調整**：  
  - `net.core.somaxconn=65535`  
  - `net.ipv4.tcp_max_tw_buckets=2000000`  
- **套用方式**：修改 `/etc/sysctl.conf`，執行 `sysctl -p` 生效

## 三、單節點硬碟效能（基準測試）  

1. **單碟直寫/直讀** (`dd bs=16M count=64 oflag=direct`)  
   - 寫入：約 137 MB/s  
   - 讀取：約 205 MB/s  
2. **JBOD 並行測試** (`iozone`，32 線程、32 KB 區塊)  
   - 寫入：約 655 MB/s  
   - 讀取：約 1.26 GB/s  

> *註：25 Gbps 網路理論最高吞吐約 3.125 GB/s，硬碟 I/O 未飽和網路*

## 四、分散式 16 節點基準結果  
- **測試參數**：2 GB 物件、32 線程、測試時間 1 分鐘  
- **總體吞吐**：  
  - **PUT（寫入）**：9.40 GB/s 總和 → 每節點約 536 MB/s  
  - **GET（讀取）**：16.38 GB/s 總和 → 每節點約 1 023 MB/s  
- **資源使用**：  
  - CPU：平均僅 1.4 %（約 670 millicpu）  
  - 記憶體：平均 11.2 GB  

## 五、磁碟性能轉換比（以單一 2 GB 物件為例）  
| 項目           | MinIO 吞吐量 | 原始 HDD 吞吐量 | 轉換比  |
|---------------|-------------:|---------------:|--------:|
| **寫入 (PUT)** |    536 MB/s  |    655 MB/s    | 82 %    |
| **讀取 (GET)** |    676 MB/s  |  1 260 MB/s    | 54 %    |

> *說明：寫入可達到底層 HDD 寫入速率的約 80 %，讀取約 50–55 %。*

## EC:2 下硬碟實際存儲轉換比率

- **設定**：Erasure Coding `EC:2`，在一組共 $N = 24$ 顆硬碟中，資料分片 $k = N - 2 = 22$、校驗分片 $r = 2$。

1. **叢集總消耗比率**  
   $\displaystyle \frac{k + r}{k} = \frac{22 + 2}{22} = \frac{24}{22} \approx 1.091$

2. **額外開銷比例**  
   $\displaystyle \left(\frac{24}{22} - 1\right)\times 100\% = \frac{2}{22}\times 100\% \approx 9.09\%$

3. **對應到 100 MB**  
   - 實際佔用空間：$100\ \mathrm{MB} \times 1.091 \approx 109.1\ \mathrm{MB}$  
   - 增加量：$109.1\ \mathrm{MB} - 100\ \mathrm{MB} = 9.1\ \mathrm{MB} \ (\approx 9.1\%)$

---

> **結論**：使用 EC:2 時，硬碟實際存儲大約多出 $9.09\%$。
