---
title: "知識庫維護、同步與備份"
tags:
  - concept/knowledge
aliases:
  - "obsidian-sync-strategies"
---

# 知識庫維護、同步與備份

知識庫應分清原始素材、單一來源摘要、跨來源概念與短期閱讀紀錄。摘要保存來源觀點；概念頁回答跨來源問題，並透過來源連結與相鄰概念導航，不需再維護一份所有頁面的總索引。

## 整理與執行

- 原始素材進 raw，整理結果寫入 wiki；使用者授權的來源變更仍依 AGENTS.md 的實際路徑與邊界處理。
- 任務先讀 router，再按需求讀 ingest、query、lint 或 reading；工具應區分讀取、建立、精準修改與尾端追加。
- 背景 queue 可降低長工作對單次 webhook/request 壽命的依賴；介面輸出與長期筆記的呈現需求分開。
- Concepts 保留定義、比較、證據與限制；不要為每份來源另造一個概念，也不以章節清單代替知識整合。
- Daily 存於 wiki/daily，保留最近 30 個日曆日；summary 的 ingested_at 支援較早日期回顧。維護與未解事項留在 log，不另建 report。

## 同步與備份

同步傳播最新狀態；備份提供回復歷史狀態的能力。同步錯誤亦可能傳播，兩者不能互相取代。已收錄來源比較雲端硬碟同步工具與 Syncthing 類點對點方案，選擇依賴隱私、維運與多裝置協調需求。

同一 vault 維持清楚的同步主路徑，先設計備份與衝突恢復；不把多套同步工具疊加視為自然增加保障。

## 限制與未解問題

第三方同步工具、行動端背景限制及大型附件的長期衝突模式仍缺跨來源比較。背景管線能完成任務，不表示資料層或引用已自動正確。延伸 [[harness-engineering]]、[[context-engineering]]、[[ai-assisted-research-workflows]]。

## 來源

- [[llm-wiki]]
- [[llm-wiki-worker]]
- [[sync-obsidian-for-free]]
