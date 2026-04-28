## [2026-04-25] social | GPT-5.5 and Developer Workflow

- pages-read: user prompt
- output: social post draft
- notes: Created a Threads-style post focusing on the transition from manual Vim editing to full AI automation with GPT-5.5.
## [2026-04-24] daily | 2026-04-24 summary

- basis: [[wiki/summaries/what-is-microsoft-entra.md]], [[wiki/summaries/introducing-gpt-5-5.md]]
- updated: wiki/assets/daily/2026-04-24.md
- notes: 整理了 2 篇內容，涵蓋 Microsoft Entra 與 GPT-5.5，並觀察到 Agent ID 與 Agentic AI 的關聯。
## [2026-04-25] ingest | LoRA & QLoRA Fine-tuning Explained In-Depth

- source: `raw/LoRA & QLoRA Fine-tuning Explained In-Depth.md`
- created: `wiki/summaries/lora-and-qlora-fine-tuning-explained-in-depth.md`, `wiki/concepts/parameter-efficient-fine-tuning.md`, `wiki/concepts/model-quantization.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 整理 LoRA / QLoRA 的參數高效微調、rank/alpha/dropout 與量化取捨，並新增 PEFT 與 model quantization 概念頁。

## [2026-04-28] ingest | vLLM + Gemma 4 + LoRA：我踩過的兩個坑

- source: `raw/vLLM + Gemma 4 + LoRA：我踩過的兩個坑.md`
- created: `wiki/summaries/vllm-gemma-4-lora-two-pitfalls.md`, `wiki/concepts/llm-serving-compatibility.md`
- updated: `wiki/concepts/parameter-efficient-fine-tuning.md`, `wiki/index.md`, `wiki/log.md`
- notes: 整理 Gemma 4 runtime LoRA 支援與 CUDA wheel 相容性兩個坑，補上 LLM serving compatibility 概念，並把 LoRA 與實際 serving 條件拆開記錄。
## [2026-04-25] daily | 2026-04-25

- basis: [[wiki/summaries/lora-and-qlora-fine-tuning-explained-in-depth.md]], [[wiki/summaries/what-is-microsoft-entra.md]], [[wiki/summaries/introducing-gpt-5-5.md]]
- updated: wiki/assets/daily/2026-04-25.md
- notes: 3 summaries processed.

## [2026-04-26] daily | 2026-04-26 summary

- basis: [[wiki/summaries/introducing-gpt-5-5.md]], [[wiki/summaries/what-is-microsoft-entra.md]], [[wiki/summaries/lin-zai-xue.md]]
- updated: wiki/assets/daily/2026-04-26.md
- notes: 3 summaries processed.

## [2026-04-27] daily | 2026-04-27 summary

- basis: [[wiki/summaries/introducing-gpt-5-5.md]], [[wiki/summaries/what-is-microsoft-entra.md]]
- updated: wiki/assets/daily/2026-04-27.md
- notes: 2 summaries processed.

## [2026-04-28] ingest | AI 能自我修正嗎？從 decoding、workflow 到 reasoning 的技術發展整理

- source: `raw/AI 能自我修正嗎？從 decoding、workflow 到 reasoning 的技術發展整理.md`
- created: `wiki/summaries/ai-self-correction-decoding-workflow-reasoning.md`, `wiki/concepts/self-correction-in-language-models.md`
- updated: `wiki/concepts/harness-engineering.md`, `wiki/concepts/verbalized-feedback-learning.md`, `wiki/index.md`, `wiki/log.md`
- notes: 整理自我修正從 decoding、workflow 到 reasoning 的三層技術路線，補上 external feedback 與 verification 算力成本的知識脈絡。

## [2026-04-28] lint | wiki health check

- checked: `wiki/index.md`, `wiki/log.md`, `wiki/concepts/harness-engineering.md`, `wiki/concepts/verbalized-feedback-learning.md`, `wiki/concepts/self-correction-in-language-models.md`, `wiki/summaries/ai-self-correction-decoding-workflow-reasoning.md`
- fixed: 移除 `wiki/log.md` 殘留的 merge conflict marker，補上新 summary / concept 的索引與交叉連結，確保 ingest 與 index 同步。
- gaps: 這次 lint 以新來源相關頁面與 `wiki/log.md` 結構修復為主，尚未全面巡檢所有 summary / concept 的孤立頁與重複內容。

## [2026-04-29] ingest | 專案管理考前複習筆記講義

- source: `raw/專案管理-期中筆記.md`
- created: `wiki/summaries/project-management-midterm-notes.md`
- updated: `wiki/index.md`, `wiki/log.md`
- notes: 歸檔自寫專案管理考前講義，補上 Obsidian 屬性，保留錯題優先順序、時程計算與敏捷考點的複習結構。
