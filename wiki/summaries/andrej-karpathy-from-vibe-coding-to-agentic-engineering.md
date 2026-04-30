# Andrej Karpathy: From Vibe Coding to Agentic Engineering

- source: `raw/Andrej Karpathy From Vibe Coding to Agentic Engineering.md`
- source link: https://www.youtube.com/watch?v=96jN2OCOfLs
- original title: Andrej Karpathy: From Vibe Coding to Agentic Engineering
- speaker: Andrej Karpathy
- host: Sequoia Capital / Stephanie Zhan
- published: 2026-04-29
- created: 2026-04-30
- type: YouTube transcript summary

## Summary

這份來源把 Karpathy 對 AI coding 的最新觀察串成一條清楚的演進線：他認為模型在 2025 年底到 2026 年初的進步，讓「vibe coding」從半玩具式體驗變成更可靠的工作流，而下一步則是更嚴肅的「agentic engineering」。在這個階段，真正重要的不是只把 code 寫出來，而是把需求、上下文、安裝、部署與操作流程包裝成 agent 能直接消化的文字與結構。

## Key Claims

1. 最新模型讓他感覺自己「比以往更落後」不是誇飾，而是因為很多原本要人工修補的 chunk，現在已經能穩定一次過。
2. Software 3.0 的核心是：context window 成為新的 programming surface，prompt 與上下文就是對 interpreter 的控制桿。
3. 對 agent 來說，最有價值的不是冗長 shell script，而是可直接 copy-paste 的指令包與清楚的操作說明。
4. 未來更有用的 infra 會是 agent-native 的，文件、設定、部署與資料結構都要先對 agent 可讀，而不是只對人類可讀。
5. Karpathy 用「ghosts」形容 LLM：它們不是動物式智能，不能靠情緒互動驅動，而是需要更細膩的 taste、判斷與懷疑。
6. 你可以外包 thinking，但不能外包 understanding；知識庫、文章整理與提問系統的價值，在於幫助人維持理解能力。

## Important Details

- 他把 vibe coding 的轉折點放在 2025 年 12 月左右，重點不是單次 demo，而是模型在日常 chunk 級任務上的穩定度明顯上升。
- 以安裝工具為例，他認為「給 agent 一段它能執行的文字」比維護一個面向人類的安裝腳本更有效，因為 agent 可以讀環境、除錯並調整行為。
- 在 MenuGen 的案例裡，真正麻煩的常常不是寫 app，而是把服務串起來、設定 DNS、處理部署流程；這正好說明 agent-native infrastructure 的需求。
- 他期待的是一種 agent-first 的世界：人類不再只是照著文件操作，而是把需求翻譯成讓 agent 能執行的結構化任務。
- 他特別提到 knowledge base 對理解的重要性，因為把文章轉成 wiki 內容本身就是一種 synthetic data generation 與思考整理。

## Practical Takeaways

- 寫給 agent 的文件要直接給可執行步驟，不要只寫人類導向的說明。
- 對部署、安裝、設定這類流程，優先思考如何改寫成 agent 可讀的結構化輸入。
- 對 AI coding 任務，真正的瓶頸常常不是輸出能力，而是人類是否還保有足夠理解來做方向控制。
- 如果要做 knowledge base，重點不只是存資訊，而是維持可以追問、可以重組、可以校正理解的工作流。

## Related Concepts

- [ai-coding-tools](../concepts/ai-coding-tools.md)
- [harness-engineering](../concepts/harness-engineering.md)
- [context-engineering](../concepts/context-engineering.md)
- [agentic-knowledge-base-maintenance](../concepts/agentic-knowledge-base-maintenance.md)

## Alignment With Current Wiki

- 這份來源延伸了 `ai-coding-tools` 裡從 Copilot 到 Agent 的趨勢，補上 vibe coding 之後更清楚的下一階段：agentic engineering。
- 它也補強了 `harness-engineering` 與 `context-engineering` 的觀點，因為 agent-first 文件、可讀結構與上下文設計，已經是實務能力的一部分。
