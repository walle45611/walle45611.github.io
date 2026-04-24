# Introducing GPT-5.5

- source: `raw/Introducing GPT-5.5.md`
- source link: https://openai.com/index/introducing-gpt-5-5/
- original title: Introducing GPT-5.5
- author: OpenAI
- published: 2026-04-22
- type: announcement

## 摘要

OpenAI 發布了 GPT-5.5，這是目前最聰明且直覺的模型，旨在成為處理複雜任務（如程式碼編寫、研究、數據分析）的強大工具。GPT-5.5 在理解意圖、規劃任務、使用工具以及在模糊環境中持續執行任務方面有顯著提升。

### 核心能力與優勢

- **代理式程式碼編寫 (Agentic Coding)**：在 Terminal-Bench 2.0 與 SWE-Bench Pro 等評測中表現卓越。它能處理複雜的命令列工作流，具備強大的規劃、迭代與工具協調能力，並能理解大型系統的架構與變更影響。
- **知識工作 (Knowledge Work)**：在生成文件、試算表與簡報方面優於前代。結合 Codex 的電腦使用能力，能更自然地在不同軟體間切換，處理如財務審核、業務報告自動化等任務。
- **科學研究 (Scientific Research)**：在基因學與定量生物學（GeneBench）以及生物資訊學（BixBench）表現出色。它能作為「共同科學家」，協助研究人員進行多階段數據分析、驗證假設並提出新的數學證明（如 Ramsey numbers 的新證明）。
- **效率與速度**：儘管模型能力大幅提升，但 GPT-5.5 的每 token 延遲與 GPT-5.4 持平，且在完成相同任務時使用的 token 數量更少，展現了極高的效率。

### 評測表現 (部分關鍵指標)

| 評測項目 | GPT-5.5 | GPT-5.4 | Claude Opus 4.7 |
| --- | --- | --- | --- |
| Terminal-Bench 2.0 | **82.7%** | 75.1% | 69.4% |
| GDPval (wins or ties) | **84.9%** | 83.0% | 80.3% |
| OSWorld-Verified | **78.7%** | 75.0% | 78.0% |
| FrontierMath Tier 1–3 | **51.7%** | 47.6% | 43.8% |

### 安全與可用性

- **網路安全防禦**：OpenAI 強化了針對網路攻擊的防禦機制，並透過「受信任存取 (Trusted Access)」計畫，讓經過驗證的防禦者能更安全地使用具備網路安全能力的模型。
- **可用性**：目前已向 ChatGPT (Plus, Pro, Business, Enterprise) 與 Codex 用戶推出，API 版本即將發布。

## 與既有知識庫內容的關係

- **[ai-coding-tools](./concepts/ai-coding-tools.md)**：GPT-5.5 是 AI 程式碼輔助工具從 Copilot 演進至 Agent 的重要里程碑，特別是在代理式編寫與複雜工作流處理上的表現。
- **[opencode](./concepts/opencode.md)**：作為開源平替方案，OpenCode 的發展可與 GPT-5.5 這種頂級閉源模型的能力進行對比與研究。

## 相關概念連結

- [[ai-coding-tools]]
- [[opencode]]
