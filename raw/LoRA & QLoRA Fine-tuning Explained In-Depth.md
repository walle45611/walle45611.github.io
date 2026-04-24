---
title: "LoRA & QLoRA Fine-tuning Explained In-Depth"
source: "https://www.youtube.com/watch?v=t1caDsMzWBk&t=139s"
author:
  - "[[Mark Hennings]]"
published: 2023-12-15
created: 2026-04-25
description: "In this video, I dive into how LoRA works vs full-parameter fine-tuning, explain why QLoRA is a step up, and provide an in-depth look at the LoRA-specific hyperparameters: Rank, Alpha, and Dropout.0"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=t1caDsMzWBk)

In this video, I dive into how LoRA works vs full-parameter fine-tuning, explain why QLoRA is a step up, and provide an in-depth look at the LoRA-specific hyperparameters: Rank, Alpha, and Dropout.  
  
0:26 - Why We Need Parameter-efficient Fine-tuning  
1:32 - Full-parameter Fine-tuning  
2:19 - LoRA Explanation  
6:29 - What should Rank be?  
8:04 - QLoRA and Rank Continued  
11:17 - Alpha Hyperparameter  
13:20 - Dropout Hyperparameter  
  
Ready to put it into practice? Try LoRA fine-tuning at www.entrypointai.com

## Transcript

**0:00** · hey I'm Mark Hennings one of the founders of entrypoint AI and today I'm excited to talk to you about low rank adaptation or Laura which is a parameter efficient fine-tuning method for large language models and we're also going to talk about Cura which I would consider Laura 2.0 so to understand the need for

**0:19** · an efficient way to fine-tune large language models first we have to look at where you use fine-tuning in the training process so the first part of creating a large language model is pre-training pre-training is when we take a very large amount of text like two trillion tokens which would be equivalent to roughly 1.5 trillion words and the model just goes through all this text and learn learns to predict the next word based on um the context of the

### Why We Need Parameter-efficient Fine-tuning

**0:45** · text pretty much everything after that is fine-tuning so once we have a base model that's been trained on a lot of text we do fine-tuning we often do instruct tuning this is how we get super fun chat models like chat GPT and then after instruct tuning there's safety tuning and safety tuning tries to prevent the model from doing things that you don't want it to do and from there we can further take these models and fine-tune them more to do very specific tasks super well and there's just a ton

**1:17** · of potential ways to do that you can also do domain fine tuning this would be taking the base model and then teaching it to be better at something like law or Finance so as you can see the possibilities for fine tuning are unlimited so let's look at how full parameter fine tuning works it updates all the model weights

### Full-parameter Fine-tuning

**1:40** · so that's all the parameters right and these are very large matrices of numbers so a 7 billion parameter model has 7 billion weights 13 billion has 13 billion and so on so that's a lot of numbers and um all these numbers get updated repeatedly so it could go through your training data like five times and that would be five epochs storing and updating all those weights takes a ton of memory and that Limits

**2:08** · Full parameter fine-tuning to only very large gpus or GPU clusters if we're constrained by Hardware then we can't test all these different paths to take our base model so Laura attempts to solve this problem in two key ways first instead of updating the weights of the model like all 7 billion weights directly we track the changes we want to make to those weights so isn't that just adding more information that you need to store in memory well yes but there's a

### LoRA Explanation

**2:40** · huge benefit to doing this and this is where the core of Laura kicks into play which is that the changes you're tracking to your model weights are actually tracked in two smaller separate matrices they get multiplied together to

**2:57** · form a matrix the size of the layer that you're fine-tuning let's visualize this um first of all these track changes you have a matrix of track changes the same size as the model's weight Matrix and you can simply add them together so every Square gets added to the corresponding square and you get a fine-tuned matrix out that has your changes added to it it's very simple addition now this Matrix that you're

