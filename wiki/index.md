# Wiki Index
## Concepts

- [agentic-knowledge-base-maintenance](./concepts/agentic-knowledge-base-maintenance.md): 統整知識庫型 agent 的維護基線，聚焦 `raw/`/`wiki/` 分層、規則路由、背景執行管線與對外輸出分離。
- [agent-design-patterns](./concepts/agent-design-patterns.md): 統整 single、sequential、parallel 等基礎 agent workflow pattern 的選型邏輯，聚焦 control、latency、cost 與責任分工的 trade-off。
- [cloudflare-queue-consumer-modes](./concepts/cloudflare-queue-consumer-modes.md): 統整 Cloudflare Queues 中 push 與 pull consumer 的選型邏輯，聚焦誰控制消費節奏、如何處理 ack/retry，以及 `visibility_timeout` 的運維含義。
- [context-engineering](./concepts/context-engineering.md): 聚焦 context window 管理的核心方法：按需載入、子代理摘要、記憶檢索、compaction 與 pruning 的取捨。
- [harness-engineering](./concepts/harness-engineering.md): 目前知識庫對 AI agent 駕馭工程的整合頁，聚焦規則檔、工具邊界與生成-評估回饋迴圈的設計原則。
- [verbalized-feedback-learning](./concepts/verbalized-feedback-learning.md): 整理自然語言回饋如何在多輪流程中改變 agent 行為，及其與檢查工具、評量設計、長期學習的關係。
- [effective-learning](./concepts/effective-learning.md): 目前知識庫對高效學習的初始整合頁，聚焦深層處理、變化練習、回饋品質與直覺校正條件。
- [microk8s-production-readiness](./concepts/microk8s-production-readiness.md): 統整 MicroK8s 在生產環境的適用條件、基礎元件基線與多節點運維重點。
- [kubernetes-gitops-delivery](./concepts/kubernetes-gitops-delivery.md): 統整 Helm、Argo CD、Image Updater 與 Sealed Secrets 在 Kubernetes 上形成的 GitOps 交付鏈。
- [cloudflare-workers-ai-pricing-model](./concepts/cloudflare-workers-ai-pricing-model.md): 聚焦 Workers AI 的 daily quota、超額計費與超限失敗行為如何影響成本與可用性治理。
- [neuron-based-ai-cost-metering](./concepts/neuron-based-ai-cost-metering.md): 整理以 neurons 作為跨模型統一成本度量的判讀方式與實務管控啟發。
- [obsidian-sync-strategies](./concepts/obsidian-sync-strategies.md): 統整 Obsidian 免費同步方案的選型邏輯，聚焦同步與備份的邊界、避免混用，以及 Syncthing 與雲端同步工具的取捨。
- [opencode](./concepts/opencode.md): 統整 OpenCode 作為開源版 Claude Code 的核心定位、優勢與進階 Agent 特性。
- [microsoft-entra](./concepts/microsoft-entra.md): 統整 Microsoft Entra 身份與網路存取產品家族，涵蓋零信任存取、ID 管理、外部 ID 與 Agent ID 等核心組件。
- [ai-coding-tools](./concepts/ai-coding-tools.md): 統整 AI 程式碼輔助工具的發展趨勢與知識庫中的代表性工具。
- [nuitka-and-docker-deployment](./concepts/nuitka-and-docker-deployment.md): 統整 Nuitka 與 Docker 部署方案，說明高效能與安全部署實務。
## Summaries

