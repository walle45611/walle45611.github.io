# real-time system RMA 與實務排程分析整理

- source: `raw/my-vault/Note/Research/real-time system RMA 與實務排程分析整理.md`
- source_sha256: `38b3258a9d05c3484dfa9e2f2be2c3ae4982f3fb9791a98bf44e7512f8c128ac`
- ingested_at: 2026-09-26
- type: my-vault note summary
- collection: Research

## Summary

原筆記涵蓋 一、RMA 基本模型與測試、基本假設、利用率上界測試（Utilization Bound, UB）、Completion-time / Response-time 測試（Theorem 3）、Schedulability Point / Theorem 2（processor demand）、二、Context Switching、Interrupt 與 Blocking 建模。

## Source Notes

- 經典 Rate Monotonic Analysis（RMA）假設
- 任務集合：$\tau_i$，皆為 periodic task
- 優先權依 period 決定：週期越短 → 優先權越高（Rate Monotonic, RM）

## Navigation

- [回到 Research 歸檔](<../archives/my-vault-research.md>)
