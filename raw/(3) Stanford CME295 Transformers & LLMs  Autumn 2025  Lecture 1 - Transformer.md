---
title: "(3) Stanford CME295 Transformers & LLMs | Autumn 2025 | Lecture 1 - Transformer"
source: "https://www.youtube.com/watch?v=Ub3GoFaUcds"
author:
  - "[[Stanford Online]]"
published:
created: 2026-09-02
description: "在 YouTube 上盡情享受自己喜愛的影片和音樂、上傳原創內容，並與親朋好友和全世界觀眾分享你的影片。"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=Ub3GoFaUcds)

## Transcript

### Introduction

**0:05** · Cool.

**0:06** · Hello everyone, and welcome to CME 295-- Transformers and Large Language Models.

**0:12** · So my name is Afshine.

**0:15** · And I will be teaching this class with Shervine, who's in the back.

**0:19** · And before I start, I'm just going to introduce ourselves.

**0:24** · So we're twin brothers, and we actually had a similar background.

**0:31** · So we both went to a school in France called Centrale Paris, and then we each went our way.

**0:37** · So on my end, I went to MIT, and then Shervine went to Stanford to do the ICME Master's program.

**0:45** · And after that, I guess our industry background is very similar as well.

**0:51** · So I first went to Uber, and then Shervine came to Uber as well, and then Shervine left to Google, and I went to Google.

**0:58** · And then very recently, I joined Netflix, and Shervine joined Netflix as well.

**1:02** · And we've been working on Large Language Models.

**1:07** · So yeah, I guess we have technical backgrounds, and mostly oriented towards LLMs.

**1:14** · OK, so why are we doing this class?

**1:17** · So since 2020, Shervine and I have been specializing in NLP.

**1:21** · And we've been giving this class in the format of a workshop that was done on a yearly basis.

**1:29** · So in 2021, 2022, 2023, 2024, ChatGPT came in 2022, and suddenly there was a lot of interest for LLMs.

**1:40** · And so it's actually last spring that we started to offer this class as a Stanford course that is now called CME 295, and this is the second instance.

**1:55** · Cool.

**1:56** · So what can you expect from this class?

**2:00** · So first of all, LLMs are basically everywhere now.

**2:05** · And I guess our goal here is twofold.

**2:08** · So the first one is to learn about the underlying mechanism that makes all this work.

**2:15** · And we're going to see the transformer, which is the foundational architecture that makes all this work.

**2:22** · And then the second thing is to know how these LLMs are trained and where they are applied.

**2:31** · So in case you're still wondering if this class is good for you, I would say that this class is great for people who just in general have an interest in this field, either because you wanted to make it your career goal, if you want to be a research scientist or an ML scientist, or if you want to develop a personal project that relies on LLMs to some extent,

**2:58** · to just knowing the caveats, I guess what works, what doesn't, or just say if you're in a separate field and you just want to know how this whole AI, GenAI, LLMs thing works and how you can apply it to your domain.

**3:14** · So now in terms of prerequisites, I would say that at a very minimum you should have some foundations in ML, like basically know how a model is trained, what a neural network is, and also some basics in linear algebra, so basically how matrices are multiplied, for instance.

**3:39** · But even if you have a developing, I guess, competency in this field, I guess it's fine, we're still be here to help you out.

**3:48** · But I guess this is like the ideal set of prerequisites.

### Class logistics

**3:54** · Cool.

**3:55** · So still on the logistics, so this class will be held every Friday from 3:30 to 5:20, and it will be held here.

**4:08** · So this class is two units.

**4:10** · And you have the choice to either take it as a letter or a credit/non-credit.

**4:18** · So as you could tell from the setup, we're basically recording this class.

**4:23** · And if you cannot for some reason attend this time, this slot, we'll make sure with Shervine to make the recordings available, either tonight, like every Friday night, or on Saturday.

**4:38** · So in terms of the grades, so what we're doing for this quarter is to have two exams.

**4:47** · So one is the midterm, which will be happening during our fifth instance, which is October 24.

**4:56** · And then the second exam will be the final exam, which will be held in the week of December 8.

**5:05** · So date is still TBD.

**5:09** · So we'll let you know.

**5:12** · Cool.

**5:14** · So every time we have a lecture, we'll be posting the slides and the recordings on the website.

**5:21** · And in case you're interested, we also have the syllabus in there, so you can know a little bit what are the topics that we'll be talking about.

**5:29** · And the class textbook is this Super Study Guide-- Transformer LLMs.

**5:36** · So we have a copy here in case you want to take a look.

**5:39** · So yeah, I guess a lot of the concepts that we have in this class will actually be in the book.

**5:44** · So I guess it's a helpful way to follow this as well.

**5:50** · And also, we did some very short, condensed version of this whole class that we called the VIP cheat sheet.

**5:58** · So this one is available on GitHub in case you're interested.

**6:03** · And yeah, we also translated it into a number of languages now.

**6:08** · By the way, if your language is not there, let us know, and happy to work on that as well together.

**6:16** · OK, cool.

**6:17** · I think it's the last things on the logistics part.

**6:19** · So in terms of announcements, we'll be posting things on Canvas.

**6:23** · In case you have any questions, you can of course reach out to us.

**6:27** · But there is also a tab on Canvas that's called Ed.

**6:32** · I'm sure you're familiar.

**6:34** · So you just click on that, just post your question, and then Shervine and I will be responding.

**6:41** · And I guess to reach out to us, you have this mailing list.

**6:45** · Or we're just two, so just ding us.

**6:51** · Cool.

**6:51** · So on the logistics, do we have any questions so far?

**6:55** · And one thing I forgot to mention is that, given that we're recording this class, I guess if you're asking a question, it may not be super clear for the viewer what your question was.

**7:08** · So I'm going to make an effort to just repeat your question.

**7:11** · It will sound weird, but I'll try to not forget, but yeah.

**7:17** · So any questions so far on the logistics?

**7:21** · Yeah.

**7:29** · So the question is whether there are coding parts in the exams.

**7:32** · So the answer is no.

**7:34** · So the exams will purely focus on concepts that we see in class.

**7:38** · And actually, it's not meant to trap you.

**7:41** · So I guess if you follow the class, if you see the slides and the concepts that we see, should be fine.

**7:48** · Yeah.

**7:53** · Oh, yeah.

**7:55** · Question is, if you're waitlisted, what do you do?

**7:57** · I think, so by experience, a lot of people will finalize their schedule.

**8:02** · Some people will drop, some won't.

**8:04** · In case you're still waitlisted, come talk to us.

**8:07** · But I'm pretty confident it's going to be OK, because I think the waitlist right now is six.

**8:12** · So, I think it should be fine.

**8:14** · Cool.

**8:15** · Yeah.

**8:19** · They will be on the website, and we'll make sure to also post a link on Canvas.

**8:25** · Yeah.

**8:26** · So the question was, where are the slides.

**8:28** · And they're on the website.

**8:31** · Cool, yeah.

**8:40** · So question is on the weighting of the exams.

**8:43** · So there is no homework.

**8:44** · So 50% is midterm, 50% is final.

**8:48** · And no grades-- I mean, no weights are from that.

**8:52** · And particular, I mean if this slot is conflicting with something, just keep in mind that we are recording this.

**8:59** · So it's fine if you cannot attend this session.

**9:02** · Yeah.

**9:06** · Sorry?

**9:11** · Oh, is the question that the final is about just the second half of the class?

**9:17** · We have not written the exam yet, but I think this is something we are thinking of.

**9:21** · So the final is probably going to be about the second half of the topics.

**9:29** · Cool.

**9:30** · OK, long story short, 50% midterm, 50% final exam.

**9:36** · And it's a fun class.

**9:39** · Cool.

### NLP overview

**9:40** · So with that, I'm going to just slowly start the class.

**9:44** · So another thing that I want to mention was every time we're talking about something, you will see that at the bottom of the slide there will be a source, and it's mostly for-- so first, to credits, whatever, we're quoting, but also for you to dig into those materials a bit more in case you're interested.

**10:04** · Because, of course, we have only two hours per week, and we only have nine or 10 weeks, so there's nowhere near enough time for us to cover everything.

**10:16** · And the second disclaimer is you will see that the field is full of abbreviations.

**10:23** · So I myself was completely scared of them when I started.

**10:28** · But hopefully by the end of the class, you will have a mental mapping of what these abbreviations mean, respect to what they correspond to.

**10:36** · So yes, so if you have a mental mapping towards the end of the class, then we know e did a good job.

**10:42** · So with that, let's start.

**10:46** · And I guess we will start at a very high level, because I will just assume that I guess we're starting from scratch.

