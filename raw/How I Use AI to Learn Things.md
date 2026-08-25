---
title: "How I Use AI to Learn Things"
source: "https://www.youtube.com/watch?v=kzcI5F4tGiU"
author:
  - "[[Eero Alvar]]"
published: 2026-08-14
created: 2026-08-24
description: "My current approach to learning with AI.0:00 Intro0:25 How we're used to learning0:55 One teaches many1:42 One learns from many3:08 One-to-one4:22 The approach5:13 The process8:01 Demo10:12"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=kzcI5F4tGiU)

My current approach to learning with AI.  
  
0:00 Intro  
0:25 How we're used to learning  
0:55 One teaches many  
1:42 One learns from many  
3:08 One-to-one  
4:22 The approach  
5:13 The process  
8:01 Demo  
10:12 Probe  
12:07 Plan  
13:04 Teach  
17:54 Close

## Transcript

### Intro

**0:00** · How could we use AI to learn better? I mean, there's got to be some way to optimize learning with AI. I think it's the perfect tool for it. Uh it is not entirely clear yet how. So, yeah. Uh this video will be me sharing my own current approach to this. First, uh I'll go through the reasoning behind the approach and the design, and then we'll do a live demo. All right. So, this is the standard way of learning that we're used to.

### How we're used to learning

**0:28** · Each outlet uh so, a teacher, a book, a course, whatever, uh they all are designed to teach many students. And also, each students learns from many outlets. So, there's a many-to-many relation between the learners and the outlets, and both directions of it produce their own kind of inefficiency. First, one outlet teaches many.

### One teaches many

**0:56** · An outlet that's designed for many people cannot be optimal for any one of them. That's because optimal, quote-unquote optimal, teaching uh that would clearly depend entirely on the learner's current understanding. And this includes both the whole teaching arc from their current understanding to their goal understanding, as well as each explanation along the way.

**1:20** · An optimal teaching path, then, uh would be one that minimizes teaching stuff that the learner already holds, and also stuff that they can't yet understand.

**1:33** · So, it would work exactly at the edge of their understanding. So, ideally, a teacher would have exactly one student.

### One learns from many

**1:42** · All right. So, second direction, uh one student learns from many outlets. This means many teaching styles to get used to, many notations, many levels of reliability, many interfaces, and the switching between them uh costs mental effort that's not gone into learning the material. But, I think there's a much deeper cost in this, which is trust.

**2:10** · With an unfamiliar outlet, the brain I I think the brain kind of hedges. So, it won't fully commit to accepting a fact until the outlet has proven itself and is familiar enough. I think a good way to illustrate this with is with the following example. Let's say we want to understand the hairy ball theorem, and we have two identical explanations. But, the other one is in a 3Blue1Brown video, and the other is on X posted by some random guy.

**2:42** · Now, even though the explanations are identical, I think it is clear that we're going to learn a better from the familiar source. The brain is going to have a much easier time internalizing the information when it trusts the outlet, even though the explanation is identical.

**3:03** · All right? So, ideally, a student would have exactly one teacher. Now, we've established that the ideal scenario is one-to-one in both directions, but there's an obvious objection to this, which is that having only one teacher means getting only one perspective. But, I think that the objection conflates a source with an interface. So, a teacher doesn't reduce the number of sources, perspectives.

### One-to-one

**3:36** · Uh instead, it aggregates all of them and delivers them through one interface.

**3:42** · So, we don't lose many perspectives. Now, with AI being the teacher, trust isn't really built over time. Rather, it's engineered into the system. The reason we have to have reliable verification and fact-checking is one, correct information. Obviously, we absolutely don't want the AI to hallucinate false information. But also, two, cuz it makes learning easier when we know that the system is reliable.

**4:14** · So, one interface fitted to one mind over all sources.

### The approach

**4:22** · Now, the two inefficiencies that we went over, they give us the two principles that the system runs on. First, optimized teaching, the answer to the first inefficiency. And two, optimized allocation of mental resources, answer to the second one. And this does not mean removing difficulty. Instead, it's about concentrating all cognitive work into the material itself. We want to maximize struggle.

**4:52** · We want to learn difficult things, so struggling is very important, but it has to be in the material itself and not in logistics, planning, finding resources, verifying facts, figuring out what to learn and in what order. All of that is for the system to absorb. Next, the actual process, how this is implemented.

### The process

**5:16** · First, optimized teaching, to teach optimally, whatever this might be very different from person to person, but in order to do that, the system obviously needs to know the person's exact current understanding. So, it has to measure, and it does this with a quiz tool, which lets it ask graded multiple-choice questions.

