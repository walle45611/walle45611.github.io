---
title: "徐天音：系统，UIUC，教授，最佳论文，Agent Infra，云计算，形式化验证，容错，纯粹，好导师"
source: "https://www.youtube.com/watch?v=N0QfgZsDBv8"
author:
  - "[[月球大叔]]"
published: 2026-08-23
created: 2026-08-29
description: "本期UIUC 徐天音，伊利诺伊香槟的终身教授，拿了很多很多计算机系统领域的最佳论文，我心中的顶级研究者，最近学术休假，来硅谷的UC Berkeley的Sky Lab进行访问时间轴00:00:00 片头观点：取消最佳论文奖、AI 奖励投机与系统漏洞00:00:34 嘉宾介绍：UIUC 教授徐天音，访问 UC Berkeley SkyLab00:00:59 第一次申请美国 PhD：被 24"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=N0QfgZsDBv8)

本期UIUC 徐天音，伊利诺伊香槟的终身教授，拿了很多很多计算机系统领域的最佳论文，我心中的顶级研究者，最近学术休假，来硅谷的UC Berkeley的Sky Lab进行访问  
  
时间轴  
  
00:00:00 片头观点：取消最佳论文奖、AI 奖励投机与系统漏洞  
00:00:34 嘉宾介绍：UIUC 教授徐天音，访问 UC Berkeley SkyLab  
00:00:59 第一次申请美国 PhD：被 24 所学校拒绝  
00:03:52 Gap Year：前往德国哥廷根大学交流  
00:04:22 足球、竞技体育与系统研究的共同点  
00:05:47 第二次申请：靠坚持拿到 UCSD Offer  
00:08:37 加入 YY 实验室：导师如何系统化训练研究思维  
00:10:26 PCheck：从配置错误到容错系统  
00:13:23 为什么 Best Paper 具有随机性和主观性  
00:15:49 《Do Not Blame Users for Misconfigurations》：不要把错误都归咎于用户  
00:19:38 AI Agent 成为系统用户后，旧系统为什么会失效  
00:21:58 为 AI Agent 重新设计登录、安全与操作接口  
00:24:12 Rock-star Advisor：好导师如何影响研究热情与创业认知  
00:27:00 为什么选择 UIUC 和香槟这座“玉米地”  
00:28:29 如何选择学生：导师与学生之间的信任关系  
00:32:56 人人都可以做 Research：研究是一种反复探索  
00:36:09 PhD 前两年怎么训练：少从论文想 Idea，多从真实问题出发  
00:38:37 AI Agent 能否取代导师和高年级学生  
00:41:34 AI 时代，传统论文和同行评审要不要改革  
00:44:08 AI 放大研究生产力，也可能进一步拉大能力差距  
00:45:44 如何持续做出好研究：招对学生并理解他的目标  
00:49:48 Fight Dragons, Not Windmills：解决真正重要的问题  
00:50:28 如何从小问题挖到基础而深刻的研究问题  
00:53:09 如何教操作系统：用 Linux 内核训练复杂工程能力  
00:57:31 AI 时代计算机科学会不会消亡  
00:58:38 AI SRE 能做什么，人类又该扮演什么角色  
01:03:11 SREGym：在真实故障环境中评估 AI 运维能力  
01:04:44 Benchmark 如何随着模型能力持续升级  
01:09:14 AI 加形式化方法：把代码转化为形式化模型  
01:09:30 Model Checking 入门：Safety、Liveness 与系统抽象  
01:15:22 从五年到五小时：AI 如何加速形式化建模  
01:16:29 形式化验证对企业的价值：从事后调试到事前证明  
01:18:27 形式化证明的成本，以及为什么先选择 Model Checking  
01:19:31 SDD：用规格驱动开发约束 AI 生成代码  
01:21:28 形式化验证的鸿沟：最难的是定义正确的需求和规格  
01:23:05 理想的 Agent-native 系统：容忍错误、提供反馈、持续恢复  
01:24:21 ROC：面向恢复的计算与“系统一定会出错”的现实  
01:26:15 人为操作为什么是系统故障的主要来源  
01:29:13 Undo 与 Microreboot：把经典系统思想带入 Agent 时代  
01:31:07 Reliability 与 Security 为什么迎来新的研究机会  
01:33:00 学术休假：为什么到 UC Berkeley SkyLab 访问  
01:35:26 UIUC 与 Berkeley：不同研究文化如何互相启发  
01:38:22 工业界研究与学术研究的目标差异  
01:40:23 PhD 实习：深入生产系统，而不是重复学校里的研究  
01:42:44 去 Google 做核心系统，还是到大学当教授  
01:45:52 如何判断一份工业界工作和一个团队是否值得加入  
01:47:21 AI 时代最重要的能力：持续学习  
01:49:05 给年轻研究者的三个建议：问题、方法与技能  
01:53:32 想创业或进工业界，PhD 阶段应该怎样准备  
01:56:17 结语：让 Research 成为每天的实践

## Transcript

### 片头观点：取消最佳论文奖、AI 奖励投机与系统漏洞

**0:15** · There are simply too many bugs in production, and nowhere near enough people to fix them.

**0:09** · These failures all came from AI gaming the reward and producing bogus proofs.

**0:04** · That's a bold take.

**0:05** · Aren't you worried about offending people?

**0:00** · I've always argued that best paper awards should be abolished—they're deeply misleading.

### 嘉宾介绍：UIUC 教授徐天音，访问 UC Berkeley SkyLab

**0:34** · Conversations with 100 fascinating people in Silicon Valley.

**0:35** · This is Uncle Moon.

**0:37** · Today's guest is Tianyin Xu, a tenured professor at UIUC.

**0:41** · He has won numerous best paper awards in computer systems, and is one of the finest systems researchers I know. He is currently on sabbatical, visiting UC Berkeley's SkyLab.

**0:52** · Welcome, Tianyin.

**0:54** · Hi, everyone.

**0:54** · It's an honor to be on Uncle Moon's podcast and chat with you all.

**0:58** · All right, let's dive in.

### 第一次申请美国 PhD：被 24 所学校拒绝

**1:00** · From 2003 to 2010, you completed both your bachelor's and master's degrees at Nanjing University.

**1:05** · Then, in 2010, you applied to PhD programs in the U.S.

**1:09** · Reports say you were rejected by 24 schools.

**1:13** · What exactly happened?

**1:16** · So what happened?

**1:18** · Honestly, I'm not entirely sure.

**1:19** · A failure like that usually has many causes.

**1:23** · But the root cause was probably that my track record wasn't strong enough.

**1:30** · I applied in 2010, when the academic environment in China was very different from what it is today.

**1:36** · I did get one interview, though, with the program where I eventually did my PhD.

**1:41** · It was with the professor who later became my advisor.

**1:45** · The interview lasted about 30 minutes, and I remember it vividly.

**1:47** · He asked me five questions, and I couldn't answer a single one.

**1:50** · A few days later, I received the rejection letter.

**1:54** · Let me share three of those five questions.

**1:56** · The whole thing was pretty embarrassing.

**1:57** · First: why do you want to pursue a PhD?

**2:00** · And ultimately, what kind of career do you want?

**2:02** · Second: compared with your peers, what are your strengths?

**2:07** · Third: what kind of research do you want to do?

**2:13** · I'd never really thought about any of those questions. They weren't technical, so my answers were very weak.

**2:20** · question at that time was, 'What will your future career development be?'

**2:23** · I heard that at the time I read some guides saying that I heard American professors all like their future students to become professors, so at that time I said I wanted to go down the academic path and become a professor. Then my future advisor at that time asked me, he said, 'So what do you think it takes to become a professor?'

**2:39** · I couldn't answer, and then I said, 'Well, if you do research well, shouldn't it naturally make you a professor?' Then he just laughed and said, 'It's not like that, actually.'

**2:45** · Later, we moved on to the second question. The second question was, he asked, 'Compared to your peers, what are your strengths?'

**2:49** · What are your technical strengths?

**2:50** · Then I hadn't thought about this question either, and I was stunned for quite a while. In the end, an answer popped out, and I said that I work particularly hard. Then he laughed and said that 'working hard' is a default; it's not really an advantage, right, it's not a strength. He said all PhD students work very hard, and then he followed up with more questions.

**3:05** · He asked what exactly your strengths are?

**3:06** · Then I said I didn't have any advantages and then moved on to the next question. The next question was about what kind of research you want to do. At that time, my preparation was also very rough. I had read a lot of his papers and said that I especially liked his ABC paper and wanted to do those projects. Then he smiled again and said that if a professor puts a paper on their homepage, it means that the paper has already been completed.

**3:28** · Otherwise, you wouldn't have put this paper on this website, right?

**3:32** · He said he actually wanted to ask what kind of research I wanted to do, rather than doing a certain project, and I still couldn't answer. A few days later, I received a rejection letter. That was my first application experience. Honestly, I later got an offer from a pretty good school, but I didn't end up going, which also laid some groundwork for my second application later on. Anyway, after that, you took a gap year and chose to go to Europe, right?

### Gap Year：前往德国哥廷根大学交流

**3:55** · Yes, at that time, Nanjing University happened to have an exchange program. So I applied to a school in Germany called the University of Göttingen. It's the same story—I felt that the domestic environment back then was very different from now. Nowadays, I think students have many choices. Even in my own lab, there are many visiting students, but around 2010, going abroad was still not a very common thing.

**4:18** · So opportunities weren't that many, and at that time there happened to be one opportunity, so I went. And during the time from Nanjing University to Germany, you were still playing football, and you even wrote on your homepage a football resume about how you played from Nanjing University all the way to Germany.

### 足球、竞技体育与系统研究的共同点

**4:33** · How did that come about? I actually can't remember this clearly. I didn't even know there was such a page. But I indeed really like football, and I really like sports.

**4:42** · I like sports.

**4:42** · Competitive sports are much like research.

**4:45** · You have to give them everything you've got.

**4:48** · Both demand your very best.

**4:51** · But sports are even more competitive: there is only one champion. Everyone else loses.

**4:53** · Research is fiercely competitive too.

**4:54** · Think about the papers PhD students submit to top conferences and journals each year.

**4:58** · Acceptance rates are low, and everyone is brilliant and exceptionally hardworking.

**5:01** · So how can a PhD student compete?

**5:02** · At top conferences and journals, so few papers are accepted.

**5:05** · Then everyone does research here.

**5:08** · People are all very smart.

**5:09** · Then very hard working.

**5:10** · Probably the most hard-working.

**5:11** · The smartest people work on it day and night.

**5:13** · Then you as one.

**5:15** · For example, a PhD student.

**5:16** · You are just starting.

**5:16** · How do you go to win the game?

**5:18** · In competitive sports and, I think, doing research, mediocrity has no place.

**5:22** · You either do very well, or you don’t do it at all. So I think there are very similar aspects between the two.

**5:28** · That’s why I’ve always been very interested in sports—not just football, basketball, anything, I really like them. It’s just that later I stopped playing football because I was too busy, and also because I’m a bit stubborn, as I feel I wasn’t playing very well.

**5:41** · When things are going well, I don't really want to kick anymore; if I do it, I want to do the best one, right?

**5:44** · Right.

**5:45** · It's just that my own mindset wasn't very good. You stayed in Germany for a year and finally got an offer from UCSD. During that year, what do you think you did correctly?

### 第二次申请：靠坚持拿到 UCSD Offer

**5:51** · And was it that your papers increased or that your recommendation letters got stronger? Honestly, I think nothing really changed; I just tried again, that's all. I think after just trying again and getting this offer, was there any story behind it?

**6:03** · Because for me, I’ve always thought that I'm not a particularly smart person.

**6:09** · Since I was young, I wasn’t especially smart either. For challenging things, the first time I do them, I definitely won’t do well. So for me, from my life experience, if I want to do something relatively challenging, I definitely have to fail many times.

**6:21** · So actually, failing the first time isn’t a particularly big blow for me.

**6:24** · And also, I’m not really someone who likes to settle; I’m not particularly the type of person who likes to compromise. I don’t really like doing something just because the previous task didn’t go well and then choosing the easier option. For example, when I was filling out my college application for the college entrance exam, I only applied to one school. At that time, I thought that if I didn’t get in, I would just retake the exam for another year, right?

**6:37** · So the second time.

**6:39** · The first application was not successful.

**6:40** · Then I guess I’ll just apply once more.

**6:42** · Try applying one more time.

**6:43** · That's exactly what I was thinking at the time.

**6:45** · Then, when I applied the second time.

**6:47** · Later, didn't I also apply to the same school?

**6:49** · And then finally went there too.

**6:50** · And at that time, I still remember that interview.

**6:52** · He/She is my advisor.

**6:53** · His nickname is Wai Wai.

**6:54** · Then WaiWai called me again at that time.

**6:56** · At that time, I made a call on Skype.

**6:58** · Then when he made the phone call.

**7:00** · I was really nervous at that time.

**7:01** · So the first time I applied, it didn’t go well. I thought I’d just apply again and try harder. That’s how I was thinking at the time. When I applied the second time, I ended up applying to the same school again, and eventually, I went there. I I still remember my second interview. YY, my future advisor, called me on Skype. I was nervous after the earlier rejection.

**7:09** · I had carefully prepared answers to his earlier questions.

**7:11** · But he didn't want to listen at that time.

**7:12** · He asked me right then why I hadn't gone at that time.

**7:15** · I didn't go to the school whose offer I received at that time.

**7:17** · Then I told him some of my situation.

**7:19** · Then he talked about me at that time.

**7:21** · They started trying to persuade me to go to UCSD for a PhD.

**7:23** · I was extremely shocked at that time.

**7:24** · Then I also asked him.

**7:25** · I said why this.

**7:26** · But he didn’t want to listen.

**7:28** · Instead, he asked me why I didn’t go to the school I had initially received an offer from. I explained my situation to him, and then he started persuading me to go to UCSD for a PhD. I was very shocked, and I even asked if he wanted to hear my prepared answers.

**7:36** · want to listen. Then he told me that doing systems research is a bit different from theory research. In systems research, persistence is very important; it’s about being someone who isn’t afraid of failure.

**7:41** · Then, if you're willing to keep trying, basically your systems research won't be particularly bad. So he told me that if you are willing to persist and willing to do this, then you've already passed my test. So I got this offer, and then I went. So when you ask me whether I improved during this time, I actually feel that I didn't improve much, I just persisted for another year. But as an observer, my feeling is just like what you said earlier.

**8:04** · You are unwilling to settle and have high standards, and that means you are willing to try and willing to fail. The impression you give me is that this year you have proven this: you do not settle, and you want to accomplish this.

**8:16** · It is possible—it's possible that if I look at various systems researchers, I find that this is one of the most valuable qualities of a systems researcher.

**8:23** · I think it's essential.

**8:24** · In my experience, it's something every systems researcher needs.

**8:26** · Some research fields may depend on a flash of inspiration. Systems research, however, demands diligence, repeated failure, and the willingness to keep trying.

**8:34** · Exactly.

**8:36** · I completely agree.

### 加入 YY 实验室：导师如何系统化训练研究思维

**8:37** · Then you joined YY's lab.

**8:39** · Advisors have very different styles.

**8:40** · Some guide students step by step, while others are very hands-off.

**8:44** · How would you describe YY's style?

**8:46** · He wasn't entirely hands-off, but he didn't hold your hand either.

**8:52** · YY was already very senior at the time, yet he was an advisor who actively guided students.

**8:58** · I think he would tell you how something ought to be done.

**9:01** · this. Just now there was a question, for example, how does a PhD student think about this idea in the first few years, right?