**10:56** · And we're going to talk about NLP in general.

**11:00** · So NLP is going to be our first abbreviation.

**11:03** · So NLP stands for Natural Language Processing.

**11:07** · And it is a field that is around manipulating text, just computing things with text.

**11:15** · And at a very high level, can basically classify NLP tasks into three buckets.

**11:22** · So the first bucket is what we call classification.

**11:26** · So we have an input text as an input.

**11:30** · And then what we want is to predict something.

**11:34** · So one example is you have a movie review and you want to predict whether the sentiment is positive, negative, or neutral.

**11:44** · So that's one example.

**11:46** · You can also have intent detection, just knowing what, for instance, the person wants to do.

**11:51** · So let's suppose you say, I want to create an alarm for tomorrow.

**11:55** · So the intent here is create an alarm.

**11:59** · So also to detect a language-- so for instance, if you write among French, you want to detect that text is in French, topic modeling.

**12:09** · The second category is what we call multi-classification.

**12:14** · So we still have a text as input, but this time we predict more than one thing.

**12:21** · So you have a number of tasks in that bucket as well.

**12:25** · So one that is very popular is called Named Entity Recognition, a.k.a.

**12:31** · NER.

**12:33** · So what that task does is, given an input text, we want to basically label some specific words, like, for instance, identifying whether something is a location, or a time, and so on.

**12:49** · And then you have some other tasks as well that are a little bit more on the linguistic side.

**12:53** · I think they're less trendy now, but I guess 10 years ago it was something that people would study a lot.

**13:00** · So part of speech tagging, which is about just figuring out which word is a noun, a verb, et cetera, or some parsing-related tasks, so dependency or constituency parsing.

**13:13** · And then the last bucket, which is very popular these days is the generation bucket.

**13:21** · So you have the text as input, and you also have text as output.

**13:28** · And here the length can be variable, meaning you don't know what the length of your output text will be beforehand.

**13:35** · So here you have several tasks.

**13:37** · So for instance, you have machine translation.

**13:39** · So for instance, something in English and I wanted to let's say German.

**13:44** · Question answering-- so typically the ChatGPT, Gemini that you're using, the assistant.

**13:50** · So you ask a question and you have a response.

**13:54** · And then you have other tasks as well, like summarization, you want to summarize an article, let's say, or just generate something.

**14:02** · So something can be generate codes, generate a poem, can also be a lot of things.

**14:11** · Cool.

**14:11** · So now what we will do is go through these tasks one by one to just illustrate what people typically handle with.

**14:21** · So we're going to start with the first bucket, which is the classification bucket.

**14:27** · And here we're going to illustrate this with the sentiment extraction task.

**14:32** · So let's suppose we have a sentence, "this teddy bear is so cute."

**14:37** · We want our model to predict this to be a positive sentiment.

**14:43** · So typically what you would use is sentiment extraction data sets.

**14:48** · So I mentioned movie reviews, so this is IMDb critiques.

**14:52** · But you also have reviews about products, so Amazon reviews or tweets.

**14:58** · Now I guess it's called X, so X posts.

**15:03** · And the way you would evaluate such outputs would be by typically using traditional classification metrics.

**15:13** · So you have accuracy, which is what is the percentage of the observations that you correctly predicted.

**15:21** · But you also have two key metrics, which I'm just going to remind.

**15:26** · I'm not sure if everyone knows about them.

**15:27** · So one is precision, which is, out of all the positive predictions that you made, which ones were correct?

**15:37** · And then the second one is recall.

**15:39** · Out of all the true labels, how many of them did you correctly predict as being positive?

**15:47** · And you have this metric called the F1 score, which basically takes the harmonic mean of precision and recall to just give you one number.

**15:57** · So now you may wonder, why do you need all these metrics?

**16:01** · So the short answer is that sometimes you have tasks and data sets where your classes are very imbalanced.

**16:10** · So for instance, you can have 99% of your data set that is a positive label, and then only 1% of the data set which is negative.

**16:20** · And so here if you take a metric like accuracy, can be very misleading.

**16:26** · Because if you have a model that would predict everything as the majority class, then you will have a great classifier, but that's not the case.

**16:36** · So that's why precision and recall really play a role.

**16:41** · So that's for the first one.

**16:44** · So now let's move to the second category of NLP tasks.

**16:47** · So this one is the multi-classification category.

**16:51** · So you have an input text and you predict multiple things.

**16:55** · And we're illustrating this with the NER task, which as I mentioned is about identifying the category of given words.

**17:07** · And so here, for instance, we want to identify a teddy bear as being an entity.

**17:13** · I guess for that, you would use classification metrics, but not at the sentence level, but more either at the token level or at the entity-type level.

**17:26** · And by that I mean, let's suppose you have a category, let's say location.

**17:32** · And you want to know how well you're predicting words in that category.

**17:37** · So you would typically aggregate these metrics as a function of that.

**17:44** · Cool.

**17:47** · OK, let's go to the last category, which is, as I mentioned, the most popular one.

**17:51** · So this one is text in, text out.

**17:56** · So I'm illustrating this with the machine translation task, which is around translating a text from a source language to a target language.

**18:07** · So here you have the example with English to French.

**18:09** · So cute teddy bear is reading, un ours en peluche mignon lit.

**18:15** · So for that, I guess it's harder to get data sets, because here you need to have pairs of texts.

**18:23** · So you have a very popular data set that's called WMT, which stands for Workshop on Machine Translation.

**18:32** · And that one contains a bunch of paired sequences in different languages.

**18:38** · So for instance, you have the English-French, English-German, coming from the European Parliament data set, for instance.

**18:46** · So to evaluate those, to evaluate the performance of your model, it's actually a lot more tricky.

**18:55** · Because, as you can imagine, you can have many different ways to translate something.

**19:01** · I'm sure many of us in the room are bilingual, trilingual.

**19:07** · So that's what is making it this hard.

**19:12** · So in the past, people have used several rule-based metrics to do that.

**19:18** · So one that you may have heard is BLEU.

**19:22** · BLEU stands for Bilingual Evaluation Under Study.

**19:27** · And it is a measure of how well your translation stands with respect to a reference text.

**19:36** · Same story for ROUGE, which is actually a suite of metrics, but captures that in a different way.

**19:45** · And you will see that the machine learning community is funny, because BLEU-- I'm not sure if you know French-- means blue, but rouge means red.

**19:54** · So I guess they tried to add some fun in this.

**20:00** · But the problem with these metrics is that you always need a reference text.

**20:06** · So you basically need labels.

**20:09** · And in practice, having labels is very cost expensive.

**20:16** · It takes a lot of time, a lot of money to get labels.

**20:21** · And we will see later in the class that with the progress that we have made in the LLM space, or that the community has made in the LLM space, we can actually forego of this reference-based metrics and go towards a more reference-free metrics.

**20:40** · And we will see that later on.

**20:43** · And then the last metric that I will say that people sometimes use is called perplexity.

**20:48** · And perplexity only looks at the probabilities that are output by the model.

**20:54** · And it basically quantifies how surprised the model is by its output.

**21:02** · So BLEU and ROUGE, the higher the better.

**21:06** · Perplexity, the lower the better.

**21:10** · And I guess LLMs have been a hot topic since 2022.

**21:19** · But actually, the field goes way back, way before that year.

**21:25** · So in the '80s, we'll see it in a second, but there's a class of models that were actually thought of, even in the '80s.

**21:35** · And the '90s, we had LSTMs that we'll see also in a second.

**21:41** · But the problem was, during that time, we didn't have the internet.

**21:45** · We didn't have a lot of compute.

**21:47** · And I guess this was one of the limiting factors which prevented the models from today from being trained.

**21:56** · And then more recently, we've had several advances.

**22:00** · So Word2vec was really one of the pioneering work in just computing meaningful embeddings.

**22:11** · And we'll see it in a second.

**22:12** · And then, of course, we had the transformers, which were part of a paper that was published in 2017, which is basically at the foundation of all of the models that you see today.

**22:24** · And then these models, they just were scaled up, both by compute, but also in terms of the data that was used to train them.

**22:34** · And that's how LLMs were dubbed.

**22:38** · And I guess these are more like the 2020s.

**22:41** · But yeah, I guess we'll see those.

**22:46** · Cool.

**22:47** · Any questions on, I guess, the high level?

**22:53** · Everyone good?

**22:55** · Cool.

**22:56** · So I guess the first question that I want to ask ourselves is, what we want to do is to have a model that handles text.

### Tokenization

**23:08** · But models, they understand numbers, they don't really understand text.

**23:13** · So we need to somehow do something with that text to make it more quantifiable, something that a model can understand.

