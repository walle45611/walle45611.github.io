[[Assets/Note/Research/Papers/Generative Agents - Interactive Simulacra of Human Behavior/Generative Agents - Interactive Simulacra of Human Behavior.pdf|論文 PDF]]

## 初讀摘要

- Generative Agents 研究如何讓代理人在互動環境中呈現連貫、可信的人類行為。
- 系統將經驗存入記憶，依相關性等條件檢索，再進行反思與規劃；作者以 25 個代理人的模擬小鎮展示社交互動。
- 消融實驗支持記憶、規劃與反思的作用。評測關注行為可信度與模擬中的互動，不代表代理人具有人類心理或能可靠完成所有真實任務。

摘要依據：PDF 摘要與導論。


第一，提出生成式代理人，也就是能根據代理人不斷變化的經驗與環境進行動態條件化的人類行為可信模擬。

第二，提出一種新穎架構，使生成式代理人能夠在動態演化的情境中記憶、檢索、反思、與其他代理人互動並進行規劃。此架構利用大型語言模型強大的提示能力，並補充這些能力，以支援更長期的代理人一致性、管理動態演化記憶的能力，以及遞迴產生更高層次反思的能力。

第三，提出兩種評估：受控評估與端到端評估。這些評估建立了架構組件重要性的因果效果，並識別出系統失效的情況，例如不當的記憶檢索。

過去有 HCI、NPC、認知架構、LLM prompt 等基礎，但它們都不足以處理長期一致、會累積經驗、會社交互動的 agent；所以作者才提出 memory stream + reflection + planning 的架構。

作者也在註腳說明，這些代理人的對話風格有時會讓人覺得過於正式，這很可能是底層模型 instruction tuning 的結果。他們預期未來語言模型會更容易控制寫作風格。另一個註腳則說，前面 John Lin 那段以分號分隔的描述，會在模擬開始時作為記憶輸入到代理人的初始記憶中。

這一整段 Section 3 的作用很明確：它不是在正式介紹 memory stream 的演算法，而是在展示 **這個 agent 架構跑起來後，看起來會產生什麼行為**。也就是說，它先給你看現象：日常作息、自然語言互動、使用者干預、資訊擴散、關係形成、活動協調；後面的 Section 4 才解釋這些行為背後靠的是 memory、reflection、planning。

![[Assets/Note/Research/Papers/Generative Agents - Interactive Simulacra of Human Behavior/generative-agent-memory-architecture.png]]

第一，**Perception**。Agent 會觀察環境，例如看到某人、聽到對話、發現咖啡機狀態改變、看到某人在做某件事。這些 observation 不是用完就丟，而是會進入 memory stream。

第二，**Memory Stream**。這是 agent 的長期記憶庫。裡面包含 agent 自己做過的事、看過的事、和別人講過的話、環境中的變化、先前形成的 plans 和 reflections。這不是普通聊天紀錄，而是 agent 行為生成的核心資料層。

第三，**Retrieval**。當 agent 要決定下一步要做什麼時，系統不會把整個 memory stream 塞進 prompt，因為記憶太多會干擾模型，也可能超過 context window。Section 4.1 說 memory stream 會用 retrieval function 取出一部分記憶給 LLM；每個 memory object 會有自然語言描述、建立時間戳、最近存取時間戳，而最基本的記憶單位是 observation。

第四，**Reflection**。只有 raw observations 不夠，因為 agent 需要從零散事件中形成高層次推論。作者在 4.2 舉例：如果 Klaus 只看互動頻率，可能會選 Wolfgang 作為想共度一小時的人；但如果能反思，就會從 Klaus 長時間做研究這些記憶中歸納出「Klaus 對研究有熱情」，再做出更合理的社交與行為判斷。

第五，**Planning / Action**。Planning 讓 agent 的行為在時間上保持一致。作者說 plan 包含地點、開始時間與持續時間，而且 plans 也會被存進 memory stream，和 observations、reflections 一起被檢索。這樣 agent 決定行為時，不只是看到當下刺激就反應，而是會考慮既有計畫、過去記憶與高層次反思。




## Section 4.3：Planning and Reacting

![[Assets/Note/Research/Papers/Generative Agents - Interactive Simulacra of Human Behavior/generative-agent-memory-stream-retrieval.png]]

Planning 解決的是「agent 行為不能只靠當下反應」的問題。沒有 planning，agent 可能每個時間點都做出看似合理但整體很荒謬的行為，例如一直吃午餐、一直跑來跑去、或計畫不連貫。作者說 plan 會描述 agent 未來一連串 action，包含 location、starting time、duration，並且 plans 也會存入 memory stream，讓 agent 後續行動時能一起考慮。

它還有 reacting and updating plans。每個 time step，agent 會感知世界，把 observation 存進 memory stream，然後判斷是繼續原計畫，還是要反應並更新計畫。例如 John 看到 Eddy 在花園散步，系統會檢索 John 對 Eddy 的記憶，知道 Eddy 是他的兒子、正在做音樂作品、喜歡在思考音樂時散步，於是 John 可能決定問 Eddy 音樂作品進行得如何。


Generative Agents 提供了一個 memory-centered LLM agent architecture。它證明 agent 若要維持長期一致行為，不能只依賴單次 prompt，而需要 memory stream、retrieval、reflection、planning。我的 LLM Wiki 則把這個想法從社會行為模擬轉移到中醫臨床推理：memory stream 不再是人物生活經驗，而是 wiki-style clinical memory；reflection 不再是人物性格推論，而是病例與方劑知識的整理；planning 不再是日常行程，而是中醫推理與處方生成流程。


結構化環境 tree / JSON → 轉成自然語言給 LLM 推理 → LLM 產生自然語言 action → 再轉回遊戲位置、物件狀態、JSON 更新
