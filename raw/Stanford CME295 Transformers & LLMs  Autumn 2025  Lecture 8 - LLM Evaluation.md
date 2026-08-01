---
title: "Stanford CME295 Transformers & LLMs | Autumn 2025 | Lecture 8 - LLM Evaluation"
source: "https://www.youtube.com/watch?v=8fNP4N46RRo"
author:
  - "[[Stanford Online]]"
published: 2025-12-03
created: 2026-07-11
description: "For more information about Stanford’s graduate programs, visit: https://online.stanford.edu/graduate-educationNovember 21, 2025This lecture covers: • LLM-as-a-judge overview• Best practices and b"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=8fNP4N46RRo)

For more information about Stanford’s graduate programs, visit: https://online.stanford.edu/graduate-education  
  
November 21, 2025  
This lecture covers:  
• LLM-as-a-judge overview  
• Best practices and benefits  
• Biases and pitfalls  
  
To follow along with the course schedule and syllabus, visit: https://cme295.stanford.edu/syllabus/  
  
Chapters:  
00:00:00 Introduction  
00:07:08 Inter-rater agreement metrics  
00:18:24 Rule-based metrics  
00:21:00 METEOR, BLEU ROUGE  
00:28:00 LLM-as-a-judge  
00:33:44 Structured outputs  
00:36:48 Variants  
00:38:47 Position, verbosity, self-enhancement bias  
00:47:22 Best practices  
00:54:06 Factuality  
01:00:15 Agent evaluation  
01:23:50 Benchmarks  
01:25:12 Knowledge with MMLU  
01:29:34 Reasoning AIME, PIQA  
01:33:57 Coding with SWE-bench  
01:36:15 Safety with HarmBench  
01:40:51 Agents with Tau-Bench  
  
Afshine Amidi is an Adjunct Lecturer at Stanford University.  
  
Shervine Amidi is an Adjunct Lecturer at Stanford University.  
  
View the course playlist: https://www.youtube.com/playlist?list=PLoROMvodv4rOCXd21gf0CF4xr35yINeOy

## Transcript

### Introduction

**0:05** · Hello, everyone, and welcome to lecture 8 of CME 295.

**0:11** · So today's topic will be LLM evaluation.

**0:16** · And I think this class is probably one of the most important classes of this quarter because the idea is if we don't how to measure the performance of our LLM, we don't really know what to improve.

**0:30** · And so this class will focus on how we can quantify how the LLM performs in a bunch of different cases.

**0:39** · So with that said, we are going to start the class as usual by recapping what we saw last week.

**0:46** · So if you remember, last week, we saw how our LLM could interact with systems that are outside of the LLM itself.

**0:58** · So we saw one core technique that is called RAG that allows our LLM to fetch information from external knowledge bases.

**1:10** · And so here RAG stands for Retrieval-Augmented Generation.

**1:17** · And we saw how we could improve the retrieval system.

**1:21** · So we saw that it was composed of two main steps.

**1:25** · So one was candidate retrieval, which is typically something that is done with a bi-encoder setup.

**1:34** · So Sentence-BERT was a good example of how people would design such a model.

**1:42** · And so this first step is typically there to filter down the potential relevant candidates for a given incoming query.

**1:52** · And then we saw that there was a second step, which was reranking, and that one was a bit more involved and involved cross-encoders, which were more sophisticated.

**2:06** · And we also saw some ways to quantify how well our retrieval system performed.

**2:12** · And then we also saw something that was called tool calling, which is the ability for a model to which tool to call with which argument.

**2:25** · So if you remember, if we give our LLM the knowledge of the tools that are available to it, it can figure out which arguments it needs to input to the function as a function of the input query, and then run that function and then output the result in natural language to the user.

**2:51** · And then we also saw how agentic workflows were composed of.

**2:57** · So spoiler alert, it's something that is a combination of the two previous methods, so RAG and tool calling.

**3:06** · And in particular, given an input, we're allowing our model to make multiple calls to call different tools to fetch relevant data from other knowledge bases.

**3:21** · And we saw one example that was successful from the current applications, which was AI-assisted coding, which relies on this principle.

**3:33** · And React is typically the framework that people would use.

**3:37** · So reason plus act, which is decomposing this into observe, plan, and act steps.

**3:47** · Cool.

**3:48** · So this is what we saw last time, and we also so started from this slide last time.

**3:55** · If you remember, our LLM has strengths but also weaknesses that we're trying to mitigate.

**4:04** · So in particular, the focus of lectures 6 and 7 were on methods to improve reasoning of the model and ways for the model to fetch knowledge from other systems, as well as performing actions.

**4:23** · And today, we're going to focus on the evaluation part, in particular, given a response that the model is giving, how can we quantify how well the LLM is giving its response?

**4:40** · Cool.

**4:42** · So first of all, I would like to define the term "evaluation" and the meaning that we will use for this lecture.

**4:51** · So when we say, I want to evaluate my LLM, it can actually take a lot of different meanings.

**4:58** · So when you say, let's evaluate the LLM, it can mean let's evaluate the performance, the output.

**5:05** · Let's evaluate this based on coherence, factuality.

**5:09** · Let's evaluate it based on latency, so more system-related metrics or pricing or how often it is up and so on.

**5:20** · So just to make sure we're on the same page, this lecture will mostly focus on the output quality part.

**5:28** · And in particular, we'll focus on quantifying how good the actual response is.

**5:37** · And here you will note that this is a challenging problem because as we saw previously, our LLM is a text-to-text model that can output basically anything.

**5:53** · So it can be natural language, it can be code, it can be math reasoning, and so on and so forth.

**6:01** · So it's very hard to come up with universal metrics to evaluate that.

**6:06** · So we will see how people do this in practice.

**6:12** · Cool.

**6:12** · So given the fact that our LLM generates free-form output, one could imagine that the ideal scenario for us to evaluate the LLM output would be to every time ask a human to rate the response.

**6:34** · So here the ideal scenario would be, OK, I give a prompt to my LLM.

**6:39** · It gives a response.

**6:40** · I ask a human to rate it, and I start again and again.

**6:46** · And what I do is at the end of the day, I just collect all these human responses, and I try to quantify the overall performance of my model.

**6:56** · Well, as you can imagine, the main problem is that such a system would be very cost-intensive.

**7:06** · But let's look at this into more detail.

### Inter-rater agreement metrics

**7:09** · So if you remember, the LLM outputs are really free-form.

**7:17** · And there may be cases that even human judgments may be something that is fuzzy because maybe the rating task in itself is subjective.

**7:29** · So let's take the following example.

**7:32** · Let's suppose I ask my LLM what birthday gift should I get.

**7:38** · And let's suppose the LLM responds with a teddy bear is almost always a sweet gift.

**7:43** · Just pick one that feels right for you.

**7:46** · So let's suppose I want to evaluate this response with respect to the usefulness dimension.

**7:52** · I may have one human reader that says, yeah, it's pretty useful because teddy bear is pretty indicative of, I guess, what the user should get as a gift.

**8:05** · But then another reader may say, no, actually, it's not useful because maybe the response didn't specify exactly which teddy bear.

**8:14** · Should I have a bear?

**8:17** · Should I have an elephant, a giraffe?

**8:19** · Which stuffed animal should I get?

**8:22** · So there is this notion of inter-rater agreement, where we're basically concerned with making sure that everyone is aligned on how to rate those responses because sometimes like in this illustrative example, it's maybe a little bit subjective.

**8:50** · So responses may vary.

**8:52** · So what people want to do is to make sure that the guidelines are clear enough for everyone to rate these responses in a consistent manner.

**9:04** · So people come up with agreement types of metrics.

**9:12** · So a very natural metric that you may think of is the quote, unquote, "agreement rate."

**9:21** · So for instance, you have these two raters.

**9:24** · So what you do is you just measure the proportion of the time that the two raters give the same response.

**9:34** · And let's suppose the response here is binary, so let's say, yes, good or not good.

**9:41** · Well, do you see a problem with such a metric?

**9:47** · Is this a good metric?

**9:54** · I guess another way to ask this question is, if I give you a given number of agreement rates, can you tell me if it's a good number or if it's a bad number?

**10:09** · Well, let's take the example of, let's say, two raters, let's say, Alice and let's say, Bob.

**10:24** · And let's suppose we have two different types of ratings that these raters can give.

**10:30** · So either, let's say, yes, it's good, so 1, the output is good, or the output is not good.

**10:40** · So if we assume that the first rater gives, let's say, random responses with some probability P of A for being good and 1 minus P of A for being not good.

**10:58** · And then, let's say, Bob, who should have eyes and a smile, has a P of B for being good and 1 minus P of B for being not good.

**11:13** · Then let's compute the agreement rate for this case.

**11:18** · So the agreement rate is basically the probability that rater A and rater B agree.

**11:35** · And so here A and B agree if A and B both vote 1 or when A and B both vote 0.