**23:23** · So if you look at a sentence, for instance, "a cute teddy bear is reading," you first need to ask yourself, how can you cut this sentence to pass it to a model?

**23:38** · So this part is called tokenization.

**23:42** · And what that entails is basically cutting the text with respect to some arbitrary unit of text.

**23:52** · So there are several ways of doing this.

**23:54** · I guess the first way is doing it completely arbitrarily.

**23:58** · So here, for instance, you would have "a."

**24:00** · That would be one unit of text.

**24:03** · "Cute" could be another unit of text.

**24:06** · "Teddy bear" would be another one, and so on.

**24:09** · And by the way, the unit of text is called a token, which is why the method is called tokenization.

**24:19** · Another way would be to just separate by words.

**24:25** · But I guess we would have always pros and cons.

**24:28** · I guess one of the goals that we want to achieve is for us to then be able to represent these tokens in a meaningful way.

**24:40** · So one con with doing this at the word level is you will end up with words that look similar, but that are actually considered as different tokens.

**24:53** · And I guess the limitation here is you will need to compute embeddings for these similar yet different tokens and somehow make their embeddings similar.

**25:07** · So I'll give you an example.

**25:08** · So let's suppose I have the word "bear."

**25:12** · And then you have another word, plural form "bears."

**25:17** · So these two words, they are very similar.

**25:20** · Just one is singular, the other one is plural.

**25:23** · If we go ahead with the word-level tokenization, then we will end up with just two different entities, which are basically just considered as different.

**25:35** · Same with "run," and then "runs," variations of verbs.

**25:42** · So for that reason, people have dug into a category of tokenizers that are called subword tokenizers, which is around leveraging roots of words in order to find what are the common roots that we can find in these words.

**26:05** · So, for instance, for bear and bears, you would have the bear particle that would be shared.

**26:12** · And so I guess the pro is that you get to leverage the root of the words.

**26:18** · But then the con here is that your sequence would be longer.

**26:24** · And we will see why this is a con.

**26:28** · I guess later on, I guess I can give you a preview.

**26:33** · So the complexity of these models is also a function of the sequence length.

**26:40** · So the more tokens you have to process, the more time it will take for your model to run, because it needs to basically process all these tokens.

**26:52** · So that's one con.

**26:53** · So pro is it leverages the root of words.

**26:57** · Con is it just makes your sequences longer.

**27:05** · You have a last category of ways of tokenizing things, which is just going at the character level, just like taking out characters.

**27:13** · So here, I guess, you and I when we write a message, we typically have sometimes misspellings.

**27:24** · And with the subword way of tokenizing things, you may not be able to recognize the word that has been misspelled.

**27:36** · And this is something that the character-level tokenizer can, I guess, take into consideration.

**27:43** · But here the problem is you have a sequence length that's much, much longer, which will make your model, I guess, take much more time to process the sequence.

**27:54** · So that's one con.

**27:55** · And then the other con is, I guess, when you want to represent each of these tokens, I guess it's very hard to know that a representation of a letter really means.

**28:08** · Like, what does the representation of the letter U mean.

**28:14** · It's very hard.

**28:17** · OK, cool.

**28:18** · So I have just a quick recap.

**28:21** · So word-level is a super naive way, super simple way of, I guess, dividing your text into arbitrary units.

**28:32** · But then the problem is, as we mentioned, we do not leverage the root of words.

**28:37** · And I did not mention this, but there is a term.

**28:41** · Whenever you cut something and then at inference time when you want to make a prediction, I guess, one prerequisite that you have is that you need to have the token that you saw at training time, you need to have it in your training sets.

**29:02** · And the problem is, let's suppose at inference time you cut your text into words.

**29:09** · And let's suppose you have not seen a word at training time.

**29:12** · You will need to mark it as unknown.

**29:16** · And so this thing is called OOV, Out Of Vocabulary.

**29:21** · So luckily, the subword-level tokenizer mitigates that problem.

**29:26** · So you have a lower risk of OOV, but still you can have.

**29:33** · And as we mentioned, in terms of the pro, you leverage the roots of the words.

**29:41** · And then character-level, it's robust to our misspellings and our casing errors.

**29:48** · But the problem is it makes computations just much slower.

**29:52** · And your sequences would be very, very long, which will also make your, I guess, inference time much higher.

**30:03** · That sound good?

**30:04** · I guess this is really the foundation, I guess, how to handle things with text.

**30:08** · But yeah, does that make sense overall?

**30:13** · Cool.

**30:15** · OK, so now what we did is we took an input text.

**30:21** · What we did is we cut it into parts that are basically tokens.

**30:26** · So in order for our model to understand these tokens, we need to find a representation for each of them.

### Word representation

**30:34** · So here, we're going to take a look at this.

**30:37** · So that's called a word representation.

**30:39** · Or I guess, in a more correct way, it should be token representation.

**30:45** · So we want to find a way to represent each of these tokens.

**30:52** · So the simple and naive way to do this would be to just assign the one hot vector for each word or for each token.

**31:03** · So for instance, let's suppose we have a vocabulary of three tokens-- book, soft, and teddy bears.

**31:12** · We would have, let's say, soft.

**31:14** · That is 1, 0, 0 vector.

**31:16** · Teddy bear, that is, let's say a 0, 1, 0 vector.

**31:20** · And book, that is, let's say, a 0, 0, 1 vector.

**31:25** · So this is called a One-Hot Encoding, OHE.

**31:30** · We'll typically see.

**31:32** · So cool.

**31:33** · This is a way to represent our tokens.

**31:37** · But basically what people want to do is compare these tokens to basically see which ones are more similar to what other ones.

**31:50** · So common similarity measure that people use is something called cosine similarity.

**31:56** · I'm not sure if you have heard of it.

**31:59** · So you can think of it as just seeing what angle these vectors make in the n dimensional space.

**32:09** · And if, I guess, they are pointing in the same direction, then maybe they're similar.

**32:16** · Maybe if they're orthogonal, maybe they're independent.

**32:19** · And if they're completely opposite, then maybe they're opposite.

**32:23** · That's basically the mental model we want to go into.

**32:28** · So the problem is, if you represent your tokens in a one hot fashion, you will end up with all your vectors being orthogonal to one another.

**32:41** · So that's the problem.

**32:43** · So ideally, what we want is for tokens that mean the same or similar to basically have a high similarity.

**32:54** · And for tokens that are not similar on about different thing, to be more orthogonal.

**33:03** · So here, just for illustrative purposes, teddy bears are soft.

**33:09** · So you want teddy bear and soft to be, I guess, with a high similarity.

**33:14** · And let's say teddy bear and book, which is independent, you want them to be closer to 0.

**33:21** · So that's what you want.

**33:22** · That's what you have with one-hot encoding, and that's what you want.

**33:27** · Yeah.

**33:31** · Sorry?

**33:38** · Oh, I see.

**33:39** · The question is, why do you care about the norm?

**33:44** · So I guess cosine similarity is actually normalized by norms.

**33:50** · So it's dot products.

**33:53** · Oh, you mean why did I just put dot product here instead of 2?

**34:01** · Oh, I see.

**34:01** · And your question is, why do we not care about the norm?

**34:06** · Cool, I guess the viewers know the question.

**34:08** · I guess these measures, they are all measures.

**34:12** · They are all ways to try to capture these similarity things.

**34:20** · So I guess why do you not care about the norm?

**34:25** · I guess it's how people have tried to quantify that.

**34:29** · I guess you will need to see how your vectors are trained and whether the norm would be indicative of something.

**34:39** · I guess the best answer I can give you is, I guess, this is a measure.

**34:42** · This is not the perfect measure.

**34:46** · People may use also dot product as a measure, but yeah, I don't have a great answer for you.

**34:53** · But as long as you capture, I guess, how these vectors they're pointing, I guess, typically what you care about is the angle between them.

**35:03** · But typically, you don't really take into consideration the norm.

**35:10** · Cool, any questions?

**35:11** · Any other questions?

**35:13** · Yeah, yeah, yeah, yeah.

**35:39** · It's a great question.

**35:40** · So the question is around size of vocabulary and how that would inform the choice with respect to word, subword, and how that changes across languages.

**35:48** · So great question.

**35:50** · So I would say it really depends, first of all, on the tasks that you're trying to achieve.

**35:54** · If your task is just about one language, you will just take that same language.

**35:59** · You would typically go with a subword tokenizer just because of the reasons that we mentioned here.

**36:07** · So I guess subwords is a nice trade-off between being able to identify words by their roots, like leveraging that, but also running less into the OOV risk.

**36:23** · So in terms of the size, I know that people have tried different things.

**36:31** · I think typically for English, you would target something on the order of tens of thousands of vocabulary size.

