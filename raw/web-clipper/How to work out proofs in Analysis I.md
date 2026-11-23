---
title: "How to work out proofs in Analysis I"
source: "https://gowers.wordpress.com/2014/02/03/how-to-work-out-proofs-in-analysis-i/?utm_source=chatgpt.com"
author:
published: 2014-02-03
created: 2026-11-23
description: "Now that we've had several results about sequences and series, it seems like a good time to step back a little and discuss how you should go about memorizing their proofs. And the very first thing to say about that is that you should attempt to do this while making as little use of your…"
tags:
  - "clippings"
---
Now that we’ve had several results about sequences and series, it seems like a good time to step back a little and discuss how you should go about memorizing their proofs. And the very first thing to say about that is that you should attempt to do this while making as little use of your memory as you possibly can.

Suppose I were to ask you to memorize the sequence 5432187654321. Would you have to learn a string of 13 symbols? No, because after studying the sequence you would see that it is just counting down from 5 and then counting down from 8. What you want is for your memory of a proof to be like that too: you just keep doing the obvious thing except that from time to time the next step isn’t obvious, so you need to remember it. Even then, the better you can understand why the non-obvious step was in fact sensible, the easier it will be to memorize it, and as you get more experienced you may find that steps that previously seemed clever and nonobvious start to seem like the natural thing to do.

For some reason, Analysis I contains a number of proofs that experienced mathematicians find easy but many beginners find very hard. I want to try in this post to explain why the experienced mathematicians are right: in a rather precise sense many of these proofs *really are easy*, in the sense that if you just repeatedly do the obvious thing you will solve them. Others are mostly like that, with perhaps one smallish idea needed when the obvious steps run out. And even the hardest ones have easy parts to them.

I feel so strongly about this that a few years ago I teamed up with a colleague of mine, Mohan Ganesalingam, to write a computer program to solve easy problems. And after a lot of effort, we produced one that can solve several (but not yet all — there are still difficulties to sort out) problems of the kind I am talking about: easy for the experienced mathematician, but hard for the novice. Now you have some huge advantages over a computer. For example, you understand the English language. Also, you can be presented with a vague instruction such as “Do any obvious simplifications to the expression and then see whether it reminds you of anything,” and you will be able to follow it. (In principle, so could the program, but only if we spent a long time agonizing about what exactly constitutes an “obvious” simplification, what kind of similarity should be sufficient for one mathematical expression to trigger the program to call up another, and so on.) So if a mere computer can solve these problems, you should definitely be able to solve them.

What I plan to do in this post is basically explain how the program would go about proving some of the theorems we’ve proved in the course. To explain *exactly* how it works would be complicated. However, because you are humans, there are lots of technical details that I don’t need to worry about, and what remains of the algorithm when you ignore those details is really pretty simple.

The rough idea is that you should equip yourself with a small set of “moves” and simply apply these moves when the opportunity arises. That is an oversimplification, since sometimes one can do the moves in “silly” ways, but merely being consciously aware of the moves is very useful. (Incidentally, the notion of “silliness” is hard to define formally but is something that humans find easy to recognise when they see examples of it. So that’s another example of the kind of advantage you have over the computer.)

### Subsequences of Cauchy sequences

I’m going to describe a way of keeping track of where you have got to in your discovery of a proof. It’s not something I suggest you do for the rest of your mathematical lives. Rather, it is something that you might like to consider doing if you find it hard to come up with typical Analysis I proofs. If you use this technique a few times, then it should get easier, and after a while you will find that you don’t need to use the technique any more.

The technique is simply to record what statements you are likely to want to use, and what statement you are trying to prove. Both of these can change during the course of your proof discovery, as we shall see.

I think the easiest way to explain this and the moves is to begin by giving an example of the whole process in action. Then I’ll talk about the moves in a more abstract way. Let’s take as an example the proof that if a Cauchy sequence has a convergent subsequence then the sequence itself is convergent.

To begin with, we have nothing we obviously need to use, and a statement that we want to prove. That statement is the following.