**3:27** · adding with your weight changes is actually calculated when you need it by multiplying the two smaller matrices this is called Matrix decomposition when you take a matrix and then you have two smaller matrices you can multiply to try to get the same numbers back in this case these are very small matrices they're only one row or column deep so

**3:53** · that's called rank one and in this case the rank one matrices there's 's a total of five numbers in them in each one so that's 10 basically parameters that you're multiplying together to get a 25 parameter like a 5x5 Matrix here you're

**4:14** · sort of sacrificing some of the Precision in your final table when you're working with these two smaller matrices in order to get a huge benefit in efficiency because 10 trainable parameters here versus 25 is a pretty good savings but the savings increase as

**4:31** · your rank stays fixed and your tables that you're working with get much much bigger like the size of the models we're talking about here is an example if you increase the rank to rank two this is what it looks like you've got these two matrices that are two values deep and you'll get a higher Precision output from that so we can change rank for our

**4:53** · lower of fine tuning jobs we can decide how precise we want our final output table to be and as that output table gets bigger we can see that you're training a lot more parameters as rank increases but even when rank is 512 for

**5:10** · a 7 billion parameter model you're only fine-tuning 86 million parameters and 512 would be considered a very high rank in percentage terms it's just kind of insane for a rank of one it doesn't even show up in two decimal places for these models of how many parameters you're working with for rank of 16 it's like 04 and as the model gets bigger you can see like 0 one% now I am making an assumption here

**5:40** · which is that this would be one giant Matrix which isn't actually true because a model has multiple layers and so those like 180 billion parameters are split across multiple matrices but this illustrates the point that as the model gets bigger the rank is training a smaller percentage of the actual parameters and you can increase rank to train like

**6:03** · a larger percent of parameters of the original model now the terminology gets really confusing here because we're talking about trainable parameters those are the two smaller Lura matrices but remember they always get multiplied together to form a matrix the same size as the one of the model that gets added to it so you're always still training all of the models parameters you're just working with a smaller set of them so naturally at this point if you're like me you're asking well what should I set

### What should Rank be?

**6:34** · rank to like what's the right value of rank um am I going to be making a big compromise with Laura if I set rank too low and the theory is that for most tasks it may not matter Downstream tasks for large language models so if they've been trained on a ton of information and they have all these parameters they're sort of like bigger than they need to be

**6:57** · and your specific task that you want to find tune it for is a subset of everything it could do before like with prompt engineering perhaps like it's it's been exposed to a lot of these things and it has prior knowledge of them you can get away with really rough updates and they they test this in the paper and they get really good results

**7:18** · otherwise we probably wouldn't be talking about Laura right now there are areas where higher rank could help so teaching complex behavior and then also teaching behavior that contradicts or Falls outside the scope of the original training of the model an example of that might be if you

**7:36** · have a chat model like Lou chat that was fine-tuned to be really safe and so it won't respond to questions about diet and health but now you want to create like a fitness or nutrition advice spot you've got your work cut out for you with fine-tuning a model that's already been trained to do the opposite so having higher rank in that case might be beneficial but that's not where the story story about rank NS and to get to the end we first have to look at Cura Cura is this quantized version of Laura

### QLoRA and Rank Continued

**8:07** · which means that it takes these precise parameters like um their 16bit floats for example which is a long decimal number it's very precise um it takes 16 bits in memory and then quantization reduces that into something like a 4bit number now I had this misconception that because it's going to be quantized that you would somehow lose some precis ision

**8:30** · well the way they've implemented Q is that you can recover the original Precision of the model so they came up with this clever way to compress the size of the model and then recover the original size which sounds impossible but they exploit this fact that all of these parameters fall into essentially like a normal distribution think of a bell curve and if you use the four bits that you're compressing it to to just tell it where that number falls on the curve for this particular data set then you can actually reduce the size to four

**9:03** · bits and then recover the Precision at the end so you just make it smaller while you're fine-tuning it and then you get back everything you ever wanted at the end so I think Q Laura is Laura 2.0 I think it's it's a great method and it uses even less memory than Laura significantly less and in the paper there's two other things I want to call out that are super helpful for using Laura in practice the first is that