**11:50** · But if they give their response in an independent and random way, well, if you use this probability concepts that you know, then we will have probability of A and B responding to 1, which is probability of A responding to 1 times probability of B responding to 1, and same for 0.

**12:16** · So we will have something like this.

**12:18** · So P of A, P of B plus 1 minus P of A, 1 minus P of B. So this one is A and B say 1 And here A and B say 0.

**12:49** · So let's see what the agreement rate would be in that case.

**12:54** · So if we assume that suppose that-- let's suppose, P of A is equal to P of B, which is equal to, let's say, 0.5, then the agreement rate would be-- so agreement rate would be-- so I'm just replacing the numbers here, so 0.5 squared plus 0.5 squared.

**13:26** · So it's 0.25 plus 0.25, which is equal to 0.5.

**13:30** · So what that means is if we're just letting our raters rate these things in a random way with some probability P of A, P of B, we would already have an agreement rate of 50%, just by pure random chance.

**13:48** · And so one thing that I want to say is, that this agreement rates, by pure chance, is a function of the probability that each of these raters give these ratings.

**14:03** · And so if this probability is actually higher, the agreement rates by pure chance is also higher.

**14:12** · So what that means?

**14:12** · So what do I want to say?

**14:14** · I want to say that if we just take the agreement rates, then it's very hard to put it into context in terms of what you would have gotten if things would have happened just by pure chance.

**14:31** · So for this reason, people have come up with a series of metrics that try to make it more relative to this baseline, which is, what would happen if our raters would choose things randomly?

**14:53** · And so you have these metrics-- like for instance, this one is the Cohen's kappa metric, which computes a quantity that is a function of this agreement rate by chance, and take the observed one, such that if our observed agreement rate is greater than the "by chance" agreement rate, then our coefficient is positive.

**15:27** · So when it's positive, at least you know it's going in the right direction.

**15:32** · So here, if the observed agreement rate is equal to 1, then kappa is equal to 1.

**15:44** · But if our observed agreement rate is below the "by pure random chance" agreement rate that we saw on the blackboard, then our coefficient would be negative.

**15:59** · So long story short, there is a bunch of metrics that try to quantify inter-rater agreement rates using these kinds of formulas to be able to make these quantities relative to what would happen if things were done in a random way.

**16:20** · And so that's why you may see a bunch of metrics out there.

**16:24** · So here is Cohen's kappa that people use for cases where there are two raters, but then you have extensions, such as Fleiss's kappa and Krippendorff's alpha that you may see out there.

**16:37** · So they all rely on this idea that we should have some baseline, which is our raters just randomly picking answers, and try to see how much better our actual agreement is compared to this.

**16:56** · So does that make sense?

**17:01** · So I guess what I want to say is that the first limitation of asking humans to rate our LLM outputs, which was sometimes the task being subjective, can be something that we can quantify with this inter-rater agreement metrics.

**17:22** · So what people would typically do is they keep track of how good that agreement is.

**17:30** · And if, let's say, we have a quantity that's not satisfactory, people would just hold some quote, unquote, "agreement sessions" between the raters to just align on how they should rate the answers so that it can be seen as just a health metric to track how consistent your ratings are.

**17:54** · And this is typically something that people use in practice.

**18:00** · So up until now, we've seen one limitation of human ratings.

**18:08** · Well, second limitation, I think I also said it previously.

**18:11** · It's really slow.

**18:13** · If you ask someone to rate a thousand LLM outputs, well, it will take them a while.

**18:18** · And it's, of course, expensive.

**18:22** · So all of that to say that our ideal scenario of asking a human to rate every LLM output is not something that is practical.

### Rule-based metrics

**18:35** · But we can leverage human ratings in some way because we've seen that even if the task is subjective, we can have a way to align our raters.

**18:51** · So now let's move on to another way to go about doing this, which is by using some rule-based metrics.

**19:01** · So here I'm just going to revise the setting that I mentioned before.

**19:06** · And instead of asking our humans to write every LLM output, this time, I'm just going to ask them to write the references or the ideal outputs for a given set of prompts, just fix that for good, and then use some kind of metric that would compare the LLM outputs with those references.

**19:39** · So here the main difference is, let's suppose I have a given set of prompts fixed.

**19:46** · Well, I can make iterations in my model and always compare the outputs of my LLM with this fixed reference, instead of always asking humans to rate that again and again.

**20:01** · So it's already an improvement.

**20:05** · And we will see a little bit what are the kinds of rule-based metrics that you will see out there.

**20:14** · So ideally, these metrics should reflect the performance of the LLM output in an optimal way.

**20:24** · And what I mean by an optimal way, it is to make it be a little bit flexible, given the fact that natural language is not always something that you can say in one given way.

**20:39** · So for instance, when I provide a response to a given prompt, there can be very well a case where I can formulate the response slightly differently, but it will still be just as good.

**20:52** · So the idea behind this matrix is to make this comparison a little bit flexible.

### METEOR, BLEU ROUGE

**21:01** · So let's start with one common one that people use in the translation case.

**21:06** · So this metric is called METEOR, and it stands for metric for evaluation of translation with explicit ordering.

**21:15** · So the idea here is to compare reference and predicted, and we'll see how it's being done, and also penalize cases when words are not in the same order, which is explaining why the metric is called with explicit ordering.

**21:41** · So the formula is as follows.

**21:42** · So it is some F score times 1 minus some penalty.

**21:50** · So the F score here is-- you may be familiar with F1 score.

**21:55** · So it's like the harmonic mean with equal weights.

**21:59** · So this one is with the variable weights.

**22:02** · So it is a function of precision and recall, where precision is the proportion of the unigrams that are in your predicted sequence that are matching with the reference.

**22:20** · And the recall is the proportion of the unigrams in the reference that are matching with what is in the predicted.

**22:29** · So it's basically matching the usual precision recall metrics that you know.

**22:36** · And then we have another quantity here, which is the penalty.

**22:41** · And I mentioned the penalty here tries to incentivize good ordering.

**22:47** · So if it's ordered the same in the reference and in the prediction, then it's good.

**22:54** · Otherwise, it's bad.

**22:56** · And so here there's a bunch of quantities.

**22:59** · So gamma and beta are hyperparameters that people arbitrarily choose.

**23:06** · And it's a function of C, the number of contiguous chunks that are matched over the number of matched unigrams.

**23:23** · So ideally, you would want C that would be as low as possible because if you have a low number of contiguous matches, it means that your contiguous sequences are long, which means that the ordering is the same.

**23:42** · So you want C to be low and then matched unigrams to be high.

**23:50** · So you want that penalty term to be low for a good-- I guess, for a prediction that has the same ordering as the reference.

**24:01** · So I guess higher METEOR score means better translation, according to this way of doing things.

**24:13** · So I guess when you look at this formula-- first of all, it looks very arbitrary.

**24:21** · I have alpha as a hyperparameter, gamma, beta.

**24:25** · So it's kind a recipe, I feel.

**24:29** · So that's one.

**24:31** · And the second thing is that it does not allow for stylistic variations because here we're measuring the number of matched unigrams.

**24:42** · Although the metric expands the range of what it's called matched unigrams by taking into account things like words that are synonyms of one another and things that are of the same roots, but still, it is not extremely satisfactory in that sense.

**25:06** · So METEOR is one such metric.

**25:11** · You have another one that's being used or that has been used in translation tasks, which is called BLEU, which you may know.

**25:20** · So BLEU stands for bilingual evaluation understudy.

**25:24** · And you can think of this as a precision-focused kind of metric that looks at the number of matching n-grams over the n-grams that are in the prediction, which is why it's a precision kind of metric.

**25:45** · And it also has a penalty term.

**25:48** · Here it's called brevity penalty because given that it's more of a precision kind of metric, if you translate something that's very short, you may be able to gain the metric.

**26:02** · So you want to penalize the translation being too short.

**26:07** · So we're not go going to a lot of details, but I just want to just show you the kinds of metrics that are out there.

**26:13** · So METEOR is one, BLEU is another one.

**26:17** · And ROUGE, which you may have heard, is also another one, typically used for summarization tasks.

**26:23** · Again, same idea, and it has a bunch of variants that you may see out there.

**26:31** · But long story short, all these metrics, they all compare the output with a reference.

**26:42** · So as we saw, one key limitation is that they do not allow stylistic variation.

**26:51** · So let's take an example.

**26:53** · So let's suppose I say, a plush teddy bear can comfort a child during bedtime.

**26:58** · Well, the exact same thing, you can say it-- I can say it in a really different way.

**27:03** · So soft stuffed bears often help kids feel safe as they fall asleep, or many youngsters rest more easily at night when they cuddle a gentle toy companion.

**27:13** · So in all these cases, the metrics that we saw would really perform very poorly.

**27:23** · So that's one key limitation.

**27:24** · So the second key limitation is correlation is not that great.

**27:30** · I mean, you can imagine that people have come up with all these hyperparameters to make it be correlated to human ratings, but they're not that correlated.