—————————————————-  
Every Cauchy sequence with a convergent subsequence converges

Let us begin by writing that very slightly more formally, to bring out the fact that it starts with $\forall$.

—————————————————-  
$\forall (a_n)\ (a_n)$ is Cauchy and $(a_n)$ has a convergent subsequence  
$\implies (a_n)$ converges

The next step is to apply the “let” move, which I’ve talked about several times in lectures. If you ever have a statement to prove of the form “For every $x$ such that $P(x)$ holds, $Q(x)$ also holds,” then you can just automatically write “Let $x$ be such that $P(x)$ holds,” and change your target to that of establishing that $Q(x)$ holds.

In our case, we write, “Let $(a_n)$ be a Cauchy sequence that has a convergent subsequence,” and modify our target to that of proving that $(a_n)$ converges. So now we represent where we’ve got to as follows.

$(a_n)$ is a Cauchy sequence  
$(a_n)$ has a convergent subsequence  
——————————————-  
$(a_n)$ converges

Maybe the purpose of those strange horizontal lines is becoming clearer at this point. I am listing statements that we can *assume* above the line and ones that we are trying to *prove* below the line.

At this point it seems natural to give a name to the convergent subsequence that we are given. Let us call it $(a_{n_k})$. This again is just one instance of a very general move: if you are told you’ve got something, then give it a name. This sequence has two properties: it is a subsequence of $(a_n)$ and it converges. I’ll list those two properties separately.

$(a_n)$ is a Cauchy sequence  
$(a_{n_k})$ is a subsequence of $(a_n)$  
$(a_{n_k})$ converges  
——————————————-  
$(a_n)$ converges

Having done that, I think I’ll remove the second hypothesis, since the fact that $(a_{n_k})$ is a subsequence of $(a_n)$ is implicit in the notation.

$(a_n)$ is a Cauchy sequence  
$(a_{n_k})$ converges  
——————————————-  
$(a_n)$ converges

The second hypothesis here is again telling us we’ve got something: a limit of the subsequence. So let’s apply the naming move again, calling this limit $a$.

$(a_n)$ is a Cauchy sequence  
$(a_{n_k})\to a$  
——————————————-  
$(a_n)$ converges

That’s enough reformulation of our assumptions. It’s time to think about what we are trying to prove. To do that, we use a process called *expansion*. That means taking a definition and writing it out in more detail. It tends to be good to *avoid* expanding definitions unless you are genuinely stuck: that way you won’t miss opportunities to *use results from the course* rather than proving everything from first principles. However, here a proof from first principles is what is required. I’m going to do a partial expansion to start with: a sequence converges if there exists a real number that it converges to.

$(a_n)$ is a Cauchy sequence  
$a_{n_k}\to a$  
——————————————-  
$\exists x\ (a_n)$ converges to $x$

Now our target has changed to an existential statement. How are we going to find an $x$ that the sequence converges to?

Sometimes proving existential statements is very hard, but here it is easy, since we have a candidate for the limit staring us in the face, and better still it is the only candidate around. So let us make a very reasonable guess that the sequence is going to converge to $a$, and make proving that our new target.

$(a_n)$ is a Cauchy sequence  
$a_{n_k}\to a$  
——————————————-  
$a_n\to a$

That’s nice because we’ve got rid of that existential quantifier. But what do we do next? We must continue to expand: this time the definition of $a_n\to a$. Note that if you want to be able to do this, it is absolutely vital that you *know your definitions*. Otherwise, you obviously can’t do this expansion move. And if you can’t do that, then you can kiss goodbye to any hopes you might have had of proving this kind of result.

$(a_n)$ is a Cauchy sequence  
$a_{n_k}\to a$  
——————————————-  
$\forall\epsilon>0\ \exists N\ \forall n\geq N\ |a_n-a|<\epsilon$

Now we have a target that begins with a universal quantifier, so it’s time for the “let” move again.

$(a_n)$ is a Cauchy sequence  
$a_{n_k}\to a$  
$\epsilon>0$  
——————————————-  
$\exists N\ \forall n\geq N\ |a_n-a|<\epsilon$