**36:40** · But nowadays, the models, they are multilingual, they are also about codes.

**36:46** · So you will see that the vocabulary size now is sometimes on the order of hundreds of thousands.

**36:52** · So with respect to Chinese, so I guess you have this difference in characters that you're using.

**37:00** · So for Latin, I guess it's the alphabet we're all accustomed to.

**37:03** · But of course for the other ones, you'd have something similar, but in I guess the target language character.

**37:13** · So yeah, I would say order of magnitude, tens of thousands for one language, hundreds of thousands if it's multilingual.

**37:22** · These are the order of magnitude that you want to target for.

**37:28** · Cool, yeah.

**37:41** · Great question.

**37:41** · So the question is, how do you get those embeddings?

**37:44** · So it's actually the next slide.

**37:45** · So I'm going to talk about this.

**37:49** · Cool, great.

**37:52** · So now that we know that the one-hot encoding is not a good way to represent tokens, what we want to do is to learn those embeddings from the data.

**38:06** · So I mentioned that there was this paper that came out in the 2010s-- so I think it was 2013-- that was called Word2vec.

**38:15** · And the reason why it was so popular is because they showed a very intuitive and interpretable way of seeing these embeddings, because they were saying something like, OK, king is to queen, what this is to that, like Paris is to France what Berlin is to Germany.

**38:33** · So there was basically a way to make sense of the embeddings.

**38:39** · So now the question is, how did they do that?

**38:43** · So they had two ways of computing these embeddings.

**38:47** · So one way was called continuous bag of words.

**38:51** · The other one was called skip gram.

**38:53** · But they all rely on the same idea, which is let's just leverage texts that we have, and then try to predict something that is part of the text, based on, let's say, the context.

**39:10** · So for instance, continuous bag of words, the goal is you take into consideration the words that are around a given target words.

**39:20** · And your goal is to predict that target word.

**39:25** · And skip gram is the opposite.

**39:27** · You go from a target word and you want to predict the words that are around it.

**39:34** · So I guess this task is commonly called a proxy task.

**39:39** · Because at the end of the day, in this exercise, what we care about is not necessarily to predict the next word, or at least not yet.

**39:49** · Our goal is to learn a representation of these words that are meaningful.

**39:56** · And so here the idea is, if you have a model that somehow knows how to predict, let's say, the next word, then it means that your model has some understanding of how language works, which is basically what you want.

**40:16** · You basically want an embedding that is reflective of, I guess, what language is, which is king and queen, or similar, Paris and France, this is a capital.

**40:32** · You want to have these associations embedded in the representation.

**40:39** · And let's go through a very simple example of what that looks like.

**40:47** · So here in our example, let's suppose that our proxy task is about predicting the next word.

**40:56** · So here what we take is a very vanilla neural network model, which basically receives a vector of size v, has multiplication and a bias term to get a hidden state, and then another set of multiplications to get our final vector.

**41:23** · So here it's basically a very simple neural network.

**41:28** · So the input is of size v. The hidden layer is of size d, which is typically much smaller than the vocabulary.

**41:37** · So vocabulary is typically like tens of thousands or hundreds of thousands.

**41:40** · So d is typically hundred.

**41:43** · Like, 768, for instance, is one example of dimension.

**41:50** · So it's much, much smaller.

**41:52** · So what we're trying to do is to learn the word representation through this proxy task.

**41:59** · And what we're going to do is try to consider the words as inputs and predict the next word.

**42:12** · So let's go with the first word of the sequence.

**42:16** · So by the way, I use token and words interchangeably.

**42:21** · So let's suppose we have the word "a," and we want to predict the next word, which is the word "cute."

**42:29** · So what we do is we take the word "a," we take the one-hot encoding representation, and we pass it through the network.

**42:42** · So here, if you're familiar with neural networks, so here you have, I guess, a multiplication between a matrix and this vector.

**42:51** · So you have a hidden state representation, which is a vector of size d.

**42:58** · So here, let's suppose it's 0.2 and 0.9, so D equal 2.

**43:04** · And then you have, I guess, another pass here.

**43:07** · And then you get, after softmax, a set of probabilities which are around seeing what is the next word.

**43:17** · So in this example, we have a vocabulary of size 6.

**43:24** · So the first word is predicted with probability 0.2, second word, 0.4, and then the other words are all 0.1 in this example.

**43:36** · So let's suppose that we want to somehow be able to maximize our prediction to be the second word of the vocabulary, which is the 0.4.

**43:49** · So we basically compare the prediction with, I guess, 0, 1, 0, 0, 0, which is the representation of the second word of the vocabulary.

**44:01** · And then we do the back prop, we update the weights.

**44:06** · I'm not sure if everyone is familiar with that part.

**44:09** · But the idea here is, once you obtain a prediction, you compute the loss, so typically cross-entropy, which will determine how far off you are from the true answer.

**44:25** · And based on that difference, you're going to update the weights in order to make your prediction closer to the truth.

**44:36** · So that's what you do.

**44:37** · And then you repeat that process.

**44:39** · Let's suppose you take the word "cute," which, as we said, is the second word in the vocabulary.

**44:45** · So the one-hot encoding representation is 0, 1, 0, 0, 0.

**44:51** · So you go through that network, you have a hidden state, like the vector is 0.8 and 0.4.

**45:00** · You do that again.

**45:01** · And what you want to do is to predict the next token, and here is teddy bear.

**45:08** · And so you see now your model in this example is predicting the next word to be uniform, but you want to somehow maximize the probability for teddy bear.

**45:19** · So you go about doing this again and again for all the words.

**45:23** · And at the end of the day, you obtain a model that learns how to predict the next word, which is basically the proxy task.

**45:34** · And what you're going to do is to take the representation that the model learns, which is the green units.

**45:43** · So what happens now is every time you have a word, you just represent that as a one-hot encoding representation.

**45:54** · And you just multiply this with these weights, and then you obtain the green representation.

**46:02** · And that is your word representation.

**46:09** · Does that make sense?

**46:10** · Yeah, yeah.

**46:33** · Yeah, great question, great question.

**46:36** · So the question is about what does v correspond to and why there's only six.

**46:39** · So yes, in this example we only have six possible words, which is basically the vocabulary size, just like a very toy example, because in practice there is many more.

**46:53** · So I guess that's one of the challenges with language.

**46:57** · So you can technically have many variations of words, which is why if you take a word-level way to divide your text into tokens, you can end up with the vocabulary that's very big, because you need to account for all the variations of given words.

**47:15** · And the other thing that I want to point out is, let's suppose you have a vocabulary size of 6, and it's the six words that you saw at training time.

**47:24** · But what happens if at inference time you have a word that you have not seen at training time?

**47:31** · And so the answer for that is typically what people do is they reserve a spot for what they call an unknown token or out-of-vocabulary token, which is basically you can think of it as a bucket for everything that we were not able to identify.

**47:55** · So if let's suppose at inference time you have a token that you were not able to identify, they will all take that representation, which is the unknown token representation.

**48:06** · And this is, by the way, something that I guess the word-level tokenizer has trouble to do, because you will have a much bigger chance of having out-of-vocabulary tokens.

**48:21** · Subword level will have a lower chance.

**48:23** · And then character level, I guess, you don't have that problem.

**48:28** · Does that answer your question?

**48:30** · Yeah.

**48:31** · Cool, yeah.

**48:53** · Great, great question.

**48:54** · So first question is, when you're done?

**48:57** · So the thing with the proxy task is when you train your model, I guess your true objective is to not really learn-- I mean, in this case-- to learn how to predict the next word.

**49:08** · Your objective is to have meaningful representations.

**49:11** · But what you can do is to somehow track the loss function for the proxy task that you're pursuing, but then also taking into consideration that this is not necessarily your end goal.

**49:24** · So I guess one very reasonable way of going about doing this is just to wait until your model converges.

**49:32** · So here, what you do is you track the loss as a function of-- so there's this term "epoch," just how many times your model sees the training set.

**49:42** · And so you compare these different curves.

**49:45** · And when this converges, this is typically a good time to stop the training process and just see if that makes sense.

**49:54** · Depends on your downstream task, of course.

**49:57** · But that's one.

**49:59** · So your second question-- sorry, can you repeat the second question?

**50:04** · Yeah, yeah, oh, great question.

**50:18** · So the question is, how do you know when the generation stops?

**50:23** · I guess, otherwise it will never stop.

**50:26** · So yeah, exactly.

**50:27** · So you have some special tokens.

**50:29** · Typically, you have end of sequence, end of sequence.

**50:33** · So typically when you have the end of sequence token generated, then it's when it stops.

**50:40** · All right.

