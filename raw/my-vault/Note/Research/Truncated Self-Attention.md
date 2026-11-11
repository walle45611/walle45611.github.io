# Truncated Self-Attention

Truncated self-attention（截斷自注意力）把每個位置能參與注意力計算的範圍限制在附近的固定視窗內。整體很像 sliding window：位置往前移一格，視窗也跟著移；輸入 token 仍在序列中，只是超出視窗的位置不直接參與該次注意力計算。

## Sliding Window 例子

以下設定為因果式注意力，視窗寬度 $w=3$，並把目前位置算進去。因此每個位置最多看自己和前兩個位置：

```text
位置 A： [A]
位置 B： [A B]
位置 C： [A B C]
位置 D：    [B C D]
位置 E：       [C D E]
位置 F：          [D E F]
```

位置 E 只看 C、D、E；滑到位置 F 時視窗變成 D、E、F，A、B 就離開可注意範圍。這是「截斷」的意思：原本完整 self-attention 可建立全序列的位置配對，現在只保留滑動視窗內的配對。

$w$ 是視窗寬度的符號，並非固定通用數值；模型設計時會依任務設定。不同論文或實作可能把 $w$ 定義為包含目前位置的總長度，或改以左側範圍 $L$、右側範圍 $R$ 分別表示。閱讀設定時要確認它們的計數方式。

## 計算量與取捨

令序列長度為 $T$，完整 self-attention 要計算所有位置彼此的注意力配對，注意力部分的計算量約為 $O(T^2)$。若每個位置只看固定寬度 $w$ 的視窗，則約為 $O(Tw)$；當 $w$ 不隨 $T$ 增長時，這部分近似線性 $O(T)$。這裡比較的是注意力配對計算，不包含其他層的成本。

視窗變大，可用的上下文變多，計算量也會增加；視窗變小則較省計算，但相隔很遠的 token 不能直接互相注意，可能損失長距離資訊。

## 用於串流語音

語音會逐幀到達，模型可以讓位置 $t$ 只看 $x_{t-L},\ldots,x_t,\ldots,x_{t+R}$。左側 $L$ 限制可用歷史，右側 $R$ 是 look-ahead：

- $R=0$ 時不看未來，可立刻依目前與過去的訊號計算。
- $R>0$ 時需要等到未來 $R$ 幀到達，通常能提供更多上下文，但會增加延遲。
- 若同時限制 $L$ 與 $R$，每個位置只計算固定寬度的注意力範圍；若只限制未來、仍允許看無限歷史，整體計算量仍可能隨序列長度平方成長。

論文 [[@transformertransducer2019|Transformer-Transducer]] 用這種方式讓 Transformer encoder 支援串流語音辨識。相關基礎可參考 [[@attention2017|Attention Is All You Need]]。

## Related

- [[@transformertransducer2019|Transformer-Transducer: End-to-End Speech Recognition with Self-Attention]]
- [[@attention2017|Attention Is All You Need]]
