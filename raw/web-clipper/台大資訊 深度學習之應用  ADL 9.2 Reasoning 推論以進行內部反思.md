---
title: "台大資訊 深度學習之應用 | ADL 9.2: Reasoning 推論以進行內部反思"
source: "https://www.youtube.com/watch?v=TFC3oM62wWk"
author:
  - "[[陳縕儂 Vivian NTU MiuLab]]"
published: 2026-10-20
created: 2026-10-20
description: "2026/10/19 Applied Deep LearningLectured by Yun-Nung Vivian Chen 陳縕儂 @ NTU CSIESlides credited from EMNLP 2024 Tutorial"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=TFC3oM62wWk)

2026/10/19 Applied Deep Learning  
Lectured by Yun-Nung Vivian Chen 陳縕儂 @ NTU CSIE  
Slides credited from EMNLP 2024 Tutorial

## Transcript

**0:01** · Let's first take a look at this reasoning aspect. This is the LingAgent diagram from earlier. Actually, F, as you mentioned, it's this middle part where it constantly dialogues with itself, like self-reflection, and the things produced can influence it, leading to different responses or action choices.

**0:21** · If we zoom in on this, will the action space change at the next time step because of the things it says to itself, resulting in a different context? It's like when you see a question, a Q, and then you think of some information related to it versus when you don't—you'll likely produce a different result.

**0:51** · So, suppose it asks a question that triggers a certain signal in you; how does this signal affect the response you generate ? It gets appended directly to its context, and then it generates the next token based on that context.

**1:11** · So, during this self-dialogue process, its short-term memory—which is its context—is constantly being updated because it's generating many, many tokens within that context. If you remember RAG, RAG also retrieves an external signal and puts it into the context. The information used here is more like long-term memory, which we will discuss later. Okay.

**1:42** · Regarding planning, during the inference phase, it chooses an action that is more aligned with its ultimate, higher-level goal. We can talk more about planning later, but the point here is that by generating self-dialogue tokens, it can lead to different action spaces or response outcomes later on.

**2:12** · The most famous example is the 2022 Google paper , Chain-of-Thought, which is very well-known; everyone should have heard of the concept it presents. In the past , language models weren't exactly great at solving math problems.

**2:37** · We tried giving them examples, showing them a problem and its answer, and then giving them a similar one. But they would still get it wrong. At the time, we found it very strange—why were they failing? Because you have to realize that this is an N-token generation problem. Asking the model to generate the final answer directly from the prompt is actually very difficult.

**3:07** · When it sees the question, it doesn't have the context to process it. Expecting it to provide an immediate reflex answer without any calculation—just jumping straight to the result—is incredibly hard. Chain-of-thought is a very simple technique. Just like teaching a child, we shouldn't just tell them the answer is "this." If you don't show them the process, they can't emulate your reasoning to think it through.

**3:33** · With the same model, if I just add to the example: "Okay, it started with five balls, then it got six more; 5 plus 6 equals 11, so the answer is 11."

**3:47** · That’s it. That is the only difference between the prompt on the left and the one on the right. Then, it can follow your process, and it generates these tokens. These green tokens are actually its reasoning. It mimics the logic: it started with 23 apples, did this and that, subtracted 20 to get 3, then bought 6 more, so 3 plus 6 equals 9.

**4:14** · Once it finishes that thought process, it can then increase the probability of generating "the answer is 9." So, with the same model, you just need to trigger this capacity for reflection, and you can get completely different results. Therefore , this green content—this reasoning —is actually an intermediate generation. We don't really want it, since we only need the final answer.

**4:43** · But to get the correct answer, we need to simulate a human-like mental process , letting the model have a dialogue with itself: "Oh right, when you see this problem, you're actually thinking about solving this part first, then adding, and finally the answer is nine.

**4:56** · " So we can also have the model do the same thing, to simulate the process of how your brain thinks, and thereby increase the probability of it generating the correct answer. And so why do we say that this kind of language agent has an advantage? It's that these highlighted reasoning patterns allow us to examine, "Oh, so that's how it's thinking, which is why it reached this final result." It possesses a greater degree of explainability.

**5:26** · Of course, we have already seen that reasoning itself can help make agents perform better. Here are some examples; the previous one was just solving math problems, but in the case of robots, adding reasoning can also lead to better performance.

**5:43** · Well, for instance, suppose we have a home robot, and it observes a state, or you could tell it, "You are cooking right now," and it finds out that there is no salt left. So, shouldn't it go and get some salt, right? So it should be supplemented with some reasoning. There is no salt left, so it needs to look for it.

**6:13** · It needs to find where the cabinet containing the salt is. Right, and only then will it perform the action of going to the cabinet to get the salt. By adding this reasoning, it can generate reasonable actions. If you ask it to learn directly— "there's no salt now, go right"—that is very difficult. But if it knows, okay, its goal is to go to the cabinet on its right that contains salt, then it knows it should turn right. It will then see the cabinet and open it.

**6:42** · All these actions are beneficial because it had that prior reasoning, allowing it to produce a logical action. Training it directly on these low-level actions is actually very, very difficult. Humans are the same; if I ask you to get salt, you first think about where the salt actually is. You wouldn't mindlessly turn right every single time just because you need salt and go to the same place.

**7:10** · In every situation, the location of the salt might be different . You might think about how you took it out yesterday, so today you should go look for it on the table. Yes, so this reasoning is actually crucial for the agent when it comes to generating actions. Beyond reasoning aiding action , action can also aid reasoning. As we saw earlier with retrieval, performing a search helps improve the response.