**50:41** · So second question was what informs the size of the hidden layer?

**50:48** · I would say it's a trade-off, because you want the embedding to be rich enough that it can be informative for your downstream task.

**50:59** · So for instance, if you want to somehow get an embedding of, let's say, your sentence, and if you want to, let's say, do a very, very specialized task, like with a lot of different outcomes, maybe you want a vector that recaptures that, so maybe you want a bigger vector, but if you had a very simple task, maybe a smaller vector might make sense.

**51:22** · So I guess the size of your hidden dimension also impacts the complexity of whatever you're running after.

**51:31** · Because of course, if you have longer vectors, you'll have more computation, so your inference will be probably more expensive, et cetera.

**51:39** · So I guess there's a lot of factors.

**51:42** · So I guess just to recap, one is how complicated your downstream task is.

**51:46** · Second one is how sensitive are you with latency, cost, all these things.

**51:52** · So it's really a trade-off.

**51:54** · But out there you would typically see embeddings of a pool of hundreds or thousands.

**52:02** · Of course, these models, they've been growing, so this number may change.

**52:06** · But that's the order of magnitude that you're looking at.

**52:11** · Right.

**52:12** · This is indeed empirical.

**52:14** · Yeah, yeah.

**52:15** · I guess you can also rely on what others found and just go from that.

**52:19** · But 768, these numbers are things that people typically take.

**52:27** · Cool, yeah.

**52:47** · Great question.

**52:47** · So the question is, how can you distinguish words that are spelled the same but in different contexts?

**52:55** · So you're way ahead of me.

**52:57** · So this is basically the basics.

**52:59** · And we're going to tackle methods that can tackle these problems of just contextualizing the word in the sentence.

**53:08** · So yeah, so we'll see that in a bit.

**53:14** · Cool.

**53:17** · I'm not on time.

**53:19** · So I'll try to get moving.

**53:21** · So OK, so now what we did was see how we could learn representations of tokens.

### Recurrent neural networks

**53:31** · But I guess you may also want to get representations of sentences or pieces of text.

**53:41** · So one very naive way to do that with what we saw before is to take something like the average of words, let's say, the word representations.

**53:53** · But the problem is you lose a lot of meaning, you lose the order, you lose-- and I guess here, I think you pointed out very well, the representations that you learn are token-specific, regardless of where they're at.

**54:09** · So that's why we have a class of models that aim at capturing the sequential nature of how text appears.

**54:20** · So we're going to talk about RNNs, which stands for Recurrent Neural Network.

**54:26** · So what RNNs do is, instead of processing words one at a time, what they do is they keep a hidden representation of the sentence so far, and they consider tokens one at a time.

**54:46** · So as I mentioned before, this technique was actually introduced a fair amount of time ago, so in the '80s.

**54:55** · And what this model does is it takes into consideration the order at which words appeared or tokens appeared.

**55:05** · And so in this example, you start the, I guess, processing at the very beginning of the sentence.

**55:13** · You have some dummy hidden states that is called A, typically denoted A or H. It's called the hidden state, activation, or even sometimes a context vector.

**55:26** · And you have some kind of a module that takes into account the hidden state so far and the word at time step t, so here time step 1.

**55:41** · So here what it does is it takes in the meaning of the sentence so far and takes into consideration the word that is happening now.

**55:52** · And it produces an output vector that here can be used to try to predict the next word.

**55:59** · So for instance here we have this hidden state and the representation of the word that then you have some matrix multiplications in this blue box.

**56:12** · And you have an output vector that you try to train on predicting the next words.

**56:20** · And then you keep on doing that by keeping track of these hidden states.

**56:33** · And so you repeat the process.

**56:35** · And I guess the way you would interpret these hidden states is it's a representation of the sequence process so far.

**56:49** · So the good thing with RNNs is now the word order matters.

**56:56** · And you're also able to encode the sentence in a more natural way.

**57:04** · So let's see roughly how it works.

**57:07** · So we have the same favorite example, so cute teddy bear is reading.

**57:12** · So you would have the token A. You want one-hot encoding vector.

**57:18** · You pass it through your network.

**57:20** · You compute the hidden state.

**57:22** · You try to predict cute.

**57:25** · But then you keep track of the hidden state.

**57:30** · And then you input that into another module.

**57:34** · And then you also consider the next words.

**57:37** · So you consider not only the word itself, but also the hidden state of this sentence so far.

**57:43** · And you try to predict the next word again, and again and again.

**57:51** · So this is RNN.

**57:54** · So RNNs were used for a bunch of tasks, and just like mapping that back to the categories that we saw before.

**58:04** · For classification purposes, you can basically use the hidden state of the last word in your sentence.

**58:14** · For instance, if you want to predict like the sentiment of a review, you would take basically the last vector here and try to project it into the space of the predictions or the labels that you want to predict on.

**58:28** · So for instance, if you want positive or negative, you basically project that vector onto that space.

**58:33** · You can do that here.

**58:34** · For multi-classification, so you would basically have the representation of the token of interest, and you would project that.

**58:43** · Or for generation, you would basically process the whole source text, and then have a context vector, a.k.a.

**58:56** · activation vector, a.k.a.

**58:58** · hidden state at the end of your processing, which will then be used to decode the output prediction.

**59:09** · So this is how you would use an RNN for each of these tasks.

**59:14** · So the reason why you have not really heard of RNNs these days is because they had some pros, but a lot of cons.

**59:26** · So one of the cons is that the meaning of the sentence is basically solely encapsulated into this hidden state.

**59:39** · So you have this problem of long-range dependencies, which basically impacts your ability to quote, unquote, "remember" what the model saw in the past, which is why you have another class of models that try to build on RNNs.

**59:59** · So this one is called LSTMs, Long Short-Term Memory.

**1:00:05** · And the goal of that extension is to have the way to somehow keep track of the things that are quote, unquote, "important" to remember, on top of the hidden state that we talked about.

**1:00:20** · So here you have a of t, which is your activation, basically the sequence so far encoded in there.

**1:00:29** · And then you have another quantity that you track that is called the cell state.

**1:00:36** · It's denoted c here.

**1:00:39** · So this architecture aims at improving that piece, but I guess it was not perfect either.

**1:00:49** · But yeah, so that was the main issue of RNN-based methods, which is that they have this issue of forgetting what was in the past.

**1:01:01** · So you will see in the literature that this phenomenon is called vanishing gradient.

**1:01:08** · And the reason why it's called that way-- so I know we're running out of time, but I'm going to just explain that part.

**1:01:15** · So in order for you to predict, let's say, the last words, you're basically dependent on every hidden state that came before that.

**1:01:30** · So far so good?

**1:01:32** · And so whenever you want to update the weights of your model to match the prediction here with the actual prediction, when you do the back propagation, you somehow need to take into account that the value here is basically not only a matter of this computation, but also this computation or this computation that basically happened in a sequential manner.

**1:02:03** · So you have this phenomenon of trying to, I guess, back-propagate through time.

**1:02:13** · But the problem is, in practice when you write that down-- so it's a very ugly formula.

**1:02:19** · But when you write that down, it ends up being a product of a bunch of quantities that can-- so if it's greater than 1, then it's exploding.

**1:02:32** · If it's less than 1, it's vanishing.

**1:02:34** · Because if you multiply a lot of things that are less than 1, it just goes to 0.

**1:02:38** · So I guess if you have something that you're trying to update that goes to 0, basically you have trouble just doing your updates.

**1:02:46** · So that's a high-level intuition.

**1:02:49** · This is not the focus of this class, which is why I'm not going into the detail of this ugly formulas.

**1:02:54** · But I hope you get the idea that for remembering things from the past, it's not doing a great job, because of this sequential, I guess, characteristic.

**1:03:10** · Does that make sense?

**1:03:13** · OK, I hope the next thing will make a bit more sense.

**1:03:16** · But before that, I'll just recap what we saw.

**1:03:19** · So our goal is to represent text.

**1:03:24** · So we first started with representing words or tokens, which was what we tried to do with Word2vec.

**1:03:33** · And we saw that it was a good way to leverage proxy tasks to learn this representation.

**1:03:39** · But we had a bunch of limitations.

**1:03:42** · And one of them that you mentioned was that this was not aware of the context.

**1:03:48** · And also the word order didn't count.

**1:03:51** · And so you have this other class of methods that is able to take into consideration the words, but then they have some trouble keeping track of things when the sequence gets very long.

**1:04:07** · And you have this problem of vanishing gradients or long-range dependencies.

**1:04:12** · So whenever you see this term, it's basically referring to that.

**1:04:16** · And also another thing that I have not mentioned, but the computations are very slow.