**9:09** · Then I remember that some of my previous practice was that a lot of times we would read that story, like how Newton came up with this, this, this Newton's law because he was hit by an apple and suddenly had an inspiration, right?

**9:21** · Then YY is actually very opposed to this kind of process.

**9:25** · He always emphasizes that an idea shouldn’t be something that comes from a sudden flash of inspiration, nor should it be something where you just sit under a tree and get hit by something and suddenly have an inspiration. It’s actually a very systematic process of exploration. What he emphasizes is starting from a problem: if you have an important problem, you start by trying to understand that problem, right?

**9:44** · Go in-depth to understand this issue.

**9:45** · After you go in-depth to understand this issue?

**9:47** · So your solution is often obvious once you understand the problem; it can even be very simple and practical.

**9:53** · This is a method of doing research that he emphasizes, and we all follow this method. This approach avoids many risks because once you deeply understand a problem, your answer naturally becomes very natural and very effective in solving the problem. You won't have a solution full of tricks from the start.

**10:08** · Then I realized after spending a lot of time that this solution couldn't solve the problem either. It actually doesn't have such a risk, so actually I think that when doing research, because of this—at that time, in any case, I was very much in agreement with his, how should I put it?

**10:19** · This is a principle, so I have always been practicing this practice, and I think it’s quite good and smooth. Continuously practicing this practice helped you get the PCheck. Could you share with We built a tool called PCheck.

### PCheck：从配置错误到容错系统

**10:33** · Could you explain what PCheck does?

**10:35** · This became the second paper of my PhD.

**10:37** · The idea is actually quite simple. Back then we were actually studying, I actually don't know how to translate it into Chinese, it's 'fault-tolerant systems,' which in Chinese might be called fault-tolerant system, right?

**10:48** · Fault-tolerant system, fault-tolerant system. From our perspective at that time, what we were studying was a type of fault called misconfiguration. When there is a misconfiguration, the system obviously won't operate very well, right?

**11:04** · At that time, YY asked a question which was: why is it that we have been studying fault-tolerant systems for so many years, and many of our systems claim to be fault-tolerant, yet when a fault like a misconfiguration occurs, the system just doesn't work, right?

**11:15** · Because according to major companies, misconfiguration is a very important cause of system failure, so we started from this perspective. At that time, I did my first project, and the first project was to answer the question of why those mature production-level systems.

**11:30** · They cannot handle faults like misconfiguration very well, and our approach is also very simple and crude, which is fault injection. Fault injection is a very mature technology, which means you try all kinds of different faults. Here, the fault refers to misconfiguration.

**11:46** · Then you inject this fault into this system, so it is injected into this system, and then you observe how this system behaves. Then, during the process of doing this project?

**11:54** · At that time, we were also very busy, that is, we had to do this kind of project to answer this question. Among them, there was one phenomenon that we found very interesting, which is that there are many misconfigurations. When you inject them into the system, the system actually doesn't show much behavior; it still works normally, right?

**12:07** · It neither initiated a fault-tolerant procedure, nor did it say there was a fault, so why?

**12:13** · Then actually the reasoning is very obvious: this system read these misconfigurations into the system's state, but it didn't use them. So why didn't it use these faults, that is, the misconfigurations?

**12:25** · It is because these codes or these executions must be used only under a specific situation, and the most typical example is that when it is used, there is an external failure, that is, it triggers these misconfigurations, and when it is used, there is already a failure, and then I start a failure-recovery program.

**12:44** · Then at this time I need to use some configuration, and then I find that this configuration is not good. Only then will it explode, and to us, this is like a time bomb, right?

**12:50** · In this system, you're just like a time bomb that hasn't been triggered yet. Once you trigger it, the problem is usually very serious because when you're doing fault tolerance, that is, failure recovery, it's already a very critical time. At this point, discovering another failure—this kind of double failure—usually leads to a major disaster.

**13:06** · When we were doing the first project, we didn't have the time to study this issue in depth because it was a different research problem, but the issue was there. Then for the second project, we said we would try to understand this issue and figure out how to solve it, so naturally the second project came about. We can talk about this first project later.

### 为什么 Best Paper 具有随机性和主观性

**13:25** · Let's talk about this best paper first.

**13:27** · Are you surprised that this paper won the best paper?

**13:29** · I was quite surprised at the time because I didn't know what the standard for the best paper was either.

**13:32** · So I feel quite happy to be recognized in the field. What about him later?

**13:37** · At that time I was surprised, and then?

**13:38** · I have been in this field for a long time now, and last year, for example, at SOSP, I was also a member of the best paper review committee.

**13:47** · Personally, I feel that the best paper thing is very random, very, that is, random and subjective, so now I completely feel that the best paper thing cannot fully represent the quality of a paper, it cannot represent it at all for me.

**13:57** · I even feel that in recent years, for example, even the best papers in our field at SOSP and OSDI have quite a few issues for me, right?

**14:04** · Are you so bold to speak without fear of offending people?

**14:06** · I am I really feel this way because I actually personally don’t know what the standard for the best paper should be?

**14:10** · Then its process, generally speaking, for me, is like this: a white paper—when it gets this white paper, it means that the reviewer, right, in other words, a reviewer particularly likes your paper and has nominated your paper. And that's it, that's it. For me, it doesn't represent what the future impact of this paper will be.

**14:27** · It also doesn't mean this is the highest quality paper here because quality itself can't really be measured, so pursuing whether a paper is the best paper is actually quite meaningless. Personally, I have always advocated for canceling the best paper award because I think it is very misleading, and how should I put it?

**14:44** · I think what I mean is that everyone does research, like trying to get the best paper, right?

**14:47** · Then I feel that having this called paper awards actually makes everyone not focus on doing research properly. I have heard a similar viewpoint before, and I strongly agree with it.

**14:55** · Right, if you remove the best paper, then how would everyone work? Then everyone would do the research they like the most, and I think that is the best state. Research really should, in essence, be like this.

**15:04** · Right.

**15:04** · And I also heard a saying that you don't need to look at the best paper, but the test-of-time paper because whether its citations are high after 10 years and its impact in various places is obvious. For me, all awards are not particularly meaningful; I'm not a person who believes in awards. Then, personally, I feel, for example, about academia in China.

**15:22** · Actually, I’ve been really messed up by awards, so my ideal is an academic environment without these awards and other frustrating things.

**15:31** · Every researcher just works on the problems they consider important, does it happily and joyfully, gets recognized, and then you just present the paper. Do you think I should share this concern?

**15:40** · I'm really not worried about this, I really think so, so let's just cut it out, okay? Yes, I actually expressed my views to many people during the conference.

### 《Do Not Blame Users for Misconfigurations》：不要把错误都归咎于用户

**15:49** · Okay, right, you just said that this work is based Was it based on your 2013 paper, “Do Not Blame Users for Misconfigurations”?

**15:55** · exactly, this work.

**15:56** · I really like that work.

**15:57** · This, this question is really good, don't blame the user for making a wrong configuration, right?

**16:01** · Right.

**16:02** · Then, what you were saying about this work is roughly that when I looked at the paper at the time, it just mentioned that a system was misconfigured.

**16:08** · Engineers would all say this was a user operation error, but that paper of yours argued the opposite: if many users are unaware of the issue and make the same mistake, then there may be a problem with your system's design itself. That was roughly the motivation I got after reading it.

**16:20** · Yes.

**16:21** · Right.

**16:21** · Exactly, I think you understood this paper. Then, in your paper, you mentioned that a user simply capitalized the letters in their name and then went through 75 rounds of communication and collected 10 logs. When you first saw this kind of problem, was it from the industry?

**16:37** · This is a case from the industry.

**16:39** · When I first saw this case, I was very shocked, but later I actually understood it because this company is actually a storage company, right?

**16:47** · Then the storage company, actually very, their model was really troublesome. At that time, the company sold those—I remember this company didn't want to put their name on it, a bit embarrassing, but actually it was a very good company.

**16:58** · Their product at that time was a storage box, right? It bundled software and hardware together, and they sold this storage solution, calling it a solution, to the users.

**17:06** · Then the problem is that for a storage company, after a user has used your product and it malfunctions, it is impossible for them to send this company's box back to you. They can't even send all of their data back to you because this data is confidential. Many times, their clients are, for example, government agencies or banking departments, right?

**17:21** · How can you send this data back and back and back again?

**17:25** · So this diagnosis is really time-consuming and tedious. I can only tell you the few pieces of information I have, and then the person on your end has to guess and try to figure out what kind of problem you actually have. So the root cause might be very simple, but once there is a fault, it becomes a very expensive and very time-consuming process.

**17:46** · Then during the whole process, you discovered 743 configuration error issues, and you also reported that a large number of these issues were confirmed and fixed by their developers.

**17:58** · Which fix made you feel that I was not just publishing a paper, but really helping the industry solve a problem? I actually always felt at that time that because our starting point was system design, right?

**18:09** · It's just a fault-tolerant system, and I think this is quite natural. I also don't feel that any particular patch or bug is especially important. Actually, I've never found discovering bugs that exciting, because I think there are too many bugs in the industry—so many that people can't fix them all. So discovering a few more bugs doesn't seem very meaningful to me.

**18:29** · Of course, I understand that many papers, our evaluation is about proving that this solution is useful, and one of our evaluation metrics is to find new bugs. I think the more meaningful part of this paper is that it may tell some, especially open-source developers, when you are designing a system.

**18:46** · You have to think about how users use it. I think there's a The work felt meaningful.

**18:53** · At the time, our department had a professor named Stephen Savage.

**18:55** · He is a very prominent security researcher in systems and networking.

**19:00** · He was on our faculty.

**19:01** · He organized a seminar, and I remember that one day a student proposed an idea at the seminar: use machine learning to detect misconfigurations.

**19:11** · Stephen's immediate response was, “Why not build the system correctly?”

**19:15** · He knew about our work, which took the same perspective.

**19:21** · His first thought wasn't how machine learning could find more misconfigurations.

**19:25** · Instead, he asked, “Why not do what Tianyin did and build the system correctly?

**19:30** · Then you wouldn't need machine learning to find these problems.”

**19:33** · That reframing may change how others approach the problem.

**19:37** · What I especially like about this work is its relevance to today's age of AI agents.

### AI Agent 成为系统用户后，旧系统为什么会失效

**19:42** · Back then, your paper argued that poorly designed systems create problems when people use them.

**19:50** · Now AI agents are the users.

**19:52** · If a system is still designed only for human users, then AI agents are bound to make mistakes.

**19:58** · Right.

**19:59** · So I'd like to ask: based on your earlier experience, do systems for AI agents need to be redesigned?

**20:09** · Absolutely.

**20:10** · Everyone now talks about AI-native systems.

**20:12** · Yet many current systems weren't designed for AI agents as users.

**20:19** · Treat an AI agent as a user, and the system is guaranteed to fail.

**20:22** · We've even been building benchmarks recently.

**20:25** · For example, one project is a collaboration with the TLA+ Foundation.

**20:28** · In it, we ask an agent to construct a simple proof.

**20:31** · Given a protocol, the agent writes a proof that the protocol is correct.

**20:37** · Then you can could say from our experience that we found TLA+ to be a very mature toolchain.

**20:44** · It has a proof system called TLAPS, which is the TLA+ Proof System, right?

**20:48** · Then people have written a lot of proofs, and they have never encountered some strange problems. Their system's convexity is also very mature, so they want to create a benchmark.

**20:56** · But if you put AI in, put an AI agent in, the agent will write all kinds of strange proofs, and it will seem like the thing has been proved, but actually it is not valid, and AI is especially good at reward hacking.

**21:07** · Because when people write a proof, they really want to prove this, right?

**21:10** · He really wants to prove that this protocol is correct, but AI doesn't actually really want to prove that the protocol is correct. AI just says, because the uncle wants to prove that the protocol is correct, so I try my best and can pass this proof system, which led to him discovering many of these strange bugs, so many colleagues from the TLA+ Foundation, like Markus.

**21:28** · Or Stephan and the others started fixing all sorts of bugs that they had never even thought about before, all of which were caused by AI trying desperately to do reward hacking and then doing proof, which led to these problems. And I think that once an AI agent becomes a user, all systems should rethink how to handle this interface, in my opinion.

**21:49** · I strongly agree with this view because I can observe that more and more companies in Silicon Valley are starting to invest in how to redesign the interface for AI agents, how AI agents should log in, how AI agents should ensure security, how AI agents should operate, and so on. I think it is an inevitable trend that AI agents will become the main force in work in the future.

### 为 AI Agent 重新设计登录、安全与操作接口

**22:09** · Yes.

**22:09** · I think the biggest opportunity for systems researchers or some system engineers next is in this area. Honestly, I agree. I think this is a really good thing for systems research. In the past, if you wanted to fundamentally redesign a system, right?

**22:22** · The cost is high, and not everyone may be willing to invest. But now, I think AI has a very positive impact on systems research. People are willing to consider this and feel it's necessary to redesign the system. I think this is a very good thing. In the system industry, I have always seen that it has to be behind products and algorithms.

**22:46** · Because if you try to go too far ahead and create a system that cannot be deployed and applied on a large scale, then it actually has no value; the cost is too high and the ratio of input to output is too low, meaning the ROI is too low.

**22:56** · Right.

**22:57** · Now it is equivalent to the rapid development of AI, which has generated an infinite number of new major demands, forcing systems researchers and system engineers to continuously push forward the development of these new systems.

**23:07** · Yes.

**23:08** · I think this is a really good thing in the era of systems researchers. I think it is a very good thing. I think every systems researcher should seize this opportunity, because it includes, for example, in the past, if you were doing a system, right?

**23:21** · Then you say that your system is very good, but it can only handle one type of workload. In fact, it's very hard to get, and even in a startup, it's hard to get funding.

**23:29** · It's even hard to convince the judges that my system is good, and the judges will definitely reject your paper saying that this is not general. But now, it's like saying that I just make a system, and this system, for example, has very good IO performance.

**23:40** · Many people would be very excited, right?

**23:42** · I feel that your system is particularly useful, and its economic market is particularly large, so I think this is a very good thing. For example, if you were to work on a single aspect, like how to allow an agent to log in.

**23:52** · Yes.

**23:52** · Right, that agent login may seem minor, but because he considered all the possible reward hacking and ways to hack this system by different agents, he made it very solid. In this way, you can ensure the security of the entire system for the users.

**24:03** · Right.

**24:03** · What a good thing.

**24:04** · Yes.

**24:04** · I think this opportunity is indeed very significant. We can talk more in depth about it later when we discuss some of your recent research and this AI system. Then going forward, when you enter the PhD stage and interact with your advisor, you have always been very grateful to your PhD advisor for constantly being supportive in various ways, and on your homepage, you said he is a rock-star advisor. Why is that?

### Rock-star Advisor：好导师如何影响研究热情与创业认知

**24:23** · What exactly did he teach you?

**24:25** · I think good mentors are all similar, while bad mentors each have their own problems, but good mentors are all similar. So I think he definitely taught me how to do research and then he told me how to think about certain problems correctly, right?

**24:36** · Then I think one thing he does particularly well is that he really emphasizes deeply understanding problems, which is something I really, really appreciate. Also, he encourages thinking differently, and I think these are the things he emphasizes that I really appreciate.

**24:51** · And I feel that he makes me constantly full of passion for research.

**24:55** · I feel that research is a really wonderful thing, and I also especially like doing it.

**24:59** · This is truly a top-level mentor, so I really think that having him as a mentor is actually very strict and very direct, but I especially like direct mentors because I don't really like some indirect advice. So I Your personalities were also a strong match.