Now things become slightly harder, because this time we do *not* have a candidate staring us in the face for the thing we are trying to find. (The thing we are trying to find is $N$.) It’s not a bad idea in this situation to try to write out in vague terms what the key statements mean. One can do something like this.

Eventually all terms of $(a_n)$ are close to each other  
Eventually all terms of $(a_{n_k})$ are close to $a$  
————————————————  
Eventually all terms of $(a_n)$ are close to $a$

The rough idea of the proof should now be clear: if all terms in the subsequence are close to $a$ and all terms are close to each other, then eventually for each term we can say that it is close to a term in the subsequence, which is itself close to $a$.

Since we are going to need to take two steps from a term in $(a_n)$, one to the subsequence and one from the subsequence to $a$, it seems a good idea to apply the two main hypotheses with $\epsilon/2$. So let’s just go ahead and do that and see what we get.

$\exists N_1\ \forall p,q\geq N_1\ |a_p-a_q|<\epsilon/2$  
$\exists N_2\ \forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
——————————————-  
$\exists N\ \forall n\geq N\ |a_n-a|<\epsilon$

Now we are once again in a position where we have been “given” something — in this case $N_1$ and $N_2$. So let’s quietly drop the existential quantifiers and use the names $N_1$ and $N_2$. (Purists might object to using the same names for the particular choices of $N_1$ and $N_2$ that we used when merely asserting that they exist. But this is very common practice amongst mathematicians and does not lead to confusion.)

$\forall p,q\geq N_1\ |a_p-a_q|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
——————————————-  
$\exists N\ \forall n\geq N\ |a_n-a|<\epsilon$

How do we propose to “force” $|a_n-a|$ to be less than $\epsilon$? We are going to try to ensure, for suitable $k$, that $|a_n-a_{n_k}|<\epsilon/2$ and $|a_{n_k}-a|<\epsilon/2$. The first hypothesis tells us that we will be able to get the first condition if $n$ and $n_k$ are both at least $N_1$, and the third hypothesis tells us that we we will be able to get the second condition if $k\geq N_2$.

So our plan is going to be to choose $p=n$ and $q=n_k$. For the plan to work, we shall need $n\geq N_1$, $n_k\geq N_1$, and $k\geq N_2$.

We are now in a position to choose $N$. We want our conclusion to hold when $n\geq N$, and the tool we use works when $n\geq N_1$, so it makes sense to take $N=N_1$. If we substitute that in, we lose the existential quantifier in the target and arrive at the following.

$\forall p,q\geq N_1\ |a_p-a_q|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
——————————————-  
$\forall n\geq N_1\ |a_n-a|<\epsilon$

Now we can apply the “let” move again, to get rid of the universal quantifier in the target statement.

$\forall p,q\geq N_1\ |a_p-a_q|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
$n\geq N_1$  
——————————————-  
$|a_n-a|<\epsilon$

We know we’re going to take $p=n$, and that we can, since $n\geq N_1$, so let’s go ahead and choose that value for $p$ in the first hypothesis. That leaves us with the following.

$\forall q\geq N_1\ |a_n-a_q|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
——————————————-  
$|a_n-a|<\epsilon$

Just to make clear what I did there, it was a move called *substitution*. If you have a hypothesis of the form $\forall u\ P(u)\implies Q(u)$ and a hypothesis $P(x)$, then you can substitute in $x$ for $u$ and get out $Q(x)$. (One can also call this *modus ponens*: I prefer to call it substitution in this case because the condition $p\geq N_1$ is somehow not a very serious hypothesis, but more like a “restriction” applied on $p$.)

Since I’ve used the hypothesis $n\geq N_1$ and am unlikely to need it again. I have deleted it.

