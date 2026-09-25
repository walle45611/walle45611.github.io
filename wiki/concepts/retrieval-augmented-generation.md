# 檢索增強生成（RAG）

## Current View

RAG 將外部文件索引和生成模型結合，使回答能利用模型參數以外的資訊。關鍵選擇不只在於是否檢索，還包括檢索器、語料、檢索時機、提供給模型的內容與評估方式。

## Source Comparison

- **原始 RAG 框架**：使用 dense retriever 與 seq2seq 模型，區分生成過程共用文件的 RAG-Sequence 與可逐 token 改變文件的 RAG-Token。後來泛稱的「檢索後提示 LLM」不必採同一機制。[原始 RAG](../summaries/my-vault-research-retrieval-augmented-generation-for-knowledge-00564a249.md)
- **少樣本與知識更新**：Atlas 利用可更新索引支援知識密集型任務，說明文件索引的內容和更新方式也會影響結果。[Atlas](../summaries/my-vault-research-atlas-few-shot-learning-with-5fc8a4cbb.md)
- **動態檢索**：Agentic RAG 透過規劃、工具與反思調整多步檢索，但也增加協調、記憶、效率與治理成本。這是架構綜述，不能作為通用最佳配置的證明。[Agentic RAG](../summaries/my-vault-research-agentic-retrieval-augmented-generation-a-bbb5a39c5.md)
- **領域評估**：醫療 QA 基準比較多種語料、檢索器與模型組合，提醒增加檢索內容仍可能受中段資訊利用率限制；圖式醫療 RAG 另外強調證據連結，基準成績不能代替臨床安全驗證。[MedRAG 評測](../summaries/my-vault-research-benchmarking-retrieval-augmented-generation-for-0c5032872.md) · [MedGraphRAG](../summaries/my-vault-research-medical-graph-rag-evidence-based-62508d3f8.md)

## Working Heuristics

- 先確定目標問題與可用語料，再比較檢索與生成配置，避免假設文件越多越好。
- 保存來源指向，並把檢索品質、回答品質與實際任務結果分開評估。
- 長輸入的位置效應與截斷策略參考 [Long Context Position Effects](./long-context-position-effects.md)；上下文載入與壓縮參考 [Context Engineering](./context-engineering.md)；評測設計參考 [LLM Evaluation](./llm-evaluation.md)。

## Boundaries

上述研究使用不同模型、資料集與任務，結果不能直接跨領域外推。醫療問答分數與臨床安全是不同問題。

## Sources

- [原始 RAG](../summaries/my-vault-research-retrieval-augmented-generation-for-knowledge-00564a249.md)
- [Atlas](../summaries/my-vault-research-atlas-few-shot-learning-with-5fc8a4cbb.md)
- [Agentic RAG](../summaries/my-vault-research-agentic-retrieval-augmented-generation-a-bbb5a39c5.md)
- [MedRAG 評測](../summaries/my-vault-research-benchmarking-retrieval-augmented-generation-for-0c5032872.md)
- [MedGraphRAG](../summaries/my-vault-research-medical-graph-rag-evidence-based-62508d3f8.md)
