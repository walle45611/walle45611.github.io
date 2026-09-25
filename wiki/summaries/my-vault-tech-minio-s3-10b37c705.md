# MinIO S3 硬碟吞吐量基準測試報告

- source: `raw/my-vault/Note/Tech/MinIO S3 硬碟吞吐量基準測試報告.md`
- source_sha256: `3914e4df0b559c0ee0ecb5ae4f5c3ef41b02b5985398e72bc10a64e9706ebc48`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 一、測試環境、二、Linux 內核調校、三、單節點硬碟效能（基準測試）、四、分散式 16 節點基準結果、五、磁碟性能轉換比（以單一 2 GB 物件為例）、EC:2 下硬碟實際存儲轉換比率。

## Source Notes

- 網路連線：所有節點透過同一 25 Gbps 網路交換封包
- 基準工具：wasabi-tech S3-benchmark（關閉 MD5 計算以飽和網路）
- 檔案描述符：fs.file-max=4194303

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