**9:27** · training all the layers of the network is essential to match performance of full parameter fine tuning in the paper they basically say they can't reproduce the quality of full parameter fine tuning unless they train all the layers of the network second rank may not matter in the range of 8 to 256 so let's look at the actual paper here in the paper body they say that the

**9:50** · most critical Laura hyper parameter is how many Laura adapters are used in total that means how many layers of the network you're training and that Laura on all all linear Transformer block layers all the layers are required to match full fine-tuning performance and then other low hyper parameters such as the projection Dimension R that's rank do not affect performance that's crazy okay but here

**10:18** · they show we find Laura R is unrelated to final performance if Laura is used on all layers so in this little chart here they show Laura R going from 8 to 16 to 32 to 64 and then on this Benchmark uh Rouge L basically it's not changing the performance in any statistically significant way now they only tested rank in the range of 8 to

**10:41** · 256 so we really can't make assertions outside of that range but if you're in that range you might be good however they set rank to 64 even though they say it doesn't matter in the end they chose 64 for the rank don't know what to think about that but maybe 64 is a good uh starting point to use and then they chose Alpha of 16 so let's talk about

**11:05** · these other hyperparameters okay we already covered Rank and in the Microsoft Laura repository they used 8 and 16 that was released in 2021 in the qora paper they went with 64 and then this hyper parameter Alpha this is a

### Alpha Hyperparameter

**11:21** · strange one okay I'm not going to Sugar Code this this is a weird hyper parameter what it does is it determines a scaling Factor that's applied to to your weight changes before they get added into the original model weights so like multiplying by two makes them double in value and then you add that or you could like divide them by two make them half the original value and add that so you're like having a less of an impact on the original model but it's determined as Alpha divided by rank so

**11:49** · you have to like do this math ahead of time to figure out what your scale factor is going to be um in the Microsoft Laura repository they set Alpha to two times the rank so then your training would have a 2X impact QA went with alpha 16 rank of 64 16 divided 64

**12:10** · is 1/4 so then their training is having like 25% of impact when it's finally added back in and I just found that really surprising and I'm also just really surprised that you have to that you interact with the Laura hyperparameters in this way um I think you should just be able to set a scale Factor right like I want this to be 2X or like 1 12 instead of having to do um Alpha divided rank but I didn't create

**12:37** · Laura so that's the way it is and now you know how to use it one possible explanation for why they set these like 2X and 1/4 scaling factors using Alpha is that you can think of it as partially redundant with learning rate if you're increasing learning rate and you're increasing the skill factor you're having an amplified impact um so they

**13:02** · recommend just leaving Alpha fixed and playing with learning rate to keep it simpler however based on my understanding I still think they're a little bit different um and worth looking at separately so I suppose we'll learn more about these hyperparameters in time and then the last hyperparameter for Laura that we need to look at is dropout dropout is this percentage that will randomly set some of the parameters to zero it can help avoid overfitting

### Dropout Hyperparameter

**13:30** · which is when the model has learned your training data so well that it can only answer your training data and it performs really poorly on data that it hasn't seen and for setting Dropout in the cura paper they went with 0.1 or 10% for the 7 billion and 13 billion sized models and then 005 or 5% for the larger

**13:52** · 33 billion and 65 billion parameter models and they also included some of their other hyper parameters like learning rate and batch size in the paper in this chart and I just think it's really interesting to look at this stuff because there's no hard and fast rules for it so you really just have to find out what other people are getting good results with under what situations and then go from there and make any adjustments for your own data so that's

**14:16** · Laura I hope you enjoyed this video and learned something um I'd love it if you check out entry point we do have Integrations that feature Laura fine tuning such as with replicate or gradient and you can get started really easily and put Laura into practice without having to write a single line of code just a few clicks upload your training data and go cheers