**5:42** · It starts off with very broad of and then basically binary search is the edge on every possible strand that the lesson will depend on. So, it's going to get a very detailed map of the learner's understanding. So, that's phase one, probe. Second, phase two is plan.

**6:07** · Basically just reasons everything out.

**6:09** · How do I teach this mind this specific thing? And here's also where it fires off its first verification and fact-checking sub-agents. And then presents the plan as a mermaid graph and diagram. And okay, basically two reasons for why why it has to show a graph. One, it gives the learner a better sense of what's to come.

**6:36** · And also two, the actual reason I implemented it is to actually force the AI to reason everything out.

**6:46** · That's the real reason.

**6:49** · So, it cannot just cheat and wing it. And then finally, the teaching itself. And this phase is is going to be very different cuz it's going to depend on it entirely how you want to learn. You want to install the system with your own learning philosophy and how you learn best. But one detail that I think is going to be important regardless of the exact way of learning is feedback.

**7:12** · So, the AI is going to quiz you periodically to check whether you actually understood the thing. And this is important for three reasons. One, it's very easy to sort of gaslight yourself into thinking that you understood something, especially when learning with AI. So, actually testing your understanding is important feedback for yourself. Two, the system needs continuous feedback to stay calibrated.

**7:44** · And three, applying the material, doing practice problems, using the stuff that you learned, also just helps you learn better. It's It's a part of how the new information and the new understanding locks in. So, yeah.

### Demo

**8:02** · Uh that's the That's the general idea.

**8:04** · Now, let's Let's see it in action. All right, we're in the Pi Agent harness now in my learning directory. And yeah, this is where I've got everything set up. So, in the dot Pi folder, I've got the teach skill, some visualization stuff, uh the quiz extension, the MD log extension, which I'll show you in a bit, and two subagents to make visuals.

**8:30** · Hopefully, we'll see those also. So, yeah. I think the only way to demo a system like this is actually to try to learn something. And to really go full circle, uh in my previous learning-related video, I mentioned the Maxwell's equations and how they can be expressed in just two equations using differential forms. And as you can see, uh I don't really know anything about differential forms. So, I think this is what we're going to learn today.

**9:01** · Now, obviously, I don't We're not going to get to this level, but uh I think this is a very good example to showcase the system with a an actual learning process. So, yeah. Uh let's begin. Let me pull up Obsidian here cuz this is what I use.

**9:19** · It's It's sort of like the UI for everything. Learning differential forms.

**9:25** · This is what the MD log extension is for.

**9:28** · It lets me link a Markdown file to the session, and then everything is going to get printed right here. Just a nicer way to see things and also get the LaTeX rendering. So, yeah, that's my solution.

**9:42** · And yeah, it's also nice to have like persistence artifacts from each learning session. So, yeah, let's begin teach. I want to get like a solid introduction to differential forms.

**9:55** · And we're using Kimik 3 on Macs cuz I really found that the intelligence of the model really matters in teaching. The teaching instructions are quite They're quite specific.

**10:10** · Anyway, \[snorts\] let's see how this goes. We're going to get questions now.

### Probe

**10:15** · A force field F acts on a particle as it moves along a curve C. What does the line integral compute?

**10:24** · Let me also I've got this note here where I can just yap into. Basically, just to give it more context and sort of talk through my reasoning.

**10:34** · It's not going to be number one.

**10:36** · I mean, number one does hold for conservative fields, but for any force field F, it's going to be number three.

**10:44** · The network done by the field on the particle.

**10:50** · All right, next one. The divergence of a vector field at a point measures the net outward flux per unit volume at that point.

**11:02** · All right, now the Stokes theorem. Yes.

**11:07** · And now we get Faraday's law of induction. I mean, you can always just give it more context at the beginning about what stuff you already understand very thoroughly.

**11:18** · In this case, I gave you very little context, so we're going to get a long probing phase.

**11:26** · Oh we're going to have to go into relativity.

**11:29** · In special relativity, when you change to a moving reference frame, what happens to the electric and magnetic fields? Okay, let's let's read through these. Both are invariants. All observers measure the same E and B at each event. They mix. A purely electric field in one frame has both electric and magnetic parts in another. Hold on. Only E changes. B stays fixed.

**11:54** · No, obviously not. Three, each transforms independently.

**12:00** · And I'll I'll go ahead and answer I don't know. And it was two.

**12:04** · They mix.

**12:06** · All right, it's stopped with questions.

### Plan

**12:09** · Um so yeah, uh finally done with the probing phase.

**12:14** · Uh it did ask a lot of questions, but now it's got a very detailed image of my understanding. And it's also just a nice warm-up as well. And I don't really got to worry about anything. Just answer the questions and it handles all the logistics. Now we've got a researcher still going on fact-checking stuff.