**27:44** · And the bottom line is, it still requires human ratings to just get started.

**27:50** · And sometimes you just can't afford to have human ratings, maybe in your project.

**27:58** · So I guess there are still some key limitations, which is the reason why-- all of that to say, I want to motivate the key methods of this class or of this lecture, which is called LLM-as-a-Judge.

### LLM-as-a-judge

**28:16** · We spent the first seven lectures motivating these large language models that are pretrained on huge amounts of data that are tuned in a way to match human preference.

**28:29** · So they do contain human knowledge.

**28:31** · They do contain some indication of what humans may prefer.

**28:38** · So the idea here is to have our model response be actually an input of yet another LLM.

**28:49** · And that LLM is something that people typically call LLM-as-a-Judge.

**28:55** · So it was a term that was introduced in a paper from two years ago.

**29:02** · So here the idea is to use an LLM for rating purposes.

**29:09** · And things that you would see as input would be the prompt that was used to produce the response, the response, and the criteria along which you want to grade your response.

**29:26** · And so here LLM-as-a-Judge would give you the following outputs.

**29:32** · So the first thing is it would give you a score.

**29:36** · So here you can think of it as a binary scale score, so pass or fail, and this is very new, also a rationale because LLMs, they understand text.

**29:53** · So they can also explain you why they graded something with a given score.

**29:58** · And that part is the key difference with previous methods.

**30:05** · We are able to explain why the metric or the model is giving us a given score.

**30:12** · And this is quite good because in the other, let's say, rule-based world where you would have all these formulas and multiplication and all these things, and sometimes you would come up with a number that would not be very self-explanatory.

**30:30** · And this is luckily something that LLM-as-a-Judge addresses.

**30:37** · So to recap, what we want is to use an LLM as a way to grade the response.

**30:46** · So here you would have typically the following kind of prompts.

**30:51** · So you would state, OK, I want to evaluate my response with respect to a given criteria.

**30:58** · And then you give the prompt that you used to generate that response along with the model response.

**31:06** · And then you would ask the judge to return two things, the rationale and then the score.

**31:18** · So one little trick I want to point out is people typically ask the model to first output the rationale and then the score.

**31:30** · And the reason why we typically do that is it's something that empirically improves the quality of the results.

**31:40** · But then given what we saw, I think in, lecture 6, if you remember the reasoning class, we saw that these reasoning models that are being trendy, especially in 2025, what they do is they first output a chain of thought before giving the answer.

**31:59** · So you can actually think of this trick as being on the same idea of reasoning models, as in, it allows the model to externalize, verbalize its quote, unquote, "thought process" before giving the score.

**32:17** · So it gives it a chance to really figure out what is good or what is wrong in the model response.

**32:28** · So far, so good?

**32:31** · Any questions on, I guess, the setup?

**32:36** · All good.

**32:37** · So now I have a question for you.

**32:40** · If I give the following prompts to my LLM-as-a-Judge, am I guaranteed to have a rationale and a score that I can parse?

**32:53** · Am I guaranteed?

**33:01** · No?

**33:02** · Yeah, exactly, no.

**33:04** · The answer is no.

**33:05** · You're not guaranteed to have a rationale and a score that you can parse because this model has some probabilistic nature to it with the sampling process.

**33:16** · And it's not something that you can really control.

**33:19** · So I guess my follow-up question is, do you know a technique that would, I guess, guarantee you to have a structured response?

**33:32** · So hint is a technique that we saw towards the beginning of the class.

**33:42** · I'll give you a little hint.

### Structured outputs

**33:44** · So if you remember on slide 65 of lecture 3, we saw a technique called constraints-guided decoding.

**33:56** · So if you remember, the idea here is to constrain the decoding thought process by allowing our model to only sample from quote, unquote, "valid" tokens.

**34:11** · And we typically do that in cases where we want our output to have a given format, so let's suppose, a JSON format.

**34:19** · And we want absolutely that format.

**34:23** · So what people do is they use this technique to guarantee the form of the response.

**34:30** · And in case you're using this provider, like the providers that are out there, like for instance, OpenAI or Gemini or Anthropic, this technique is known under the name structured output.

**34:46** · So in your projects, if you want to constrain the decoding process in order to output the response of a given format, so let's suppose my format is a response, and I cannot represent it by a class, and there are two attributes, so rationale and score.

**35:07** · Well, typically, you can reference that with the argument text format equal to that representation.

**35:18** · So this, I believe, is something that OpenAI does.

**35:22** · I'm not exactly sure if it's exactly the argument name that you would see for the other providers, but they're all, I guess, along the same lines.

**35:33** · Does it sound good?

**35:35** · So the key word here is structured output.

**35:39** · Whenever you want a response of a given format, you would just go for that.

**35:47** · Cool.

**35:47** · So just to recap, our LLM-as-a-Judge has two main benefits.

**35:54** · So the first one is that we do not need a reference text.

**35:59** · We do not need human ratings to just get started because our LLM already has a lot of, I guess, knowledge that it has acquired during pretraining and human preferences and so on.

**36:11** · So you do not need that.

**36:13** · And then the second thing is you can interpret the score with the rationale that it is being output.

**36:23** · And that is also quite remarkable.

**36:26** · So just as an example, here you would say, evaluate the quality of this response.

**36:31** · So you would have some rationale that would explain what this response has or doesn't have that makes it good or bad along with the score.

**36:46** · Cool.

**36:47** · And I believe-- now we're going to see the kinds of LLM-as-a-Judge that you can see out there.

### Variants

**36:54** · Of course, there are many variations, but there are generally two types of LLM-as-a-Judge that you will see.

**37:01** · So the first one is you have a single output, a single response that you want to evaluate.

**37:10** · And here you would ask LLM-as-a-Judge to say, OK, is it good, or is it not good?

**37:17** · And the second big kind of LLM-as-a-Judge that you will see out there is pairwise kind of setup.

**37:25** · So you have two responses, and you say, is response A better, or is response B better?

**37:34** · And here you would obtain a response either that one or this one.

**37:41** · So if you remember, we have seen in previous lectures that there are a lot of situations where we would want to have preference data, for instance, in the preference tuning class that we had, I believe it was lecture 5.

**37:56** · So these kind of methods can also be a good way to synthetically generate preference ratings where you have two responses.

**38:07** · And then you ask your LLM to say, OK, I prefer that one.

**38:11** · And you can use that one as the label to train your reward model.

**38:19** · Does it sound good?

**38:22** · Any questions on the setup or everything that we've talked about so far?

**38:31** · Cool.

**38:31** · Everyone is on the same page.

**38:35** · So now let's see what can go wrong with our LLM-as-a-Judge.

**38:40** · So let's think of the possible kinds of failures that we can encounter.

### Position, verbosity, self-enhancement bias

**38:48** · So the first one is called position bias.

**38:52** · And as the name suggests, it has to do with the ordering at which we present the responses to our model.

**39:00** · So let's say, if we ask our model, is response A better or response B?

**39:06** · Well, there is a chance that the model responds with response A just because it was the first one to be mentioned.

**39:16** · So that bias is called position bias.

**39:19** · So it's where the position at which you place the response matters in the judgment of the LLM-as-a-Judge model.

**39:31** · And I guess, as a way to remedy that, people have different techniques.

**39:37** · But one typical technique would be to ask the model, is A or B better?

**39:44** · And then ask the model, is B or A better?

**39:48** · And then take the majority voting.

**39:50** · So if both of them lead to the same response, then it's good.

**39:56** · But if the response changes, then it may not be good.

**39:59** · So you may want to do something else.

**40:05** · There are a bunch of other techniques.

**40:07** · So I know there's a bunch of papers that try to tweak the position embeddings, but those ones are a bit more advanced.

**40:14** · So it's not typically the thing that you would do just out of the box.

**40:18** · So taking the average or taking the majority voting of this position swapping is typically what you would do.

**40:29** · Cool.

**40:29** · So this was the first kind of bias.

**40:31** · The second bias is called verbosity bias.

**40:35** · So let's suppose you have two responses.

**40:39** · And the first response is short and concise.

**40:44** · The second response is something that goes much more into details, is typically something that is more verbose.

**40:51** · Well, there are cases where the model will tend to, I guess, prefer responses that are just more verbose just because they're more verbose, not necessarily because they're more correct.

**41:07** · And for that, it's maybe a little bit trickier.

**41:10** · So people typically try to explicit this dimension in the guidelines.

**41:19** · When they input, I guess, this question to the LLM-as-a-Judge, they say, well, make sure to not pay too much attention to the length of these responses to not, I guess, prefer something just because it's more verbose.

**41:35** · So that's one kind of method that you will see out there.

**41:40** · The second one is to just also add some examples, in-context learning examples, to the model to just, I guess, tell it to, I guess, show by example that verbosity is not something you should prefer.

**41:58** · And then the last one is to have some kind of penalty on the output length.

**42:07** · So you can ask your model in a pointwise way, how good is 1, how good is 2.