**1:04:22** · So when you want to train these models, at training time, in order to predict this word, you basically need to compute all these hidden states before.

**1:04:34** · So when your sequence gets very long, it just takes a very long time.

**1:04:46** · So for all of these reasons, for I guess what reason.

**1:04:51** · So for the fact that the model has trouble remembering things from the past, people have tried having more direct connections between something and the thing from the past, and this is the idea behind attention.

**1:05:14** · So what attention does is it tries to have a direct link between what we're trying to predict and something from the past.

**1:05:26** · So in this example, let's suppose I'm trying to translate an English sentence into a French one.

**1:05:34** · So here I guess the input sentence is given.

**1:05:38** · I'm computing the hidden state.

**1:05:40** · I'm processing words one at a time.

**1:05:41** · This is my traditional RNN.

**1:05:43** · So "a cute teddy bear is reading."

**1:05:45** · So here I have a hidden state that I'm then decoding.

**1:05:50** · And you can imagine that when wanting to generate the next word of my translation, it would be great if I knew what word I'm trying to predict.

**1:06:07** · Or in other words, it would be great if I could take a peek at a certain area of the input text.

**1:06:16** · So the idea behind attention is to have a direct link between what you're trying to predict and things before.

**1:06:26** · This is the idea behind attention.

**1:06:29** · And so it was introduced in 2014.

**1:06:32** · And yeah, again, this is trying to solve for these long-range dependency issues.

**1:06:42** · And so yeah, this example we want to do that.

**1:06:45** · And this concept is going to actually be key for this class.

### Self-attention mechanism

**1:06:51** · Because we're going to see that the attention mechanism is the thing that is going to make everything-- I mean, most of the things work.

**1:07:02** · And this is actually the main principle that the transformer paper relies on.

**1:07:08** · So the transformer, which is the core architecture that we will see in this class, has been introduced or was introduced in 2017 in this paper named Attention is All You Need.

**1:07:22** · So even from the title, you can see that the authors wanted to just rely on that part.

**1:07:28** · So what the authors tried to do was to move away from this sequential way of processing the text, and instead let the model just have direct connections with all parts of the text at once.

**1:07:49** · So that is called self-attention.

**1:07:52** · So they tried that on translation tasks.

**1:07:55** · And they just realized that it was giving great results.

**1:08:00** · So back to the example that we're still using, a cute teddy bear is reading Here what we would say is that in order to compute the representation of the token teddy bear, we're going to look at all the other tokens in the sequence at once, and directly with direct links.

**1:08:27** · So I guess back to your question-- here, we would have a representation of teddy bear that would be unique to the context that it is part of.

**1:08:36** · So back to your question about riverbank and robbing a bank, like here the bank would have different representations.

**1:08:46** · So this is the idea.

**1:08:49** · I guess, does the idea roughly make sense?

**1:08:55** · And again, this is called the self-attention mechanism.

**1:08:59** · Afshine, how am I doing on time?

**1:09:02** · 8.

**1:09:03** · I have 8?

**1:09:04** · OK, cool.

**1:09:06** · OK, so this is the idea.

**1:09:09** · So now I'm going to just introduce another set of ideas which is more terminology but is going to be very important.

**1:09:17** · So when you want to express something in terms of something else, we use the words "query," "key," and "value," Q, K and V.

**1:09:30** · So in this example, our goal is to figure out what other tokens is the query teddy bear more similar to.

**1:09:43** · So here the question is, OK, you have a query and you want to see what other tokens are most similar.

**1:09:52** · And so what you're going to do is to look at all the other tokens which are basically composed of keys and values.

**1:10:03** · So we're going to compare the query to the key to quantify how similar your query is to a given key, and take the corresponding value.

**1:10:16** · So we'll see that in this example, so let's suppose you want to express teddy bear in terms of everything else.

**1:10:21** · What you're going to do is you're going to take the query teddy bear, and you're going to compare that query with all the other keys to see which element is most similar, and then weight the more similar ones and take their associated value.

**1:10:41** · So that's a very high level idea of how these things are.

**1:10:48** · Of course, we're going to see exactly how they work.

**1:10:50** · But that's the general idea.

**1:10:55** · OK, cool.

**1:10:56** · And speaking of query and key and value, we will also see that one benefit of expressing things this way is that we can express doing this self-attention computation across the whole sequence in a matrix format.

**1:11:16** · And GPUs love matrices.

**1:11:20** · So it's really made for the hardware that we have.

**1:11:24** · And I guess what I mentioned here can be expressed in a form of softmax of the query and the key, which is basically a way to get some kinds of weights of which values will be more important.

**1:11:40** · So for instance, if a value is more important, you have bigger weight.

**1:11:43** · And another one will be less important, you'll have a smaller weight.

**1:11:46** · And you basically multiply that by the value.

**1:11:51** · So don't worry, we'll have a detailed example after.

**1:11:55** · So if it still feels very high-level fuzzy, don't worry.

**1:12:00** · We'll have a detailed walkthrough.

**1:12:05** · And yes, so this is how it works.

**1:12:08** · OK, cool.

**1:12:11** · Any questions on what self-attention is?

**1:12:16** · Yeah.

**1:12:40** · Great, great question.

**1:12:41** · So the question is, what is value, what is key?

**1:12:44** · I guess, how to get those?

**1:12:45** · What do they mean?

**1:12:47** · So first of all, I just want to say that these quantities, they are learned.

**1:12:51** · So you are not fixing them.

**1:12:53** · But from an interpretation standpoint, you can interpret that the key is there for you to figure out which one is most similar to the query.

**1:13:04** · And the value is the actual value that is associated with that element.

**1:13:10** · So here, you will have something like you want to express this in terms of all the values.

**1:13:18** · So the weights in your weighted average will be basically the dot product between-- basically-- between the query and the key.

**1:13:26** · And the value will be the actual vector that you will use.

**1:13:31** · But again, these things are learned.

**1:13:33** · And something I have not mentioned, but you mentioned it correctly, so we're going to actually do projections to obtain these quantities.

**1:13:40** · And these projections are actually learned by the model.

**1:13:48** · That's good?

**1:13:50** · OK, cool.

**1:13:52** · So with that, we have 15 minutes, right, to talk about the architecture.

### Transformer architecture

**1:14:01** · OK, so at a very high level, in order to make the self-attention mechanism happen, the authors propose an architecture that is composed of two parts, an encoder, which is on the left side, and a decoder, which is on the right side.

**1:14:26** · So the application that they have is translation.

**1:14:29** · So what will go through the encoder is the input text in your source language.

**1:14:37** · And what is going to go through the decoder is the target language that you're predicting.

**1:14:43** · So the high-level idea is you're going to compute meaningful embeddings from your input text by passing them through the encoder.

**1:14:57** · And you want that self-attention mechanism to apply, meaning you want to compute representations of each token as a function of others.

**1:15:07** · And you do that by using a layer called the attention layer.

**1:15:13** · So multi-head attention layer, but multi-head is just doing this computation in different ways to just allow the model to learn different representations or different projections.

**1:15:25** · But the idea here is you are going to input your input text, and all the tokens in your input text are going to attend to one another.

**1:15:40** · So for instance, a cute teddy bear is reading, you are going to compute the representation of all the tokens in this text basically as a function of others.

**1:15:52** · And you're going to do that with the encoder, so here with the multi-head attention.

**1:15:57** · And then you have a feedforward layer, which is just to let the model learn another kind of projection.

**1:16:05** · And what you're going to obtain at the end of your encoding process is rich representations of the tokens from the input sentence.

**1:16:19** · So far, so good?

**1:16:21** · But now your goal is to actually translate the input sentence.

**1:16:26** · So what you're going to do is to start your translation with, let's suppose, the beginning of sentence token, so your first token.

**1:16:37** · And what you're going to do is use all the representations from your input sentence in order to figure out what to predict next.

**1:16:50** · So this what I just said is the cross-attention layer, which is the one that is the second, so this one, which basically-- I'm not sure if you see the arrows, but there are two arrows coming from the encoder, one arrow coming from the decoder.

**1:17:10** · Can anyone tell me what the arrow from the decoder represents?

**1:17:14** · Is decoder a query key or value?

**1:17:22** · I guess there is 1 over 3, 33% chance.

**1:17:27** · Who wants to try?

**1:17:30** · Is it key?

**1:17:31** · OK.

**1:17:35** · Query?

**1:17:36** · OK.

**1:17:36** · So the way to think about it is you're trying to ask yourself, what are the words from the input that matter.

**1:17:48** · Right?

**1:17:49** · So basically, you want know, given your query, what are the elements from the inputs that matter?

**1:17:58** · So here, this arrow is indeed the query, because this is the thing that you want to figure out.