**25:14** · But I think the essential quality— what matters most— is a genuine interest and passion for research.

**25:19** · That's what I admired most about him.

**25:23** · Right. And he was also a very successful entrepreneur.

**25:25** · I often see his talks on entrepreneurship online.

**25:29** · He comes across as decisive, forceful, and demanding.

**25:33** · But with you, the fit clearly worked.

**25:37** · That's the impression I get.

**25:38** · Fit may be the most important thing between an advisor and an advisee.

**25:42** · He founded three companies, all quite successful.

**25:47** · Did he ever discuss entrepreneurship with you?

**25:48** · Yes. I listened to him talk about starting companies for three years.

**25:52** · Right.

**25:53** · What did you take away from it?

**25:54** · I've always believed that research and startups share the same core.

**26:00** · Both require innovation, and both demand the pursuit of excellence.

**0:19** · in other words, how do you teach the operating systems course and why do students like this course so much?

**26:04** · These things are basically the same at their core. You need a team; it's very difficult for one person to accomplish something alone. You also need a good advisor who can tell you certain things, right?

**26:13** · The difference is that research might be more particular about how to say it, how should I put it?

**26:18** · It’s more about valuing intellectual merits, or rather, valuing expanding the human knowledge base. And I think doing a startup might require more consideration of marketing aspects, right?

**26:27** · Things in the market, but I think there are really many commonalities between the two; it depends on what you like. Of course, there’s also the matter of timing, location, and people—whether you can start this company. There are many very good technologies, but if the time isn’t right, you still can’t start the company. But I think that there are very many similarities between the two. The feeling I get is that you’re the type of person who doesn’t want to start a company for the short term, right?

**26:48** · Because this person is relatively free and easygoing, and I haven't thought about it too much yet. Yes, yes, although you don't want to start a company, you have gained a lot of successful entrepreneurial insights at a glance. Indeed, I was brainwashed, brainwashed for three or four years, and indeed there was no problem at all. In 2018, you completed your When you finished your PhD, you received faculty offers from five universities, and ultimately chose UIUC.

### 为什么选择 UIUC 和香槟这座“玉米地”

**27:06** · Why UIUC?

**27:07** · At the time, among the offers I had, UIUC was the fairly obvious choice.

**27:13** · One reason was that my former advisor had been on the faculty at UIUC for seven years.

**27:20** · He had many fond memories of the university, which certainly influenced me.

**27:26** · More broadly, with any offer— whether in academia or industry— you need to consider what the place can offer you as a platform.

**27:37** · What can you learn there?

**27:38** · A faculty offer comes down to a few things.

**27:41** · Can you find excellent collaborators?

**27:44** · Are there good mentors?

**27:46** · Are there senior colleagues you deeply respect and would want as mentors?

**27:54** · Can you recruit outstanding students?

**27:56** · UIUC checked all of those boxes.

**27:58** · And Champaign suited my temperament very well.

**28:00** · I really like Champaign. Now that I've come to the Bay Area, I still often miss Champaign and want to go back there.

**28:07** · So I think UIUC is a pretty good choice, indeed different from others, haha. Most of the students I know who are familiar with Champaign feel that it's just a pure cornfield where you can't even stay for a day.

**28:15** · Yes, it's a cornfield, so it's really a very pure area suitable for researchers. I think it's a very simple place without much else, and you can just stay there and think about some problems. I think now that you have been a professor for 8 years, what kind of students do you like the most?

### 如何选择学生：导师与学生之间的信任关系

**28:33** · I think this question is too difficult. This question is actually an equivalent question, which is what kind of partner you want. And I think there is no answer to it.

**28:42** · I once watched a documentary, right? It was directed by a documentary filmmaker.

**28:46** · He interviewed many single men and women and asked them to describe what they wanted their future partner to be like. They would list very detailed descriptions, such as wanting someone who looks a certain way, has red hair, went to college, and so on. But five years later, the director interviewed them again and looked at how many of them already had partners and tried to match them, only to find that it was not the same at all.

**29:08** · The spouse they eventually found was not at all like how they had described at the time, and I think it's the same with students—you can say it's really hard to get to know a person.

**29:16** · Finally, it is said that you can work very closely with this person for five years and have a very close relationship, because I think the advisor-advisee relationship is a lifelong relationship. For me, it means thinking about what kind of person would ultimately be able to work out, that is, to establish a very personal relationship.

**29:35** · Students like this can definitely work out, but if this relationship can't be established, I generally don't know how to work, that is, work with this student. Then I know this view is very controversial because many of my colleagues think this is fundamentally wrong. And I have a colleague whom I respect very much, at UIUC.

**29:52** · His point is that you shouldn't establish a personal relationship with students. I've seen a lot of this kind of view, which is true, but this kind of thing really depends on how you think about it.

**30:01** · But for me, I completely can't work closely with someone I can't establish a personal connection with—it feels like you can't even speak, right?

**30:09** · Because I think, it's still that sentence, I feel doing research, especially very difficult research, is inherently a very hard thing, right? When you are in a difficult situation, you have to be very direct in saying some things. For example, I once watched a football match—in football matches, you can see those people, for example in the World Cup, when they play football, they are not playing while smiling.

**30:33** · Especially those defenders, right?

**30:34** · Many times you just have to yell loudly and then say whether you should handle the ball this way or not, or that you should kick long, or that you shouldn't kick long, or whatever, and then shout loudly, yell loudly, and get the team going. If every time you are smiling, that football team I think plays badly, and I think of it like doing research; every research is like an ambitious project.

**30:50** · It definitely has very difficult moments, and at such times, how you can inspire the team to continue moving forward relies on a personal trust. Without trust, I personally don't know how to push this matter forward. In fact, what you just described reminded me of many scenes I have seen, for example, Gu AIling or Cristiano Ronaldo when they face certain challenges.

**31:13** · Their eyes aren't happy; they are the kind of eyes that want to win, all of them twisted, right?

**31:17** · For example, when you play in the NBA, look at how no one goes for a rebound with a smile on their face.

**31:23** · Everyone is contorting their faces and giving their utmost effort to grab the rebound. I think research often has many moments like this too. The same goes for entrepreneurship.

**31:31** · Many top entrepreneurs tell their teams that we are a sports team; we are here to win, not to fool around, and this is how we win.

**31:40** · We are here to make the customers happy; we are not here to have fun ourselves. So, this kind of thing can only happen in situations of extreme trust and where a personal connection can be established.

**31:51** · Right.

**31:52** · So you're saying that you firmly believe there should be some good connection with a PhD. I think this is a very personal thing. This is my way; it doesn't represent a universal way. It's like the professor I just mentioned, who is also very successful. They think that everyone should just do things according to their own ideas. I think the best thing about the academic world is that everyone does things according to their own thoughts. There isn't a standard method here, so what suits you is the best for me.

**32:13** · I can completely understand that point because the core of that point is that if you are too personal with a student, then the way you teach that student is actually biased, right?

**32:22** · It is very biased by your own taste or preference, so it may not necessarily be the best for the student.

**32:27** · On the contrary, if you have a very personal relationship with the student, when you think about the student, you might be able to give some very objective advice. But I can't do this myself because my personality is just not that kind of person.

**32:40** · So for me, the best thing is to have a connection and be able to speak more directly about some things, right? It goes back to some of the topics we talked about at the very beginning, which is that people still need to be themselves. Being yourself makes it easier to be more consistent and makes doing things go more smoothly.

**32:55** · Right.

### 人人都可以做 Research：研究是一种反复探索

**32:56** · Is there a certain trait that makes you feel this person just shouldn't do research and makes you not want them to be a student?

**33:03** · I feel like, I feel like, the word 'research'—if you break it down, what I tell the students is, what does 'research' mean?

**33:07** · It's just 're' and 'search', right?

**33:09** · It's this word, it's like this: research. 'Re' means doing something repeatedly, you do 're' many times. And 'search' is actually the process of searching. Anyone who is willing to continuously research, then they are a researcher, right?

**33:21** · It is a very common practice, and then you can do research in the industry, you can also do research in academia, and you can do research anywhere, so I don't think research is a very narrow concept.

**33:32** · Anyone can do research, of course, for me. The style of research in academia and industry is not quite the same, and what they pursue is also a bit different. So I think depending on what kind of research you want to do, you might need to choose an environment to do the research you want. For example, if you want to do a very imaginative research, right?

**33:49** · You want to do a very theoretical research, and this research may not be very suitable to be practiced in some places. For example, didn't someone just get the Fields Medal a couple of days ago, right? Wang Hong and Deng Yu won the Fields Medal, and the research they did is very beautiful, right?

**34:01** · It's very wonderful, but you must say that this research, if you're going to do it in, for example, a startup, then I think it might not be very suitable to do this kind of research. But in a startup, you can completely do research on your own product, and you will definitely reflect on it, right?

**34:13** · Then go and think about how to improve or think about some fundamental issues, you're doing research. So I don't think there are people who cannot do research. Of course, whether to do research with me or not fundamentally comes down to whether the two of us can establish some connection and whether I can help this student. I think this is the most important question for me. So if I think some students may be particularly outstanding.

**34:31** · Then I feel that I can't help this student, maybe I might not really want to admit this student as a PhD student, I think.

**34:37** · Understood.

**34:38** · What exactly do you mean by that kind of research being very high-dimensional?

**34:41** · Everyone can do research. Researching the size of a button for a product in a startup is a kind of research because you need to 're' and 'search'.

**34:48** · Right.

**34:49** · Then you do mathematics in that school and proving the Goldbach conjecture is a kind of research.

**34:53** · Yes.

**34:54** · I think the research you do at school is also very meaningful, right?

**34:57** · Fundamental and applied research both matter.

**34:58** · advanced way of thinking, meaning that as long as you seriously focus on innovation, pushing things forward, and optimizing, it all counts as research. Because back in 2017, didn't I spend a year at Facebook?

**35:08** · Back in 2017, didn't I spend a year at Facebook?

**35:10** · Facebook's systems research: it doesn't have a systems research department because Facebook doesn't believe in a systems research department completely separate from production. So all systems researchers are actually working on production services. So how do we do it?

**35:26** · Actually, it's because Facebook is not a research company, right?

**35:29** · We worked with many engineers to build some systems, and the way we conducted this research was that we built the system first, and then we reflected on it afterwards. We looked at why we did it that way at the time, possibly because there weren't other good ways to do it, right?

**35:44** · Because when you are actually going live in production, you are really nervous and you don't really have much time to think about this, then you keep thinking about where this can be improved, where it can be done differently, or what the costs and benefits are. As long as you do this, it is research, and we also document all those production services that we have done.

**36:00** · I ended up writing a paper and published it on OSDI. This kind of paper is my personal favorite — prolific and really flavorful, that's really how it is. So what do you think a PhD student in the very early first or second year should do if they join your group or if you are mentoring them?

### PhD 前两年怎么训练：少从论文想 Idea，多从真实问题出发

**36:18** · Go read 100 papers and then jump in to work on a project with a fifth or sixth grader, or just think about it and write code by yourself over there.

**36:25** · Right.

**36:26** · Because I feel that a PhD is a very personal thing and it doesn't have a standard answer, so first of all, I don't think reading 100 papers is a very good practice. Then I remember my PhD advisor at the time was actually very opposed to reading papers to get ideas.

**36:40** · He had a rather radical point of view: he thought that every paper is very biased and is just the author's own perspective, right?

**36:47** · Actually, each paper is essentially a perspective, it's the author's own viewpoint. And you know, the current practice is that top conferences are very rigorous, so everyone uses a lot of techniques when writing papers and is not necessarily that honest.

**36:56** · So, if you read a lot of papers and try to get ideas from them, it’s not a very good way to innovate. Sometimes, you probably shouldn’t even read them, at least that's what I personally think.

**37:10** · I feel that many papers are like feed; if you consume too much feed, your brain becomes numb, and then you won’t be able to innovate. So I think the best way is not to read papers to get ideas, but to come up with ideas based on real problems. Here is a very important question: we need to solve this problem, right?

**37:25** · Then, then how do we solve this problem? If this problem is important and difficult, it requires research, rather than just because there is a paper that won the best paper award and you decide to read someone else's limitations section and then write your next paper. I think this is not a good approach, so I actually do not advocate reading 100 papers from the beginning.

**37:41** · That recent college graduate started reading for a PhD, and he doesn't have much experience in discovering really impressive I think this is part of an advisor's job.

**37:51** · An advisor should give students good questions to pursue.

**37:56** · on. Basically, I feel that all the PhD students I know started like this. Then, working on projects together with senior students, I think it's a very good practice.

**38:05** · My former advisor did it this way, and now at Berkeley, basically every student does this when they enter Berkeley.

**38:10** · And then Berkeley is more of a bigger lab, right?

**38:14** · This professor-student interaction is not very hands-on most of the time, and actually, you end up learning more from your peers than from the professor. This was also the case for my own group. I think working with a senior student on some difficult problems is a really good way to start, and you can also learn a lot and build some connections.

**38:34** · Then if you have some questions, you can just come up and do them.

**38:36** · Right.

### AI Agent 能否取代导师和高年级学生

**38:37** · In the face of such a situation, especially in the current AI era, you don't even need to interact with peers; you can just interact with agents every day and continuously produce a large amount of content. So what is the meaning of university then? What is the meaning of teaching? I personally feel that agents are still very difficult to replace—at least for now, agents are still hard to replace. Agents are quite a cold thing, I think.

**38:58** · Then the agent is not really something that considerate or comprehensive. I think guidance from a person is still quite important, whether that person is a senior student or an advisor. I think the agent has many capabilities; it can improve your coding productivity.

**39:13** · Right, the productivity of system building, and it can also help you come up with some ideas, but in the end, even if it helps you come up with ideas, you still need to figure out yourself, how should I put it?

**39:22** · So basically, it's about understanding these ideas and figuring out which ones can be done and which ones can't. Overall, my impression after listening is that you have to trust people's judgment and taste, right?

**39:30** · These things are irreplaceable by agents in a short period of time. I don't really feel that, I actually don't believe that in the future the world will be completely replaceable by agents. I still think that the future world will be a world of humans, but a large amount of labor will be taken over by agents. Even if AGI is realized, don't you think it will be like this?

**39:46** · I don't know what your definition of AGI is?

**39:49** · It's just what I think.

**39:51** · First of all, personally I feel that thinking about what will happen after AGI is achieved is quite a waste of time.

**39:57** · To me, it's like thinking about what to do after the apocalypse.

**40:00** · It's the same thing, it's a fun thing to think about, but don't take it too seriously for me. And then if there's a student who is always panicking there, saying that AI will replace everything I've learned and it's all useless, then essentially you can't make progress, right?

**40:12** · And on the contrary, I think we shouldn't overthink things.

**40:16** · Let's just think a little about the future—what can agents do now? Then we can use current agents to improve our learning abilities and work skills. I think this is a more practical approach. Yes, because if in the end people become slaves to AI, then I believe human learning ability becomes even more important, otherwise how would we compete with AI?

**40:37** · Skynet, right?

**40:38** · Right, but if we, if we people are doing the right thing, then AI is a servant of humans. Essentially, it's still humans who decide how this world operates, right?

**40:45** · So your own abilities, your own learning ability, and your own work capability are still very important. Just now, we went through a short half-hour interview, but I could feel from all your words that you are a natural systems researcher.

**40:58** · No.

**40:58** · It's just that this feeling is very obvious, very obvious, because this system really requires you to lower your head and think carefully about the tasks in your hands every day, doing them step by step. It works through doing; it doesn't require you to think about what will happen in 5 or 10 years every day.