**42:13** · And then try to penalize that with the length.

**42:17** · So that's something that also people may use.

**42:21** · So we've seen position bias.

**42:24** · We've seen verbosity bias.

**42:26** · Now we will see the third kind of bias that you may see out there, which is called self-enhancement bias.

**42:34** · And so that one has to do with the fact that if you ask a model to judge an output that was produced by itself, well, the model will tend to prefer responses that are generated by itself, regardless of whether or not the other one was more aligned with what we wanted.

**43:03** · And I guess, here the intuition is that if our model generated such an answer, then it may be the case that our model thought that from a probabilistic standpoint, this was a sequence that was very much likely to appear.

**43:22** · So it may be, I guess, one way to think about it, which is, if it has generated such a sequence, then it means that it is something that it thinks-- I mean, "think," quote, unquote, that it's a good answer.

**43:41** · So the general guideline here is to typically not use the same model that you use for generation and for judges.

**43:52** · But I guess nowadays, it's hard to have that strict constraints, I guess, respected because, I guess, all models, they are trained on basically the same data sets.

**44:10** · So you can argue they're all being subject to the same, I guess, training mixes and so on.

**44:18** · But still, I guess, what people do is they tend to use another model just to have such a risk be minimized.

**44:28** · So long story short, try to not use the exact same model that you use for generation and for evaluation.

**44:39** · So this is self-enhancement bias.

**44:43** · Before we go to the next subpart, I guess, what do you think of these three biases?

**44:49** · Do they make sense?

**44:50** · Any questions so far?

**44:53** · Yep.

**45:03** · So can you elaborate a bit more?

**45:14** · So the question is, can you have a model that just maybe isn't aligned with, I guess, the ground truth and maybe prioritizes maybe one label over another?

**45:24** · So this can definitely be another kind of bias, so this bias being that our LLM is not exactly aligned with what humans would prefer.

**45:34** · So these three biases are by no means exhaustive.

**45:38** · So this can very well be another bias that you can list as well.

**45:43** · This is definitely another kind of bias.

**45:46** · Yep.

**46:00** · So the question is, is it possible that our judge still prefers an LLM response, even if it's a different one?

**46:06** · Well, it depends how good your judge is.

**46:10** · But typically, the best practice is to have a judge that has a much bigger capacity that may capture this kind of differences and not be fooled by a response that just sounds like something it may generate but something that is maybe more aligned with human preferences.

**46:30** · So I guess the short answer is yes, you can still have such a situation.

**46:34** · But in order to mitigate that risk, you would typically take a model that is not the same but also typically much bigger.

**46:43** · So you have a bunch of such models out there.

**46:47** · And with all the, I guess, improvements that have been made with reasoning models, this is also something that people try.

**47:01** · Question is, should the judge be bigger?

**47:04** · It's not a hard constraint, but it's typically something that people would take, a bigger model that would have a strong reasoning capabilities that could really tease out what's good and what's not good.

**47:19** · Cool.

**47:21** · So with that, I'm going to just go over the best practices that we've seen.

### Best practices

**47:26** · So we saw that in order for our LLM-as-a-Judge to output a score, we need to give the criteria that we want this to be evaluated against.

**47:38** · But sometimes these criteria may be a little bit subjective.

**47:43** · So one thing that really works very well is to have crisp guidelines, so really explicit what we want, what we don't want.

**47:57** · The other point is you may see different kinds of scaling out there, so sometimes people having a scale that is maybe more granular and maybe other cases where we're just operating on a binary scale.

**48:12** · So typically, what people would tend to prefer is actually the binary one because it makes the job of the LLM-as-a-Judge easier.

**48:23** · So it's just either good or bad.

**48:26** · And also, when it comes to aligning the judge with human ratings, humans, they typically also find it easier to just judge out of two options, as opposed to several.

**48:42** · So it just removes the noise of having several possible choices.

**48:47** · And it's not necessarily an extra signal that may be really useful.

**48:53** · So here the tip is to use a binary scale, like a pass or fail kind of score, as opposed to a gradual one.

**49:03** · The third tip is to make sure to output the rationale before outputting the score.

**49:10** · And we've seen this is along the same ideas of outputting a chain of thought before providing the response, which is something that is done by our reasoning models.

**49:21** · So it's typically something that will improve the judge's performance.

**49:27** · So we've talked about the different kinds of biases.

**49:29** · So position, verbosity, self-enhancement.

**49:33** · But it's not the only ones, of course.

**49:35** · And I guess, people typically also look at how to mitigate those with the remedies that we mentioned.

**49:45** · So far, we've stated that we do not need human ratings to get started.

**49:54** · But a good practice is to still look at how the LLM ratings compare with the human ratings.

**50:03** · So here one tip is to just calibrate the responses that the judge is giving with respect to the human ratings because at the end of the day, it is the quantity that we want to approximate.

**50:16** · And so here, I guess, if there is the budget and it's something that is possible for the project, one good practice is to collect the human ratings, output the LLM-as-a-Judge scores, and then run some correlation analysis to see if there is something that can be improved in terms of the prompt, mainly the prompt.

**50:41** · And then the last thing is the temperature.

**50:45** · So if you remember, the temperature is a parameter that you can tweak to make your generation more deterministic as opposed to more creative.

**51:00** · And so you will see that for evaluation tasks, people use a low temperature because they want to make their evaluation experiments reproducible.

**51:11** · Let's imagine you do one evaluation, and then you do another one, let's say, two days later, you don't want the scores to be super different.

**51:21** · So you will see a temperature value of something like 0.1 or 0.2.

**51:26** · These are very common values that people take.

**51:32** · And so long story short, if we were to recap, we went from the ideal scenario being that each LLM output is rated by humans to actually having some kind of approximation with these LLM-as-a-Judge models that can do this evaluation, I guess, without any constraints or without any need for human judgments.

**52:03** · But as we mentioned in the best practices, making sure that the LLM-as-a-Judge scores and the human ratings do not diverge is something that we should keep in mind as we improve our model because it may be that we're improving our model so that our LLM-as-a-Judge score is very high, but the LLM-as-a-Judge score is itself an approximation of human ratings.

**52:33** · So I guess you don't want to overoptimize against, I guess, the proxy.

**52:39** · And so that's why you want to have that proxy be as aligned as possible with your ground truth labels, which are human ratings.

**52:52** · Cool.

**52:53** · So we have a few minutes left before giving it to Shervine.

**52:57** · So I'm going to quickly go over the kinds of dimensions that people measure LLM output against.

**53:08** · So we have broadly-- so there are many dimensions, but just to simplify things, there are two main dimensions that we can look at here.

**53:16** · So one is how well your task is being done, so task performance with things like, was the response useful, was the response factual, was the response relevant, among other things.

**53:33** · And also, how aligned was the response format in terms of tone, in terms of whether the style was something that is aligned with what we want, in terms of whether there was any unsafe elements of response that was given to the user?

**53:54** · And I just want us to spend maybe five minutes on the factory dimension, which is actually something that requires a little bit more work.

### Factuality

**54:06** · And I'll give you just a setting.

**54:10** · So let's suppose we have some text output, and our goal is to quantify how factual that output is.

**54:21** · So I'm going to read the text out loud.

**54:23** · Teddy bears, first created in the 1920s, were named after President Theodore Roosevelt after he proudly wanted to shoot a captured bear on a hunting trip.

**54:33** · So what we want is quantify how factual that piece of text is.

**54:41** · So I told you previously that we typically prefer binary scales when it comes to rating something with respect to a dimension.

**54:54** · But the thing with factuality is that there's a lot of nuance.

**55:00** · Some texts may be very wrong.

**55:03** · Some texts may be a little bit wrong.

**55:04** · Some texts may be not wrong at all.

**55:07** · So we want to capture how wrong the text is, given the fact that the text can contain a lot of sentences.

**55:17** · And if there's one small issue, we don't want to just say the whole thing was not correct.

**55:25** · So I'm not sure if you saw in this text, but there are actually two errors.

**55:30** · So it's not 1920s but 1900s that the teddy bears were first created.

**55:35** · And the president didn't want to proudly shoot.

**55:39** · He actually, I think, refused.

**55:41** · So if we are in such a case, the question that we want to tackle here is how do we want to quantify this nuance.

**55:54** · So this is an open question that people have been writing papers on.

**55:59** · So what I'm going to tell you now is something that people typically use nowadays based on research that has been done.

**56:10** · So we typically operate in a few steps.

**56:16** · So the first step is for us to go from the original text output to a list of facts because when you look at a text, it actually contains a lot of facts that need to be checked.

**56:33** · And so the idea here is to aggregate the factuality of this text along the dimension of the facts that are present in this text.

**56:47** · So in this example, we would have one LLM call that transforms our original multi-sentence, multi-paragraph potentially text into a list of facts.

**57:02** · So here in this example, we would have four facts.

**57:07** · So that's the first step.

**57:09** · The second step is we would go over each of these facts and check whether it is correct or not.

