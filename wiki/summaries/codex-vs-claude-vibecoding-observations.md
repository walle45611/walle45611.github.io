# Codex 还是 Claude？分享一下我的 VibeCoding 编程观察

- source: https://www.youtube.com/watch?v=zKtYusISp4Y
- original title: Codex 还是 Claude？分享一下我的VibeCoding编程观察
- speaker: 可乐AI实验室
- published: 2026-03-29
- type: YouTube transcript summary

## Summary

這份來源以量化交易實作經驗比較 GPT/Codex 與 Claude 在實戰編程中的差異：作者認為 GPT/Codex 在「指令執行精準度」高，但若需求缺少架構與效能約束，容易產生邏輯正確卻效能災難的直譯式實作；Claude 則較常主動做架構層優化，但偶爾出現幻覺。來源主張的核心工作流是「架構規劃層與代碼執行層分離」。

## Key Claims

1. 只看跑分與代碼正確率不足以判斷實務可用性，因為架構與效能問題可能被掩蓋。
2. GPT/Codex 在需求執行上「過度聽話」，若指令不含性能邊界，容易輸出低效但語法正確的程式。
3. Claude 在部分場景更傾向主動做架構優化，使用體感更順，但伴隨幻覺風險。
4. 若把「規劃」與「執行」交給不同模型，整體開發效率與可維護性可能更高。
5. 真正的關鍵不是模型名稱，而是人類是否先把需求翻譯成具架構約束的計畫再下發。

## Important Details

- 來源案例是量化回測：在分鐘級資料下可達數百萬筆 K 線，若採逐筆全量迴圈可能造成 O(N^2) 級效能問題。
- 作者描述 GPT/Codex 的優勢在公式與逐步實作準確，弱點在缺少大局式效能優化。
- 作者描述 Claude 的優勢在架構調整與性能敏感度，弱點是偶發幻覺需二次修復。
- 來源建議加入第三方模型（如 Gemini）先做「架構翻譯」，再交由執行型模型產碼。

## Practical Takeaways From This Source

- 在 AI coding prompt 中明確加入複雜度目標、資料規模上限與性能約束。
- 把「架構計畫產生」與「代碼落地」拆成兩步，並在兩步間保留可審核計畫稿。
- 對高資料量任務先要求演算法與複雜度說明，再允許模型輸出實作細節。
- 把幻覺修復視為可回圈處理問題，但將架構錯誤視為高成本風險優先預防。

## Related Concepts

- [harness-engineering](../concepts/harness-engineering.md)
- [context-engineering](../concepts/context-engineering.md)

## Alignment With Current Wiki

- 與既有 `harness-engineering` 一致：模型表現取決於工作流與約束設計，不僅是模型本體能力。
- 這份來源屬個人實戰觀察，補強了「planner/executor 分層」在程式開發情境的可操作細節。
