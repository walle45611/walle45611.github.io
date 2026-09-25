# NVIDIA NVML / Driver 版本不相容問題

- source: `raw/my-vault/Note/Tech/NVIDIA NVML Driver 版本不相容問題.md`
- source_sha256: `36fc41c540751f4b342758fb708a077e9bf6c4b90f03e6644521a3b1f2d56b2f`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Tech

## Summary

原筆記涵蓋 問題現象、問題原因、確認方式、查看目前 Kernel Driver 版本、查看 NVML library 實際版本、查看程式實際載入哪一份 NVML。

## Source Notes

- 目前系統載入的 NVIDIA Kernel Driver 版本是
- 但 nvidia-smi / nvitop 實際載入的 NVML library 是
- 兩者版本不同，導致 NVML 無法正常和 NVIDIA Kernel Driver 溝通。

## Navigation

- [回到 Tech 歸檔](<../archives/my-vault-tech.md>)