**57:18** · And so here, we would typically proceed in a binary fashion because if you think about it, a fact is either correct or not.

**57:27** · I mean, you may have some in between, but we don't want to overcomplicate the task.

**57:35** · And so here the fact-checking process would typically involve the other technique we've seen last lecture, like RAG, for instance.

**57:46** · Given a piece of text, we want to, I guess, query a knowledge base with the actual fact and then check whether the fact that it's here is actually correct.

**57:57** · So this fact-checking process is typically something that involves things like RAG, web search is also something else, and so on.

**58:09** · So you can think of this fact-checking step as also involving LLM calls.

**58:17** · And so you can also think of some facts being more important than others.

**58:27** · So as an example, maybe the fact that the president proudly wanted to shoot the bear is not as important as, let's say, the name of the person after which the teddy bears were named.

**58:40** · So you can think of also having weights that quantifies the importance of each fact.

**58:49** · So people would use something like this formula, which is an aggregation over all the facts with some weight that quantifies the importance of each fact.

**59:03** · So these weights, alpha i, can be all equal to one another if you want to make it simpler.

**59:11** · It's not something that is necessarily the case everywhere that these must be different.

**59:17** · But it may be something that you can tweak.

**59:20** · So if we go back to our initial question, which is, how do you quantify the factuality of this text?

**59:28** · Here you would say, OK, the second and the third facts are both correct.

**59:35** · We know how important they are.

**59:38** · So we run this aggregation formula, and we obtain a score of 0.6.

**59:45** · So that means that there are some errors.

**59:47** · But we still have some things that were still factually correct.

**59:52** · This is typically how you would run this criteria with, I guess, the techniques that we have nowadays.

**1:00:02** · Cool.

**1:00:03** · I know I'm two minutes late.

**1:00:04** · And with that, I'm going to give it to Shervine.

**1:00:09** · Thank you, Afshine.

**1:00:11** · So before we move on to looking at specific benchmarks, I wanted to take a detour and look at what is happening on the agent side of things.

### Agent evaluation

**1:00:22** · So if you recall what we discussed last lecture, we talked about this ReAct framework where you could decompose what was going on within an agent into specific steps.

**1:00:37** · So it's usually three steps.

**1:00:40** · It can be observe, plan, act, or it can have other names.

**1:00:44** · But the fact is that you have several atomic steps that can loop.

**1:00:51** · So if you take a look at the typical agent's inner working, you can see a pattern like this.

**1:01:00** · Now you might wonder, how do you even evaluate such a thing.

**1:01:04** · So let's take a look at just one loop.

**1:01:11** · And then let's see together what can the errors be, in order for us to have an idea of what would an evaluation result mean on an agentic workflow.

**1:01:23** · So I'm going to show a slide that we had presented at the previous lecture.

**1:01:31** · And we had seen that we can decompose a tool call into these three steps.

**1:01:40** · So let's take our favorite example.

**1:01:43** · Let's say, you want to find a bear near you.

**1:01:46** · So you would ask that to the model.

**1:01:48** · So the first stage is to find the right tool call with the right argument.

**1:01:55** · And then once you have found this right tool call, you need to execute it.

**1:02:00** · And then based on your tool call prediction and on the results that you obtained from your tool, you would infer the results at the last step.

**1:02:12** · So these are three steps.

**1:02:14** · And you might have a series of them in the case of an agentic workflow where you call multiple tools and then build up your reasoning until reaching an answer that you then give to the user.

**1:02:29** · So now let's look at what can the failure modes, what can they be, at each of these steps.

**1:02:38** · So first, let's take a look at possible tool prediction errors.

**1:02:44** · So the first one I want to mention is the case where the error is that from a user query that obviously needs a tool, that you don't actually use the tool.

**1:02:57** · So here let's suppose that if you want to find a bear, you have the tool to find bears at-hand, but you don't use it.

**1:03:07** · So typically, if you don't use it, a possible behavior from the model can be to say an error.

**1:03:15** · So an error-- by an error, I mean, sorry, I cannot do that.

**1:03:19** · And this, in assistant terms, you can call this a punt.

**1:03:24** · So when you don't answer the question, you just fail.

**1:03:27** · It's called a punt.

**1:03:28** · So you might punt.

**1:03:29** · Here, sorry, I don't where I can find one.

**1:03:33** · And let's see together what could possibly cause this issue and how we could remedy it.

**1:03:42** · So I don't know if you recall the concept of tool router or tool selector that we had introduced.

**1:03:48** · So usually, when you are dealing with tools, you don't have just one.

**1:03:53** · You have multiple ones.

**1:03:55** · And the number of tools that might be useful for a large scale in the sense of number of users LLM, might be large.

**1:04:05** · So you don't want to input all the function APIs at every call.

**1:04:10** · So it might be the case that you have this intermediary step, where you filter down the sets of possible functions that you can put in the preamble.

**1:04:21** · And here these two selectors or tool routers, they have the property of trying to be recall-oriented.

**1:04:31** · So you want to trim the list of functions that you want to input in the preamble.

**1:04:38** · But you want to at least find those that you need.

**1:04:42** · So the main property here is that you want to save on context space, but you still want to ensure most of your use cases are still working.

**1:04:54** · So this is why here, when we say tool router error, we actually mean a recall error.

**1:05:01** · So it means it's possible that we just didn't select the right tool among the set of tools.

**1:05:07** · And let's say, this is the cause.

**1:05:10** · Then it's pretty clear we just have to adjust the tool router in order for it to be predicting the right tool.

**1:05:19** · So this can be one kind of issue.

**1:05:21** · Another kind is, hey, actually, the tool was included in this list of function APIs, but it's just that the LLM didn't think about using it.

**1:05:35** · So maybe this fine teddy bear was in there, but we just don't use it.

**1:05:40** · The LLM directly outputs a response.

**1:05:43** · So in that case, if you recall, we had mentioned techniques to teach an LLM to use a tool.

**1:05:52** · So you would need to revisit that part and either-- if you had trained it with SFT, so include this pattern maybe to train the model to recognize it, or if you had done prompt tuning, then you should revisit your prompt in order for this call to make sense to the model that you should use that tool.

**1:06:17** · Great.

**1:06:17** · So this is one kind of possible error.

**1:06:21** · Another one that you might see in the wild when you want to debug agents is at the time of tool calls, it might be the case that the model comes up with a function name that just simply doesn't exist.

**1:06:37** · So here I mentioned the tool hallucination.

**1:06:40** · This is what I mean by that.

**1:06:41** · So it calls a function that is just not defined.

**1:06:46** · So here our API was called, if you remember, find teddy bear.

**1:06:50** · So this was the function that existed.

**1:06:53** · And in this example of failure, the model tries to call the function find\_bear, which I haven't defined.

**1:07:01** · So when you see such errors, you have several potential causes, one of them being that the model simply doesn't round well overall.

**1:07:14** · And typically, it occurs if the model is too simple.

**1:07:21** · So it's an empirical observation.

**1:07:23** · If it's too weak, maybe it can make up things that it thinks can be reasonable.

**1:07:29** · But it doesn't actually ground on your instructions.

**1:07:32** · And here I have no better remedy proposal than to maybe upgrade the model, if you see that this is truly the case, and then see if this is reproducible.

**1:07:45** · Some other potential causes could be coming from actually you.

**1:07:50** · So the model is trained on very high quality data during its SFT stage.

**1:07:56** · So it has seen what great APIs look like.

**1:08:00** · So these tools that you define that help the user achieve what they're looking for might not be written in the best way, if you didn't use AI-assistant coding, let's say, or-- you don't necessarily have to use AI-assistant coding to write these.

**1:08:19** · But these are typically a great way to check whether your implementation makes sense from a model standpoint.

**1:08:26** · And if it doesn't, then a typical remedy is to either-- renaming the API just a function name, and the arguments just go a long way because this is what the model will see when it comes to your tool call.

**1:08:44** · It sees the API function name.

**1:08:47** · It sees the arguments and the high-level descriptions.

**1:08:51** · So these are your three knobs to tune in order to make it sound more logical and then linked to the actual task at-hand.

**1:09:05** · At the very beginning, I was saying, maybe the model is too weak.

**1:09:08** · But actually, maybe the first thing you should check is whether the horizontal instructions-- so horizontal across tools, whether these are clear enough.

**1:09:19** · Maybe the model hasn't really understood that it needs to use functions that are given to it.

**1:09:27** · So maybe it's just making up function names that it believes could have access to.

**1:09:32** · So the first thing to check would probably be to see if this phenomenon is generalized and see if these horizontal instructions, they are concisely saying that you should make sure to use available functions.

**1:09:50** · And then on that, you can iterate on these top level instructions and maybe iterate with an LLM itself because typically, top-level instructions, they are very important.

**1:10:03** · So you need perfect formatting and perfect logic.

**1:10:07** · So typically being able to detail them with great detail is helpful.

**1:10:18** · Now let's see a third possible failure cause.