**1:18:05** · And the keys and values are actually coming from the encoder, which are basically coming from the input sequence.

**1:18:14** · And then you have another attention layer, which is this one.

**1:18:17** · And that one is trying to figure out what other tokens of the output sentence that you're decoding is going to be useful to predict the next token.

**1:18:32** · So let's suppose you start decoding and you say, un ours en peluche, which is in French.

**1:18:40** · To predict the next word, you want to basically figure out what are the tokens translated so far that are going to be useful to predict the next word.

**1:18:51** · So this is what this attention layer is about.

**1:18:54** · And it's called masked, because it only looks at the tokens that translated so far.

**1:19:03** · It does not look at tokens that were not translated, because of course they were not translated.

**1:19:08** · So there's no way, like on the right side of the token that you're trying to predict.

**1:19:15** · Cool.

**1:19:17** · So at a very high level, you have this attention layer, which is present in the encoder, which is present in the decoder, but it has several, I guess, use cases.

**1:19:28** · So the attention layer here aims at computing embeddings from the input sentence as a function of themselves.

**1:19:42** · And then the ones from the decoder, so the first one, the masked self-attention layer, aims at expressing something as a function of everything that has been decoded so far.

**1:19:56** · And the second one, the cross-attention layer, tries to express things as a function of what has been seen in the input.

**1:20:05** · So here, given that you are having direct links to different tokens, you don't have this sense of order.

**1:20:17** · Because in the RNN, you are basically expressing things one at a time.

**1:20:23** · So you had some sense of the word order, but here you don't have it, because it's like a direct link, which is why you have position encodings, which are there to inform on the position of the word in the sequence.

**1:20:40** · So we're not going to dig into that today, but I just want to call that out.

**1:20:46** · So at a very high level, and we're going to see this in the detailed example, what we do is in order to translate a sentence from source language to target language, we're first going to tokenize the text, so dividing into arbitrary units.

**1:21:03** · We're going to learn an embedding for these tokens.

**1:21:07** · So this is what the input embedding is about.

**1:21:09** · Then we're going to add some encoding with respect to the position.

**1:21:14** · We're not going to talk about it today, but just good to note.

**1:21:19** · And then we go through the encoder.

**1:21:22** · So the encoder tries to figure out how to express things as a function of other things from the inputs.

**1:21:30** · So it does that in the multi-head attention layer.

**1:21:35** · And then it goes through a feedforward neural network, which is just a way to just project the vectors to just have some more degrees of freedom to learn things.

**1:21:48** · And then once you have these representations from the input, you're then going to start your translation.

**1:21:56** · So you start with the BOS token.

**1:21:58** · And what you're trying to do is figure out what the next word is.

**1:22:03** · So you're going to see, OK, what are the words that were translated so far that are useful for translation.

**1:22:09** · So this is what the masked multi-head attention layer does.

**1:22:14** · And then you have another attention layer, which is about expressing things as a function of what was in the input, which is the cross-attention layer over there.

**1:22:27** · And then you have a feedforward neural network to, again, give some more degrees of freedom.

**1:22:32** · And at the end of the day, you have a vector that you then go through softmax.

**1:22:40** · And it just is a way for you to guess what is the next word.

**1:22:46** · So you have a vector of vocabulary size.

**1:22:51** · And you're going to use these values to determine what is your next word.

**1:22:58** · Easy, right?

**1:23:01** · Any questions on this?

**1:23:04** · Yeah.

**1:23:09** · All right, that's a great question.

**1:23:11** · The question is, what does head mean?

**1:23:16** · So I guess I went too fast.

**1:23:17** · I ignored that part.

**1:23:18** · But when you do the self-attention computation, you basically make queries interact with keys, and then take the corresponding value.

**1:23:32** · But nothing prevents you from doing that several times.

**1:23:36** · So the term "head" is given to the projection matrices that you use to obtain the query, key, and value.

**1:23:46** · And when you have several heads, what you're doing is you're allowing your model to learn different projections.

**1:23:55** · So it's basically an additional degree of freedom for your model to learn different associations between your vectors.

**1:24:03** · So it's a great question.

**1:24:04** · So typically it will be noted lowercase h, number of heads.

**1:24:10** · And this is what this corresponds to.

**1:24:13** · Does that answer your question?

**1:24:16** · Cool, very cool.

**1:24:23** · OK, we have a lot to discuss, but here I had the slide actually for this.

**1:24:30** · So this is the multi-heads that you were mentioning.

**1:24:32** · So we're basically running the self-attention computation several times in parallel, again, with different projection matrices that the model learns.

**1:24:42** · So in case you have a computer vision background, it is similar to having multiple filters in your convolution.

**1:24:52** · So it's the same idea, but it's different here.

**1:24:58** · Yeah, great question.

**1:25:07** · So the question is, are the projections different?

**1:25:09** · So we're typically not constraining things.

**1:25:13** · We're just letting the model learn.

**1:25:15** · But in practice, it just tends to learn different ways of saying the same thing.

**1:25:21** · So yeah, typically there is no constraint.

**1:25:23** · Of course, you have papers that dig into how about if you change this.

**1:25:28** · But typically you don't have any constraint.

**1:25:34** · Cool, great.

**1:25:39** · OK, I will just mention one trick, another trick that the transformer authors use.

**1:25:45** · So it's called label smoothing.

**1:25:49** · Who has heard of label smoothing?

**1:25:53** · So one new thing here is in NLP when you want to predict what comes next, there's typically more than one way.

**1:26:06** · When you say, what a great day, what a great lecture, what a great book, what a great-- there's always multiple choices.

**1:26:16** · There's more than one way of filling that gap.

**1:26:19** · So label smoothing is a technique that tries to intuitively address that.

**1:26:25** · And what it does is, instead of saying predict this word 100% is this one, there is no other words, what it does is it says, predict this word, but there's a chance it's not this word.

**1:26:40** · And in practice, what it does is it takes the one-hot encoding.

**1:26:44** · And instead of saying it's a 1, 0, 0, 0, that you need to predict, it says it's actually 1 minus epsilon, and then epsilon over v minus 1, I guess, is you're trying to predict.

**1:27:00** · So in practice, it's a method that tends to make your model be more unsure.

**1:27:09** · Again, to be less sure about this prediction because you always tell it, OK, try to predict this, but actually it's possible it's not the correct value.

**1:27:19** · But in practice, the authors see that it tends to improve metrics like BLEU, which is a proxy metric for translation tasks.

**1:27:31** · So yeah, I think this method is pretty general for NLP.

**1:27:35** · So yeah, it's a good one to know.

**1:27:37** · And with that, I think there's about 20-ish minutes left.

**1:27:42** · So yeah, Shervine is going to walk you through an end-to-end example.

**1:27:48** · And with that, you-- yeah, Oh, u?

**1:27:59** · So I guess here you can think of this as, I guess, some quantity.

**1:28:06** · It's not defined, so some quantity.

**1:28:09** · And the delta is like one hot, if you want, something like this.

**1:28:18** · It can also be a constant.

**1:28:20** · Yeah, yeah.

**1:28:32** · So question is, is there a relation with explore and exploit?

**1:28:38** · It's an interesting one.

**1:28:42** · So why would softmax give it for free, by the way?

**1:28:48** · Right, but I guess it would still-- so I guess at the end of the day, what you're trying to do is to compare your prediction with respect to the label.

**1:29:00** · So I guess the question here is, do you want to compare with 1, 0, 0, 0, or do you want to compare with something that is not 1, 0, 0, 0.

**1:29:09** · So I guess softmax does not allow you to do that.

**1:29:14** · Exactly, yes.

**1:29:15** · So I'm not sure if this was super clear, but this is actually the label.

**1:29:18** · So what you're trying to predict is not 1, 0, 0.

**1:29:21** · But we change the label in a way that makes the model, I guess, predict something that is less sure, I guess.

**1:29:30** · Cool, thanks.

**1:29:31** · And with that, yeah, Shervine.

**1:29:34** · OK, great.

**1:29:34** · Thank you, Afshine.

**1:29:35** · And yes, so we saw basically how the transformer worked.

**1:29:39** · And now we're going to piece it all together with the one specific example.

**1:29:46** · OK, great.

**1:29:47** · So let's take our favorite example again.

**1:29:49** · So a cute teddy bear is reading.

**1:29:51** · And then we'll go all together through each step.

### Detailed example

**1:29:57** · So first we start with tokenization.

**1:30:01** · So as we said, we can use any arbitrary decomposition to decompose this into tokens.

**1:30:10** · And then as someone mentioned, you need to have some way to indicate the start and the end of a sequence.

