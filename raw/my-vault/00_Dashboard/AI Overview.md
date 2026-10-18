# AI Overview

## Paper Notes

### NLP and Representation Learning

[[@efficientwordrepresentations2013|Efficient Estimation of Word Representations in Vector Space]]
[[@attention2017|Attention Is All You Need]]

[[@bert2019|BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding]]
[[@chainofthought2022|Chain-of-Thought Prompting Elicits Reasoning in Large Language Models]]
[[@distributedwordsphrases2013|Distributed Representations of Words and Phrases and their Compositionality]]
[[@gemma42026|Gemma 4 Technical Report]]
[[@generativepretraining2018|Improving Language Understanding by Generative Pre-Training]]
[[@llama2023|LLaMA: Open and Efficient Foundation Language Models]]
[[@unsupervisedmultitask2019|Language Models are Unsupervised Multitask Learners]]
[[@aligntranslate2015|Neural Machine Translation by Jointly Learning to Align and Translate]]
[[@roformer2021|RoFormer: Enhanced Transformer with Rotary Position Embedding]]
[[@seq2seq2014|Sequence to Sequence Learning with Neural Networks]]

### RAG, Knowledge Graphs and Domain Applications

[[@zhifangdantai2026|ZhiFangDanTai（閱讀中）]]

[[@agenticragsurvey2025|Agentic Retrieval-Augmented Generation: A Survey on Agentic RAG]]
[[@atlas2023|Atlas: Few-shot Learning with Retrieval Augmented Language Models]]
[[@benchmarkingragmedicine2024|Benchmarking Retrieval-Augmented Generation for Medicine]]
[[@localglobalgraphrag2024|From Local to Global: A Graph RAG Approach to Query-Focused Summarization]]
[[@hierarchicalmemorywiki2025|Hierarchical Memory Organization for Wikipedia Generation]]
[[@medicalgraphrag2025|Medical Graph RAG: Evidence-based Medical Large Language Model via Graph Retrieval-Augmented Generation]]
[[@rag2020|Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks]]
[[@llmkgroadmap2024|Unifying Large Language Models and Knowledge Graphs: A Roadmap]]

### LLM Preference Learning and Alignment

[[@dpo2023|Direct Preference Optimization（DPO，閱讀中）]]

[[@domainfinetuningpeft2025|Domain Specific Finetuning of LLMs Using PEFT Techniques]]
[[@lora2021|LoRA: Low-Rank Adaptation of Large Language Models]]
[[@qlora2023|QLoRA: Efficient Finetuning of Quantized LLMs]]
[[@tcmftp2024|TCM-FTP: Fine-Tuning Large Language Models for Herbal Prescription Prediction]]

### LLMs and Long Contexts

[[@lostmiddlehow2024|Lost in the Middle: How Language Models Use Long Contexts]]

### Agents and Memory Systems

[[@chatprocess2026|From chat to process: A conversational agent framework for social business process management]]

[[@memgpt2023|MemGPT Towards LLMs as Operating Systems]]
[[@generativeagents2023|Generative Agents Interactive Simulacra of Human Behavior]]

[[@agenticarchitecturessurvey2026|Agentic AI: A Comprehensive Survey of Architectures, Applications, and Future Directions]]
[[Agentic AI - Autonomous Intelligence for Complex Goals - A Comprehensive Survey|Agentic AI: Autonomous Intelligence for Complex Goals---A Comprehensive Survey]]
[[@agenticteamscience2026|Agentic AI and the Rise of in Silico Team Science in Biomedical Research]]
[[@lifelongagents2026|Lifelong Learning of Large Language Model Based Agents: A Roadmap]]
[[@react2023|ReAct: Synergizing Reasoning and Acting in Language Models]]

### Clinical AI and Traditional Chinese Medicine

[[@medicaltextreview2026|Artificial Intelligence Language Models for Medical Text Analysis: A Systematic Review]]
[[@cprs2025|CPRS: A Clinical Protocol Recommendation System Based on LLMs]]
[[@kgapg2021|KGAPG: Knowledge-Aware Neural Group Representation Learning for Attentive Prescription Generation of Traditional Chinese Medicine]]
[[@llmmedicine2023|Large Language Models in Medicine]]
[[@llmphysicianreasoning2026|Performance of a Large Language Model on the Reasoning Tasks of a Physician]]
[[@tcmevalsdt2025|TCMEval-SDT: A Benchmark Dataset for Syndrome Differentiation Thought of Traditional Chinese Medicine]]
[[@tcmllmpr2024|TCMLLM-PR: Evaluation of Large Language Models for Prescription Recommendation in Traditional Chinese Medicine]]
[[@tcmpr2022|TCMPR: TCM Prescription Recommendation Based on Subnetwork Term Mapping and Deep Learning]]