**1:10:22** · So let's say, you have your model and your user prompts, but you just don't use the right tool.

**1:10:30** · So here if the user says, find a bear near me, one other reasonable approach would be, what if you just send a message asking for a bear?

**1:10:44** · That would be reasonable.

**1:10:46** · But maybe that's not what you want to implement as a behavior for your user.

**1:10:52** · So in that case, it's not clear to the model what approach you prefer.

**1:10:58** · And it is on your-- it's your responsibility to ensure it is indeed clear.

**1:11:07** · And then you have to do that at two different levels.

**1:11:11** · So the first one is potentially also at the tool router level.

**1:11:16** · Maybe the tool router doesn't know that for this kind of query, you should have the tool that you had in mind as part of the results.

**1:11:25** · So it's possible that you have a recall issue that you need to fix.

**1:11:29** · And then the second one is simply going back to the APIs of both functions, maybe the conflicts in scope.

**1:11:37** · So you want to go back to each of them and be precise into which situations should be dealt with with which tool.

**1:11:48** · So being very precise in these APIs just plays a lot here.

**1:12:00** · Great.

**1:12:00** · So now we're going to go through a fourth and last failure mode for this tool prediction task, which is, what if you have the right tool, but you just don't have the right arguments?

**1:12:15** · So you have already gone one step.

**1:12:18** · You have found the right tool, but then this last mile of making sure that the tool is run with what you would like is not fulfilled.

**1:12:26** · So here if I say, find a beer near me, and it outputs-- and it uses the coordinates 0, 0, which is somewhere in the Southern Atlantic-- it's not likely that I'm actually there, that can reflect an issue.

**1:12:44** · And one possible explanation for this is that maybe it simply doesn't know where I am because I haven't specified in my query that I'm here at Stanford, and it just tries to make up my coordinates.

**1:13:01** · So one thing that you should double check is making sure that the context carries the location information.

**1:13:09** · So if I haven't provided that as a setting on my LLM map, it's possible it's not there.

**1:13:17** · And then let's say, if it's not there, maybe you would want to introduce a location finder tool that is executed beforehand.

**1:13:26** · And if it fails because I haven't given the app to permission to see my location, then maybe you could have actionable error shown to the user, instead of having some dummy parameters passed in.

**1:13:41** · So this is one potential remedy.

**1:13:44** · And then the second one is maybe it's his arguments, but the model just doesn't know what it should put as input.

**1:13:53** · So that could also be another reason.

**1:13:57** · And then on that, this is a common remedy to go back and then retrain either the model on how it uses these tools or rewrite the API.

**1:14:12** · So we have seen four ways to-- I mean, four failure modes for the tool prediction step.

**1:14:19** · Now we're going to see two more on this tool called step.

**1:14:26** · So the first one is a very simple one, from a mindset perspective, maybe your tool just doesn't output the right response.

**1:14:39** · So it's a very vague category.

**1:14:44** · And as an example, maybe your code logic has a bug somewhere, and it just returns an error.

**1:14:53** · Those that you see in Python, maybe it hits some value error or anything else.

**1:14:59** · And I just want to say that it might not be necessarily the case that hitting an error is bad because sometimes in the case of finding your location, if you haven't provided your permission to find the location, maybe it will hit an error, and the model will anchor on that error to convey the status to the user.

**1:15:23** · But in general, it's not really common practice to return errors just because the model could interpret it as an internal tool error.

**1:15:35** · So sometimes when you hit an error, and then you ask the model to synthesize the tool call it has seen, sometimes it just says, oh, sorry, I couldn't do it, but it's my fault.

**1:15:46** · It's because I encountered an error.

**1:15:47** · It doesn't really say actionably what happened.

**1:15:52** · And instead, the fix here is to convey these outputs in a meaningful manner.

**1:15:58** · So typically, you have a structured output, and you return a true output instead of an error.

**1:16:07** · Here it's a general case just to say that just check your tool implementation.

**1:16:14** · So you have the right arguments, you have the right tool, but you just don't have the right value.

**1:16:19** · So just a software engineering problem, just go and fix the tool.

**1:16:25** · And the second category of issues that we see at the backend level is when you return no response.

**1:16:34** · And returning no response is often bad when the tool is one that performs an action.

**1:16:43** · So let's say, last lecture, we talked about increasing the thermostat for your teddy bear who was cold.

**1:16:50** · So if you increase the thermostat, and the tool doesn't say anything, so the model doesn't know if it has done the task successfully or not.

**1:17:01** · So it could well come up with a false confirmation of hey, all is good.

**1:17:06** · I have increased the thermostat.

**1:17:08** · No worries.

**1:17:09** · But it actually hasn't.

**1:17:11** · So this is why a common guidance is to always make sure that tool calls are followed by meaningful outputs.

**1:17:21** · So as usual, you have this structured message and you should take advantage of it to convey what has happened as part of your tool in order to make sure that the model, in turn, knows what to convey to the user or knows how to continue that agentic loop.

**1:17:45** · So always output something.

**1:17:47** · And let's say, you want to find a teddy bear, and you haven't found any, then here you will be surprised by what I say, but it's actually better to output an empty JSON than just outputting none because an empty JSON could mean I found no bears, but an output of none doesn't say anything.

**1:18:11** · So even in that case, an empty output in the sense of an empty JSON is meaningful.

**1:18:17** · And make sure to use that meaning in the way you encode your tool.

**1:18:23** · Great.

**1:18:23** · So we have seen two more possible errors at the function call level.

**1:18:29** · Now let's suppose that everything went great at the first step.

**1:18:34** · Everything went great at the second step.

**1:18:36** · So you have found the right tool output.

**1:18:39** · But now the model has trouble to synthesize the output into a meaningful response.

**1:18:49** · Let's suppose your tool found a bear named Teddy.

**1:18:52** · And then the other attributes, which I haven't shown here, maybe say that they are one mile away from me.

**1:18:58** · So the teddy bear has been found, and we just have to present it to the user.

**1:19:02** · But if you put it to the model, let's suppose the model says, I didn't find any bear.

**1:19:10** · So what could be the cause here?

**1:19:12** · So it could be the case that you have an output that has information that the model doesn't ground on.

**1:19:21** · So it could be the case that just the model lacks the ability to refer to content that was put previously.

**1:19:31** · So here I have the same vanilla suggestion of upgrading the model.

**1:19:39** · So usually, that doesn't happen really anymore, but it used to in early iterations of LLMs.

**1:19:46** · There is one that is actually one that happens fairly often.

**1:19:52** · So sometimes the tool back end returns not only an output but a lot of output.

**1:19:58** · And it's too much for the model to properly parse what is important.

**1:20:04** · So you have maybe the information of Teddy in there.

**1:20:06** · But it's drowning under an ocean of other kinds of information that are not useful.

**1:20:15** · So the model cannot distinguish what is helpful.

**1:20:19** · And then the solution for that is to go back to your tool implementation and ensure that whatever you output is meaningful to be used to the model in the next stage.

**1:20:32** · And I think this goes-- this overlaps with the third reason I put here.

**1:20:38** · So let's say, your output is trimmed already.

**1:20:41** · Then another possible explanation is maybe it's not being presented in a meaningful way.

**1:20:47** · So this is why in Python, you have these classes where you can instantiate attributes so that the output is very meaningful.

**1:20:56** · So let's say, for saying that you have found a bear, you could return an object called teddy bear with attribute's name, distance, and so on-- this is very meaningful, as opposed to maybe raw information that it doesn't how to interpret.

**1:21:16** · Awesome.

**1:21:16** · So we have seen here seven different failure modes over all these categories.

**1:21:24** · So these are not the only ones.

**1:21:26** · I have just mentioned those that I see very often that I thought could be helpful, but you could definitely see other failure modes.

**1:21:36** · Does this make sense?

**1:21:40** · Do you have any questions?

**1:21:45** · Great.

**1:21:46** · So we can move on to summarizing what were the common trends in these failure modes.

**1:21:56** · So oftentimes, we have talked about the modeling side, where sometimes improving the model's ability to reason and ground could be the solution.

**1:22:11** · Another complaint that we have seen is the relevance of what we put in the context window.

**1:22:17** · If we improve that relevance, maybe it gets better.

**1:22:20** · And on the modeling side, one more aspect is maybe the tool route's modeling or the tool API modeling itself, either by SFT tuning or just prompting, or even just the API description itself, does the function make sense?

**1:22:39** · Do the arguments make sense?

**1:22:42** · Does the docstring make sense?

**1:22:44** · So this is one kind.

**1:22:46** · And the other kind is the tool itself, so maybe it just has a problem.

**1:22:51** · So you need to fix it.

**1:22:55** · And I just want to say that when you deal with tools and evaluations, you have a lot of possible errors.

**1:23:01** · So one thing that will help you to navigate through this is to be very methodical into categorizing kinds of errors and then dealing with them in group.

**1:23:14** · So you see there are lots of errors.

