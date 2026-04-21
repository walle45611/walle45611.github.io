---
title: "AI agent design patterns"
source: "https://www.youtube.com/watch?v=GDm_uH6VxPY"
author:
  - "[[Google Cloud Tech]]"
published: 2026-02-28
created: 2026-04-22
description: "Agentic Pattern lab→ https://goo.gle/agenticpattern Multi-Agent Pattern blog → https://goo.gle/multiagentpatternDesign agentic pattern → https://goo.gle/agenticpatterndesignLearn how to design and"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=GDm_uH6VxPY)

Agentic Pattern lab→ https://goo.gle/agenticpattern  
Multi-Agent Pattern blog → https://goo.gle/multiagentpattern  
Design agentic pattern → https://goo.gle/agenticpatterndesign  
  
Learn how to design and build AI agentic systems! In Part 1 of this series, we use the Agent Development Kit (ADK) to walk through code and live demos for the three foundational AI agent architectures.  
  
In this video, we cover:  
\- The Single Agent: Great for simple tool-use, but struggles with complex multi-step logic.  
\- The Sequential Agent: An "assembly line" approach for highly reliable, predictable workflows.  
\- The Parallel Agent: Running multiple specialized agents concurrently to drastically reduce latency.  
  
Optimize your AI projects.  
  
Chapters:  
0:00 - Intro  
1:01 - Pattern 1: Single agent  
3:05 - Pattern 2: Sequential agent  
5:21 - Pattern 3: Parallel agent  
7:08 - Recap  
  
More resources:  
ADK Doc → https://goo.gle/40ACYEw  
Foundations of multi-agent systems with ADK → https://goo.gle/4tXUkIU  
Workflow agents and communication in ADK → https://goo.gle/4rCONWJ  
  
Watch more AI agent crash course→ https://goo.gle/AIforBeginners  
🔔 Subscribe to Google Cloud Tech → https://goo.gle/GoogleCloudTech  
  
  
#GoogleCloud #AIAgents #ADK  
  
Speakers: Annie Wang  
Products Mentioned: Agent Development Kit

## Transcript

### Intro

**0:00** · \[MUSIC PLAYING\] ANNIE WANG: Hi, everyone.

**0:05** · Welcome to this agentic pattern series.

**0:08** · So if you're building with AI, you probably wonder how to really design the agentic system.

**0:15** · Sometimes you need a single agent, and other times you need a whole team of them working together.

**0:21** · And today, we are diving into AI agent design patterns, and each pattern we're going to provide you with practical examples of code.

**0:30** · And we're also going to walk through a live demo.

**0:32** · And by the end of this agentic pattern series, you will learn how to build agentic solution from single agent pattern to different multi-agent pattern.

**0:42** · And for today's episode, we will focus on practical examples of single agent, sequential agent, and parallel agent.

**0:50** · For next episode, we're going to cover orchestrator pattern, review and critique pattern with loop agent, and agent as tool.

**0:58** · All right, let's dive in.

### Pattern 1: Single agent

**1:01** · So let's begin with the most fundamental pattern, the single agent.

**1:05** · Imagine you want to plan a trip.

**1:08** · So with a single agent, you want to give an instruction on how to use a tool.

**1:13** · And we can have a set of tool, like how to check the weather, check the traffic, and schedule.

**1:19** · But for our example today, we will simplify things with only Google Search tool.

**1:24** · So you will write a comprehensive prompt telling the agent how to plan the trip using the search tool, so the agent then relies on the model's reasoning capability to figure out the sequence of steps.

**1:37** · As you can see from the screen, the code for this is very straightforward with ADK Agent Development Kit.

**1:44** · And let's test if it's ADK web UI by typing ADK app.

**1:49** · So if I type, plan a trip to San Francisco, we can see in the tracing that agent is using the search tool to gather all the necessary information in just one go.

**2:01** · We can see how it is using the tool from the tracing tab over here.

**2:05** · However, this works for single task, but as you have more tools or the tasks are getting more complex, for example, if I have a request finding a sushi in San Francisco that's open late and finding the fastest way getting there.

**2:19** · So if I build this with single agent, we need to define the logic in system instruction with this massive prompt and the behavior can become very unreliable.

**2:31** · Since AI is non-deterministic, you cannot always guarantee that it will follow your multi-step logic perfectly every time.

**2:39** · So the single agent lack of control is its main weakness.

**2:45** · With single agent pattern, the benefit is it is very simple to implement, and it's great for straightforward, multi-step tasks.

