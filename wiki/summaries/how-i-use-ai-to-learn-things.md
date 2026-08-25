---
title: "How I Use AI to Learn Things"
aliases:
  - "How I Use AI to Learn Things"
tags:
  - effective-learning
  - ai
  - harness
---

# How I Use AI to Learn Things

- source: `raw/How I Use AI to Learn Things.md`
- source link: https://www.youtube.com/watch?v=kzcI5F4tGiU
- original title: How I Use AI to Learn Things
- author: [[Eero Alvar]]
- published: 2026-08-14
- source_created: 2026-08-24
- ingested_at: 2026-08-25
- type: YouTube transcript summary

## Summary

這份影片不是在說「把課本丟給 AI，讓 AI 代替你學習」，而是在展示一個個人化教學 harness：讓 AI 先測量學習者現在真正懂到哪裡，再規劃依賴關係與教學路徑，最後以一小步一小步的方式教學、出題與校準。AI 吸收的是找資料、排順序、查證、畫圖、保存紀錄等學習物流；學習者仍然要在材料本身上感到困難、回答問題、應用概念，才會真的學會。

一句話概括：把「一個老師教很多人、每個人又到處找不同教材」的低效率，改成「一個介面服務一個學習者」；這個介面可以彙整多個來源，但必須靠驗證建立信任，並把認知力集中在真正困難的內容上。

## The Problem: many-to-many learning

作者先指出傳統學習有兩個方向的浪費：

1. **一個教學來源面對很多學生。** 書、課程與老師通常為群體設計，不可能完全貼合某一個人的先備知識。太簡單的內容會浪費時間，太難的內容又超出當下理解邊界；理想教學應該緊貼學習者「目前懂到哪裡」與「下一步剛好能懂什麼」。
2. **一個學生從很多來源學習。** 不同教材的符號、說法、介面、可靠程度與教學風格都不同，切換本身會消耗心力。作者特別強調信任：即使兩份解釋內容相同，學習者也比較容易把熟悉且可信的來源內化。

作者的解法不是只保留一個資訊來源，而是讓一個 AI 介面把多個來源彙整後，以一致的方式交付給同一個人。因此，「一對一」指的是教學介面與學習者的配對，不是把世界上的知識縮成單一觀點。

## The Two Principles

### 1. Optimized teaching

系統要根據學習者的實際理解狀態，決定先教什麼、哪些前置概念需要補、每一步要解釋到什麼程度。它不能只根據使用者自己說「我懂了」來判斷，而要透過測驗建立更細的理解地圖。

### 2. Optimized allocation of mental resources

作者所謂「最佳化心力分配」不是把所有困難移除，而是把困難集中到內容本身。找教材、確認先備知識、安排順序、查證事實、畫圖與記錄進度等物流交給系統處理；學習者的心力則用在理解微分形式、辨認概念關係與解題上。

因此，影片說的「maximize struggle」不能理解成故意讓學習變痛苦，而是保留有助於理解與記憶的 productive struggle，移除不會增加理解的行政摩擦。

## The Process: probe → plan → teach

1. **Probe：探測理解邊界。** 系統先用分級選擇題從寬到窄地問問題，近似用 binary search 找出每一條前置知識鏈的邊界，建立學習者的先備知識地圖。
2. **Plan：規劃教學路徑。** 系統依照目前理解與目標，推理出應走的依賴圖；它還使用查證與 fact-checking 子代理，並把計畫呈現成 Mermaid 圖。圖不只是給人看，也迫使 AI 先把路徑想清楚，不能直接跳到結論。
3. **Teach：逐步教學與回饋。** 系統一次只走一個推理步驟，搭配視覺化、LaTeX 與小測驗；學習者可以在任何一步提問，系統也會週期性測試是否真的理解，而不是只產生「看起來很會講」的解釋。

測驗在這個流程中有三個作用：防止學習者把熟悉感誤認成理解、讓系統持續校準教學難度，以及透過應用讓新知識固定下來。

## Demo: learning differential forms

作者用微分形式作示範，因為自己對這個主題只有向量微積分等相關背景。系統先探測線積分、散度、Stokes 定理與狹義相對論中電磁場變換等前置理解，再規劃通往廣義 Stokes 定理的路徑。

教學過程依序碰到 covectors、covector fields、wedge product 與更高階的形式，並使用 SVG 圖像、LaTeX 與 Obsidian 內的持久化紀錄輔助理解。作者特別喜歡「每次只走一個推理步驟」與「圖像能提供另一個觀點」這兩點；他沒有在影片中完整學完微分形式，而是用這段過程展示系統的工作方式。

實作上，作者在 Pi Agent harness 中配置 teach skill、視覺化工具、quiz extension、Markdown log extension，以及生成與檢查視覺素材的子代理。這些都是作者目前的個人設定，不是影片證明唯一或普遍適用的技術架構。

## What the source is really arguing

- **AI 的主要價值是個人化與校準，不是替人吞下知識。** 它可以依照學習者當下狀態改變路徑與解釋，但理解與應用仍需由學習者完成。
- **教學流程比單次回答更重要。** 單次聊天容易跳太快、假設使用者已經懂了；探測、規劃、分步教學與測驗組成的是一個回饋迴圈。
- **可靠性是學習功能，不只是安全功能。** Fact-checking 與驗證不只避免錯誤，也能讓學習者放心把資訊內化；不可信的老師會讓大腦持續保留懷疑。
- **視覺化與持久化紀錄是認知工具。** 圖像協助建立不同表徵，Markdown log 則保存每次學習的上下文，讓學習不必每次從零開始。

## Evidence Boundary

- 這是作者分享的「目前個人方法」與現場 demo，不是控制實驗，也沒有證明這套系統比傳統老師、家教或其他 AI 教學法普遍有效。
- 「最佳化教學」高度依賴先備知識測量、題目品質、模型能力、查證工具與教學指令；測驗答對也不必然代表能長期遷移或獨立應用。
- AI 把物流吸收後，學習者仍要保留有意義的困難；若連思考、回憶與應用都外包，系統可能只產生流暢的理解幻覺。
- Demo 中曾出現視覺化代理過載失敗，說明這套 harness 仍是可改善的工程系統，不是無摩擦的自動家教。

## Related Concepts

- [[effective-learning]]
- [[harness-engineering]]

## Alignment With Current Wiki

- 這份來源把 `effective-learning` 中的 deep processing、適度困難與回饋校正，具體實作成 probe、plan、teach 與 quiz 迴圈。
- 它也補強 `harness-engineering`：規則、子代理、查證器、視覺化工具、持久化 log 與測驗共同構成教學 harness；重點是工作流設計，不只是換一個模型。
- 它和既有學習摘要一致地把筆記理解成壓縮與校正工具，而不是完整抄錄的終點；但個人化 AI 教學的長期成效仍是待驗證問題。