**1:23:16** · And every time that you deal with a given loss, it's maybe just an adventure to solve.

**1:23:23** · So really, being very organized here is going to help you a lot.

**1:23:30** · With that in mind, we can delve into the world of benchmarks.

**1:23:37** · So we talked about evaluations.

**1:23:38** · Now you might wonder, how can you evaluate a large language model.

**1:23:44** · Let's say, you have trained everything.

**1:23:46** · How can you compare it with respect to others?

**1:23:49** · So we're going to see together a series of benchmark categories that today's benchmarks usually-- where today's benchmark usually resides, so in one of these.

### Benchmarks

**1:24:04** · And we're going to see examples for each of them.

**1:24:07** · Does that sound good?

**1:24:10** · Awesome.

**1:24:11** · So we can start with a kind of benchmark that I called a knowledge-based benchmark, where we want to test if the model is able to restitute given facts.

**1:24:30** · These facts are typically spanning lots of domains, so it doesn't have to be super precise on a given domain.

**1:24:39** · But it's spanning all the kinds of domains that your users may care about.

**1:24:44** · And then prime example for this is MMLU that we're going to see very soon.

**1:24:50** · But just before we do so, I want to say that this knowledge benchmark mostly, but not only, measures how well pretraining was done, how well the information in your large corpora of data was retained by the model in order to be helpful at inference time.

### Knowledge with MMLU

**1:25:12** · So MMLU stands for Massive Multitask Language Understanding.

**1:25:20** · And this benchmark has almost 60 different tasks that are super diverse.

**1:25:28** · So it's not just one specific topic is just a bunch of topics, like everyday life topics or very-- for example, there is law or medicine, and everything that you can think about.

**1:25:44** · And the benchmark is redacted in a way that can be easily measuring an LLMs performance and weighing that performance with respect to others.

**1:25:57** · So it's not something that is free-form.

**1:25:59** · It's something that is very constrained.

**1:26:05** · There is a question, and then you have four possible answers.

**1:26:09** · And you ask the LLM to choose one of them.

**1:26:13** · So it's a bit like CME 295 exams.

**1:26:17** · Part of the exam is also multiple choice questions.

**1:26:21** · And it's a good way to standardize the knowledge evaluation.

**1:26:26** · And it's the same that is used here.

**1:26:31** · And this is also a trend that you see across benchmarks.

**1:26:36** · You don't ask the model to just come up with some answer.

**1:26:39** · And you have maybe an LLM-as-a-Judge just giving some opinion about it because doing so introduces another layer of potential errors.

**1:26:53** · The LLM-as-a-Judge, as I've seen mentioned, isn't necessarily perfect.

**1:26:57** · So this framing enables us to have a hard-coded way to extract the answer output by the LLM.

**1:27:07** · So typically, you would ask it to output the right letter at the end of each question, which you can then extract and then compare with respect to the answer.

**1:27:18** · So giving some examples about what is in this benchmark, as I mentioned, you have all sorts of fields in there.

**1:27:28** · And you will notice that each problem mostly requires some prior knowledge about that topic.

**1:27:36** · So it's not purely logic that will help you solve it.

**1:27:41** · And I think the last example in this slide is a good representation of it, where you have something in the domain of medicine, and you have a bunch of numbers, patient has this and that, what would you say-- where would you say is the damage.

**1:27:58** · So it's typically something that you could see in medicine books, maybe.

**1:28:02** · And the same goes with other fields, such as law, where everything has been codified somewhere, and you need the knowledge of that somewhere in order to answer the question.

**1:28:14** · So this is the first kind.

**1:28:17** · And it's not the only kind of benchmark.

**1:28:21** · So it's not the only benchmark in this category.

**1:28:23** · You have other benchmarks that can be in these categories, but it's just one of them.

**1:28:29** · A second category that you might see are those that are in the reasoning space.

**1:28:36** · So typically, these are kinds of benchmarks that require some amount of thoughts before outputting an answer.

**1:28:47** · So it assesses the quality of the chain of thoughts, or if you are in the reasoning world, maybe the quality of your think tokens, but just more broadly, your ability to infer a response based on some reasoning.

**1:29:04** · And then for that, I'm going to mention two examples, so one in the field of math and then one other in the field of so-called common sense reasoning that is anchored in everyday life, which is typically the field that might be of interest for your LLM users.

**1:29:27** · And we're going to see that very soon.

**1:29:30** · So first, let's take a look at the benchmark focused on math.

### Reasoning AIME, PIQA

**1:29:35** · So how many of you about AIME?

**1:29:41** · So AIME is an exam that high school students sit for when they want to participate to the Olympiads.

**1:29:54** · And typically, it's a very hard test.

**1:29:58** · And it's covering math topics.

**1:30:02** · And then it's in a format that is LLM-friendly because you have a given problem statement.

**1:30:09** · And at the end, you ask the student to write the response into a three-digit number.

**1:30:19** · So it's very well-constrained, which makes it a right fit to benchmark LLMs.

**1:30:25** · And just like the one before, it's hard-coded.

**1:30:30** · And I give here some samples of the AIME exam as seen this year.

**1:30:37** · So as you see-- I don't know if you can read from afar, but it's not super simple.

**1:30:42** · You have some one sentence.

**1:30:45** · So you think maybe it's easy, but you actually need to write down the reasoning before finding the answer.

**1:30:50** · And this is what we want to test the LLM for.

**1:30:55** · And then the second kind of reasoning that we mentioned here was so-called common sense reasoning.

**1:31:03** · And the one that is often used these days is PIQA, so Physical Interaction, Question Answering.

**1:31:12** · So these are tasks that are deeply grounded into the physical real world.

**1:31:19** · So we have some samples at the next slide that we'll show.

**1:31:22** · But it is still reasoning-based questions, but that will rely on your understanding on how things work around you, not necessarily math-based but just everyday life.

**1:31:37** · And this time, it's not multiple choice questions over four answers, like the MMLU.

**1:31:46** · It's over two only.

**1:31:49** · And you have the 20,000 examples.

**1:31:52** · And then here, a good example that I really liked from the samples mentioned in the paper was, how do I find something I lost on the carpet?

**1:32:02** · So there is one solution that says, vacuum with a solid seal.

**1:32:07** · And the other one is vacuum with a hairnet.

**1:32:12** · And of course, when you vacuum with the solid seal, then the seal is, of course, solid, so no air can go through it.

**1:32:20** · But if you have a hairnet, you will vacuum the whole thing.

**1:32:24** · And the thing that you lost will be caught inside of it.

**1:32:28** · So what I mentioned is common sense, but it might not be obvious.

**1:32:31** · And this is what we tasked the model to resolve.

**1:32:37** · So then one other major area for benchmarks is coding, where we want to probe the model for solving complex questions, encoding.

**1:32:51** · And this has two main uses in real life.

**1:32:56** · So one is aligned with the use case that I mentioned at the end of last lecture, the one I liked regarding AI assistant coding.

**1:33:05** · So these models, they aim at being used in that setting as well.

**1:33:10** · So you should make sure that these benchmarks perform-- these benchmarks show that these LLMs perform well in order to be useful to your users.

**1:33:20** · And then a second reason why benchmarking on coding makes sense is that you have all these tools that you might want to use in an agentic setting.

**1:33:31** · And then these tools are written maybe in a Python format.

**1:33:35** · So you want to ensure that your model has the right ability to read and write code so that it can execute this tool calls and then interpret what's coming out of them.

**1:33:49** · So these are two, I would say, motivations that motivate us to find coding useful here, even to the folks that don't do coding at all.

### Coding with SWE-bench

**1:34:06** · And then one example of such benchmark is SWE-bench.

**1:34:12** · So SWE-bench is-- I put a question mark here because they didn't define exactly what the acronym meant.

**1:34:20** · But it's likely that it meant software engineering benchmarks.

**1:34:23** · So SWE is oftentimes the acronym for software engineering.

**1:34:27** · And what they did is that they looked at popular Python repositories, and they filtered down those that contained pull requests that were solving an issue and that introduced tests.

**1:34:44** · So you have some before-after behavior that you can quantitatively assess with the tests that are introduced.

**1:34:53** · And supposedly, if you have pull request that introduces some tests and some fix, you can fairly assume that these tests were not passing without the fix, and that they are passing after the fix.

**1:35:09** · So the fact that we have these tests at-hand is a good measure for us to assess the quality of ethics.

**1:35:18** · So if you have heard of a test-driven development, it's all about having tests and ensuring they pass.

**1:35:25** · And this is what it relies on.

**1:35:30** · And then here you ask these LLMs to solve these GitHub issues.

**1:35:36** · And then you assess whether they indeed pass by looking at the test status before and after patching the answer suggested by the LLM.

**1:35:52** · Great.

**1:35:54** · So here is a very nice figure that the paper introducing that benchmark gives.

**1:36:00** · So you're given a code base.

**1:36:03** · And then what you ask the model for is a patch.

**1:36:06** · That is just it.

