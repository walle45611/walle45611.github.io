# Self-Correction in Language Models

## Current View

在目前知識庫中，language model self-correction 指的是：模型在沒有逐步人工介入的情況下，能偵測自身答案可能有誤，並透過 decoding、workflow 或訓練後形成的 reasoning 行為，主動修正輸出。

## Stable Conclusions

1. 自我修正不是單一技術，而是至少可分為 decoding 介入、workflow 驅動、reasoning / RL 訓練三個層級。
2. Contrastive decoding 類方法可在不改模型參數下提升答案品質，但本質上通常是用額外 test-time compute 換較高正確率。
3. 單純 internal self-reflection 的效果不穩定；外部可驗證 feedback 往往比模型自行反思更可靠。
4. 模型知道正確事實，不等於它具有自我修正能力；self-correction 可能是獨立於 factual recall 的行為能力。
5. verification 是否值得加入，不能只看絕對正確率，還必須和同算力預算下的 majority vote、better sampling 等 baseline 比較。
6. reasoning / RLVR 常會自然誘發 verification 與修正行為，但研究界仍在爭論 RL 是重排既有正確 path 的機率，還是真的讓模型學到新推理能力。

## Working Heuristics

- 若目標是不重訓快速增強，先評估 decoding 層方法，但要同步算清 latency 與 compute overhead。
- 若任務有執行器、檢查器、搜尋或 checklist，優先把這些 external feedback 放進迴圈，而不是只加一句「再想想」。
- 設計 self-correction 流程時，把 error detection 與 error correction 視為兩個不同問題。
- 評估 self-correction 時，不只看最終答案，也要檢查中間 reasoning path 是否真的更正確。
- 若模型過度固執或過度搖擺，反思提示詞本身就是可調控制桿，會改變 confidence 與 critic score 的平衡。

## Open Questions

- 何種任務最適合用 training-free sampling / decoding 逼出既有 reasoning，而不必進入 RL 訓練，仍缺穩定選型原則。
- RL 何時只是強化原有 path，何時真的學到新能力，目前仍取決於評測方法與訓練階段。
- self-correction 的最佳 reward signal 該偏向最終答案、過程正確性，還是兩者混合，仍在演進中。
- 不同模型族對肯定式 / 質疑式 reflection prompt 的敏感度差異，還缺足夠跨模型實證。

## Related Concepts

- [harness-engineering](./harness-engineering.md)
- [verbalized-feedback-learning](./verbalized-feedback-learning.md)
- [context-engineering](./context-engineering.md)

## Sources

- [ai-self-correction-decoding-workflow-reasoning](../summaries/ai-self-correction-decoding-workflow-reasoning.md)
