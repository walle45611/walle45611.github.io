---
title: "What is the ROUGE metric?"
source: "https://www.youtube.com/watch?v=TMshhnrEXlg"
author:
  - "[[Hugging Face]]"
published: 2021-11-15
created: 2026-05-14
description: "The ROUGE metric is often used in summarization tasks, but how is it computed exactly? This video will explain this to you.This video is part of the Hugging Face course: http://huggingface.co/course"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=TMshhnrEXlg)

The ROUGE metric is often used in summarization tasks, but how is it computed exactly? This video will explain this to you.  
  
This video is part of the Hugging Face course: http://huggingface.co/course  
Open in colab to run the code samples:  
https://colab.research.google.com/github/huggingface/notebooks/blob/master/course/videos/rouge\_metric.ipynb  
  
Related videos:  
\- 🤗 Tasks: Summarization: https://youtu.be/yHnr5Dk2zCI  
\- Data processing for Summarization: https://youtu.be/1m7BerpSq8A  
  
Don't have a Hugging Face account? Join now: http://huggingface.co/join  
Have a question? Checkout the forums: https://discuss.huggingface.co/c/course/20  
Subscribe to our newsletter: https://huggingface.curated.co/

## Transcript

**0:05** · What is the ROUGE metric? For many NLP tasks we can use common metrics like accuracy or F1 score, but what do you do when you want to measure the quality of a summary from a model like T5?

**0:16** · In this video, we'll take a look at a widely used metric for text summarization called ROUGE, which is short for Recall-Oriented Understudy for Gisting Evaluation. There are actually several variants of ROUGE, but the basic idea behind all of them is to assign a single numerical score to a summary that tells us how "good" it is compared to one or more reference summaries.

**0:32** · In this example we have a book review that has been summarized by some model.

**0:36** · If we compare the generated summary to some reference human summaries, we can see that the model is pretty good, and only differs by a word or two.

**0:44** · So how can we measure the quality of a generated summary in an automatic way?

**0:48** · The approach that ROUGE takes is to compare the n-grams of the generated summary to the n-grams of the references. An n-gram is just a fancy way of saying "a chunk of n words", so let's start with unigrams, which correspond to the individual words in a sentence.

**1:03** · In this example you can see that six of the words in the generated summary are also found in one of the reference summaries. The ROUGE metric that compares unigrams is called ROUGE-1.

**1:14** · Now that we've found our matches, one way to assign a score to the summary is to compute the recall of the unigrams. This means we just count the number of matching words in the generated and reference summaries and normalize the count by dividing by the number of word in the reference.

### Recall of the Unigrams

**1:28** · In this example, we found 6 matching words and our reference has 6 words, so our unigram recall is perfect! This means that all of words in the reference summary have produced in the generated one. Perfect recall sounds great, but imagine if our generated summary had been “I really really really really loved reading the Hunger Games”.

**1:47** · This would also have perfect recall, but is arguably a worse summary since it is verbose.

**1:53** · To deal with these scenarios we can also compute precision, which in the ROUGE context measures how much of the generated summary was relevant. In this example, the precision is 6/7. In practice, both precision and recall are usually computed and then the F1-score is reported.

### Precision

**2:07** · We can change the granularity of the comparison by comparing bigrams instead of unigrams.

**2:12** · With bigrams we chunk the sentence into pairs of consecutive words and then count how many pairs in the generated summary are present in the reference one. This gives us ROUGE-2 precision and recall, which we can see is lower than the ROUGE-1 scores we saw earlier. Note that if the summaries are long, the ROUGE-2 score will be small as there are typically fewer bigrams to match. This is also true for abstractive summarization, so both ROUGE-1 and ROUGE-2 scores are usually reported.

**2:41** · The last ROUGE variant we'll discuss is ROUGE-L. ROUGE-L doesn't compare n-grams, but instead treats each summary as a sequence of words and then looks for the longest common subsequence or LCS. A subsequence is a sequence that appears in the same relative order, but not necessarily contiguous. So in this example, "I loved reading the Hunger Games" is the longest common subsequence. The main advantage of ROUGE-L over ROUGE-1 or ROUGE-2 is that is doesn't depend on consecutive n-gram matches, so it tends to capture sentence structure more accurately.

**3:11** · To compute ROUGE scores in Hugging Face Datasets is very simple: just use the load\_metric() function, provide your model's summaries along with the references and you're good to go!

### Output

**3:28** · The output from the calculation contains a lot of information! The first thing we can see here is that the confidence intervals of each ROUGE score are provided in the low, mid, and high fields. This is really useful if you want to know the spread of your ROUGE scores when comparing two or more models. The second thing to notice is that we have four types of ROUGE score.

### Types of Root Score

**3:48** · We've already seen ROUGE-1, ROUGE-2 and ROUGE-L, so what is ROUGE-LSUM? Well, the “sum” in ROUGE-LSUM refers to the fact that this metric is computed over a whole summary, while ROUGE-L is computed as the average over individual sentences.