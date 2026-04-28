# AI 能自我修正嗎？從 decoding、workflow 到 reasoning 的技術發展整理

- source: `raw/AI 能自我修正嗎？從 decoding、workflow 到 reasoning 的技術發展整理.md`
- source link: [https://www.youtube.com/watch?v=m3i2mk5hs8U&t=4372s](https://www.youtube.com/watch?v=m3i2mk5hs8U&t=4372s)
- original title: AI 能自我修正嗎？從 decoding、workflow 到 reasoning 的技術發展整理
- author: [[Hung-yi Lee]]
- speaker: 李一駿助教
- published: 2026-04-25
- created: 2026-04-28
- type: YouTube transcript summary

## Summary

這份來源把語言模型的自我修正拆成三層：inference 時直接調整 decoding、在 harness / workflow 中加入 generation-verification 迴圈、以及透過 reasoning 訓練讓模型把檢查與修正內化。核心訊息不是「模型已經穩定會自我反思」，而是不同層級的自我修正各自有明確 trade-off：decoding 多半用額外算力換正確率，workflow 的 self-reflection 效果不穩定，reasoning / RL 則可能讓模型更自然地展現 verification，但它究竟是在喚醒既有能力，還是學到新能力，仍在研究中。

## Key Claims

1. 自我修正可從三個層級介入：修改 inference、修改 workflow、修改模型參數。
2. Contrastive decoding 類方法的核心是把輸出往「較不可能錯」的方向推開，優點是不必重訓模型，代價是額外計算。
3. DoLa、MTI 等方法都在想辦法降低 contrastive decoding 的額外成本，例如重用模型淺層輸出或利用 KV cache。
4. 單純 self-reflection 有時有效，但整體不穩定；外部 feedback 往往比模型自己反思更可靠。
5. 在有限算力下，verification / reflection 不一定比多 sample 幾次再做 majority vote 划算。
6. 模型擁有正確知識，不代表它就有自我修正能力；self-correction 可能是獨立能力。
7. RLVR 等 reasoning 訓練常會自然誘發 verification 與修正行為，但研究界仍在爭論這是在提高既有正確路徑的出現機率，還是真的學到新推理能力。

## Important Details

- 早期做法顯示，模型的 hidden representation 中可抽出「可能答對 / 答錯」的訊號，甚至可用 steering vector 方式把錯誤答案往正確方向推。
- 原始 contrastive decoding 以「大模型輸出減去小模型輸出」來放大差異；DoLa 則改用同一模型淺層與深層 logits 對比，減少額外模型成本。
- MTI（Minimum Test-Time Intervention）主張只在關鍵 token 位置介入，並透過在輸出尾端追加短 token 來保留 KV cache 重用能力，降低額外 decode 成本。
- 大規模實證顯示，internal self-reflection 平均只有小幅且不穩定的改善；若加入 checklist、error message、搜尋結果等 external feedback，改善幅度通常更穩。
- 有研究把修正行為拆成 `confidence level` 與 `critic score`：前者代表模型能否守住原本正確答案，後者代表模型能否把原本錯誤答案改對，兩者常互相拉扯。
- 插入的反思提示詞會改變模型個性：偏肯定的提示會提高 confidence、降低修正意願；偏質疑的提示會降低 confidence、提高 critic score。
- 若把算力預算納入比較，很多情況下先增加 sample 次數、再做 majority vote，比提早投入 reflection 更划算；verification 更像在高算力區間才值得加的奢侈品。
- 有論文指出 self-correction 與 factual knowledge 不等價，因為模型可能知道正確事實，卻無法在自己答錯後主動察覺並修正。
- REVISE 類方法把自我修正拆成 error detection 與 error correction 兩段學習，但也暴露一個問題：模型一旦 fine-tune 後會犯不同的新錯，原本教會的修正路徑不一定還適用。
- RLVR 把中間 reasoning 視為自由生成過程，只用最終可驗證答案給 reward；這類訓練常讓模型自然長出先提解法、再檢查、再修正的行為。
- 關於 RL 是否真的創造新能力，目前至少有兩派證據：一派認為 base model 本來就有正確 path，只是 RL 讓它更容易被 sample 到；另一派則認為若把推理過程正確性納入評估，RL 模型確實學到新的 reasoning path。

## Practical Takeaways From This Source

- 若不想重訓模型，可先考慮 decoding 層介入，但要明確評估額外 latency 與算力成本。
- 設計 agent workflow 時，不要把 self-reflection 當成萬靈丹；優先引入可驗證的外部回饋來源。
- 做 verification 類實驗時，baseline 不能只比單次輸出，還應比較同算力預算下的 majority vote 或更好的 sampling 方法。
- 若要提升模型的自我修正能力，不能只假設「知道更多就會改對」，而要把 error detection、feedback quality、reward design 分開處理。
- 在 reasoning 任務中，應同時關注最終答案與中間推理品質，否則 pass@k 可能高估真實能力。

## Related Concepts

- [self-correction-in-language-models](../concepts/self-correction-in-language-models.md)
- [harness-engineering](../concepts/harness-engineering.md)
- [verbalized-feedback-learning](../concepts/verbalized-feedback-learning.md)

## Alignment With Current Wiki

- 這份來源補上目前 wiki 尚未獨立整理的「self-correction」主題，並把它和既有 `harness-engineering`、`verbalized-feedback-learning` 連成一條從 workflow 到 training 的技術路線。
- 與既有 `harness-engineering-language-models-need-human-guidance` 一致：工作流與回饋設計會深刻影響模型表現；但這份來源更進一步指出，單純內部反思的穩定性有限，外部 feedback 與算力預算比較才是實務關鍵。