### Computer Vision and Edge AI

[[@residuallearning2015|Deep Residual Learning for Image Recognition]]
[[@yolov8riscv2024|Energy-efficient implementation of YOLOv8, instance segmentation, and pose detection on RISC-V SoC]]
[[@fpgacnnyolo2026|Low-Cost FPGA-Enhanced CNN Accelerator for Real-Time YOLO Object Detection & Classification]]

### Evaluation and Benchmarks

[[@llmjudge2023|Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena]]
[[@bfcl2025|The Berkeley Function Calling Leaderboard (BFCL): From Tool Use to Agentic Evaluation of Large Language Models]]

### Neural Network Architecture

[[@layernorm2016|Layer Normalization]]

## Concept Notes

### Fundamentals and Loss Functions

[[Logistic Regression 與 Log-Odds 計算|Logistic Regression and Log-Odds]]

[[Note/Research/01 - Entropy and Information|Entropy and Information]]
[[Note/Research/02 - Likelihood NLL and Cross-Entropy|Likelihood, NLL and Cross-Entropy]]
[[Note/Research/03 - MAE and MSE|MAE and MSE]]
[[Note/Research/04 - Softmax|Softmax]]
[[Note/Research/05 - Feature Normalization|Feature Normalization]]

### Neural Networks and Activation Functions

[[Note/Research/01 - Neuron Features and Parameters|Neuron, Features and Parameters]]
[[Note/Research/02 - Hidden Layers and Multiple Features|Hidden Layers and Multiple Features]]
[[Note/Research/03 - Activation Functions|Sigmoid and ReLU]]
[[Note/Research/04 - Batch Normalization|Batch Normalization]]

### Gradient Descent and Optimization Basics

[[Note/Research/01 - Gradient Descent and Learning Rate|Gradient Descent and Learning Rate]]
[[Note/Research/02 - Local Minima Saddle Points and Critical Points|Local Minima, Saddle Points and Critical Points]]
[[Note/Research/03 - Taylor Expansion Hessian and Eigenvalues|Taylor Expansion, Hessian and Eigenvalues]]
[[Note/Research/04 - Batch Mini-batch and Batch Size|Batch / Mini-batch / Batch Size]]
[[Note/Research/05 - Learning Rate Scheduling and Warm-up|Learning Rate Scheduling and Warm-up]]

### Optimizers

[[Note/Research/01 - Momentum|Momentum]]
[[Note/Research/02 - RMSProp|RMSProp]]
[[Note/Research/03 - Adam|Adam]]
[[Note/Research/04 - Optimizer Comparison|Optimizer Comparison]]
[[Note/Research/05 - AdaGrad and Gradient Scaling|AdaGrad and Gradient Scaling]]

### Backpropagation

[[Note/Research/01 - Chain Rule and Total Derivative|Chain Rule and Total Derivative]]
[[Note/Research/02 - Computational Graph and Backpropagation|Computational Graph and Backpropagation]]
[[Note/Research/03 - Gradient Accumulation Across a Batch|Gradient Accumulation Across a Batch]]
[[Note/Research/04 - Differentials Gradients and Jacobians|Differentials Gradients and Jacobians]]

### Generalization and Model Evaluation

[[Note/Research/01 - Train Validation and Test Sets|Train / Validation / Test]]
[[Note/Research/02 - K-Fold Cross Validation|K-Fold Cross Validation]]
[[Note/Research/03 - Underfitting Overfitting and Model Bias|Underfitting / Overfitting / Model Bias]]
[[Note/Research/04 - Model Complexity Data Augmentation and Optimization Failure|Model Complexity / Data Augmentation / Optimization Failure]]

### Sequence Models

[[Recurrent Neural Network (RNN)]]

---
## IT Notes

### LLM Systems and Runtime Environments

[[vLLM + Gemma4 LoRA 安裝與啟動流程]]
[[miniconda-troubleshooting]]
[[NVIDIA NVML Driver 版本不相容問題]]

## 模型部署實務

[[vllm-gemma-4-lora-我踩過的兩個坑|vLLM、Gemma 4 與 LoRA]]