- [cloudflare-pull-consumers](./summaries/cloudflare-pull-consumers.md) · 2026-04-21: 整理 Cloudflare Queues pull consumer 的啟用方式、ack/retry 機制、`lease_id` 與 `visibility_timeout` 邊界，以及 pull 與 push 消費模式的選型差異。
- [llm-wiki-worker](./summaries/llm-wiki-worker.md) · 2026-04-22: 整理 LLM-Wiki-Worker 如何以 `raw/`/`wiki/` 分層、router rules、Cloudflare Queue 與 LINE webhook 建立可持續維護的知識庫 agent 流程。
- [llm-wiki-worker-project](./summaries/llm-wiki-worker-project.md) · 2026-04-23: 整理 LLM-Wiki-Worker 開源專案的架構、流程、技術組件與目錄結構。
- [ai-agent-design-patterns](./summaries/ai-agent-design-patterns.md) · 2026-04-22: 整理 single agent、sequential agent、parallel agent 三種基礎 agent workflow 的適用條件，以及 control、latency、cost 之間的取捨。
- [sync-obsidian-for-free](./summaries/sync-obsidian-for-free.md) · 2026-04-20: 整理 Obsidian 免費同步方案，比較 Syncthing 與雲端同步工具 (Autosync/Dropsync) 的適用情境與風險。
- [openclaw-ai-agent-operating-principles](./summaries/openclaw-ai-agent-operating-principles.md) · 2026-04-19: 以 OpenClaw 拆解 AI agent 的執行閉環，涵蓋 system prompt 組裝、工具調用、記憶檔、排程與上下文壓縮，以及對應的安全風險與防禦。
- [harness-engineering-language-models-need-human-guidance](./summaries/harness-engineering-language-models-need-human-guidance.md) · 2026-04-19: 李一駿助教以實作案例與近期研究說明 harness engineering 的核心：透過規則、工具與工作流設計，讓模型在多輪任務中更穩定完成目標。
- [how-to-learn-anything-faster-using-modern-research](./summaries/how-to-learn-anything-faster-using-modern-research.md) · 2026-04-18: Justin Sung 以六條過時學習規則整理現代學習研究的實務含義，涵蓋練習設計、deep processing、筆記、困難學習與直覺形成。
- [microk8s-production-deployment-guide](./summaries/microk8s-production-deployment-guide.md) · 2026-04-19: 整理 MicroK8s 從安裝、插件、節點治理到 Argo CD/Sealed Secrets/Image Updater 的可重複部署流程與生產化邊界。
- [cloudflare-workers-ai-pricing](./summaries/cloudflare-workers-ai-pricing.md) · 2026-04-20: 彙整 Workers AI 的神經元計價規則、免費額度、UTC 日切重置與跨模型 token-neuron 成本換算重點。
- [codex-vs-claude-vibecoding-observations](./summaries/codex-vs-claude-vibecoding-observations.md) · 2026-04-20: 以量化交易案例比較 Codex/GPT 與 Claude 的編程風格差異，提煉「架構規劃與執行分層」的實務工作流。
- [opencode-detailed-guide](./summaries/opencode-detailed-guide.md) · 2026-04-20: 技術爬爬蝦關於 OpenCode 的攻略，介紹其作為開源版 Claude Code 的核心定位、免費模型接入與 MCP/Agent Skills 支援。
- [lin-zai-xue](./summaries/lin-zai-xue.md) · 2026-04-21: 林宅血案的事件概述、主要人物、相關概念與參考來源。
- [choose-a-design-pattern-for-your-agentic-ai-system-cloud-architecture-center](./summaries/choose-a-design-pattern-for-your-agentic-ai-system-cloud-architecture-center.md) · 2026-04-22: 整理 Choose a design pattern for your agentic AI system 文章，提供 pattern 選擇框架與比較矩陣。
- [nuitka-and-docker-for-high-performance-and-secure-python-deployment](./summaries/nuitka-and-docker-for-high-performance-and-secure-python-deployment.md) · 2026-04-22: 介紹 Nuitka 與 Docker 結合的高效能與安全部署方案。
- [postgresql-books](./summaries/postgresql-books.md) · 2026-04-22
- [codex-for-almost-everything](./summaries/codex-for-almost-everything.md) · 2026-04-22: 整理 Codex app 的主要更新與功能
- [what-is-microsoft-entra](./summaries/what-is-microsoft-entra.md) · 2026-04-24: 整理 Microsoft Entra 產品家族，涵蓋身份管理、零信任存取、外部 ID 及 AI Agent ID 等核心組件。
- [introducing-gpt-5-5](./summaries/introducing-gpt-5-5.md) · 2026-04-24: 整理 OpenAI GPT-5.5 發布資訊，聚焦其在代理式編碼、知識工作與科學研究領域的突破性能力與效率提升。