**7:45** · Conducting a search is essentially a tool, an action chosen to gather different contexts, which then leads to a better response. Action and reasoning can complement and assist each other, ultimately producing better results.

**8:05** · Here, I'd like to mention a very famous paper called ReAct. "ReAct" stands for reasoning and acting. If we simply ask the question without any reasoning or action, the original answer is incorrect. Let's take a look at what this question is. It seems to be about connecting an Apple device, perhaps some remote device. Yes.

**8:35** · So, the original answer is completely off-base, mentioning an iPod. Do you kids nowadays even know what an iPod is? It was a music player from the past; you can all look up what an iPod is. Now, what if we add reasoning? Adding reasoning means letting it "think" through the process. Thinking step-by-step.

**9:02** · It considers that the Apple Remote was originally for connecting to Apple TV, so it thinks it should be the iPhone, iPad, and iPod Touch. Even after performing reasoning, it’s still actually incorrect. What if we allow it to take action, like calling tools? Here, it is given tools to search the web, similar to RAG.

**9:33** · It decides to think about the Apple Remote first, then performs searches and gathers various results. After getting that information, it concludes with an answer, but it's still incorrect. So, even if you allow it to take action, without reasoning, it still doesn't get this question right.

**10:04** · And the example they provide shows that you need both reasoning and acting together. You must first think, "Okay, I need to search for something," which is the thinking process. So, its action is to perform that search, and it obtains observations, which are the results from that search. Next, it must continue to think and continue reasoning, which leads to the second part of the thought process.

**10:32** · After the second part of the thought process, it searches for other things, gets some results, and so on; it goes through some thinking, then searches for more things. You can see, if you carefully compare this with the previous "Action Only" approach, the things it searches for are actually quite similar, but because it thinks about why it needs to search for these things, after getting the complete results and processing them, it generates the correct answer.

**11:05** · That is, some keyboard function piece.

**11:11** · It’s not very important, but the point he wants to highlight here is that reasoning and action are both actually very important. For an agent to have sufficiently good thinking ability and enough external tools, both must coexist so that it can effectively utilize its reasoning and tools to finally achieve what you expect it to accomplish. Alright.

**11:33** · Of course, there will be situations where even if you add these reasoning and action components, it may still be wrong. So here, this is the original result, and he added one more thing: having humans teach the agent how to think or how to act.

**12:00** · He added a "ReAct," with human thoughts added; for some of its thoughts that it felt were incorrect, we have humans revise these thoughts.

**12:18** · After revising this thinking, hey, when you feed it back in, it can finally get a better result, and in the end, it’s correct. It’s a bit like if we want to synthesize some data with reasoning or action, we need to verify if its thought process matches what we expect; if it does, then of course we can take it and use it directly for training.

**12:41** · If it doesn't, we can make some changes and updates to the data, letting it learn from data that better matches human thought patterns; we can also make the agent more capable of performing behaviors that meet our expectations, because it will think just like you.

**13:03** · For example, if it wants to buy this item and hopes to pay less than 500, but the current asking price is 550, its thinking might be: "Since my budget is only 500, I should negotiate with them," like asking, "Can you give me a better price?" and so on.

**13:32** · If the model thinks the price is a bit over 89 but proceeds to buy it at 550, you’ll notice that its thoughts and actions are inconsistent. So, we may need to modify the data to better suit our needs, ensuring that its reasoning and its actions are aligned.

**13:49** · Depending on your use case, you teach the model how to think so that it arrives at the desired actions; having good reasoning provides the "why" behind those actions .

**14:07** · With this reasoning, it essentially conditions the subsequent output on these thoughts, increasing the probability of the corresponding action , which in a sense controls the agent's behavior. So.

**14:27** · Looking at it from a macro perspective, incorporating reasoning effectively expands your action space. If your input directly generates an output, the action space is limited, but by adding a reasoning step, you include these patterns in the process, thereby enlarging your action space. Because it now incorporates the content of your thoughts.

**15:06** · With these reasoning processes included in your action space, you allow for a much more diverse range of choices. Of course, we want this reasoning to guide the model toward choices we consider better. So, a larger, more diverse action space.

**15:37** · But what are the downsides? The benefit is a larger capacity—your action space allows for more possibilities— but when it comes to decision-making, it can be...Well...Large language

**16:07** · models learn from human knowledge, so they likely already understand certain concepts from their training data, which provides a good prior for reasoning. For instance, if there's an obstacle ahead and it's close by, the model inherently knows that moving straight would lead to a collision, so it might choose to turn right instead.

**16:42** · This, because the fact that if an obstacle is too close, you will hit it, is actually quite like common sense.

**16:52** · Our language model is supposed to learn these common senses from a vast amount of knowledge, so it can provide us with a pretty good reasoning prior to help with its subsequent actions. Yes. To improve this reasoning, the concept of the chain of thought we just saw is that between the input and output, there are these thoughts in between to guide it to generate a good output, which is actually a kind of planning process. But you can also think that there might be even better results.

**17:23** · For example, can I expand this process into many different paths? After getting many different results, I can find the best one by majority vote. It's a bit like, for instance, a math problem where I might have many ways to solve it; so this path might be one solution, and that is another, and if these methods reach the same result, it means this result should be quite trustworthy .

**17:50** · So if we can let it unfold different paths to plan, and in the end, several of them lead to the decision to take this action or provide this answer, then that overall might be a better answer. This is called self-consistency with chain of thought, which is finding the consistent result from within. Of course, this can also be more complex; this chain of thought does not have to be linear.

**18:22** · We can have this tree-like structure, expanding and expanding, and it can grow very, very complex, because at every single node, your thinking can inherently be very diverse and unfold like this.