**41:12** · As long as you work on this system bit by bit, step by step, it can eventually be completed. With your personality traits, my goodness, you are really suited for working on systems. Research still needs to be done continuously, I think, anyway, I feel that you definitely have to do research, you absolutely need to be open-minded, and you need to know what is happening around you, right?

**41:29** · But it is indeed necessary to have a main thread of your own, I think. OK, so our next question is that many professors now say that theses need to be reformed, and that they no longer require the traditional sections. Some propose using only an abstract and introduction you want to tackle this problem, and then directly include the dialogue with the agent. What do you think of this opinion? I think this opinion might be suitable in some cases.

### AI 时代，传统论文和同行评审要不要改革

**41:51** · It holds true under a specific context, right? I think it's because, for me, the key to a paper is what the contribution of this paper is, right?

**41:59** · It's just that I don't feel like, for example, the Fields Medal winners I just mentioned, like Wang Hong and Deng Yu, right?

**42:04** · I don't think their paper should be presented in this way. in other words, if the contribution of your work ultimately is an artifact, and this artifact is very useful, everyone can use it and uses it well, then perhaps your method is enough, and you have an abstract to tell others how this artifact is used.

**42:19** · Then it later became an open-source project, and it might even be possible to implement an agent there.

**42:24** · But a lot of research is actually a process of thinking, because I feel that the essence of research is understanding the world, right?

**42:30** · Regarding your understanding of this matter, I think it shouldn't be entirely presented through AI. Leslie Lamport is a Turing Award winner, and many of our current works actually use some of the tools he developed. But there is one viewpoint of his that I really agree with: if you don't write something down, you actually can't fully think it through.

**42:51** · So I think the writing process is a process that helps people truly understand this. Personally, I really don't think it should be skipped. I can't agree with this view at all because I also think that the main training in a PhD, this process of writing a paper, is precisely a process of training your brain.

**43:08** · Right.

**43:09** · You can't write clearly; this question of yours definitely hasn't been thought through.

**43:12** · Yes.

**43:13** · So I think a lot of fundamental things haven't been changed, and many viewpoints are brought up because the current review system is under a lot of pressure. Since AI has come out, everyone's productivity has increased, and there are a lot of paper submissions. This includes, for example, some traditional systems where the circles are very small, right?

**43:30** · The system's community is particularly small, and in the past, there might have been only 200 submissions per year, but now it has become 1,000 submissions.

**43:38** · If you continue to handle things according to the original organization, the pressure now is very high, and after the pressure increases, your system is actually not scalable. When your workload scales up, your system begins to break. I think what we should reflect on is this.

**43:48** · How to make this system work well so that it can scale up to the current volume of submissions, rather than simplifying it into another system—for example, if you remove the writing part or the thinking part, I think the essence of the research is broken. I think that's indeed very reasonable; we really need to find a way to improve the system's scalability.

**44:07** · Right.

### AI 放大研究生产力，也可能进一步拉大能力差距

**44:08** · I’ve heard Terence Tao say before that mathematicians nowadays are different from mathematicians in the past. Previously, mathematicians had to come up with many ideas. Now ideas have become so easy, mathematicians have to constantly review and verify to validate these ideas.

**44:22** · We don’t know what might happen next, like how this paper system will turn out.

**44:27** · But we definitely need to undergo some reforms, and I completely agree with this. I think I completely agree, which is to say, we should think about what systems research should change in AI era.

**44:37** · It should do it, right?

**44:38** · Because I feel that AI has raised this bar. In the past, without AI, you might have been able to do some research, and the idea for this research wasn't particularly interesting, but it involved a lot of engineering work, or it required some special skills to accomplish. Now, this kind of research has become relatively difficult because AI can help you do these things.

**44:57** · So what this means is that in AI era, the way you do research is different. It's about how you push the frontier because this frontier is different, and the way you push the frontier is different. But essentially, the nature of research, I think, hasn't changed. A good researcher is still a good researcher.

**45:10** · Terence Tao is still Terence Tao, right?

**45:12** · It's not like anyone uses AI and instantly becomes more powerful than Terence Tao. I don't think that's the case. It's obvious, especially now, that after having AI, those experts who can write papers and do research are more efficient.

**45:25** · Right.

**45:25** · I think AI can increase a person's productivity by 5 times, 10 times, or even 100 times.

**45:28** · Some people say, 'I think so,' but the question is, what is your baseline? If your ability is 0, then multiplying by 100 is still 0. I also want to point out this: when we talk about this 100 times increase, it means that for someone who is already at 100 points, it becomes 10,000, but for someone at 1 point, it becomes 100. Oh my, the gap becomes even bigger.

**45:42** · Yes.

**45:43** · Right, so let's go back to your research itself, from your PhD to you've won more best paper awards than almost anyone I know.

### 如何持续做出好研究：招对学生并理解他的目标

**45:49** · received are among the most I have ever seen. So, how do you view these awards, even though you just kept saying you're not optimistic and that it doesn't matter? I think these are all just luck for me, but if a person's luck keeps happening repeatedly, I consider it to be a reflection of a kind of standard of ability, not that I think otherwise.

**46:06** · I actually don't agree; I just feel that it was a matter of luck at the time.

**46:11** · For example, in the past two years, I haven’t won as many awards for my papers, but I don’t think the quality of my papers has dropped that much. So looking back at having published so many good papers, how do you think it’s possible to continuously produce this kind of good research?

**46:26** · I actually think doing research is very simple.

**46:27** · If you hire the right student, they will naturally do top research. Doing top research is about a person's quality; with that quality, they can produce very good work. So the real issue here is how to hire the right person and then how to help this person understand the method of doing this work. As long as you do these two things well.

**46:46** · After that, there are some results. I think let's break down this step. First, how do you recruit this person, and second, how do you bring this person in? Let's break down how to recruit this person. I think it's completely based on gut feelings for me because I'm not someone who thinks about things in detail. For example, when I recruited students, I just chatted with them for a while.

**47:06** · Then see if there is any chemistry because personally I don’t believe I have it.

**47:12** · I have a friend, this friend of mine is really interesting. He just wants, he uses dating apps, and after he uses dating apps, he often goes on dates with others, right?

**47:23** · Then think about how to go there, and I asked him if dating apps are accurate?

**47:25** · Because dating apps are basically where you initially arrange to meet someone and spend a night with them. In the end, whether this person will become your long-term partner, he says, even spending just one night isn’t acceptable, right?

**47:36** · Then I think, as a student, you interview this student for half an hour, but could this person be someone you have to work with for the next five years?

**47:41** · So I think this is too difficult, so it's more about seeing fate or chemistry, just looking at some gut feelings of yours. Let's break down the second half: the student is recruited, so how do you help him publish good papers? Publish papers, but the same sentence: that's not our goal, right?

**47:55** · So how should we...

**47:57** · I think it's about letting them do the research they want to do, because I think every student...

**48:01** · His goals are also somewhat different.

**48:02** · Some students might want to start a startup; he doesn't necessarily say they have to publish papers. I think that's also a very meaningful thing. But maybe the research he wants to do is different from a student who says they want to go to another school to become a professor or to make some breakthroughs in fundamental research. The ways to train these two people are different.

**48:18** · It should be very different, so I don’t think there is a magic formula. I just think that if you have a very deep connection with this student, you should be able to understand what this student really wants.

**48:29** · What does he like to do and what is his ultimate goal?

**48:32** · So what things can make him very happy and what does he need?

**48:34** · Then after you have sorted all these things out, I think you should be able to help this student, and I think anyone should be able to help this student. But the most difficult part is how you can establish this connection, how you can understand this student, and then help him find some problems or resources or something.

**48:47** · Many times it is also very difficult; I think the hardest part is when the student says, 'I don't even know what to do.'

**48:51** · I don't know what I like either.

**48:52** · At that stage, you have the student try different directions.

**48:56** · Then you ask which ones they genuinely enjoy, and use your experience to help them choose a topic.

**49:01** · Listening to you describe it, I'm reminded of the classic master–apprentice tradition in research.

**49:09** · You know?

**49:10** · There's something beautiful about that kind of relationship.

**49:14** · Today, many professor–student relationships feel more like employment.

**49:18** · But in the older European and American tradition, the model was different: established scholars personally guided students through an apprenticeship.

**49:27** · From what you've described, that kind of relationship is something AI agents cannot replace.

**49:34** · Right.

**49:34** · You understand the person.

**49:36** · Exactly.

**49:36** · And you can truly work with them.

**49:37** · Right.

**49:37** · Then together you can do meaningful work that advances technology.

**49:40** · What could be better than that?

**49:41** · It's genuinely enjoyable, I think.

**49:43** · but it’s just about doing interesting things, right?

**49:45** · Life is very happy, really great.

### Fight Dragons, Not Windmills：解决真正重要的问题

**49:48** · Even though you are very humble, always saying 'best paper', it doesn't matter, but on your homepage you said a sentence: 'fight dragons, not windmills', which means to conquer the evil dragons, not the windmills, right?

**49:59** · How does that windmill facing the wind manifest in your interactions with students?

**50:04** · I think that the evil dragon actually represents a real problem, while the windmill represents a false problem.

**50:11** · So I believe that it is very important to solve a problem that is truly meaningful, and solving a false problem, I think, doesn't have much significance, especially for systems research, because systems research is something very close to practice. So if this problem is not very important.

**50:24** · I think going to solve it might not be that meaningful.

**50:27** · Two points: the first point is how you help students discover this problem; the second point is, in a cornfield in Illinois, Champaign, so far away from Silicon Valley, how do you discover the real problem? This is a very profound question.

### 如何从小问题挖到基础而深刻的研究问题

**50:39** · I think, although this... firstly, the second question is quite simple because I think...

**50:44** · Even the most important problems in the field of systems are just a few problems; there aren't that many fundamental problems. But the manifestations of these fundamental problems in actual systems are varied.

**50:52** · So personally, I always have a view that many people disagree with, but my view is that I think there is no such thing as a small problem, only some problems are not explored in depth. in other words, for me, there are only that many fundamental problems.

**51:06** · Then there are some fundamental issues in this, and in the practical implementation of the system, its reflection may not be the same.

**51:13** · Some problems may seem big, some problems may seem small, but if you dig deep enough, if it is a difficult or important problem, it will inevitably return to a very fundamental deep issue.

**51:24** · For example, many reliability issues ultimately come down to a few fundamental problems, such as how to reason about concurrency, right?

**51:30** · How to reason about asynchrony and then how to reason about some nondeterministic behavior. If you solve this problem, then the higher-level problems are all solved.

**51:36** · But these problems are very difficult, so even if you have a very small problem, you can eventually make it very big. And actually, I suggest this to students.

**51:47** · At the beginning, start with a relatively specific and smaller problem, rather than going for what we call 'boiling the ocean.' If you start too broadly, it will be difficult to handle.

**51:54** · It's better to be focused with a specific issue, which will make it easier to manage. You want to dig as deep as possible, but just do your best, and work to whichever level you can, then move on to the next project.

**52:05** · Can we go on to solve some more fundamental problems, right?

**52:08** · This is the second one, this is your second question I think, and then the first question is, what is the first question?

**52:13** · It's about how you help students find this most important question, right?

**52:16** · this, as I said, depends on the student's interest. Some students may prefer theoretical questions, or some students might really enjoy reasoning-type things. Some students really like hacking; their greatest strength is being able to hack a very complex thing very clearly. At this time, based on their interests, you can help map it to something for them.

**52:39** · Compare the more important issues in reality and then start working on those issues. These issues include some students in my own group who like to do formal verification, right?

**52:47** · Formal verification means you need to define the specification, use the correct tools to write proofs, and then finally the entire system can be formally verified. Some students prefer the empirical method, which might include ourselves; I myself, when doing my PhD, looked at many empirical cases.

**53:02** · Then think about how to solve these problems through some automatic methods. I like the second one. And when you were at UIUC, you took on some responsibilities, like teaching the operating systems course.

### 如何教操作系统：用 Linux 内核训练复杂工程能力

**53:15** · Regarding responsibility, your operating systems course, I saw online that students rated it as really awesome, like extremely impressive. There is nothing, nothing, nothing—it's just about how you teach the operating systems course and why students like this course so much.

**53:29** · I feel like I don't know why students like it either. Maybe it's because I told a few jokes in class.

**53:35** · I think I should tell everyone about it.

**53:38** · I feel that this operating systems course doesn't really teach operating systems, at least for me, because at UIUC, the operating systems course is actually divided into two courses. One course is called system programming, and it basically teaches everything in operating systems except for the kernel.

**53:52** · It's just that it doesn't teach kernel programming, but it teaches everything else, including concurrency, process control, memory control, all those things. This course is also very good. It was taught by a colleague of mine named Lawrence Angrave, who manages it, and it's very good.

**54:04** · And for the major assignment, it might be like having you write a malloc, right?

**54:08** · Then they have you write a shell and these kinds of things. Actually, this course has already covered most of the basic training. The next course is actually about kernel programming. But if you look at the students taking the kernel programming course, I think less than 10% of the students each year, maybe even less than 5%, I believe that after graduation, these students will go on to do it.

**54:29** · For example, someone coming to the Bay Area to be a Linux developer—people like that—5% of them, 95% of them, after they graduate, they won't end up being a kernel developer. What they might do later is, for example, hacking Kubernetes or building some AI infrastructure, right?

**54:44** · So what exactly did Linux kernel hacking teach him?

**54:48** · So I think the reason why this is valuable to them in this course is not that it completely explains this one thing. Kernel hacking is a medium, but the skills they gain are how to manage this complexity. Because before this OS course, basically all the courses you took, for example when doing big assignments, you were building something from scratch, right? Like your data structures.

**55:10** · You write a small data structure, in your algorithms class you write a small algorithm, and even previously you wrote a small malloc, wrote a small shell; they are all very small things, but what about our class?

**55:18** · You are going to hack Linux, but Linux is a very complex and very dirty thing. For a student, in just one semester, it is actually very difficult to fully hack and understand it.

**55:28** · So, in this process, how can you make progress? Actually, this skill set is very useful in a company, including for myself when I went to Facebook.

**55:34** · I am facing a huge codebase, and then you want me to be able to make progress in a month and build some things. So I need to have the ability to abstract a lot of things, understand the complexity, and still be able to make progress in order to communicate, right?

**55:48** · Then the documentation isn't there, and the code is all very hacky, and this is a problem that exists in Linux as well. Of course, Linux documentation is actually quite good, but some things in device drivers are not so, not so elegant.

**56:00** · And I think that if a student, after finishing this course, can gain the confidence to face, as you just said, a fierce dragon or a world that is not so perfect.

**56:10** · I think this is the biggest skill that this class teaches them, so I think teaching OS is meaningful, not that I want to train all 100 of them to become kernel hackers.

**56:20** · I feel really deeply about this, haha. Having graduated and worked for three to four years, I really increasingly feel that if you are an undergraduate majoring in computer science, courses like OS, operating systems, and network systems really need to be studied well. What I regret most now is that I didn’t study these courses thoroughly enough because you will find that no matter how big data infrastructure, cloud infrastructure, or even AI infrastructure develop now.

**56:43** · It hasn't escaped the original one, which is the graphical architecture, including the basic ideas of computing, storage, and networks, but?

**56:52** · It's about how you make the complex things in your mind become abstract.

**56:58** · Right.

**56:58** · It is about becoming clear when working on a problem, such as taking on a major task at a company or even a big entrepreneurial task, breaking it down step by step into the smallest units, and then stacking these units back together step by step.

**57:10** · Yes.