**12:34** · Although usually math isn't really something it would need to fact-check.

**12:39** · But well, it's it's good practice anyway. Then after this, it's going to fully plan everything out. All right, finally, we're getting the plan.

**12:50** · I did start rendering the mermaid in the 2A, but don't know what happened. It was probably in the most recent release, I believe, added mermaids. But anyway, we get it in the obsidian. This is the plan. Let's go. A lot of things I'm going to be reading reading the plan. All right, so that's cool. We're going to get to generalized Stokes well before the actual goal.

### Teach

**13:16** · Okay, perfect. It's it's using the visualization skill. So we're going to get some visuals soon. All right, it's making an SVG. And the reason that these are done in sub agents is to obviously preserve context but also the sub agents will look at the image to verify whether it actually looks correct. It wrote an SVG and now it's going to view it. It's going to edit it a bit, look at it again.

**13:47** · Yes. So, we're going to get that in the next message. But now the first node. So, it's going to It's going to slowly walking down the path one reasoning step at a time. Because what usually happens if you're like talking to chat GPT or something trying to get it to explain or teach you anything it usually just it's way too excited and it rushes through the whole thing. And so, I like to keep this very slow, one reasoning step at a time.

**14:18** · So, now we're introducing co-vectors. Let me read through this.

**14:24** · All right. So, now it's introduced co-vectors and also given a new perspective on the X.

**14:32** · This is really cool. Yeah, it's going to quiz me on the this reasoning step to confirm that everything is actually understood, to confirm my understanding.

**14:41** · Now, yeah, it's going to incorporate the visual which will be all rendered into Obsidian and automatically here as an embed embedded file. Let's alpha be 3D X minus 2D Y and V this. What is alpha of V?

**15:01** · Which is going to be -4.

**15:04** · Okay, this makes perfect sense.

**15:06** · The picture helped. Oh, yeah, the Pi also got like built-in LaTeX rendering. Okay, which is kind of cool.

**15:13** · Yeah, it does the job but it's definitely not the same as this.

**15:20** · But yeah, cool features though. All right, so now we're extending the covectors to a covector field. All right, so now we get a new perspective on what the line integral actually is.

**15:33** · Yeah.

**15:34** · One thing I don't like about this is it's I haven't really configured its style of speaking. So we get a lot of it sounds very AI, which I don't really mind cuz I don't want to cram it with too many instructions to worry about. So I'm fine with these LLM-isms.

**15:56** · It's not X, it's Y.

**15:59** · Wow, insane. The quality of the teaching is what matters. But yeah, it's basically walking down the dependency tree. Seems like we're still here.

**16:10** · So when we're going to get I think this is the wedge product. But yeah, I like that it moves one reasoning step at a time. So if at any point I have questions, I can always ask.

**16:21** · And it doesn't rush forwards. The each step is very easy to digest. And it's going to give me everything that I can accept at face value like this. All integration over a one-dimensional thing is the integration of a one-form. Now, I assume that this continues, yeah.

**16:41** · A K-form will be the kind of thing a K-dimensional surface can eat.

**16:47** · Sure.

**16:49** · All right, we're getting a new new SVG.

**16:54** · Seems like our uh SVG maker died. Agent error. Overloaded. Bro, not now. Trying to learn differential forms.

**17:05** · Oh no, we're getting it. Now we're getting the wedge products. That's cool.

**17:10** · Machines that are bilinear and antisymmetric. Yeah, where do we get them? I'm sure this is not antisymmetric. The minimal fix is the oldest trick in the book. Aha. Yeah, that gives something anti-symmetric.

**17:24** · So, is that just what it is? Sure, okay, fine. But yeah, anyway, this is we're obviously not going to go through the whole dependency graph here.

**17:35** · Um this is going to be this one, I believe. Yeah.

**17:40** · But yeah, anyway, where did we get to in the DAG? Here.

**17:44** · Yeah, I think we got here. Yeah, we're approaching generalized Stokes now.

**17:49** · Yeah, that's cool. Um but yeah, I think this is enough for a demo. So, this has been sort of a second part or continuation of where we left off last time in my previous learning related video. So, what this video has been really about is how to take a learning philosophy or way of teaching and implement that as an AI system.

### Close

**18:10** · I think the main reason for why this has worked so well for me is none of the stuff I talked here about, rather just the style of teaching I covered in the previous video, which affects exactly two things: the learning arc, the path that we're going to take from our current understanding to our goal understanding, and two, the individual steps and explanations along the way.

**18:39** · Yes. Anyway, just some ideas for you for you to think about. I'd like to hear your your thoughts on this. And how how how could the system be improved? How do you use AI in learning? I'd like to know. I'm very invested into refining this further. So, yeah, uh that's it.