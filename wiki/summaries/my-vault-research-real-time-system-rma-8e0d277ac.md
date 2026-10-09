# real-time system RMA 與實務排程分析整理

- source: `raw/my-vault/Note/Research/real-time system RMA 與實務排程分析整理.md`
- source_sha256: `93559500cec5367d3ac44375c4460ed9f6b6fe3b06040d09fb69cc868dbd1e53`
- source_reviewed: 2026-10-09
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

## 2026-10-09 來源複核

本次依目前來源重新核對章節涵蓋範圍；既有摘要保留，以下列出原先短摘要未完整呈現的導航。設定、證明與範例的適用條件仍須依原文，不把章節涵蓋視為實作已驗證。

目前來源章節：一、RMA 基本模型與測試、1. 基本假設、2. 利用率上界測試（Utilization Bound, UB）、3. Completion-time / Response-time 測試（Theorem 3）、4. Schedulability Point / Theorem 2（processor demand）、二、Context Switching、Interrupt 與 Blocking 建模、1. Context Switching Overhead、2. Cyclic Executive 與 Run-time Scheduling、3. Priority Inversion 與來源、4. 中斷（Interrupt）的建模：兩種做法、4.1 把中斷當成獨立 periodic task、4.2 把中斷當成 blocking time（或額外執行時間）、5. Rule of Thumb：三種效應的頻率、三、同步協定與阻塞時間、1. 評估同步協定的三個指標、2. 常見協定特性、四、RMA 典型算例模板、範例 1：UB 測試直接通過（最基本）、範例 2：UB 失敗，但用 Theorem 3 證明可排（無 blocking）、範例 3：有 periodic interrupt，直接當成一個 task、範例 4：同一個 interrupt，改用 blocking time + Theorem 3、範例 5：interrupt 太頻繁導致不可排。