**57:11** · I found that I completely agree that the greatest significance of these courses for me is actually that I think these courses teach engineering thinking, computer engineering thinking, which will not be affected by external influences. As long as you have this kind of thinking, no matter how AI develops, you will be able to find a highly valuable job in this society.

### AI 时代计算机科学会不会消亡

**57:31** · I actually agree with what you said, that there are some arguments now.

**57:34** · It means don't study CS anymore. CS has already collapsed. I don't know how to put it. I think maybe for some parts, like the simpler kinds of software engineering development, the demand is indeed getting smaller, but overall, for building these big kinds of projects, I think the CS discipline will still not disappear. This discipline will not disappear. I don't think it will disappear.

**57:57** · Because I feel that such big statements are all comparative, and I find it hard to understand what the argument behind them is.

**58:05** · I personally do feel that the CS job market isn't as big as it used to be because AI can take over a lot of tasks, right?

**58:11** · In the past, even some very simple tasks required you to hire a CS undergraduate student to do them, but now AI can help you with that. So I think, if what we are worried about is that this market will shrink, I do think it will shrink. But if you say that OS will disappear, I don't think that's possible. It is still something that requires people, just like any other profession, it will still require people to do the work in the future.

**58:31** · The entire current modern society is building large-scale computer systems.

**58:36** · Right.

### AI SRE 能做什么，人类又该扮演什么角色

**58:38** · Then let's talk about the relationship between AI and these systems next.

**58:43** · You've been studying for over ten years. From what I see, the main thread of your papers is actually quite clear to me: how to find problems when systems have issues and how to fix them, mostly focusing on reliability and mostly on fault tolerance. in other words, there are now many kinds of AI that are essentially SRE.

**59:01** · It's about letting AI debug this, which is particularly popular right now, super, super popular, even getting funding is popular, and then.

**59:06** · Yes.

**59:06** · Is this the one that's popular in academia too?

**59:08** · I wonder if it's not popular in academia?

**59:11** · I don't think the fire affects that. Maybe we can talk about it later. So, what do you think these AIs are actually capable of doing in terms of human SRE work, and what can't they do? Where are the boundaries?

**59:21** · Actually, I think the capabilities of AI now, right?

**59:24** · Regarding the capabilities of those current frontier models, like Fable or that GPT-5.6 Soul, right?

**59:31** · Then I think their abilities are already very strong, so basically I feel that for any system task, Siri task—I’m not saying, I mean any system task—if you tell AI how to do this task, AI is basically capable of doing it. So if the question is whether AI can do these tasks, I think it definitely can. The key question is how you go about doing it and what the cost is, I think.

**59:52** · in other words, if we can lower this cost, right, and also if we can have a more general method to tell AI how to perform these tasks, then I think AI can definitely do these things well. I don't think AI would do poorly, and I even don't think AI lacks the abilities to do these things.

**1:00:08** · So, what role does a human play in this?

**1:00:10** · I think the role a person can play in this is what?

**1:00:13** · I have always had a thought, and I don’t know if it’s a very stupid one, about guiding AI to do some right things. Because for me, AI is like a very smart person. It’s like in a company, let’s suppose your company has an enormous number of people, and all these people are very hard-working, you still need to tell them how to do the right thing.

**1:00:33** · I think that's the role people should play right now: showing junior engineers what good work looks like.

**1:00:40** · how to go about doing this, how to think about your trade-offs, and what you should do?

**1:00:44** · How to decompose this task and then tackle it step by step, then let AI handle some of the repetitive work. But I don't think AI—at least with its current capabilities—if you just give it a very vague definition or specification, it can actually do it, but it may require many iterations, which would be very costly. And this introduces another problem.

**1:01:02** · It's just that people go to guide these AI agents and then let them complete some tasks, and moreover, they are experienced and tasteful people.

**1:01:11** · So the question arises: how can a fresh graduate become such a person?

**1:01:15** · Then there is no such training guidance process, and I think this is a huge impact on current CS education.

**1:01:24** · I think, let's imagine how we could change this to help current CS students gain this experience.

**1:01:30** · I think that now students should use AI to do some learning and work. I actually don’t really understand why these things are considered difficult to learn now, because originally, as a senior engineer for example, you accumulated that expertise over years of hands-on learning.

**1:01:50** · what they need to do is to...

**1:01:53** · By guiding AI to do this, as a learning student, you just practice this. You practice your learning through AI and also practice your research through AI, then naturally you will reach this level.

**1:02:05** · There are two problems: the first problem is that you are too junior.

**1:02:13** · Some of your taste and guidance may not be enough, and then AI output might tell you some things that are not really high-quality or high-level. That's the first problem. The second problem is that these positions are becoming fewer and fewer. Originally, people would slowly progress through small positions, which would allow them to learn interpersonal communication and improve themselves.

**1:02:34** · Right.

**1:02:35** · So these two issues lead me to think that the first issue is what I mentioned before: I feel that people are still a very important element in this, and we shouldn't imagine an action that is completely without humans. So I think things like advice or mentorship will not be replaced by AI. At least, that's my point of view. in other words, if you have such a growth environment, you can still achieve something using AI.

**1:02:56** · Very good quality, and then the market has shrunk, which is a very real problem, but actually I don't really know how to solve this problem.

**1:03:05** · Then let's all reflect and discuss together, so that the audience can chat in the bullet comments and the comment section.

### SREGym：在真实故障环境中评估 AI 运维能力

**1:03:11** · Right.

**1:03:11** · You've recently begun working on AI for systems.

**1:03:16** · One project I found especially interesting was your work on AI for SRE.

**1:03:23** · You built SREGym, an environment where AI agents tackle site reliability problems.

**1:03:32** · It contains 102 real-world IT operations scenarios, yet agents solve only 11.4 percent of them.

**1:03:42** · What did you observe?

**1:03:44** · Why could the agents solve so few?

**1:03:46** · What were they missing?

**1:03:48** · Those results were from about a year ago, I believe.

**1:03:51** · Yes.

**1:03:52** · First, I should point out that my students led this project.

**1:03:55** · I didn't initiate it, so I can't claim it entirely as my own work.

**1:04:01** · As for the original benchmark, I think today's models have largely solved those tasks.

**1:04:09** · Run it again with the latest models, and I think they'd score close to 90 percent.

**1:04:13** · That much progress in one year?

**1:04:14** · Yes, in just one year.

**1:04:15** · AI capabilities have advanced dramatically.

**1:04:17** · AI's capabilities, anyway, there was a big fluctuation at the end of last year, that is, Fibonacci is not correct, not 4.4, 4.6, 4.8.

**1:04:23** · Right.

**1:04:24** · At the end of last year, I felt it was the end of last year and the beginning of this year, that is, the end of 2025 and the beginning of 2026, I felt there was an essential capability, an essential leap, and also, recently this year, anyway, a new wave of models came out, and I think its ability has increased again. I felt that during last summer, its ability was still average, and this work was actually done at the end of the year before last, I think.

### Benchmark 如何随着模型能力持续升级

**1:04:46** · So at that time, AI capabilities were not that strong, and there were various problems, including very high reasoning hallucinations, and its context was not that large, so its capabilities were very limited. But now, these problems that existed before are actually very easy for AI to overcome. The fundamental issue here is that the problems we originally wrote were very simple.

**1:05:08** · Because I think the practice of benchmark research is that it gradually builds the benchmark according to the capabilities of AI, right?

**1:05:16** · Because a benchmark, a good benchmark, can measure the boundary of AI, so when AI capability is very weak, if you create an extremely difficult benchmark and AI cannot complete any of it, then this benchmark is actually not good. And of course, if an AI is very strong and can complete 90% of the benchmark, then this benchmark is basically a bit useless as well.

**1:05:31** · So the best benchmark is when you can say that 30%-40% can be done by AI, and 60% AI cannot do, so it can keep challenging AI. Now, actually, the biggest challenge of our benchmark is this, and a very clear problem is that a benchmark itself is originally a small, small environment, right?

**1:05:48** · As for us, we can only have a small environment. Although our benchmark can actually have some cases, for example, a system with three nodes or five nodes, it is still very different compared to hyperscale systems like Meta's. So AI is very strong in these small environments, but whether it is strong in these huge environments, I think, is not necessarily the case. But how do you scale a small one...

**1:06:09** · How does the environment simulate a huge and complex environment?

**1:06:11** · Which things should be simulated in this small environment and which things can you abstract out, and this is a point we pay quite a lot of attention to, and this is actually quite difficult, quite difficult.

**1:06:23** · Right.

**1:06:23** · Because since you go from a big system to a small system, you inevitably have to throw some things away.

**1:06:27** · So what do you throw away, right?

**1:06:29** · What I mean is that if the things you throw away are very important, then this small simulator of yours becomes useless.

**1:06:34** · But if you want to keep everything, then essentially you can't turn a big system into a small one. Yet you need the small one because if you want to do RL, you can't do it if it's too complex or online. So I think this is one of the challenges of SRE.

**1:06:47** · Then the coding benchmark doesn't have this problem because the coding benchmark is an offline thing—you just keep generating code, right?

**1:06:51** · Then going to do it for SRE is too real-time, meaning that after you solve a problem, can it really solve the issue, fix the online bug, and get feedback?

**1:07:03** · And the whole feedback process isn't offline, it's online.

**1:07:05** · So what we want to solve now is this issue of fidelity. Can you explain this in detail?

**1:07:10** · It's about how to simulate a specific production failure.

**1:07:16** · For example, recently our school has a program, an undergraduate research experience program called the REU program, and this summer our program is letting those undergraduate students...

**1:07:24** · What did he do?

**1:07:24** · He just went to read a postmortem, this postmortem could be from those high-tech companies, like Cloudflare or something.

**1:07:31** · Then they announced some failures, meaning they said that at a certain time they had a failure, and they put the detailed description of it online. Then the students, based on their description, implemented the failure in this relatively small simulated environment and tested AI, and we found that even though this was a small environment, it still didn't… It can completely represent this larger environment, but compared to the most basic failures the realistic failures you described pose a very different challenge for AI.

**1:07:57** · when facing these more realistic failures.

**1:07:58** · I heard two points of information: the first point is that I think students can indeed gain a lot of training in this process, and the whole school helps them to find it.

**1:08:11** · He can get hands-on with some of the real problems of these big companies in advance. The second point is that in the process of abstracting it into a small simulator, this simulator can later be used for RL.

**1:08:24** · Right.

**1:08:25** · this, nowadays this kind of RL environment company simulator company is really popular in Silicon Valley, right?

**1:08:31** · It's too hot, too hot. Any environment that can truly help this model improve its capabilities—issues that previous models couldn't solve can be solved after training on it—then this environment will be sold at a very high price. I see.

**1:08:46** · This is something that has already become an unspoken convention in Silicon Valley.

**1:08:50** · So, regarding this, moving forward, SRE.

**1:08:53** · This environment will increasingly grow, I can feel it will grow more and more, and then AI needs to learn through such an environment to continuously study the ability to solve industrial-level production problems in the real world.

**1:09:07** · Yes.

**1:09:08** · That's great, it feels like UIUC is already ahead and has started training students.

### AI 加形式化方法：把代码转化为形式化模型

**1:09:14** · My next question is about another project.

**1:09:15** · Before this interview, you also published a new paper.

**1:09:18** · I found that work fascinating.

**1:09:19** · It combines AI agents with formal methods to improve testing.

**1:09:25** · Right.

**1:09:26** · Could you walk us through how it works?

### Model Checking 入门：Safety、Liveness 与系统抽象

**1:09:30** · Sure, what we are actually doing with model checking is... what is the essence of model checking?

**1:09:36** · The essence of model checking is that you have a very detailed low-level code, and then you need to write this code into a high-level abstraction, and this abstraction is described using a mathematical language.

**1:09:46** · This mathematical language can have various forms, right?

**1:09:48** · The language we are using, called TLA+, was written by Leslie Lamport, whom I mentioned earlier. And if you can describe this system using a formal language, then you can do a lot of formal analysis called formal reasoning. You can reason about many properties of this system, such as safety.

**1:10:07** · Safety and liveness: safety obviously means that bad things will never happen, and liveness means that good things will eventually happen. But as long as you describe it as a formal model, you can check these properties. The most difficult part here is how do you?

**1:10:22** · You're going to write a high-level description of a very low-level system, right?

**1:10:29** · What about an abstraction?

**1:10:30** · So this matter is very intellectually challenging, and for a PhD student, this could be four years of their time, four years of their life. Before AI, we had a project and a very talented student who is now a researcher at Microsoft Research; he spent a full five years doing this work.

**1:10:47** · Why does he have to spend five years? It's because he has to learn this formal stuff from the beginning. TLA+, this formal stuff, is actually not taught in any undergraduate courses, right?

**1:10:55** · This wasn’t taught in the course, right?

**1:10:56** · You might spend a year or two to learn this set of things thoroughly, and then what happens after you have learned it?

**1:11:00** · You also need to write a specification. We say that specification is a very overloaded term. Actually, you are writing a formal formal models and invariants.

**1:11:08** · Invariants are the properties you want to preserve.

**1:11:10** · To write them— to build a good model of a system— you must understand that system very deeply.

**1:11:16** · Otherwise, you can't abstract it properly.

**1:11:18** · You have to know the system inside and out.

**1:11:20** · For a systems researcher, that should be a real system.

**1:11:24** · Let's use an example.

**1:11:25** · Uncle Moon, what's your favorite system?

**1:11:27** · A distributed system?

**1:11:28** · My favorite distributed system would be Google's Borg, or perhaps Kubernetes.

**1:11:32** · Kubernetes. Okay.

**1:11:34** · When we started, we used ZooKeeper.

**1:11:35** · ZooKeeper is a consensus system.

**1:11:38** · already very complex.

**1:11:39** · It has a consensus protocol called ZAB. So, what about this student?

**1:11:42** · He has to go understand all the details inside ZAB and be able to understand how ZooKeeper implements ZAB, and then understand all these things?

**1:11:49** · He can go and write a formal model, and after he finishes writing it, your system is changing, your code is changing, and tomorrow there will be a new release. You still have to follow the evolution of the code to write a different model, right?

**1:12:01** · This matter is very costly and also requires a lot of skill, so the student named Ruizé, Ruizé, spent four or four to five years writing all of these things, and after finishing writing, then?

**1:12:09** · You can plug in all those formal and very good tools and then let them translate and then discover the problems inside. You can do model checking, and you can also do proof. Proof is a stronger guarantee, but this requires doing for 5 years, and in 5 years we only completed one system. But after we completed this system, then what?

**1:12:27** · We solved a lot of problems in this system that hadn't been solved for many years, right?

**1:12:32** · There was one issue, I remember, in ZooKeeper, and many people had studied it before and it was very complex.

**1:12:36** · But through formal reasoning, it could clearly tell you what was going on. However, it was very expensive; it would require a PhD to spend five years to do this.

**1:12:45** · But now, with AI, it's different.

**1:12:47** · It turned into a five-hour job—is Is the improvement really that dramatic— down to just five hours?

**1:12:50** · Yes, five hours.

**1:12:52** · Five years of work became five hours, because Five hours of work because of AI — it doesn't need to learn all these formal specifications; it has already learned them. Now, frontier models can write TLA+ really well, just like writing in C or Python, very skillfully, right?

**1:13:06** · Moreover, AI actually has a deep understanding of the system because it has already learned all the source code and the like. If you give it the code again, it can immediately learn from it, so all this time is automatically handled by AI, meaning it naturally already knows it. The only thing not solved is the previously more intellectual tasks.

**1:13:22** · The most difficult part when you are modeling this system is how exactly you go about modeling it.