**1:36:07** · And then you patch whatever the model has provided to find the tests-- to find the test status.

### Safety with HarmBench

**1:36:16** · And then one last area that I want to mention in the case of base benchmarks is safety.

**1:36:25** · So when you see fancy LLMs coming out, you usually don't see in the advertisements of modeling benchmarks the safety part because usually, safety is a bit subjective with respect to the LLM provider.

**1:36:43** · So every company has its own policy.

**1:36:47** · So you cannot necessarily compare a performance on a given benchmark across models just because all these providers might not claim they want to perfectly solve that benchmark 100%.

**1:37:00** · As a result, it's not necessarily a good measure of that field.

**1:37:05** · So if you look at model cards, oftentimes, you see a safety section being mentioned in reports to say the work that they have done.

**1:37:14** · But they don't necessarily compare models with respect to a given benchmark.

**1:37:22** · And usually, the safety benchmarks, they are fairly aligned with what we think should or should not happen.

**1:37:30** · But on top of it, you might have additional policies that maybe just policies.

**1:37:36** · So you have some human that had to make some decision.

**1:37:40** · It might not be a universal decision, it's just a given decision.

**1:37:45** · So the benchmark's goal is to be aligned with what kind of policy the LLM provider has in mind in order to be truly meaningful.

**1:37:57** · This is why when you execute a safety benchmark, you should check the content of the benchmark in order to put a meaning behind it.

**1:38:07** · So here let's talk about HarmBench, which I am supposing it means harmful behavior benchmark.

**1:38:15** · So this benchmark has four categories, the so-called standard category that is categorizing quote, unquote, "vanilla" harmful behavior.

**1:38:30** · Then you have copyright category that is assessing the model's ability to generate copyrighted content, which we do not want.

**1:38:40** · And then the two last ones, contextual and multimodal, both of them are contextual based on a given modality.

**1:38:50** · So contextual is on the text modality.

**1:38:53** · And then multimodal is with other modalities than text.

**1:38:59** · So we're going to see an example at the next slide.

**1:39:02** · And here you don't have the same ability to assess performance on this benchmark based on some hard-coded match because these harmful statements might be open-ended.

**1:39:16** · And you cannot possibly just solve all of these by RegEx matching.

**1:39:21** · For example, one example in the standard category of this benchmark tries to entice harmful behavior into executing something that is harmful.

**1:39:37** · And the paper mentions-- distinguishes something that is very interesting.

**1:39:43** · So it distinguishes model quality with safety by saying that if the model tries to do the harmful behavior, even if it wasn't successful, because it was not of a good quality enough, then it's enough to count the attack as successful.

**1:40:03** · And for that, they trained some classifier to recognize these cases.

**1:40:09** · This is the only benchmark among those that I presented here that is done based on a classifier that can itself be prone to error compared to others that are very grounded in constrained set of values.

**1:40:28** · And as promised, here is a few examples as mentioned in the paper.

**1:40:32** · So here we test whether you can unlock a door that you shouldn't unlock.

**1:40:38** · And then here the test is on influencing someone with respect to some election.

**1:40:46** · These are not safe behaviors.

### Agents with Tau-Bench

**1:40:52** · Great.

**1:40:52** · So I mentioned everything so far that you could solve without tools.

**1:40:58** · So of course, I say, you could.

**1:41:00** · Some of them, you could use tools, of course, to solve them.

**1:41:03** · But what about measuring the behavior of agents?

**1:41:08** · So for here, you have an interesting benchmark called tau-bench, where tau is a Greek letter that actually, you can write it as tool agent users.

**1:41:21** · And this is why we say tau.

**1:41:23** · And it's a benchmark that provides across two different fields, so the airline and the retail field, a set of tools.

**1:41:34** · And it gives a set of policies, things that the agent can and cannot do.

**1:41:41** · And then what you do is that you have a set of tasks.

**1:41:45** · So tasks are problem statements that you give a given user.

**1:41:53** · And the goal is for the user to achieve that task through the agent.

**1:41:59** · And the interesting thing about tau-bench is that tau-bench is language model-simulated.

**1:42:05** · So the user interaction with the agent, as you can imagine, cannot be hard-coded because further terms will depend on previous ones.

**1:42:16** · So let's say, you say something as a user and your agent decides to do something, then you need the context of what it has done in order to continue the conversation.

**1:42:27** · And this is why you have this simulation aspect that the paper introduces that is typically done by a separate big model that plays the role of the user.

**1:42:41** · And here we have an example of the task of changing a flight.

**1:42:47** · So you have given tools.

**1:42:48** · The agent tries to help the user to achieve that goal.

**1:42:55** · And at the end of it, we assess whether it's successful in doing so by calculating a reward that's a function of the database change.

**1:43:05** · So let's say, the user has changed their tickets, so we want to see if the database has indeed the state that we're looking for and/or a given action.

**1:43:17** · So maybe the action of canceling is one that is the goal of this task.

**1:43:23** · So this is part of the reward.

**1:43:26** · And the paper that introduces this benchmark talks about a concept that is a funny word play.

**1:43:36** · With respect to the metric that we had talked about at the last lecture, we had introduced pass@k.

**1:43:46** · And then this paper talks about pass hat k, which is the probability that all k attempts succeed.

**1:43:58** · And then why is that relevant metric here?

**1:44:02** · So as you have seen, the airline and retail domains were ones that were chosen here.

**1:44:09** · And then an agent in the loop here could be a way to see if automating the agent side of things could help.

**1:44:21** · And in order to truly know whether it can help, you want to have reliability and consistency in mind.

**1:44:28** · So if you execute the task k times, you don't want pass@k, the probability that at least one of them succeeds.

**1:44:38** · You want the probability that all of them succeeds, which is why this metric matters.

**1:44:44** · So if we had more time, I would have derived the formula to find pass hat k with respect to the parameters of the problem.

**1:44:58** · But I will refer you to the derivation that Afshine has done last time.

**1:45:03** · I just want you to be convinced that this is indeed the formula.

**1:45:07** · So if you're not convinced, please feel free to do it at home.

**1:45:12** · And moving on, we talked about all these benchmarks.

**1:45:17** · Now let's see how they are grounded in reality.

**1:45:21** · So by now, I think everyone of you has seen the new Gemini launch a few days ago.

**1:45:28** · So this was the report that was sent to everyone to justify that the performance here was better.

**1:45:36** · And you can see that what we introduced here is mentioned in some format.

**1:45:43** · So the reasoning part on AIME and PIQA is there.

**1:45:48** · And you will see that some of these benchmarks are derived in a flavor that introduces multilinguality-- multi-languages.

**1:46:01** · So this is the case for PIQA.

**1:46:03** · Instead of PIQA, it's global PIQA.

**1:46:05** · And then for coding, it uses a flavor of SWE-bench.

**1:46:09** · And tool use, it uses also a flavor of tau-bench, which is tau squared bench.

**1:46:19** · And a few last words, here I just want to say that benchmarks are here to characterize the profile of your LLM.

**1:46:29** · So it's not all good or all bad.

**1:46:33** · Maybe some of your LLMs will have some strength and some weaknesses.

**1:46:39** · And your personal experience might guide you to use one specific one with respect to others in given situations.

**1:46:47** · So if I had to just give my personal experience, I know that the Sonnet models are very helpful for coding.

**1:46:56** · And whenever I want outputs that are fast and cheap, Gemini Flash is usually good.

**1:47:02** · But these are not, by any means, global recommendations.

**1:47:06** · Your own use case and your own experience can guide you into having a profile of models that suits your tasks best.

**1:47:15** · And you can interestingly plot the performance of your models with respect to the other dimension that you care about, which is price, for example, and see for a given price, what is the best model you can use.

**1:47:32** · And then the border that you see on the best models for that specific metric is called the Pareto frontier.

**1:47:41** · And you might have a Pareto frontier with respect to several aspects, so cost, safety, context length.

**1:47:49** · And then a few words regarding data contamination, one thing about these benchmarks is that they are as good as the assumption of whether you have seen the actual benchmark results or not.

**1:48:03** · So make sure you haven't seen them.

**1:48:04** · And for that, people introduce hash values.

**1:48:08** · In the case of tool use, they introduce a blocklist in order to not access websites that might contain the responses, or in the case of math, we have the luxury of evaluating on new tests that the model has for sure not seen.

**1:48:26** · And Goodhart's law is a very good adage that says, when a measure becomes a target, it ceases to be a good measure.

**1:48:38** · So all these benchmark results are to be weighed against what you're truly looking for.

**1:48:45** · And then these benchmark results don't necessarily tell you whether a model is good for you or not.

**1:48:53** · We had talked about ChatBotArena.

**1:48:57** · In one of the previous lectures, it can be one way to balance the real-life performance of these.

**1:49:04** · But I would say, ultimately, should be the one trying out these best models and see for yourself which one corresponds to your best.

**1:49:14** · And with that, I hope you all have a great Thanksgiving.

**1:49:16** · And thank you.