**1:30:17** · So typically, this is done with the BOS and EOS tokens.

**1:30:21** · So you add them.

**1:30:25** · So now let's focus on the composition of each token representation.

**1:30:30** · So you have its embedding.

**1:30:32** · That is learned.

**1:30:33** · And as Afshine mentioned, in order to have an idea of what is the position of the words or the token, I should say as part of the sequence, you have some added information that is in the form of a position embedding.

**1:30:47** · And here, the original paper uses the convention of some sines and cosines that it adds additively to the representation.

**1:30:58** · So it's like an element-wise addition.

**1:31:01** · OK, great.

**1:31:02** · So now you have a position-aware embedding for your token.

**1:31:07** · And you repeat that for each of your tokens.

**1:31:11** · So now you can see all of these embeddings in the format of a matrix, which is of size d model, which is the size of your embeddings.

**1:31:20** · And then the other dimension is the length of the sequence, so typically n.

**1:31:29** · So all make sense so far?

**1:31:31** · Any questions on the inputs?

**1:31:34** · OK, great.

**1:31:36** · So now we will send this representation through the encoder.

**1:31:41** · So as Afshine said, you have this concept of self-attention.

**1:31:46** · And the way you perform self-attention is that you take this input and project it on three spaces.

**1:31:54** · So you project it to the space Wq, you get queries.

**1:32:00** · You project the same embeddings in the space Wk, you get keys.

**1:32:06** · And you do the same for values, you get your values.

**1:32:09** · And then Wq, Wk, and Wv are learned by the model.

**1:32:14** · They are basically projection matrices.

**1:32:19** · So far, so good?

**1:32:21** · So now with all of that in mind, you can apply the formula that Afshine mentioned, that is the self-attention formula, which is softmax of Qk transpose over square root of dk times v, which gives you another matrix out of all of this.

**1:32:42** · Now let's pause for a second and look at how this computation is done in practice and what every step means.

**1:32:52** · So let's look at Q. When you compute Q, basically you project your embeddings into that space.

**1:32:59** · What do you obtain?

**1:33:00** · You obtain a matrix where each row represents a given query.

**1:33:07** · When you say k transpose, it's basically the same matrix, but transposed, where each column represents the key representation of each token.

**1:33:19** · Now let's mix them together with the matrix multiplication.

**1:33:25** · So when you multiply each of them, you see that each row represents the projection of the query over each key, such that when you take the matrix multiplication and get the softmax of all of this, you get a probability distribution of the projection of the query over keys for each query.

**1:33:51** · Each line will have this.

**1:33:55** · And I don't know if anyone asked the question regarding why do we scale by square root of dk?

**1:34:03** · So it could be dq as well because matrix multiplication, like the dot product here, enforces the fact that dq equals dk.

**1:34:12** · And basically, what you see is that these dot products has the dimension of key and queries grows, it will tend to grow as well.

**1:34:20** · So you want to normalize these dot products.

**1:34:23** · And this is why you divide by square root of the dimension of keys.

**1:34:30** · OK, great.

**1:34:31** · And then now you have your softmax of all of these, and then you multiply it with the matrix v. And this is Afshine explained as having the query projected on the space of keys, and then multiplied by the corresponding value.

**1:34:49** · So the value is the representation of the corresponding key that we project on.

**1:34:55** · So you end up with a weighted sum of values for each query.

**1:35:03** · OK, great.

**1:35:05** · Does that make sense so far?

**1:35:12** · OK, awesome.

**1:35:13** · And someone asked, what is the multi-head stuff?

**1:35:17** · So you're right, it's not just single one, one time that it's done.

**1:35:21** · It's actually done each times.

**1:35:23** · And what you obtain is, like, all of that is done in parallel.

**1:35:28** · And at the end, you obtain h such matrices.

**1:35:33** · And you concatenate them with respect to the columns.

**1:35:37** · And at the end of this, you have another projection matrix that you call Wo that will project all of these back to the original dimension of embeddings.

**1:35:49** · So it's a way for the network to basically have a dimension-invariant way-- bless you-- to go from the original dimension back to the original one.

**1:36:03** · Any questions?

**1:36:05** · Yep.

**1:36:15** · So the question is regarding h.

**1:36:18** · Is it possible to get the same result each time?

**1:36:20** · And if so, concatenate the same thing, will it be helpful?

**1:36:24** · Did I get the question right?

**1:36:28** · So what makes it different?

**1:36:29** · So it's the magic of gradient descent.

**1:36:32** · So the network has an objective function at the end.

**1:36:34** · It has degrees of freedom.

**1:36:36** · Its incentive is to build a representation that will be helpful to learn the next word.

**1:36:42** · So it doesn't have an incentive to copy the same thing or do the same mechanism.

**1:36:46** · And this is why in practice you see the model converge towards building different representations that it can then concatenate and then project into something useful.

**1:36:57** · So what makes it such that you don't have the same thing?

**1:37:00** · Nothing.

**1:37:01** · Like, you don't have any constraints.

**1:37:03** · But the nature of the learning that you let the model have makes it do so in practice.

**1:37:14** · Any other questions?

**1:37:17** · And that is a great question.

**1:37:19** · I mean, typically gradient descent does wonders.

**1:37:25** · OK great.

**1:37:27** · So now that we have gone through the self-attention layer, you have another component that is the FFN.

**1:37:34** · And I think there was a question just here regarding how to choose the dimension of the hidden layer with respect to the input and outputs.

**1:37:41** · So when Afshine mentioned Word2vec, typically you have a smaller dimension than the input and output.

**1:37:48** · But here, actually, the hidden layer is of a bigger dimension than input and output.

**1:37:54** · And the rationale for that is that you want to have enough degrees of freedom for the model to learn useful representations.

**1:38:03** · So it's a way to complexify the features that you learn.

**1:38:06** · And, yeah \[INAUDIBLE\].

**1:38:10** · OK, great.

**1:38:11** · And you don't have just one encoder module.

**1:38:15** · You have actually n of them, big N in the original paper.

**1:38:19** · And at the end of all of this, you have an encoded context-aware set of embeddings.

**1:38:27** · And then each of these setups encoded embeddings will be those that will be fed to the n decoders.

**1:38:35** · So you have like a stacked succession of n encoders.

**1:38:41** · You have a stacked succession of n decoders.

**1:38:43** · And the last representation of the encoder is what you will feed to the cross-attention of each decoder.

**1:38:54** · So yeah, we're going to see that more in detail.

**1:38:57** · So how do we even start the decoding process?

**1:39:01** · So you start with the BOS token.

**1:39:03** · Basically saying to the model, hey, we need to predict the next word, let's start.

**1:39:10** · So what happens to the BOS token at the very beginning?

**1:39:14** · So you feed it to the decoder.

**1:39:16** · And then similarly as before for the encoder, you have a self-attention layer.

**1:39:21** · And as Afshine mentioned, the self-attention layer is causal.

**1:39:27** · So the attention will be done on the same token and the tokens that precede it.

**1:39:33** · So on this first BOS token, you don't see a difference, because it will just attend to itself.

**1:39:38** · But when you have other tokens that you want to decode, you will have this difference in where you attend with respect to decoder.

**1:39:48** · OK, great, goes through the self-attention layer.

**1:39:51** · And then you have what I mentioned to be the cross-attention that takes as keys and values these encoded embeddings as inputs.

**1:40:04** · And then the queries are those that come out of the self-attention layer.

**1:40:12** · OK, great.

**1:40:13** · And then once you do this cross-attention, you have, just like in the encoder, an FFN component that makes the representation richer.

**1:40:22** · Was there a question there?

**1:40:24** · No.

**1:40:28** · And then at the very end, so you do all of that n times.

**1:40:32** · And at the very end of the decoding process, you have a linear projection and a softmax layer to turn the prediction of the next word into a probability distribution over the vocabulary.

**1:40:46** · OK, great.

**1:40:46** · So we saw how to do that for the next word here.

**1:40:50** · And basically you do that again and again.

**1:40:54** · So you have found your next token, which is like a one-hot basically encoding of what you want.

**1:41:03** · And then you take that embedding and then put it back in the decoder and continue this process.

**1:41:10** · And when do you stop?

**1:41:13** · It's a question for you all.

**1:41:17** · When you hit the EOS token.

**1:41:18** · Yeah, yeah exactly.

**1:41:23** · OK, great.

**1:41:24** · And with this process, it's basically how the authors of this original landmark paper did machine translation.

**1:41:33** · So this is typically the use case that was presented.

**1:41:37** · Any questions?

**1:41:45** · OK, awesome.

**1:41:47** · And with that, thank you for your attention.

**1:41:49** · \[APPLAUSE\]