**1:13:27** · Because if you still stick to that idea of wanting to keep everything, in the end, you won't be able to build this model, because you want everything. Your model will become large enough that you won't be able to do formalized things. But if the model is too simple, you won't discover any problems either. Deciding what should be modeled and what shouldn't used to rely on the experience of skilled veterans.

**1:13:45** · Or rely on a very talented PhD student, on his intuition, his intelligence.

**1:13:48** · Now we are handing this over to AI. We say, AI, you're quite smart, so can you help us figure this out by yourself and help us figure out the method for this?

**1:13:59** · Because a mature open-source project has gone through many years, it actually knows which parts are tricky and which parts are unlikely to have problems, right?

**1:14:10** · Then this is actually us.

**1:14:12** · When human intuition comes into play, it's easy to make mistakes, so I want to model it more precisely. On the other hand, I think there’s no big problem in another area, so I won’t model it. So originally we were writing this model, and now we just tell AI that based on your experience, this kind of code is very difficult, very error-prone, and very easy to go wrong, so you should model it more carefully.

**1:14:31** · That place may have never had any problems in history, so you don't need to model it. You just abstract it out, and AI actually does this very well. So after running it once, AI can actually write a very good model, and this model can conform to your interpretation. Once you get this whole system mature, you can leave the subsequent work to it.

**1:14:48** · Traditionally, formal methods meant model checking or constructing proofs.

**1:14:51** · Of course, the main challenge is that AI isn't human.

**1:14:56** · It may engage in reward hacking.

**1:14:57** · It can produce a model that looks correct, but isn't what you asked for.

**1:15:03** · So you need ways to guide or even constrain AI, preventing reward hacking and helping it understand the system step by step, until it can complete the task on open-source distributed systems.

**1:15:19** · It takes only five hours.

### 从五年到五小时：AI 如何加速形式化建模

**1:15:22** · That's a lot to take in.

**1:15:23** · First, has productivity in this field really improved that dramatically?

**1:15:27** · From five years to five hours?

**1:15:29** · Yes.

**1:15:29** · That's why AI for formal methods is now such a hot field.

**1:15:33** · Formal methods themselves have existed for decades.

**1:15:37** · I've always believed fundamental research matters.

**1:15:41** · We can move this fast today because earlier researchers laid the groundwork.

**1:15:44** · The road was already there.

**1:15:45** · The reason it wasn't widely adopted in industry was the cost.

**1:15:51** · It required highly specialized expertise, which made it expensive.

**1:15:55** · It didn't fit Silicon Valley's “move fast and break things” culture.

**1:15:58** · But at NASA, this approach makes perfect sense.

**1:16:02** · They verify their systems, because sending a spacecraft to Mars only to have it fail would be extremely costly.

**1:16:09** · They even avoid multithreaded code because concurrency is so difficult.

**1:16:11** · They use single-threaded code and verify it formally before deploying it on missions like Mars Pathfinder.

**1:16:15** · That approach doesn't work in Silicon Valley, where competition rewards speed.

**1:16:19** · But once AI automates formal verification, the economics change completely.

**1:16:24** · That's why so many AI-for-science startups are emerging.

**1:16:28** · For a general audience, what does this mean for companies of all sizes?

### 形式化验证对企业的价值：从事后调试到事前证明

**1:16:34** · It is something that can help you solve some things that are very difficult to do through human reasoning alone. Now you can use some mathematical tools to help you verify or check certain behaviors in your system. Are these things normally impossible to find through repeated debugging and trial and error?

**1:16:56** · Debugging is a reactive approach, meaning you only debug when a problem occurs, right?

**1:17:01** · But our dream is to ensure that the failure never occurs.

**1:17:05** · If we can formally verify the entire system, then that class of failure cannot happen.

**1:17:11** · Verification means we have ruled it out.

**1:17:14** · We have mathematically proven that the system will not suffer the failure we're targeting.

**1:17:20** · Before, I understood why many companies found the return on investment too low.

**1:17:26** · Formal verification took too much time.

**1:17:27** · Right.

**1:17:27** · It could be cheaper to learn through failure.

**1:17:28** · Exactly.

**1:17:29** · Deploy the system, and then buy insurance, if you have to.

**1:17:34** · Right.

**1:17:35** · If a failure happens only once every ten years, buy a large insurance policy.

**1:17:37** · If it does happen, let the insurer cover the loss.

**1:17:39** · Right.

**1:17:39** · But now, instead of buying insurance, a formal proof can guarantee the failure won't occur for ten years— or even a hundred.

**1:17:46** · make this mistake. So you don't need to buy insurance and you can happily run there. I had a colleague before who used to help some companies do this, for example, helping large cloud computing providers.

**1:17:55** · Right.

**1:17:55** · Every time it goes down, it's very expensive, right? Like that kind of EC2 going down would make half of the U.S. unable to access the internet. That means the whole service is down, so they have to maintain a team specifically to handle this. Before AI era, the most enthusiastic about this—and still the most enthusiastic now—is Microsoft because they have Azure, and reliability is very important. And then there's AWS. Basically, it's just these two companies.

**1:18:17** · It is nurturing the team.

**1:18:18** · Yes.

**1:18:19** · But after having AI, I feel that this barrier has become very low, and then many people can do this. So what do you think will happen to the cost?

### 形式化证明的成本，以及为什么先选择 Model Checking

**1:18:28** · How many tokens it would consume is to say that the issue of generating these things and the value produced in the end is more technical, because it depends on what you want to achieve. I think that if you want to do a complete proof, at least for now, it is still very expensive. Previously, we didn't work on that TLAPS thing, right? He had written this for five years before, and now he wants AI to write this proof.

**1:18:50** · Then he ran a ZooKeeper system spec. A system spec is a specification that describes the system; it is not a specification that describes the protocol, so it is relatively low-level. Then he ran it at that time, and it cost $2,000 to run for 5 days.

**1:19:02** · Well, I think this is still...

**1:19:05** · A relatively expensive expensive thing, but when we chose to do this project at that time, we did not choose to do proof; we chose to do model checking.

**1:19:11** · Model checking is relatively cheap and you can get feedback immediately. I thought we wanted to make a tool at that time, so I felt that such a tool would be more practical.

**1:19:20** · It's somewhat more practical than doing a proof, or even more, more, more practical, at least at the present moment. But we also want to do some proof to make some investment for the future. To further understand it, many companies can start adding this during the process of developing their own systems; it's completely possible to use this system while developing.

### SDD：用规格驱动开发约束 AI 生成代码

**1:19:42** · Yes.

**1:19:42** · Go and continuously verify.

**1:19:43** · Yes.

**1:19:44** · In that case, he won't have any problems with every step.

**1:19:45** · Yes.

**1:19:46** · It's just because we all work on systems and know that premature optimization is the root of all evil.

**1:19:50** · Right.

**1:19:50** · Yes, but during the process of building the system, you start applying it right from the simplest MVP process, and then each time you add something, you put it in, and with each addition, it seems like a very beautiful scene.

**1:20:02** · Yes.

**1:20:03** · Especially now, AI agents, whether under someone's supervision or not, even if someone is watching them, you no longer look at their code. Many people do this, right?

**1:20:09** · I directly threw out a requirement and immediately a software came out.

**1:20:12** · Right.

**1:20:13** · If, under this circumstance, we can provide a service that helps these various people check the messy AI systems they have written before going online, it seems that many problems could be avoided.

**1:20:27** · Yes.

**1:20:28** · I think the concept that everyone is talking about now is called SDD, which stands for specification-driven development.

**1:20:32** · Right.

**1:20:33** · in other words, you first write a formal specification, and then each time you let AI write something, you are able to prove it or verify that the code you wrote satisfies your specification. In this way, no matter what AI does, the final product you produce can be verified. So if this can be done, I think it can reduce a lot of AI-related problems, I think.

**1:20:51** · Whether from an academic or entrepreneurial perspective, this is a very good idea. I think now there are many startups working on AI for formal methods; it just depends on what your anchor point is, applying various formal methods to systems built for the current AI agents.

**1:21:10** · Right.

**1:21:11** · Go verify it, go prove it. I find that it really requires constant communication with everyone. For someone like me who has never done this before, I was completely blank on this. Just now, you probably already explained it to everyone in the simplest language. Although I still have a lot I don't understand, I can sense that this is now feasible. But what I want to point out is that this is how I feel now.

### 形式化验证的鸿沟：最难的是定义正确的需求和规格

**1:21:31** · How should I say this?

**1:21:32** · in other words, for one of the very difficult questions, you can see that there are now two voices, right?

**1:21:35** · For example, right now there are two opinions in the field of reliability. The first opinion is that in AI era, the biggest problem with using AI to build systems is reliability and security.

**1:21:43** · Yes.

**1:21:43** · You can't guarantee it, and you are particularly anxious, you are particularly anxious, this matter is too alarming.

**1:21:49** · But there is also a voice saying that now systems can be formally verified, and this process has already started, taking 5 years to be reduced to 5 hours or 5 minutes. So why is there such a huge gap here?

**1:21:58** · Here, if the first thing is true, the second thing should not be true.

**1:22:02** · Why is that?

**1:22:03** · The reason lies in the fact that even if you can formally verify it, it only tells you that your system code is consistent with the specification you need; that is actually all it does. But I still think it is a very difficult problem, which is what kind of system you need and what your requirements are.

**1:22:19** · What kind of specification you want, I think this matter is very difficult to solve completely relying on AI; it is hard to solve, and it requires experienced people to handle this matter.

**1:22:30** · The higher-ups need someone to guide the completion of this specification. Yes, this matter is quite difficult because sometimes at the beginning, you want this system, but you yourself can't figure out what kind of system you actually want, right?

**1:22:40** · Right.

**1:22:40** · You might, at the beginning when you set up this system, say that mine is within this formal specification, there is something called a fault model, which means you envision how your faults will look, and then you can build a set of fault-tolerant systems. But maybe tomorrow or the day after, you find that there are some issues in your infrastructure.

**1:22:55** · What about the other faults described by your fault model?

**1:22:59** · So defining this specification is actually not that easy, I think it's too difficult, too difficult.

**1:23:04** · Right.

**1:23:04** · So the first half of your entire career was spent designing these reliability, that operating system, making it harder for people to make mistakes, better helping people solve problems with these systems. Now you can see that you are gradually starting to design some things that connect with agents, that is, allowing agents or various AI tools to help people do this maintenance and build better systems.

### 理想的 Agent-native 系统：容忍错误、提供反馈、持续恢复

**1:23:29** · Some system stuff of agent-native, hmm, so what's your ultimate, for example, ideal?

**1:23:35** · in other words, in the end, you might not be able to reach the state you hope the computer operating system will become.

**1:23:43** · I think this question is quite deep, and I still believe it should start from the perspective of reliability. A reliable system is one where, in the future, at least 80% of its users are agents, and agents are not perfect—they can make mistakes, right?

**1:23:58** · Then it might hallucinate, and then it might be off-base, so how can your system tolerate these things and also provide some feedback? Because the agent is also a very smart thing; as long as you give it feedback, it will do better in the second round. And I think there are There are many systems problems that still need to be solved.

**1:24:17** · Without solving them, this vision won't become reality.

**1:24:20** · I feel the same way.

### ROC：面向恢复的计算与“系统一定会出错”的现实

**1:24:22** · The future of these systems looks incredibly promising.

**1:24:26** · But when you examine the details, you see how many hard problems remain— how many PhD students and systems engineers it will take to solve them one by one.

**1:24:34** · It feels a little like the Manhattan Project.

**1:24:36** · I think many of the core principles already exist.

**1:24:40** · Then I don't know if you know have you heard of a project called ROC— No, I haven't.

**1:24:46** · Around the year 2000, David Patterson, then a professor at Berkeley— a Turing Award winner and one of the pioneers of RISC-V— led a project called ROC: Recovery-Oriented Computing.

**1:24:57** · Its central premise was this: computer systems inevitably fail.

**1:25:02** · In other words, even if you spend all your resources on testing and formal verification, the system will still fail.

**1:25:11** · So we should invest substantially in recovery, rather than assume a system will always be reliable.

**1:25:20** · Instead, ask: what can we do when the system does fail?

**1:25:25** · Can we build a system that recovers quickly after a crash?

**1:25:32** · This is their big project. It is a collaboration project between Stanford and Berkeley. At that time, they received a lot of money from the NSF, and they had They had many ideas that could not be realized at the time.

**1:25:41** · The first idea was to look at what makes a good system fail.

**1:25:45** · Their approach was human-centered.

**1:25:47** · They studied real production systems, and asked a fascinating question: when a system fails, is the hardware at fault?

**1:25:57** · Is it the network?

**1:25:58** · Or is it the people operating the system?

**1:26:00** · When we think about fault-tolerant systems, we usually assume the hardware failed, a machine crashed, or an Intel processor suffered a bit flip.

**1:26:08** · Something along those lines.

**1:26:09** · But their empirical study found that the largest share of failures came from human error— operator error.

### 人为操作为什么是系统故障的主要来源

**1:26:15** · People have to operate these systems, and the systems are complex.

**1:26:19** · So people make mistakes, and those mistakes are often the leading cause of system outages.

**1:26:23** · Jim Gray explored this issue.

**1:26:24** · He later won the Turing Award and wrote a seminal paper on the subject.

**1:26:27** · At the time, he was with a Silicon Valley company called Tandem Computers.

**1:26:30** · Its product line was the NonStop system.

**1:26:33** · The company aimed to build a system so robust that it would never stop— hence the name NonStop.

**1:26:39** · Gray studied it in depth.

**1:26:42** · The company had invested enormous effort in reliability, yet the system still failed.

**1:26:46** · So he wrote a paper based on an empirical study.

**1:26:48** · He pointed out that the company's core selling point was reliability.

**1:26:52** · So in the end, when we sold it to the user and it crashed, what was the actual reason? Then he found the same problem, which was that the main reason was human operational error. It wasn't the hardware; the hardware wouldn't crash. The system software is already capable of fault tolerance, right?

**1:27:03** · Then David Patterson, anyway, just pushed this matter forward a step, and he said that we cannot assume that users won't make mistakes; we should assume what to do when our system crashes.

**1:27:12** · Then he had two very cute ideas. One idea is called system-wide undo, which means if a person makes a failed operation, how can I undo this?

**1:27:22** · I'll redo it, right?

**1:27:23** · Because people make mistakes, right?

**1:27:24** · So if someone makes a mistake, I would do it again, again, just like when I first applied for a PhD and failed, I would reapply. But he thinks this system must be able to allow you to do this. If this system doesn’t let you do it, after failing once, how can you undo it?

**1:27:36** · Because actually this matter is very difficult since there is no system to support you.

**1:27:40** · For example, if you send an email, once you have sent it, how can you undo it, right?

**1:27:43** · So it must have a very systematic mechanism to ensure that you can do this undo, and at that time they had a paper, and this matter was very difficult to generalize, so they made a special case. They built an email server, and they felt that an email server was a very difficult thing because sending emails or what?

**1:27:56** · These things are very difficult to roll back, so they said we will focus on a relatively difficult problem and create an email server that can be undone. Then they had three, maybe five, principles. This was a paper they published at USENIX ATC at the time, presenting one of their cute ideas. Another cute idea is called microreboot.

**1:28:10** · Then what he meant at that time was, when we have a problem in the system, what is the most routine action we take? It's restarting. In fact, we rarely, for example, if we have a blue screen, very rarely do we say, after I get a blue screen, I need to find out where this bug is?

**1:28:22** · If we just restart directly and it can work again and again, then we can continue working, right?

