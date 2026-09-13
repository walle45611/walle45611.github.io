---
title: "LLM 推理引擎到底是怎么工作的：从一个 token 到 KV 缓存、连续批处理，再到 Agent Loop 如何把 GPU 榨干"
source: "https://x.com/shao__meng/status/2098783138118041760?s=46"
author:
  - "[[@shao__meng]]"
published: 2026-09-12
created: 2026-09-14
description: "How Inference Engines Actually Work大多数 AI 开发者使用 LLM 时基本就是“提示进去、回答出来” 的 LLM API 思维，中间是一团模糊的“模型在推理”。来自 TogetherAI @togethercompute 团队 Zain @za..."
tags:
  - "clippings"
---
![Image](https://pbs.twimg.com/media/HSBfI01awAA_WAM?format=jpg&name=large)

## How Inference Engines Actually Work

大多数 AI 开发者使用 LLM 时基本就是“提示进去、回答出来” 的 LLM API 思维，中间是一团模糊的“模型在推理”。来自 TogetherAI [@togethercompute](https://x.com/@togethercompute) 团队 Zain [@zainhas](https://x.com/zainhas) 分享内容把这团迷雾拆开，沿着**一个请求的完整生命周期**​，讲清楚推理引擎（vLLM、SGLang、TensorRT-LLM 这类系统）内部到底发生了什么。整体有一条清晰的主线：

> **推理引擎的本质，是一台“让 GPU 始终饱和”的机器。**

GPU 很贵，同时有成百上千个用户、每个用户的请求进度都不同，而每个用户每步只产出 1 个 token、显存又总是比算力先耗尽——所有引擎设计，都是围绕这个矛盾展开的。

[

![](https://pbs.twimg.com/card_img/2098576852067737600/S0XVEvEr?format=jpg&name=360x360)

](https://docs.google.com/presentation/d/1ljkXsGI8fCtHEwg7XVaTLc8R2aGtaZIox2bpjhJiVnY/edit?usp=sharing)

[

docs.google.com

How Inference Engines Actually Work

OPEN SOURCE AI SUMMIT · 2026 How inference engines actually work Zain Hasan · Together AI

](https://docs.google.com/presentation/d/1ljkXsGI8fCtHEwg7XVaTLc8R2aGtaZIox2bpjhJiVnY/edit?usp=sharing)

> Sep 12
> 
> how inference engines actually work - releasing full talk slides! i cover everything in the lifetime of a request e2e: > the inference engine > kv + prefix caching > continuous batching > paged attention > chunked prefill > sampling > agentic loops from inside the engine

## 第一部分：一个请求的一生

**1\. 输入端：模型只认识整数**

模型从不接触文本。分词（tokenization）发生在 CPU 上、GPU 介入之前："What are the top 3 things to do in NYC?" 会被切成 11 个 token，变成 4827 553 290 ... 这样的整数序列。词表约 10 万条目，平均一个 token 约四分之三个英文单词。这解释了为什么“计费按 token”以及为什么 token 数不等于字数。

**2\. 核心循环：一次前向传播，一个 token**

自回归生成的真相极其“笨拙”：把目前所有 token 喂进模型 → 得到下一个 token → 拼回去 → 再跑一遍。一段 500 词的回答大约需要 700 次完整的前向传播，直到出现停止符或达到长度上限。没有一次性“生成一段话”这回事，只有这个循环。

**3\. 关键分野：Prefill 与 Decode**

同一个模型，两种截然不同的负载——**这是全篇最重要的一个洞察，后面几乎所有引擎设计都由此推导而来**​：

- **Prefill（预填充）**​：一次性并行处理整个 prompt，大量矩阵运算、GPU 算力被喂饱，属于**计算受限**（compute-bound）。它决定了**首 token 延迟（TTFT）**​。
- **Decode（解码）**​：每步只生成一个 token，计算量极小，却要读取之前所有 token 的状态，属于**内存带宽受限**（memory-bound）。它决定了**每秒输出 token 数（TPS）**​。

用一句话记住：prefill 是“一大口算完”，decode 是“每次嚼一粒，但要翻出全部历史”。

**4\. KV Cache：用空间换掉平方级的时间**

注意力机制需要访问之前每个 token 的信息。没有缓存时，每次 pass 都要把全部历史重算一遍，总计算量随长度**平方增长**​。KV cache 的做法是：每个 token 的 Key/Value 只算一次、存下来，之后每步只“新增一列计算 + 读取缓存”。

代价是：**这个缓存正是吃掉 GPU 显存的东西**​。它的大小正比于 token 数 × 层数 × 头数 × 精度字节数，而且和 batch 里的用户数相乘。这就是“显存总是比算力先耗尽”的根源——也是后文所有内存管理技术的出发点。

## 引擎设计：一台“饱和机器”的四个部件

**架构：一条流水线，只有最后一站碰 GPU**

```text
API server (CPU)  →  Scheduler (CPU)  →  Worker × N (每 GPU 一个)
HTTP 收发/分词/反分词    每步决定谁跑、跑多少 token      前向传播 + 采样
```

CPU 与 GPU 是**流水线并行**的：GPU 跑第 N 步时，CPU 已经在反分词第 N−1 步的输出。调度器（Scheduler）是真正的心脏——每一次前向传播前，它都要回答：哪些请求进这一批？各占多少 token 预算？用哪些 KV 块？

**连续批处理（Continuous Batching）：每一步都重新组批**

传统静态批处理把一批请求锁死到最长的那个结束为止——先完成的请求所在的位置，就是“付了钱的 GPU 在空转”（幻灯片里那些灰色格子）。连续批处理则**每一次 pass 都重新决定批的组成**​：某请求一结束，新请求立刻补进空位，没有一次空闲的 pass。这是 Orca 论文（2022）带来的范式转变，也是现代引擎吞吐量数量级提升的第一功臣。

**PagedAttention：像操作系统管内存一样管 KV cache**

早期做法按“最长可能长度”为每个请求预留显存，结果三个请求就把 GPU 塞满，且大部分预留空间是空的——第四个用户只能排队。PagedAttention（vLLM 的核心创新）把 KV cache 切成**固定大小的块（block）**​，从共享池里按需分配、逻辑上用块表（block table）串联。

效果：同样的显存跑四个请求，池子还剩一半。碎片率从预留制下的 60–80% 降到 4% 以下。这里幻灯片点出一个行业常识：**限制 batch size 的通常不是算力，而是 KV 容量**​。

**Chunked Prefill：长 prompt 切片，别让老用户卡顿**

一个 8K token 的 prompt 塞不进单次 2K token 的 pass 预算，那就切成 4 片逐次处理。关键在于每一步的预算里还会**捎带上活跃用户的 decode token**——也就是说，一个新用户的巨型 prompt 进来时，正在输出的用户不会停下来干等，首 token 延迟的尖峰被摊平了。

**采样与输出：10 万个分数变成一个词**

最后一段流水线：GPU 输出 logits（词表里每项一个分数）→ 依次施加 temperature、top-p、top-k、各类惩罚 → 采样出 token id → CPU 反分词成 "mat" → 流式发回给你。两个 GPU 阶段加一个 CPU 阶段，每生成一个 token 都要走一遍。

## 第二部分：Agent 把这一切推向极致

**Agent 循环在引擎眼里长什么样**

用户提问 → 模型决定调用工具 → 工具执行 → 结果追加进对话 → 重复 N 次。关键事实：**每一轮都是一个全新请求，重发到目前为止的完整对话**​。

于是引擎看到的是：turn 1、turn 2、turn 3……每一轮的 prompt 都是上一轮的超集，只有末尾一小段是新的；而每轮的输出（一次工具调用）只有几十个 token。**输入越来越长、输出始终很短**——这正好打在 prefill 和 KV cache 的痛点上。

**“缓存为王”：55 倍与 10 倍的差距**

全文最有说服力的一页。在一个 100 轮的 agent 循环里：

- **没有 prefix caching**​：每一轮都重算整个对话，总 prefill 工作量随轮数**平方增长**​，约等于第一轮的 55 倍；
- **有 prefix caching**​：每轮只 prefill 新增的尾部，总工作量**线性增长**​，约 10 倍，而且每轮的首 token 都很快。

Prefix caching 的原理：不同请求共享相同前缀（系统提示、40 个工具的定义、few-shot 示例），引擎对每个 KV 块做哈希，相同哈希的块直接物理复用——"You are a helpful assistant… here are your 40 tools…" 只算一次，被所有以同样方式开头的请求共享，**用户无需任何配置**​。

（值得注意：这项技术正是为 agent 时代而生的。演讲把它放在第二部分讲，而幻灯片页码显示它原本排在第一部分——结构上能看出作者意识到“缓存”才是 agent 与引擎的交汇点。）

**引擎针对 agent 流量的四项对策**

| Agent 流量特征 | 引擎的对策 |
| --- | --- |
| 跨轮次、跨用户共享的长前缀 | Prefix caching |
| 并行工具调用与子 agent 的突发流量 | Continuous batching |
| 超长输入、极短输出 | Chunked prefill；prefill 与 decode 分集群部署 |
| 工具执行期间会话静默 | KV cache 卸载到 CPU 内存，下一轮恢复 |

最后一行尤其有意思：agent 在等一个搜索或代码执行结果时，它的 KV cache 在 GPU 上白白占着地方。引擎把它换页到主存，把显存腾给活跃请求，等下一轮请求到来再换回来——又是操作系统式的内存管理思路。

## 四点总结

1. **一次传播，一个 token**——模型从不见文本，每次只回答一个整数；
2. **Prefill ≠ Decode**——先是计算受限，后是内存受限，大部分引擎设计由此推导；
3. **引擎是饱和机器**——连续批处理填满每一次 pass，分页式 KV 内存为它腾出空间；
4. **Agent 活在缓存上**——保持前缀稳定，速度和成本优势自动到手。