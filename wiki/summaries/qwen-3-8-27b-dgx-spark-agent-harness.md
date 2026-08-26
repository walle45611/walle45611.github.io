# 從模型部署到 Agent Harness：Qwen 3.8 27B 與 DGX Spark 實機示範

- source: `raw/從模型部署到 Agent Harness：Qwen 3.8 27B 與 DGX Spark 實機示範.md`
- source link: [https://www.youtube.com/watch?v=7tNIw_gHeSI](https://www.youtube.com/watch?v=7tNIw_gHeSI)
- original title: 從模型部署到 Agent Harness：Qwen 3.8 27B 與 DGX Spark 實機示範
- author: Will 保哥
- speaker: 胡嘉璽（joshhu）
- published: 2026-08-22
- event: 2026-08-21
- source_created: 2026-08-26
- ingested_at: 2026-08-26
- type: YouTube livestream transcript summary

## Summary

這場直播把地端模型的落地拆成一個完整 stack：模型權重、推論引擎、相容端點、工具與 skills、agent harness，以及容器、VM 或 sandbox 形成的執行邊界。講者以 DGX Spark 執行來源所稱的 Qwen 3.8 27B，搭配 PiAgent 和文件處理工具，示範對假資料中的個人資料進行去識別化，並比較雲端模型、地端模型、未隔離 harness 與 sandbox 的風險差異。

來源的核心訊息不是證明某個模型在所有環境都更強，而是：地端模型可以把資料路徑留在組織控制範圍內，但真正的操作風險仍由 harness 的工具、權限、網路與隔離方式決定。部署時也不能只看參數量或權重大小，還要同時考量 KV cache、context window、併發量、系統保留空間、推論引擎與硬體架構。

## Key Claims

1. Agent 可拆成 LLM 與 harness 兩部分：LLM 產生文字或工具呼叫，harness 負責組裝上下文、解析工具、執行程式、回傳結果與管理權限。
2. 開放權重模型可部署在公有雲、私有雲、區域網路或單機；來源將地端部署視為控制資料邊界與服務固定團隊的重要選項，但開放權重或地端位置本身不等於安全。
3. 模型容量估算至少要包含權重、KV cache、context window、併發請求與作業系統等保留空間。MoE 的 active parameters 也不等於載入時需要的全部權重。
4. 模型名稱可攜帶參數規模、Dense/MoE、量化格式、MTP 與 instruction tuning 等資訊；這些標記會影響載入需求、推論速度與工具遵循能力，但仍需回到實際 model card 與 runtime 驗證。
5. llama.cpp/Ollama 以 GGUF 打包和快速試用見長；講者則偏好在正式服務情境使用 vLLM 或 SGLang，並指出 NVIDIA 專用引擎與模型、CUDA、容器映像檔及 ARM 架構的相容性需要個別確認。
6. 講者的部署習慣是以 Docker 固定服務、開放 OpenAI-compatible endpoint、指定 tool parser，再讓 coding harness 參考 model card 或 GitHub 說明完成設定；這是個人工作流示範，不是唯一部署方法。
7. 安全性是分層問題：模型放置位置決定資料是否離開組織，harness 的權限與 sandbox/VM/container 決定 agent 失誤的爆炸半徑，網路政策則決定外部內容與資料能否流出。
8. 能由固定工具或 CPU 程式完成的工作，應盡量交給可審查、可重複的工具執行，讓模型負責理解與編排；工具、技能與 review loop 的品質會直接影響 agent 的可靠性。

## Demonstration and Reported Observations

- 示範使用假造的銀行稽核資料，涵蓋姓名、身分證字號、地址、電話、電子郵件，以及 Excel、CSV、純文字與 DOCX 等形式；PiAgent 透過 office skill 或 Python 程式完成去識別化。
- 來源先展示「雲端模型＋未隔離本機 harness」的高風險路徑，並以一則客戶經驗說明將含個資內容送上雲端可能觸發資安事件；這是講者敘述，未在來源中提供獨立稽核證據。
- 另一組示範使用 DGX Spark 上的 Qwen 27B 與開源 PiAgent，講者表示資料沒有送出網路；再把 harness 放入 OpenShell sandbox，示範對 Google/GitHub 的一般連線被拒絕，但仍可使用已配置的本地工具完成任務。
- 講者以個人測試報告約 30 tok/s 的單一流程、平行時約 80–100 tok/s，並表示 FP8 的 Qwen 27B 在一次示範中約占 80GB；這些數字沒有固定 token 長度、延遲定義或可重現 benchmark，不能直接外推到其他硬體或版本。
- DGX Spark 的 128GB unified memory 被定位為容量優先的 POC 平台；來源也提到較低頻寬、溫度與時脈限制會影響速度，並以 MTP 作為可能的 engine-dependent 加速手段。

## Important Terms

- Qwen 3.8 27B
- DGX Spark / unified memory
- Dense / MoE / active parameters
- FP8 / NVFP4 / Q4_K_M / GGUF / QAT
- KV cache / context window / MTP
- vLLM / SGLang / TensorRT-LLM / llama.cpp / Ollama
- PiAgent / Agent Skill / tool parser / OpenAI-compatible endpoint
- OpenShell / Docker / VM / sandbox
- de-identification / prompt injection / permission boundary

## Evidence Boundary and Uncertainty

- 摘要沿用來源標題與 metadata 的「Qwen 3.8 27B」；逐字稿另有「千問 2.3.8 27B」及「千問 2.5 27B / 3.8 27B」等寫法，未自行判定確切模型版本。
- 來源是 2026-08-21 的直播與逐字稿，不是控制變因一致的效能測試；arena 排名、模型能力、硬體價格、溫度、tok/s 與引擎支援都應視為當時的講者報告。
- 去識別化示範使用假資料，且只涵蓋講者展示的脫敏流程；它不構成對真實個資合規、可逆還原或資安保證的獨立評估。
- 「本地」或「sandbox」只能描述示範配置；實際安全性仍需驗證容器權限、掛載目錄、憑證、網路出口、工具實作與供應鏈。

## Related Concepts

- [local-llm-deployment](../concepts/local-llm-deployment.md)
- [harness-engineering](../concepts/harness-engineering.md)
- [llm-serving-compatibility](../concepts/llm-serving-compatibility.md)
- [model-quantization](../concepts/model-quantization.md)
- [context-engineering](../concepts/context-engineering.md)

## Alignment With Current Wiki

- 這份來源把既有的 `harness-engineering` 從規則、工具與工作流延伸到部署層：模型、endpoint、tool parser、skills、權限與 sandbox 必須一起設計。
- 它補充 `llm-serving-compatibility`：model card、格式、推論引擎、Docker image、CUDA 與 ARM/統一記憶體架構的匹配，都是「能否真的跑起來」的一部分。
- 它補充 `model-quantization` 與 `context-engineering`：低精度降低權重占用，但 KV cache、context 與併發仍會重新分配記憶體預算。
- 地端模型與隔離執行邊界的組合在既有 wiki 尚未獨立成頁，因此新增 `local-llm-deployment` 作為後續累積硬體、runtime 與資料邊界案例的概念頁。
