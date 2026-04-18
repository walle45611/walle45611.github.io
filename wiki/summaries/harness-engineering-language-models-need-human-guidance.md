# Harness Engineering：有時候語言模型不是不夠聰明，只是沒有人類好好引導

- source: `raw/Harness Engineering：有時候語言模型不是不夠聰明，只是沒有人類好好引導.md`
- original title: Harness Engineering：有時候語言模型不是不夠聰明，只是沒有人類好好引導
- speaker: 李一駿助教（課程素材由 Hung-yi Lee 團隊發布）
- published: 2026-04-13
- type: YouTube transcript summary

## Summary

這份來源把 AI agent 的表現問題，從「模型本體能力」轉向「harness 設計」。核心主張是：同一個模型在不同的操作框架下，任務完成率可能差很多；人類若能把規則、工具與工作流設計好，常能把原本看似不可靠的模型導向可用結果。

## Key Claims

1. AI agent 的能力由「LLM + Harness」共同決定，優化 harness 是提升任務完成率的主要槓桿之一。
2. 以自然語言規則檔（如 `agents.md`、`CLAUDE.md`）建立行為框架，雖非絕對強制，但通常能顯著影響 agent 的行為模式與效率。
3. 工具決定能力邊界：同一模型在不同平台與權限設定下，能完成的任務範圍會有本質差異。
4. 給模型「像人類用的工具」不一定有效；對模型友善的工具往往偏向可摘要、可結構化、低噪音的介面。
5. 以 generator/evaluator（或 planner/generator/evaluator）構成的回饋迴圈，可穩定改善多步任務品質。
6. 回饋品質比回饋語氣更重要；情緒化、羞辱式 feedback 可能導致更差行為，具體、可執行的 feedback 更有效。

## Important Details

- Gemma 4 2B 的示例顯示：補充簡短且可操作的工作原則（先列目錄、先讀檔再改、定義完成條件）後，任務行為明顯改善。
- 來源將 harness 控制拆成三類：認知框架（規則文字）、能力邊界（工具/權限）、行為流程（標準工作流與評估迴圈）。
- 以 `agents.md`/`CLAUDE.md` 為代表的「Natural Language Harness」可跨框架移植，凸顯規則檔在遷移時的實務價值。
- 引述研究觀察：`agents.md` 可能提升速度或邊緣案例效率，但對正確率提升並非在所有模型都成立，且 LLM 自動產生的規則檔常不如人類撰寫版本。
- 來源引用實務建議：規則檔不宜做成百科全書，應偏向「導航地圖」，引導模型到正確資訊位置。
- 來源強調「verbalized feedback」是長期 agent 設計的關鍵題目：回饋常是自然語句，而非明確標籤或標準答案。
- 對 benchmark 的提醒：AI 與 AI 對話評測可能高估真實人機互動成效，評量設計本身也是 harness 問題的一部分。

## Practical Takeaways From This Source

- 設計 agent 時先明確寫出「第一步要做什麼」與「完成定義」，避免模型直接幻覺式作答。
- 優先提供能壓縮上下文成本的工具（例如先回傳摘要/定位，再按需打開檔案）。
- 需要程式編修時，編輯工具應搭配語法或測試檢查，形成最小可行回饋迴圈。
- 給 feedback 時避免情緒化措辭，改用具體修正方向與可驗證標準。
- 長期運作 agent 需安排記憶整理機制，避免脈絡膨脹造成退化。

## Related Concepts

- [harness-engineering](../concepts/harness-engineering.md)
- [verbalized-feedback-learning](../concepts/verbalized-feedback-learning.md)

## Alignment With Current Wiki

- 這是目前知識庫第一筆聚焦 AI agent harness 的來源，與既有 `effective-learning` 主題無直接衝突。
- 來源同時涵蓋部落格觀察、論文轉述與個人案例，後續若納入原始論文，應把跨來源可重現結論回寫到概念頁，保留此摘要頁作為來源邊界。
