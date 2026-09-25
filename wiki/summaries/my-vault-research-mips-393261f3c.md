# 計算機組織MIPS 浮點數整理

- source: `raw/my-vault/Note/Research/計算機組織MIPS 浮點數整理.md`
- source_sha256: `2d7e2c8ca5eb1556f2e9b2fc10de1f92cd8744af3de15073a5fc727f4718e5ca`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 暫存器架構與重點、ASCII 架構圖、指令分類與範例、算術（Arithmetic）、比較（Compare）+ 狀態旗標（FP condition bit）、分支（由 FP 条件旗標決定）。

## Source Notes

- 整數暫存器：32 個 $0~$31（zero, at, v0–v1, a0–a3, t0–t9, s0–s7, k0–k1, gp, sp, fp, ra）。
- 浮點暫存器：32 個 $f0~$f31（屬於 Coprocessor 1, FP）。
- 單精度 .s：每個 $fX 放 32-bit 浮點數。

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
