---
title: "台大資訊 深度學習之應用 | ADL 9.3: Memory 記憶中有各種重要資訊"
source: "https://www.youtube.com/watch?v=B07lRYe2D2w"
author:
  - "[[陳縕儂 Vivian NTU MiuLab]]"
published: 2026-10-20
created: 2026-10-20
description: "2026/10/19 Applied Deep LearningLectured by Yun-Nung Vivian Chen 陳縕儂 @ NTU CSIESlides credited from EMNLP 2024 Tutorial"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=B07lRYe2D2w)

2026/10/19 Applied Deep Learning  
Lectured by Yun-Nung Vivian Chen 陳縕儂 @ NTU CSIE  
Slides credited from EMNLP 2024 Tutorial

## Transcript

**0:02** · Next, for the second part, we just finished talking about reasoning, so now what is memory? We just mentioned the concepts of long-term and short-term, which are actually very easy to understand. As for short-term memory, it’s basically all the information you get in the moment when the agent is interacting with the environment, like if the environment spits out an error message; that becomes part of the short-term memory, which gets appended.

**0:30** · For this short-term memory, everything obtained during this short-term interaction with the environment is put up there, but the downside is that your context window is limited. So, if you just mindlessly put everything there, it might get very noisy. How can you ensure it effectively utilizes this short-term information?

**0:58** · These things still need to be included because, if you want this agent to place an online order for a product for you, it has to first click into the search page, search for the product, and once the product appears, it needs to click on the corresponding page, select the specifications you want, and finally place the order. During this process, it must know what happened previously to understand what it needs to do next.

**1:33** · So, all the actions that occurred and all the pages it saw are its short-term memory, and they all have to be appended to its context to help it determine its next move. Right. The original model, at the start, might have some things stuffed into the context, such as its instructions, as well as what the model outputs regarding how it thinks, what it's thinking, what actions it can take, what it observes, and how it should process things. What is long-term memory?

**2:05** · The short-term memory we discussed here can simply be thought of as an external document, such as information you’ve retrieved. If we want to access this external information, we might treat it like a notebook, where you keep things you've jotted down off to the side. So, if I want to access these notes I've already written, I have to open the file to read it, or perhaps add some key points to it.

**2:33** · So, this part is more like, when you are currently processing a task, you don't record the real-time information, but you might record some important information on the left side.

**2:48** · For example, uh, for instance, after completing many tasks, you might gain some takeaways, like this website actually has a bug, so you have to click search twice for it to work.

**3:01** · Right, so you might need to take this key point you learned after a series of actions and write it down in your long-term memory, so you'll remember it next time, yes. So here, you might store some experiences, or knowledge and skills you believe are important for the agent's execution. So, if everyone is using these agents now, they have many different skills written into them, like browsing the web or querying data, and we hope that as they gain new experiences, they can update their long-term memory.

**3:33** · So both long-term and short-term memory will occupy the context, as some long-term memories are relevant to the task, so they must be brought in, and short-term memory as well, because the actions you are currently taking have a history that informs the next decision.

**4:00** · Coordinating these different pieces of information and putting them together into the context to help the agent make the next decision is actually a very important part of current agent development. So, there isn't really a clear-cut way this should be done, just to let you know that there is short-term memory information and long-term memory information, but what to put in which, and how they interact —which is related to your task—is something everyone can further develop and research.

**4:30** · Let's look at the generative agent's memory situation; since the context window is fixed, we obviously want to put what we consider relevant into the context, as relevant information might help us make better final output choices, whether for a response or an action.

**4:57** · Right, so we want it to be able to use what is relevant to it. But since we might not know which information is relevant— meaning what in the memory is relevant —a common approach is that we first find some relevant memories.

**5:19** · For example, once I perceive some current signals, we will retrieve relevant content from the memory, just like doing RAG to search for relevant knowledge from external data. You can also perform retrieval from these previous short-term memories and then place them into its context to help it with the final response.

**5:47** · There are actually many different ways to search and store these memories. Because as human beings, any memory you have experienced is stored in your brain; it’s just a matter of whether you recall it or not, and even if you don't recall it, it’s possible you still remember small pieces of information.

**6:18** · But within current agent frameworks, if you don't retrieve it and place it into its context, it actually knows nothing at all; it simply wasn't trained into its "brain." Yes, so we might want to stuff everything it has experienced into its context, but the context isn't that long, so that's where the difficulty lies. I'd like to share a quite interesting experiment involving generative agents, where in 2023, they wanted to observe the behavior of these agents by simulating a sequence of events.

