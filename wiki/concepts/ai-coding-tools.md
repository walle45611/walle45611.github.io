# AI Coding Tools

AI 程式碼輔助工具（AI Coding Tools / AI Coding Agents）是利用大型語言模型（LLM）來協助開發者編寫、重構、除錯與解釋程式碼的軟體工具。這些工具正從單純的「自動補全」演進為具備任務規劃與執行能力的「AI 代理 (AI Agent)」。

## 發展趨勢

- **從副駕駛 (Copilot) 到自主代理 (Agent)**：早期工具主要提供行級補全；新一代工具（如 Claude Code、[opencode](./opencode.md)）已能透過終端機執行指令、讀寫檔案、呼叫子代理（Subagent）來完成更複雜的專案任務。
- **從 Vibe Coding 到 Agentic Engineering**：當模型在日常任務上變得更穩定後，工作重心開始從「把 code 寫出來」移到「把需求、上下文、安裝與部署流程包裝成 agent 可直接消化的文字與結構」。
- **標準化協定**：開始支援如 MCP (Model Context Protocol) 這樣的標準協定，讓工具能更安全地與外部環境、本地工具或資料庫互動。
- **開源平替方案興起**：隨著閉源商業工具（如 Claude Code）可能帶來的地域限制、費用或封號風險，開源且可自定義模型的平替方案（如 [opencode](./opencode.md)）逐漸受到歡迎。

## 知識庫中的相關工具與概念

- **[opencode](./opencode.md)**: 被譽為開源版 Claude Code，支援免費接入頂級模型與 MCP 等高階特性。
- **GPT-5.5**: OpenAI 的最新旗艦模型，在代理式編碼 (Agentic Coding) 與科學研究領域展現了極高的自主性與效率。
- **Claude Code**: Anthropic 官方推出的終端機 AI 程式碼代理。

## Usage and Cost Governance

Codex 的 credits 可能依 workspace 適用的 rate card，以 token 類型或 message / pull request 平均值計算。方案遷移、模型、fast mode、任務大小與輸出量都會影響實際消耗；定價頁的數字具有時間性，使用前應確認原始文件與目前方案。

## 關聯資源
- [opencode-detailed-guide](../summaries/opencode-detailed-guide.md)
- [codex-vs-claude-vibecoding-observations](../summaries/codex-vs-claude-vibecoding-observations.md)
- [andrej-karpathy-from-vibe-coding-to-agentic-engineering](../summaries/andrej-karpathy-from-vibe-coding-to-agentic-engineering.md)
- [codex-rate-card](../summaries/codex-rate-card.md)