**1:28:25** · So 70% of recovery mechanisms are reboots, but rebooting is very expensive in industrial systems because you have caches and things like TLBs, right?

**1:28:35** · You just rebooted, and all your states were lost, so this is very expensive, but it is a very effective method. So what should be done?

**1:28:43** · So can I build this system into one where you don't need a global reboot but can do a microreboot, meaning you only reboot that one component instead of rebooting the entire memory or something like that?

**1:28:53** · This was an idea they had at the time. They also created a case study back then, which was hard to generalize, so they just made a case study. But this has Today, that idea is reflected in microservices and serverless systems.

**1:29:06** · You can microreboot a single microservice or restart one service.

**1:29:08** · Services are restarted all the time.

**1:29:10** · So the idea already exists in practice.

### Undo 与 Microreboot：把经典系统思想带入 Agent 时代

**1:29:13** · In the age of AI agents, we can revisit those classic principles and reuse them directly.

**1:29:17** · We can ask which ideas remain valuable and can help AI agents, and which we no longer need.

**1:29:24** · Then we won't have to optimize for every old constraint, and can design a much better system for AI agents.

**1:29:31** · Wow. I feel I really need to learn more about the history of computing, as Professor Juncheng suggested.

**1:29:39** · That history contains a great deal of hard-won wisdom.

**1:29:44** · Exactly.

**1:29:45** · Applying that wisdom in this new era can lead to remarkable results.

**1:29:50** · now, when you were talking about this, undoing this matter, designing a system — this system is used by an agent, and then every operation of the agent can be undone. What a beautiful set of systems.

**1:29:59** · Yes.

**1:30:00** · I think this is completely achievable. This is not something that is theoretically very difficult; it's just a matter of whether anyone is willing to do it. I think a big challenge of AI systems nowadays is that they are developing too fast, right?

**1:30:12** · It’s rare for people to be willing to say that I should focus and spend a year or two building a system. I think this is a challenge. in other words, if your AI capabilities are constantly changing, or many of your paradigms are constantly changing, then indeed, the challenge is how you quickly build a system, be able to think it through clearly, and make it usable for future agent paradigms as well.

**1:30:32** · I think this is a challenge—a real challenge.

**1:30:34** · Moreover, previously everyone could feel that after Claude's upgrade, some aspects actually became less user-friendly. But this is a training process—you add data and then remove data. If this system is applied here, you can undo, right?

**1:30:51** · You don't need to retrain, you can undo something, right?

**1:30:54** · Each training session costs hundreds of millions of dollars, and this seems useful whether from the perspective of large AI training systems or small AI applications.

**1:31:04** · Yes.

**1:31:04** · But it's quite difficult; this requires dedicated practice. I think now is a very good opportunity because in the past, for example, twenty or thirty years ago, reliability and security were never a first-class principle, right? Because reliability and security don’t make money, only performance makes money, performance makes money.

### Reliability 与 Security 为什么迎来新的研究机会

**1:31:23** · Basically, almost everyone building a system relies on the first thing, which is performance. If you improve performance, or features—features along with performance—these two things make money. As for reliability, that is, or security, you just worry about not losing money, so you can actually achieve it through many other methods, including, as I just said, buying insurance, buying some insurance.

**1:31:42** · Or you could say that even though you write a lot of test cases, and the test cases are never comprehensive, at least you have a lot of confidence, so you have a company invest a lot in security and reliability, which many companies are unwilling to do. But I think the agent has turned this around because with you, if you let the agent take the lead in doing many things, including writing code.

**1:32:00** · So reliability and security are the biggest issues, and they are not problems you can solve just by scaling test cases. So I think this is a great opportunity, and researchers can take advantage of it, especially reliability and security researchers, who can then obtain a lot of resources and do some more meaningful things.

**1:32:17** · In fact, it's like this: nowadays, the people who hold resources in every company have also realized that having agents do things to raise the ceiling isn’t that difficult. Instead, the real challenge is how to continuously trust these agents, which makes everyone very anxious.

**0:06** · I actually don't really believe in AI, but I think people should be believed.

**0:24** · Because I think research is a really wonderful thing, right?

**0:28** · Then I feel that he has no set standard for all beautiful things.

**1:32:30** · Including when you are using some AI agents now, you will find that when it comes to high-value tasks related to money, filling out forms, or personal information, if you ask it to do these things, the official stance is that they do not recommend you do this because they do not want to take on this responsibility. But if someone can help solve the security issues generated by these systems, the value that this brings to everyone would be immeasurable. I think this is the current situation.

**1:32:54** · For example, one of the hottest topics is trustworthy AI and so on. Yes, that's right. So recently, you also came to the Bay Area, to UC Berkeley for a visit. What kind of considerations led to this?

### 学术休假：为什么到 UC Berkeley SkyLab 访问

**1:33:06** · I haven't given it much thought, because I'm not someone who plans very far ahead.

**1:33:13** · I never make five-year or even three-year plans.

**1:33:16** · I mostly think about what I want to do this year.

**1:33:21** · My planning horizon is fairly short.

**1:33:22** · I'm currently on sabbatical.

**1:33:24** · American universities usually have a system under which, after six years of service, a professor may take a year-long sabbatical.

**1:33:33** · The basic idea is that every institution has its own constraints and biases.

**1:33:37** · If you stay in one place for too long, it becomes hard to think outside the box.

**1:33:45** · You start taking everything for granted. A sabbatical encourages you to experience another environment, one very different from your own, and reflect on what you see.

**1:33:53** · When you return to your university, you can adopt some of the good practices you've encountered and improve some of the bad ones.

**1:34:00** · That's the purpose of the sabbatical system.

**1:34:03** · I became eligible for one last year.

**1:34:05** · I had originally planned to take it after completing six years at UIUC. But last year, the Trump administration made major cuts to NSF funding.

**1:34:15** · And UIUC is a very large public university— one of the largest in the country.

**1:34:19** · The steep cuts in federal funding put enormous pressure on the university.

**1:34:23** · To be honest, there was a fair amount of anxiety in our department, and I felt that anxiety spread to many PhD students.

**1:34:31** · They saw the funding cuts, alongside the impact of AI, and everyone was nervous.

**1:34:36** · At the time, I felt it wouldn't be right for me to leave in the middle of that, so I decided not to take my sabbatical just yet.

**1:34:46** · I postponed it for a year.

**1:34:47** · Things felt more stable this year, so I decided to go ahead with the sabbatical.

**1:34:53** · I didn't have a specific plan at first.

**1:34:55** · Then one day I ran into my current host, Ion Stoica, and mentioned it to him.

**1:34:59** · He said, “You're welcome to come to Berkeley.”

**1:35:02** · I thought, “Sure, why not?”

**1:35:05** · And that's how I ended up here.

**1:35:06** · He was very thoughtful about it, though.

**1:35:07** · He wanted to make sure I'd be happy here and that the visit would give me what I was looking for.

**1:35:14** · So he invited me to a SkyLab retreat.

**1:35:16** · It was much larger than I'd expected.

**1:35:19** · There were people from industry and academia, as well as students, all presenting.

**1:35:22** · I went, liked what I saw, and decided to come.

**1:35:24** · There really wasn't much more planning than that.

### UIUC 与 Berkeley：不同研究文化如何互相启发

**1:35:26** · There was a lot packed into that answer.

**1:35:28** · First, given the current climate, there is real anxiety at both the institutional and individual levels.

**1:35:37** · It's hard not to be affected by upheaval on that scale.

**1:35:40** · Yes, some things are simply beyond our control.

**1:35:44** · Second, once your work reaches a certain level, it's easier to visit leading research groups.

**1:35:49** · A sabbatical visitor doesn't place much burden on the host institution.

**1:35:52** · Instead, you become a colleague they can exchange ideas with, so most universities are very welcoming.

**1:35:55** · If any professors watching this are considering a sabbatical, don't worry too much about imposing.

**1:36:04** · Reach out and ask.

**1:36:04** · There are more opportunities than you might think.

**1:36:07** · That's also one reason I chose Berkeley.

**1:36:09** · Berkeley's research culture differs sharply from UIUC's.

**1:36:12** · In some ways, they're opposites.

**1:36:14** · It's not about one being right or wrong; they're simply very different.

**1:36:17** · For a sabbatical, I wanted to go somewhere genuinely different.

**1:36:20** · That made Berkeley an excellent choice.

**1:36:23** · I could have visited a university more like UIUC.

**1:36:24** · It might have been an outstanding school, but if the culture were too similar, what would be the point of going there?

**1:36:31** · I've heard it said that a sign of real intellectual maturity is the ability to hold multiple viewpoints, methods, and worldviews in your mind.

**1:36:42** · Exactly.

**1:36:42** · You already have a strong methodology and the practical Midwestern research worldview.

**1:36:47** · Now you want to see something different.

**1:36:50** · Right.

**1:36:51** · And given your personality, you didn't arrive with a fixed idea of what you had to accomplish.

**1:36:55** · You simply came to explore.

**1:36:56** · Yes.

**1:36:56** · I think research is a beautiful thing.

**1:36:59** · And beautiful things don't follow a single standard.

**1:37:04** · No rule says everyone must follow Method A or Procedure A.

**1:37:10** · What makes academia and research so appealing is that many schools of thought can flourish.

**1:37:15** · People can use different methods and philosophies, or follow different principles, and still succeed.

**1:37:18** · If the result is something beautiful that people value, then we should make room for many different styles.

**1:37:27** · Berkeley's style is very different from UIUC's, and UIUC's style is also different from MIT's. That's a good thing.

**1:37:35** · All of those approaches should be able to coexist.

**1:37:38** · That's one reason sabbaticals exist: they encourage you to experience something different and exchange ideas across institutions.

**1:37:45** · There's another memorable line: beautiful things have no single standard.

**1:37:49** · Exactly.

**1:37:50** · Let a hundred flowers bloom.

**1:37:50** · Yes.

**1:37:51** · I think that's part of human nature.

**1:37:54** · Now that AI agents are so capable, if you want to live a happy life, you need to seek out things that cannot be standardized.

**1:38:03** · People have been saying this a lot lately.

**1:38:04** · What is AI best at?

**1:38:06** · Standardized tasks— problems with clear questions and clear answers.

**1:38:09** · What is AI worst at?

**1:38:10** · Things that are difficult to define, things that resist standardization. I think you're absolutely right.

**1:38:14** · Yes.

**1:38:16** · That wraps up our discussion of your research.

### 工业界研究与学术研究的目标差异

**1:38:22** · Let's move on to some broader, more philosophical questions.

**1:38:26** · During your PhD, you interned at several places—NetApp, Facebook, and Microsoft Research— and you've now spent many years in academia.

**1:38:39** · What do you see as the biggest difference between industry and academia in computing?

**1:38:45** · By industry, do you mean industry research?

**1:38:49** · Otherwise, the comparison isn't really apples to apples. So what's the difference between industry and academic research?

**1:38:53** · Either interpretation is fine.

**1:38:54** · Answer it however you think makes sense.

**1:38:55** · Here's how I see it.

**1:38:58** · Industry and academia approach research differently.

**1:39:00** · Both are forms of research, and share the same fundamental purpose, but they take very different forms.

**1:39:05** · I think academia— especially now that we have AI— needs to focus even more on fundamental research.

**1:39:11** · Academia has fewer resources now, partly because of government intervention.

**1:39:16** · With fewer resources, you have to pursue truly fundamental questions.

**1:39:20** · Academia's job is to push the frontier.

**1:39:23** · You may push it today, and AI may learn from it tomorrow.

**1:39:26** · But unless we keep pushing that frontier, AI will have nothing new to learn.

**1:39:30** · That's what academia should do.

**1:39:32** · That's what good academic research looks like.

**1:39:35** · Industry research, by contrast, is applied research.

**1:39:37** · It should be tested in practice.

**1:39:41** · It has to be useful, and it should produce results in the near term, ideally with significant impact.

**1:39:49** · So the two styles of research are very different.

**1:39:53** · different, and the methods of carrying them out are also very different.

**1:39:56** · This is why I think, for example, when a person is facing a choice, the first thing they deal with is the question of what kind of research they want to do. If someone says, 'I just want to do very practical research that can benefit many people.'

**1:40:09** · Many people say tomorrow or next month, so I think industry research might be a very good goal.

**1:40:13** · Some people say they just want to do a pie-in-the-sky research, right, just looking at the sky and imagining what the future world would be like.

**1:40:20** · I think that kind of research is quite good at Princeton. Before that, Jun Cheng had a point.

### PhD 实习：深入生产系统，而不是重复学校里的研究

**1:40:24** · It means that when a PhD goes for an internship in the industry, they shouldn't do research. He said, Jun Cheng said that a PhD in the industry should be completely immersed in the industry's system every day, seeing what the system engineers around them are working on.

**1:40:37** · What does he write?

**1:40:38** · Then get the most frontline understanding of this system, and then go back to school to slowly refine it to see if you can make something better.

**1:40:45** · What do you think of this viewpoint? I think this is a very good method. I think the reason why we should do internships is because I still strongly encourage students to intern. I think internships are also a very good thing because you change your environment and then see some different systems that you originally had no chance to be exposed to, right?

**1:41:00** · And also, you work with different people, so I think if you go for an internship and still follow the school's way of doing things, that's obviously not a very good practice, right?

**1:41:10** · So I think if you go to the industry or do an internship, you should do something different this summer, and then learn something. Of course, there are many ways to learn. I think, for example, in my first period, I applied my own research to the industry system, and I think this is a very good, very good method. You can also do that.

**1:41:29** · It is about going into the industry to learn about their problems, and then the most important problem there. First, I have a student, Xudong.

**1:41:35** · His first internship was at Walmart. I remember that in his first two weeks, he would just catch people and ask them, 'What is the most important problem in your company?'

**1:41:43** · I want to address your most important questions, and then he received a dozen answers.

**1:41:47** · Many people also couldn't frame it as a research question, but everyone would honestly tell him what they think is the most important or the hardest problem in their work. Then, at this point, you refine it yourself. I think this might be a bit similar to what Juncheng said, maybe too similar, but I think it's about understanding this, because now.

**1:42:02** · Academia is no longer the academia of the 70s, right?

**1:42:05** · Back in the 1970s, there might have only been five computers in the United States, all in schools, and the industry would immediately follow up on whatever the schools were doing. But now, you see, the industry is often completely ahead of academia.

**1:42:17** · Even when I was doing my PhD, Google had already implemented systems like MapReduce and Bigtable and then published a paper.

**1:42:21** · Then thousands of papers are optimizing MapReduce, and then publishing Bigtable; thousands of papers are optimizing Bigtable. So I think that many times industry is ahead of academia. Therefore, I think when you go into industry, you should do things that industry needs to do. Wasn't there an old saying about this before?

**1:42:38** · When you go to Rome, you should do what the Romans do.

**1:42:41** · I think this is right, I think.

**1:42:42** · Totally agree.

### 去 Google 做核心系统，还是到大学当教授

**1:42:44** · So if one of your students asks you when they graduate, 'I got two offers: one is to go to Google to work on this system, and the other is to go to a certain school to be a professor. How would you help me make a decision?'

**1:42:59** · One is to go to Google and work on core systems.

**1:43:02** · Right.

**1:43:02** · Another option is to go to an elite school to be a professor.

**1:43:06** · This choice is very personal. I think it has to do with what he and the other person are pursuing, including his financial situation, right?

**1:43:15** · Or his need for certain things, and then his personality, I think all of this applies—he is not someone who can absolutely... Still, it's the same statement: there is no formula. He has to make a choice based on the different situations of each person, I think, including going into the industry to create a system. And when you create this system, what exactly are you doing, right?