**2:54** · However, it is less reliable for complex workflow and is harder to control, and can fail as tasks becoming more complex.

**3:04** · Now let's get to our second part of this video, which is the sequential agent.

### Pattern 2: Sequential agent

**3:10** · So with the task we covered earlier, how do we add in more control?

**3:14** · This brings us to our first multi-agent pattern, the sequential agent.

**3:18** · And this pattern is for highly structured, repeatable task, because the order of the operation is fixed.

**3:26** · So the output of one subagent becomes the direct input for the next subagent.

**3:31** · It's like an assembly line.

**3:33** · And for our trip, we can break down to two specialized agents.

**3:37** · So we have this first food finding agent and the second, a transportation agent.

**3:43** · This sequential agent ensures that we always run this food finding agent first, and then this transportation agent next.

**3:51** · So this gives us predictable, reliable execution.

**3:55** · And you can take a look at a screen for this code example of how to write it in sequential agent.

**4:02** · All right, let's try it out in ADK web UI.

**4:08** · And you can look at the tracing tab.

**4:10** · You can see that it's executed the food agent first and then the transportation agent next.

**4:15** · Perfect.

**4:16** · So how do they communicate with each other?

**4:19** · So they share information through this shared session state, which act like a shared scratchpad.

**4:25** · You can check the session state value at this tab.

**4:28** · And once the first agent writes its finding, and then the second agent reads it from it by using this curly braces in its system prompt.

**4:38** · So this is a form of short-term memory for your agent system.

**4:42** · And the advantages of sequential agent, is that it has high degree of control and reliability.

**4:50** · It is more predictable than a single agent, but it can be very inflexible.

**4:55** · This rigid predefined structure can't adapt to dynamic situations.

**5:00** · Now let's go to the third part of this video, the parallel agent.

**5:05** · So, what if some tasks don't need to be happen in order?

**5:09** · Let's say, if I want to plan a full trip, need to find a good museum, find a good concert, and a good restaurant.

**5:16** · If I'm doing them in sequential order, it will be slow.

**5:20** · This is where the parallel agent pattern really shines.

### Pattern 3: Parallel agent

**5:23** · It allows multiple specialized agents to run independently at the same time.

**5:29** · We can have three agents museum finder, concert finder, restaurant finder, all searching concurrently.

**5:37** · As you can see, this will be a lot faster compared to doing them in sequential order.

**5:43** · Of course, after they all find something, we need to bring these results together.

**5:48** · So a common approach, is combine this with a sequential agent.

**5:53** · So first, we will run the search in parallel.

**5:57** · And then second, we will run a final aggregator agent to synthesize all the results in a single trip plan.

**6:04** · And here's the agent code on how to put together them.

**6:08** · \[DING\] All right, let's test in ADK web UI.

**6:14** · After we type the question from this tracing tab, we can see three searching agents kick off all at the same time.

**6:22** · So once they're done, the results are returned to the session state, as you can see from this tab.

**6:27** · And then the final summarizing agent reads that state and generate our plan.

**6:32** · And this logic is defined in the system prompt over here.

**6:36** · As you can see, this is a great way to reduce latency for tasks that can be broken down into independent subtasks.

**6:44** · So the benefit of this design is very obvious that a significantly reduced latency by running tasks at the same time.

**6:51** · However, it can have higher initial costs, because it is running multiple agents all at once.

**6:58** · And in a lot of use cases, it requires gather or synthesize step to combine the result, which can add complexity to our design.

**7:06** · All right, let's recap what we covered so far.

### Recap

**7:09** · So we covered the single agent.

**7:11** · It is simple to implement.

**7:13** · It can be very flexible, but it lacks certain control over this whole system.

**7:18** · We also cover the sequential agent.

**7:20** · It adds a certain level of control and make the system more reliable, but it is not very flexible.

**7:26** · And lastly, we covered the parallel agent.

**7:29** · It is fast, efficient, and great for independent tasks, but it can add cost and complexity to your system.

**7:36** · With those pattern, you can already build some powerful workflow.

**7:39** · And in our next video, we're going to level up with more advanced patterns for handling even more complex and dynamic problems.

**7:47** · We will explore cases with different practical examples.

**7:50** · And the pattern we're going to cover next episode, including the loop and critique pattern for self-correction, so coordinated pattern for dynamic routing, and also the powerful concept of using agent as a tool.

**8:02** · All right, I will see you in next video.

**8:04** · Bye.

**8:05** · \[MUSIC PLAYING\]