Now we have to decide how to choose $q$ and how to choose $k$. Recall that we needed $k\geq N_2$ and $n_k\geq N_1$. In a human proof one just writes, “Let $k$ be such that $k\geq N_2$ and $n_k\geq N_1$.” It’s a bit trickier for a computer to find it obvious that such a $k$ exists, but again that doesn’t matter to us here. I’ll use $r$ to denote the $k$ I’m choosing, and write down the conditions I’ve made sure $r$ satisfies.

$\forall q\geq N_1\ |a_n-a_q|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
$n_r\geq N_1$  
$r\geq N_2$  
——————————————-  
$|a_n-a|<\epsilon$

Now we can substitute $n_r$ into the first hypothesis.

$|a_n-a_{n_r}|<\epsilon/2$  
$\forall k\geq N_2\ |a_{n_k}-a|<\epsilon/2$  
$r\geq N_2$  
——————————————-  
$|a_n-a|<\epsilon$

We can also substitute $r$ into the second hypothesis.

$|a_n-a_{n_r}|<\epsilon/2$  
$|a_{n_r}-a|<\epsilon/2$  
——————————————-  
$|a_n-a|<\epsilon$

And now we are done by the triangle inequality.

### What were the moves we used?

Now that we have gone through a proof, let me list the main proof-generating moves we used.

#### The “let” move

If you are trying to prove a statement of the form “For every $x$ such that $P(x)$ holds, $Q(x)$ also holds,” then write, “Let $x$ be such that $P(x)$ holds,” (or words to that effect) and adjust your target to proving that $Q(x)$ holds.

#### The “naming” move

If you are told that something exists, then give it a name. For example, if you are given the hypothesis $(a_{n_k})$ is convergent, then you are told that a limit exists. So give it a name such as $a$ and change the hypothesis to $a_{n_k}\to a$.

#### Expansion

If you are trying to prove something and you can’t find a high-level argument (by which I mean one that uses results from the course that are relevant to the statement you are trying to prove), and if what you are trying to prove involves concepts such as convergence or continuity that can be written out in low-level language (often, but not always, involving quantifiers), then rephrase what you are trying to prove in this lower-level way. That is, expand out the definition.

#### Substitution into a hypothesis

If you are given a hypothesis of the form $\forall u\ P(u)$, then given any object $x$ of the same type as $u$, you are free to substitute it in for $u$ and obtain the hypothesis $P(x)$.

For example, in the proof above, we had the hypothesis “$(a_n)$ is Cauchy”. In expanded form, this reads

$$
\forall \eta>0\ \exists N\ \forall p,q\geq N\ |a_p-a_q|<\eta
$$

We decided to substitute in $\epsilon/2$, which is of the same type of thing as $\eta$ (both are positive real numbers), and yielded for us the statement

$$
\exists N\ \forall p,q\geq N\ |a_p-a_q|<\epsilon/2
$$

(We then applied the “naming” move to get rid of the $\exists N$.)

#### Modus ponens

Often a hypothesis takes a slightly more general form, where *conditions* are assumed. That is, it takes the form

$$
\forall u\ P(u)\implies Q(u)
$$

or still more generally

$$
\forall x\ P_1(u)\wedge\dots\wedge P_k(u)\implies Q(u)
$$

There the symbol $\wedge$ means “and”, so this is saying that whenever you can find a $x$ that satisfies the conditions $P_1(x),\dots,P_k(x)$, then you can give yourself the hypothesis $Q(x)$.

#### Substitution into a target

Suppose that you are trying to prove a statement of the form $\exists u\ P(u)$, and suppose you have identified an object $x$ of the same type as $u$ that you believe is going to do the job. Then you can change your target statement from $\exists u\ P(u)$ to $P(x)$. (In words, instead of trying to show that there exists something that satisfies $P$, you are going to try to show that $x$ satisfies $P$.)

We did this when we moved from trying to prove that $(a_n)$ converges to *something* to trying to prove that it converges to $a$.

---

This is not a complete set of useful moves. However, it is a start, and I hope it will help to back up my assertion that a large fraction of the proof steps that I take when writing out proofs in lectures are fairly automatic, and steps that you too will find straightforward if you put in the practice. I’ll try to discuss more moves in future posts.