**6:49** · For example, you normally wake up in the morning, brush your teeth, take a shower, eat breakfast, make breakfast, then maybe go to school to catch up with everyone, discuss some information, or go to work, and so on; simulating a series of these events and giving them to the agents—it's sort of like this is Agent 1.

**7:10** · And then, we hope that on some later day, the agent can recall these relevant memories, so here we are testing how to retrieve the memory relevant to the current situation—for instance, if one day it receives a question asking, "What are you looking forward to most?"

**7:35** · Right, because this agent has some previous experiences, it will retrieve some related events from those past experiences to help it answer, "Oh, I'm most looking forward to the Valentine's Day party."

**7:57** · Since it might have retrieved that someone invited it to this party and that the party was at a certain location, it has to pull in these relevant events to be able to generate this current sentence, just like how a human's response relates to the experiences you have gone through before. So, how do we retrieve this information?

**8:27** · It might generate many different criteria here; for instance, more recent memories might be more vivid, so you have these more salient memories, plus factors like how important or relevant a memory is. As for the level of importance, if it’s a minor matter, it’s obviously not that important, and in that case, you probably wouldn't need to retrieve it.

**8:52** · So, the memories that are ultimately retrieved actually imply many different criteria, meaning simply finding what's relevant won't necessarily produce good results. This is also a problem people might encounter when building agents now, because these agents might just be searching for relevance, but relevance doesn't necessarily mean it should be used. So, figuring out how to expand on the memories that should be used is actually very important.

**9:19** · I'd like to share something quite interesting, which is this kind of simulation experiment focused on social scenarios.

**9:33** · It found that when you are in different moods, you will have different responses, which is actually very reasonable. For example, if your friend is a law student, and they say they just passed this law exam, like the bar exam. Okay, so, If this person thinks, "Wow, they are so amazing," and they feel very proud or excited, their response might be, "Oh, I'm so glad to know that, let's have a party to celebrate."

**10:04** · That's just because, for example, you aren't a law student either, so you're just happy for them when they share their feelings. But if another person had also taken the exam but didn't pass, hearing that classmate say this might make them feel a bit jealous or disappointed, and their response would be quite different; they might say, "Oh, I'm happy for you," but they probably wouldn't say, "Let's have a party to celebrate," as that would seem a bit strange.

**10:35** · So, here we know that when you have different moods, your response may vary. This was their motivation, so they conducted a series of experiments to simulate it. Suppose you are the same person; you start your morning, spill coffee on yourself, and you're in a bad mood, then you receive an email mid-day from an old project lead constantly telling you to hurry up with a feature, which just makes you feel even more annoyed.

**11:07** · Okay, so after finally getting this bug fixed, I felt really happy. So, you might experience different moods throughout the day due to various events. They simulated a bunch of these to see what kind of emotional reactions or expressions would follow different moods.

**11:31** · For instance, if someone is feeling nervous , it might trigger certain subsequent emotions; others might express care or concern, or if you’re anxious, others might show empathy and so on.

**11:48** · Basically , they simulated a great number of social interaction events and moods to observe what kind of behaviors would arise from those specific states. And here, they conducted an experiment that I found to be really interesting.

**12:14** · Building on the previous scenario, where there are many moods and various events occurring, your mood changes accordingly. Next, in this company, they hold a meeting to discuss certain matters. There are multiple agents, each with their own prior experiences, and they start discussing these things.

**12:39** · They discovered that when you’re in a good mood, the meeting is more likely to reach a consensus; when you're in a bad mood, the meeting often fails to reach a consensus, with people objecting to this and that. This is actually quite similar to what has been discovered in sociology, which is that if you have a positive mood, you are more likely to reach a peaceful decision during group discussions.

**13:04** · For example, this is why when you and your classmates need to discuss something with a teacher, you should pick a time when the teacher is in a good mood; you'll reach a consensus more easily and arrive at a conclusion. But if, due to some event, they are in a bad mood, and you approach them for a discussion, they might not mean to be difficult, but you just won't be able to reach a consensus, and they might think you’re doing it wrong or that you should do it this or that way. Such differences are definitely possible.

**13:37** · So , this is actually quite similar to sociology, where they use agent behaviors to simulate how different experiences and memories can lead to different actions from these agents.

**13:53** · Right, so these positive ones are where there's agreement, and the native ones are where there's no agreement. And here you can see that the scenarios ending in a decision are actually more frequent, which is the red part. If your mood is positive, you’ll find that it's much more likely for the meeting to reach a conclusion.