**1:43:31** · in other words, if you go to this factory to screw in screws, it is possible that the system is very pretty, but when you screw in the screws, you may not necessarily feel that it's pretty.

**1:43:41** · So I think any experience, the first thing for me is that in this experience, what kind of things you can learn. I think this is the most important question, whether you go into academia or industry.

**1:43:49** · If you go to an extremely beautiful place but don't learn anything, then I think it's still nothing, for me, right?

**1:43:55** · What it means to interrogate yourself is asking, 'If I work in this position for five years, what kind of skills will I be able to learn, and what abilities will I be able to develop?' Once you answer this question, I think you will have a very clear next step.

**1:44:10** · And many times, what you are even asking is, 'Who should I work with?' I think who you work with depends on your level, and on whether they are very enthusiastic.

**1:44:19** · Working with people who are excited or very enthusiastic about technical things is different from working with people who might just want to get by.

**1:44:27** · Even in a very big company, I feel like you can't really learn much from those people. So, whether you go to a startup, a big company, or academia, essentially it's about what you can learn.

**1:44:37** · Of course, there are many other constraints as well, right?

**1:44:39** · I think a big problem with academia is that everyone is very poor, so if you really need money and your family needs money, I think it might not be a particularly good choice.

**1:44:49** · Anyway, we've basically covered everything, right?

**1:44:52** · There are interests in certain professions, in working with certain people, and in various kinds of family situations.

**1:44:57** · Right.

**1:44:57** · Especially what you just said about working with very exciting people—I’ve heard countless statements from big shots about this, which means that the things this team is doing and the people in this team make me willing to join this company on the day I graduate even if they don’t pay me. I think everyone is just—in fact, finding a job is getting increasingly difficult, and everyone’s career development is a bit unclear.

**1:45:21** · Climbing the ladder, going for a promotion, these things are all very vague, but a lot of fans in the background often ask questions about how to choose. Actually, I think there is only one standard, which is really just what you mentioned earlier: whether you can learn something and whether being with these people is exciting or not. Yes, especially in the early stages of your career, this is the most, most, most, most important.

**1:45:38** · Right.

**1:45:38** · The other things you just mentioned are very honest words. Some big companies or certain places may seem dazzling, but the small team isn't really up to par, and then you are really correct because I think that's the case. Academia is a very standardized place. The best schools in the U.S.

### 如何判断一份工业界工作和一个团队是否值得加入

**1:45:55** · and just an average school in the U.S. actually don't have much difference. As a faculty member, you just do those three things, right?

**1:46:00** · One is teaching, one is doing research, and another is mentoring students, but in industry.

**1:46:06** · Different teams in different companies vary greatly, so it's actually hard to summarize in one sentence whether you should take an industry job. You can do a very creative job in the industry, or you can do a very boring job. Therefore, you need to understand exactly what this job entails. A good way to do this is to talk with people in the team, ideally, and see what their thoughts are.

**1:46:26** · Actually, once you talk to someone, you'll know whether you want to work with these people. Many times, this is the core reason, because one company has too many people, too many teams, and too many projects.

**1:46:35** · Right.

**1:46:35** · Can't you figure out what you want?

**1:46:37** · Then you can't work with these people in a highly excited way every day, which I think is especially a waste of your life in the early stages of your career.

**1:46:43** · Yes.

**1:46:44** · My former PhD advisor once made an excellent point.

**1:46:47** · He said that in computing, your first job is not your forever job.

**1:46:51** · So don't agonize over your first job, or imagine it's a secure position you'll keep for life.

**1:46:58** · Treat it as a learning process.

**1:47:01** · If you learn— really learn— your next job will be better.

**1:47:05** · If you don't, even a huge compensation package won't stop you from being laid off two years later.

**1:47:09** · That's true.

**1:47:10** · The computing industry changes incredibly fast.

**1:47:11** · The pace over just the past five or six years has made that obvious.

**1:47:16** · We don't even need to go back to the early internet, the dot-com bubble, or the mobile internet.

### AI 时代最重要的能力：持续学习

**1:47:21** · Just look at AI era.

**1:47:22** · In only two or three years, so much has changed.

**1:47:24** · AI has gone from basic chatbots to writing code.

**1:47:28** · Next, for all we know, robotics could take off.

**1:47:30** · You might have happily built a chatbot when they first appeared, only to find yourself far behind this year.

**1:47:35** · That's how much can change in two years.

**1:47:36** · As for industry, this is just my personal view, and it may be biased, but I believe the most important thing is the ability to keep learning.

**1:47:44** · The greatest danger in industry is taking a position and then stopping learning.

**1:47:49** · With technology changing every day, you will inevitably be left behind.

**1:47:55** · The advantage of academia is that it forces you to learn.

**1:47:58** · As a faculty member, you have to advise students, apply for funding, take on all kinds of responsibilities, and teach.

**1:48:03** · The environment compels you to keep learning.

**1:48:06** · If you don't, you can't keep up.

**1:48:08** · In industry, however, you may settle into a job without making a conscious effort to learn.

**1:48:13** · You can remain comfortable for two years, but what happens after that?

**1:48:16** · That's the problem.

**1:48:17** · Right.

**1:48:19** · Some say academia has better long-term prospects, while industry pays more early on.

**1:48:22** · People use all kinds of criteria, but to me, those distinctions are simply too vague.

**1:48:29** · After our conversation, one point stands out clearly: what matters is the person.

**1:48:34** · Exactly.

**1:48:35** · What matters is you.

**1:48:36** · As long as you can keep learning, whether you're in academia, in industry, or anywhere else, you're likely to do well.

**1:48:42** · position, yes, yes.

**1:48:44** · I feel that, in the end, a person, for me, is essentially this. I think a high-quality person always has a way, always moves forward. I really believe that one of the reasons for this person is moving forward.

**1:48:56** · I don't really believe in AI, but I think people should be believed.

**1:49:00** · I've been doing this podcast for a while, and I also feel the same way.

**1:49:03** · It's true that you have to always believe in people. So the next question is, in the process you just described, everyone can also tell that you pay special attention to people, especially to the connections between people, and are particularly willing to guide a student hand in hand to understand their essence and lead them as a professor. So if we are facing this...

### 给年轻研究者的三个建议：问题、方法与技能

**1:49:23** · With so many students in the audience, you definitely can't guide them hand by hand. If you were willing how they can develop into strong PhD researchers— how they can train themselves— what is the most practical advice you would give them?

**1:49:40** · Well, I think all advice is— all advice is bad advice.

**1:49:46** · How should I put it?

**1:49:46** · Three things matter most in research, especially during a PhD.

**1:49:49** · To do well, you should find every way you can to get those three things right.

**1:49:53** · First, find a problem you truly care about.

**1:49:55** · That's the first thing.

**1:49:57** · When you care deeply about a problem, the work doesn't feel exhausting.

**1:49:59** · Research is hard, and only genuine interest can sustain you.

**1:50:03** · When you wake up each morning and brush your teeth, look at yourself in the mirror.

**1:50:07** · Are you excited to keep working on the problem?

**1:50:10** · Or are you thinking, “Oh no, I have to go back to the office and work on this again”?

**1:50:13** · So first, find a problem you love.

**1:50:15** · That is genuinely difficult.

**1:50:17** · It may take one, two, or even three years.

**1:50:19** · But if you keep looking, you will eventually discover what kind of work you care about.

**1:50:24** · Second, it is crucial to develop a sound approach to research.

**1:50:29** · Research is not the same as engineering.

**1:50:31** · Often, there is no shortcut.

**1:50:36** · What matters is finding the right method.

**1:50:38** · Speed itself isn't important.

**1:50:40** · You can make up for speed with time.

**1:50:42** · But with the right method, if your approach is sound, you will eventually get there.

**1:50:46** · Third, keep sharpening your skill set.

**1:50:49** · Once you have a problem to pursue, you can see which skills are useful and which are not.

**1:50:54** · Your time and energy are limited.

**1:50:55** · You cannot do everything.

**1:50:56** · Once you know what you need to do, you can deliberately develop the skills required to do it well.

**1:51:01** · Get those three things right, and I believe you can succeed.

**1:51:03** · Systems research isn't pure theory.

**1:51:05** · It doesn't require extraordinary genius.

**1:51:07** · Systems research is not especially dependent on raw talent.

**1:51:09** · If you get those three things right, you can do good work.

**1:51:13** · I believe anyone can do systems research.

**1:51:16** · You still call that bad advice?

**1:51:18** · That sounds like perfect advice to me.

**1:51:20** · Very much so.

**1:51:20** · I just think everyone already knows these things, but putting them into practice is difficult.

**1:51:24** · Perhaps it sounds a little abstract?

**1:51:27** · Yes, very abstract.

**1:51:28** · It's a cliché, really.

**1:51:30** · The same is true of many things.

**1:51:31** · For example, earlier we discussed starting a company, or writing a paper.

**1:51:38** · How do you write a good paper?

**1:51:39** · How do you build a good company?

**1:51:40** · Hundreds of books have been written on both.

**1:51:42** · But reading those books doesn't mean you can write a good paper or build a successful company.

**1:51:47** · You still have to learn through practice.

**1:51:51** · That's why I think you also need good mentors.

**1:51:54** · to say about this is that I saw a saying before, which is that people may hear a lot of correct advice, but why do some people grow quickly while others grow slowly.

**1:52:04** · You want to see the person who grows quickly, right?

**1:52:07** · Finding a mentor who can provide timely feedback to him is crucial.

**1:52:11** · In the process of entrepreneurship, whether you look at those successful entrepreneurs or those PhD students who do very well, it all comes down to being able to get the most timely feedback from a very experienced mentor.

**1:52:23** · If people can't get this kind of thing, then they need to make an effort to find it. That's how I feel. And this mentor, I think he...

**1:52:30** · It doesn't necessarily have to be a very big-name person, right?

**1:52:33** · Right.

**1:52:33** · It's really someone very famous; he is a person who is willing to invest in you, someone who is willing to spend time on you. I think the more someone is like this, the more important it is. How to get others to bet on you and then invest in you is another big topic. You can't really put it into words, you know?

**1:52:50** · It's hard for everyone to see.

**1:52:51** · Yes.

**1:52:52** · This is a topic that requires lifelong learning, and it goes back to what we just talked about about lifelong learning and continuous growth. Without this energy, if you yourself don't have this energy.

**1:53:01** · Right.

**1:53:01** · You want to become a person worth investing in, right?

**1:53:04** · Right.

**1:53:05** · This is your point, it’s worth it to you, it’s worth other people spending time. Worth other people spending time. He doesn’t invest just by saying please. The kind of thing where you can guide me, it won’t happen.

**1:53:13** · It will never happen.

**1:53:14** · Yes.

**1:53:15** · Only by constantly striving and improving yourself, this is good, good, good, very fantasy, all nonsense, whether the chicken or the egg came first, this matter is indeed correct, but maybe when I reach the 100th episode later I will tell everyone how to do this, but for now I have only done a few episodes so I don't dare to say. And there is another question.

### 想创业或进工业界，PhD 阶段应该怎样准备

**1:53:37** · If students are willing to start a business, you would support them, and previously your mentor was a very successful entrepreneur, and you have some very successful cases around you.

**1:53:44** · Right.

**1:53:44** · So what if this student tells you from the beginning, 'OK, professor, after I finish this PhD, I want to become an entrepreneur'? How would you guide him?

**1:53:51** · Then I would definitely tell this student that I have never started a company, so I probably can't help him much in terms of entrepreneurship, right?

**1:53:57** · So I currently have a student who really wants to start a business, and my suggestion to him was that he might need some other connections or some other advice, so he also got some, well, some help, I think, but how should I put it?

**1:54:12** · I just feel that there are two types of entrepreneurship. One is where you start a business based on your own research.

**1:54:16** · For example, you develop a particularly interesting technique and then use this technique to start a business.

**1:54:21** · In the process of developing this technique, I think it can still be somewhat helpful. Of course, if you say that your startup is completely based on taking an existing technique to create a company.

**1:54:29** · Then I might not be a very good advisor, I think that's the Building a company around an existing technology or a new technical innovation is certainly one path.

**1:54:38** · Many companies that emerged from Google started that way.

**1:54:42** · But I once heard your advisor caution against it.

**1:54:45** · He said you should avoid taking a hammer and looking for a nail.

**1:54:49** · It's usually better to start with a real business need.

**1:54:52** · That's another topic, though.

**1:54:53** · Everyone has a different view, and there are certainly both successes and failures.

**1:54:57** · Right.

**1:54:57** · Now for the third question.

**1:55:00** · Suppose a student wants to enter industry and knows from the outset that they want to run large-scale systems and build practical systems.

**1:55:08** · How would you advise that student?

**1:55:09** · Well, I think academia and industry require different skill sets.

**1:55:15** · That said, I don't think academia and industry are fundamentally different.

**1:55:18** · Doing creative work is rewarding in either one.

**1:55:21** · What matters, ultimately, is whether the place you choose gives you an environment that helps you do the work you want to do.

**1:55:27** · That place might be academia or it might be industry.

**1:55:30** · But the two paths do require different skill sets.

**1:55:33** · Some things matter little in academia but are crucial in industry.

**1:55:37** · If a student knows from the beginning that they want to enter industry, the most important thing is to identify the skills that industry requires.

**1:55:45** · Then, during the PhD or other graduate study, they should deliberately develop those skills and become a more competitive candidate.

**1:55:55** · When it comes to publications, industry doesn't care much about how many papers you've published.

**1:55:59** · One paper may be enough to graduate.

**1:56:00** · Spend the rest of your time coding, learning system design, and exploring the latest frontier ideas.

**1:56:07** · Right.

**1:56:08** · AI, for example.

**1:56:09** · If you want an academic career, you'll need to build a publication record and focus on that.

**1:56:14** · Build it up.

**1:56:14** · Exactly.

**1:56:16** · We've talked with Tianyin for more than two hours today, and it's clear that he is a truly exceptional systems researcher.

### 结语：让 Research 成为每天的实践

**1:56:23** · There's a saying in Silicon Valley: when someone starts a company, there needs to be founder–problem fit.

**1:56:30** · The right person is suited to the right problem.

**1:56:32** · That comes from something deep within them.

**1:56:34** · After these past two hours, I can clearly see that in you.

**1:56:38** · I've met many systems researchers, and I work in systems research myself, though I'm only average.

**1:56:42** · But you really are a natural systems researcher.

**1:56:47** · I don't believe anyone is born that way.

**1:56:48** · You just need to enjoy the work.

**1:56:49** · Research is a wonderful thing.

**1:56:51** · I think everyone should try it, especially now that the barriers are lower.

**1:56:54** · Research used to feel like something confined to an ivory tower, something only a small number of people could do.

**1:56:59** · But with AI and modern technology, research should become a daily practice.

**1:57:05** · At its heart, it's a process of reflection.

**1:57:07** · So I think everyone should give it a try.

**1:57:10** · All right. Thank you, Tianyin.

**1:57:12** · I hope that after hearing this, everyone will do a little research every day.

**1:57:16** · Exactly.

**1:57:17** · It can make you more effective at work.

**1:57:18** · Yes.

**1:57:19** · And it can strengthen all kinds of skills.

**1:57:20** · Absolutely.

**1:57:21** · You can research something as everyday as how to build your abs or how to get in shape.

**1:57:25** · That's research too, isn't it?

**1:57:27** · It's an iterative process of trial and error.

**1:57:28** · Exactly.

**1:57:29** · Thanks for the suggestion.

**1:57:30** · All right.

**1:57:30** · Thank you.