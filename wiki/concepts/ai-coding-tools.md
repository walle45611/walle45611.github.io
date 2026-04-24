# AI Coding Tools

AI 程式碼輔助工具（AI Coding Tools / AI Coding Agents）是利用大型語言模型（LLM）來協助開發者編寫、重構、除錯與解釋程式碼的軟體工具。這些工具正從單純的「自動補全」演進為具備任務規劃與執行能力的「AI 代理 (AI Agent)」。

## 發展趨勢

- **從副駕駛 (Copilot) 到自主代理 (Agent)**：早期工具主要提供行級補全；新一代工具（如 Claude Code、[opencode](./opencode.md)）已能透過終端機執行指令、讀寫檔案、呼叫子代理（Subagent）來完成更複雜的專案任務。
- **標準化協定**：開始支援如 MCP (Model Context Protocol) 這樣的標準協定，讓工具能更安全地與外部環境、本地工具或資料庫互動。
- **開源平替方案興起**：隨著閉源商業工具（如 Claude Code）可能帶來的地域限制、費用或封號風險，開源且可自定義模型的平替方案（如 [opencode](./opencode.md)）逐漸受到歡迎。

## 知識庫中的相關工具與概念

- **[opencode](./opencode.md)**: 被譽為開源版 Claude Code，支援免費接入頂級模型與 MCP 等高階特性。
- **GPT-5.5**: OpenAI 的最新旗艦模型，在代理式編碼 (Agentic Coding) 與科學研究領域展現了極高的自主性與效率。
- **Claude Code**: Anthropic 官方推出的終端機 AI 程式碼代理。

## 關聯資源
- [opencode-detailed-guide](../summaries/opencode-detailed-guide.md)
- [codex-vs-claude-vibecoding-observations](../summaries/codex-vs-claude-vibecoding-observations.md)
