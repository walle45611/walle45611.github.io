---
title: "Understanding Kubernetes Networking in 30 Minutes - Ricardo Katz & James Strong"
source: "https://www.youtube.com/watch?v=Mj04QOqAaJ8"
author:
  - "[[CNCF [Cloud Native Computing Foundation]]]"
published: 2024-11-15
created: 2026-10-04
description: "Understanding Kubernetes Networking in 30 Minutes - Ricardo Katz, Broadcom & James Strong, Isovalent at CiscoYou are learning Kubernetes and started to face concepts like Pod CIDRs, Services, CNI, k"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=Mj04QOqAaJ8)

Understanding Kubernetes Networking in 30 Minutes - Ricardo Katz, Broadcom & James Strong, Isovalent at Cisco  
  
You are learning Kubernetes and started to face concepts like Pod CIDRs, Services, CNI, kube-proxy? Welcome! you have reached the amazing area of Kubernetes networking! We all have already been there and know how complex it may seem on the beginning, but in this talk, Ricardo and James will demystify the Kubernetes network concepts and model on a fun way, exploring how it is designed, why the is a "pause" container on every Pods, how the communication between Pods work, what are kube-proxy and CNI and their importance. In the end of this talk we expect you to get your learning path on Kubernetes Networking clear to better understand not only what are the concepts about, but also see on a live demo how every component correlates and makes the communications possible on a Kubernetes cluster.

## Transcript

### Introduction

**0:00** · everyone thank you for coming today my name is jameses strong we're going to talk about understanding kubernetes networking in 30 minutes or as I like to call a Mario speedrun through kubernetes networking um my name is James strong like I said I've already done that I'm a Solutions architect at ISO valent now which is a part of Cisco I am a maintainer of Ingress and gentic so if you have any questions about that uh ask me in an hour at my next talk um so I won't be taking those questions I'm also the author of networking kubernetes I have an A CL Guru course on the same thing and um unfortunately uh I could

**0:31** · not bring my gimly cosplay here but I do have a full gimly cosplay outfit cool my name is Ricardo I am a software engineer at vmb broadcom uh I have created the cook I don't know if someone still uses that like it's kind of broken those days like Ingress like Ingress in gyx I'm one of the maintainers of Ingress in gyx so everything that I touch usually breaks sorry and I breaks it I approve it yeah and I am a Lego Enthusiast so I I love Legos so today we going to talk about

**1:01** · understanding Linux uh kubernetes networking but one of the when I was first learning kubernetes one of the old adages was that you know kubernetes is just Linux and um sorry for any of the windows folks out there we will not be talking about that one and when we say this um I like to

**1:17** · bring up this picture I found this is a great one for the talk um for those who don't know this is the data flow of a packet through the kernel so um was setting the bar really low for us so let's strap in so agenda we're going to talk about Linux networking we're going to talk about what a pod is what's uh what part of it is and talk about container networking and how we build those criar abstractions on top of

### Linux networking basics

**1:41** · Linux Ricardo you want to go ahead yeah sure so um we are going to start doing some Demos in in a minute but I want you you don't need to take pictures we going to put that on the scum promise there's lots of pictures so yes so uh this is how uh a kubernetes networking actually

**1:58** · looks like and we are going to actually start building our networking over this with demos and showing like uh how the routing actually works why we have a CNS and what's Cube proxy and why everyone actually hates Cube proxy I can't say anything sorry yeah so

**2:18** · we'll start by talking about Linux networking and what the N networking stack is so the base of every ku's workload is working on a node a node has an Ethernet interface for external connectivity uh we would say has routing rules so we know where to send packets um IP tables so that we can break things with those packets and maybe black hole some of them and then we have connection tracking um contract pretty straightforward name it makes you know it tracks the connections inside of the kernel force and uh at a very simplistic

**2:51** · terms this is what we would consider the Linux networking stack um it's not everything there's lots of other things are involved in this but for Simplicity and 30 minute stake let's think that this is the Linux networking stack with that we're going to talk about how that gets replicated in a pod and Ricardo wanted to make sure that we talked about this guy the pause container okay so uh whoever actually entered into the node and did like a Docker PS and saw a bunch of pauses and wasn't had no idea what that thing was about great okay so when you

### Pod networking structure

**3:25** · start a pod you need somewhere to have the network right because a pod is just like uh you know a network that is shared between containers so the PA it's what we call the infrastructure container it's a container that is created just to be there so you can start creating the other things it's a container that will be there and won't be on Crash loop back or something like that right so that's what the pause is it's just a container that should be started and should be there so you can have Network and with that we have containers

**3:54** · so our actual applications are running inside of this pod and it is part of that pod namespace but what is that pod namespace or network namespace there it is a complete copy of the Linux networking stack this is what is this is what allows us as app developers to run

**4:11** · something on port 8080 multiple different times so we have multiple applications running on the same port it's because you have a copy of the Linux networking stack so if you ever see sometimes when you try to run you know several copies of engine x on a host node you'll get Port conflicts because by default it runs on 80 this is why we can do do this inside of

**4:32** · containers and so container uh pods run on nodes right we need a node until somebody figures out how to not do that even in ECS you still need a node and on a node you have what is called the root Network namespace and that's where you know by default all

**4:50** · interfaces go unless you tell it not to so we've created we've got the container we've got the Pod Network namespace we've also added a an interface in there so that we have again an interface that runs on there but how do these two communicate again we have um technology

**5:07** · called V ethernet a v ethernet is think of it as uh just a pipe between the two interfaces packet comes in packet goes out packet comes in packet goes out and these two are connected by another Linux technology called a bridge and then

**5:24** · again we like to reiterate that you know all of these commands here are from the Linux command line you can there was actually really good talk and I think it was reinvent a couple years ago where someone actually you know recreated a container with all of these commands so again trying to reiterate all this there it's just Linux underneath the hood and then we wire those the bridge up so now we have external connectivity between our pod Network namespace and our root root Network namespace and then we have this beautiful picture here multiple nodes

**5:54** · multiple copies of the network deack and I think I I missing the demo the demo slide oh you moved it oh see do that this is what I get for working with you Ricardo so we're going to talk about um how all of this um works together and how we make those kubernetes um extractions I like to call them the kubernetes networking Fight Club rules and so we talked about um that um

### Kubernetes networking rules

**6:23** · the Pod container the Pod the two containers talking to each other through that network name space and so we get this first rule highly coupled container to container communication is solved by Local Host communication so this is a complete copy of the network stack so these two processes can talk to each other over Local Host pod to pod communication all pods can communicate with other pods um with their own IP addresses and this is

**6:49** · again worked through with the V ethernet and the bridging interfaces we haven't talked about Services yet but how to get P to services communication is covered by kubernetes service so again that's another abstraction um ignore the fact that kubernetes is spelled wrong exter external to services

**7:09** · communication so everything that we've been talking about up till now has been internal to the cluster so how pods can communicate with each other and how the nose can communicate with each other so we have to have some way to reach out of the cluster because you know clusters aren't that great if nobody can actually use them right why are we using them at least secure is it

**7:31** · secure a secure container a a secure cluster has no network okay so we're going to talk about um how those nodes can communicate and we'll let Ricardo talk about the Pod cider okay so uh I will actually start the demo now okay cool uh and we are going

**7:47** · to understand a bit on this uh Communications and what this means like uh one node using the other as a router and how uh it works on this pot tot communication being transparent so we are going to start exploring all of those rules that James did but uh using uh demos right I'll be your mic stand oh thank you cool so uh we have kind of recorded

### Pod networking demo

**8:10** · before because usually I have problems when I don't record but think that we are actually doing a live demo uh first of all we are using kind who knows here who is kind oh that's great okay so whatever we

**8:27** · are going to do here and whatever we are going to test here we want to do in kind because the the idea is that you can actually explore at your home later and you don't need to pay like a huge effort of money for any cloud provider just to create a kubernetes cluster to say like look those folks they are actually right or wrong feel free to probably wrong yeah \[Music\] so well is it fine the size can you guys see it in the back one more no good it looks good okay

**8:57** · if you need just yell at us like I'm used it to James keeps the okay so the first one the first demo that we are going to show here is I I want to show you I have some uh kubernetes spts here each of them they got this uh IP address differently right so this is the PO IP and let's keep playing here I want to do

**9:17** · a curl so I went from this pod that it's running on this note on this worker too calling this pod running on worker 3 right and it work it so we could get hello cucon and if I take a look into the locks here I'm going to see that the same address that was from myod got here so one of those rules that we got which was like the communication should be flat right it's happening here cool uh

**9:40** · yeah so we can see here the IP addresses that that has been used so let's take a bit I look a bit better on how this uh let me just see if I'm I have the right demo okay so pot cidr okay how this actually works right uh

**9:57** · so again taking a look into the PO we can see that we have a different sub ranges here of network right so worker 2 has like this 10 244 3 and then worker 3 D 102 441 and so on so those networks they are kind of SE segmented segregated right and as you can see here when we configure kubernetes when we create a kubernetes cluster usually when you are not using a kind or when you are doing by your own you configure a directive which is called po cidr that says this

**10:26** · is the range of ips that my pods are going to get right and that's internally they are just use it for this internal communication uh usually they don't need to be routable for your network but it would be good that you don't clash with your network as well otherwise imagine one pot trying to call like your service on the same IP and it's going to actually try to Route internet through the server to the cluster right so we can see that the PO subnet here it's like this 10244 0016 and if we take a look into the nodes configuration we are going to see that each node got this PO cidr spec

**10:59** · saying like Okay so my worker uh has this 10 244 244 sorry to Z three and one which actually matches with the IPS that Theos are getting on on each node right uh and they have those external IPS as well so those are the IPS that are like the public node IPS uh and as you can see when I do an IP route on one of those nodes uh it actually what we have is like okay so if this node wants to reach the 104 24410 it should use this IP here this

**11:30** · external IP 172 1803 so in fact what we can see here is that one node can reach each other and they can reach using tunnels it can be they can be on the same network doesn't matter this is how the C cni is going to implement that right but you use each of those nodes as routers okay uh so oh you have a c oh

### Container network interface

**11:54** · you have a cve here James cool so where is that I've lost the presentation so let's let me just come here to whatever we had here uh so taking a look here again we can see that every uh node is been used as a router for each of the pods that are running on those routers right so you we have segregated those on uh on on subnets and the way that it happens it's with a uh component which is called it cni so you probably have

**12:26** · heard about that right we are going to speak a bit more on that but the cni it's the container network interface it's the plugin that manages those uh those uh the networking of the container so it is a thing that does all of those IP allocations it reads the PO C IDR and so on so that's this configuration that exists on every node and it says and when cuet is running it will read those configurations and say like Okay cool so when I'm creating this new pod I should actually call uh those scripts here from

**12:54** · those from the cni and then start allocating the IP addresses based on those configurations usually when we have kind or when you have like a managed uh uh cluster you don't care much about that right unless you are uh actually you know doing something on premises or you are doing something uh using some different cni that is been provided by the cloud provider okay so I

### Understanding services

**13:15** · think we can move yeah there was this P that I already did so let me yeah you kept using that word um cni so we're going to go ahead and discuss that so as we're talking about um adding those interfaces you saw all of those commands it can be very cumbersome very uh burdensome so what this project is the

**13:35** · cni project container network interface it's a separate piece of software so one of the things that especially when you're starting out you have to understand is that kubernetes is a collection of software that works together right you got the cuet you got the API server all the controllers those are all separate pieces of software and then with the cni it helps with a standard way to manage network interfaces these are open source projects there's lots of different options I work for a company that supports one so ilium is there CU

**14:00** · brouter I think is the default and then you know flannel there's lots of other ones there's the repo you can go check that out so it is a specification for how to define that file and how to create and add interfaces and manage routes and IP addresses so we talk about this the cni is required to implement that kubernetes networking model so we've talked about the kubernetes networking model and the cni implements

**14:22** · that for us and so we have see here we have that picture so we continue to build on that picture so we have the cni that manages the the interfaces the routes and then allocates the IP address the the IP addresses and the interfaces for us so we did mention Services when we talked about the fight club rules um there are several different types where again have 30 minutes so we're going to talk about the two big ones uh cluster IP and node Port so you've one of the

**14:53** · key characteristics with Services is that you know pods are going to change pods are going to die nodes are going to die those IP addresses will change and that's not really helpful from a client's perspective having to memorize and know all those IP addresses how many of you guys know the IP addresses of your applications yeah so from a Services

**15:12** · perspective that gives us a single IP address for an entry point so if the orange bot here is trying to communicate to the green service to the green pods to get information whatever the API is the database whatever is running there it knows that it can use this IP address to communicate and reach all of those backend services and I think at this point we have oh I also want to show this again continue to iterate that it's using underlying Network Linux networking technology so here we've got

**15:39** · an IP tables rule that's throwing the natat routing and I think we've got no we've got another we don't have the demo yet um so this the next one is node Port so from a nodeport perspective right we talked about um those external IP addresses for the nodes and if we have an application that's running with a Services of node Port what kubernetes will do for us is it allocates a port on

**16:03** · each one of those so that I can reach 17211 on 32,767 and it will reach my applications Back in Forth so instead of an IP address I can use the noport service from that one and this will get a little easier when Ricardo demos that for us okay I was going to say that what whoever knows the IP from the head they are not using IPv6 probably so let

### Service networking demo

**16:31** · cool so let's see here uh first of all as James said and I think that we are kind of use it to right so I have the parts here and I have those IPS and uh I can actually again do a curl call uh from my IP right so I'm calling this first IP here and I say I get okay hello cuon and actually we have logs right but

**16:53** · what will happen when I delete that P right so I went to the P I'm going to delete that P uh I'm kind of you know I keep forgetting my things so that's why I do keep CT get always uh okay so the IP of the Pod actually changed right so actually the node changed so if we see

**17:10** · uh the Pod from the back end was running on the on this uh worker node here right so was the node one and then it moved to the node 3 and as we've seen before even the cidr the network of that that that pods on those those notes change it right so if I try to call it I'm going to get some time out cool uh and the way

**17:29** · that I can actually uh solve that right for okay it's with a service so the service is an abstraction it will allocate also an IP but that IP won't change if I delete the Pod right so I'm creating a cluster IP here I will take a look into the service IP uh and as we can see I have now this back end which is this 1096 14226 right uh just up

**17:57** · house not the container uh if you sorry if you take a look into the second IP here which is kubernetes it ends on one we are going to say uh a bit about that IP specifically IP later right but that's an special service IP uh so now

**18:13** · if I go again to my container to my other pod and do a curl I'm going to see that I can C with uh from the service IP right and not from the Pod IP anymore uh but still the IP that I'm getting here at least because the traffic is internal of from the cluster it's the original uh pod IP right and I now I deleted my backend pod again uh the Pod IP changed and it changed the node again and if I did a curl to my service it's still working right uh behind the scenes

**18:44** · what's happening here it's uh and we are going to see we have this component which is called Q proxy and it's injecting rules on the nodes saying okay so now I have this service IP and whatever my pods on the on on my node they try to reach the service IP I will select the pods that are serving the

**19:03** · service and I will reach them and this is implemented by this concept which is called this resarch which is end point slices right so uh when you create a service you have that yo file and you say like I want to select all of those pods and it will keep reconciling that and saying like Okay so the the Pod IP changed so I change on the endpoint slice and then qu proxy will note that and say like okay so the IP Chang it so I will change on the rule here right cool so so uh just as just another example here so the service IP is also configurable right so as we can see here

**19:37** · uh during my Cent configuration I have set this service IP but as opposite to the Pod IP it it's not going to break on on cidrs for the node it's just going to allocate this there are two exceptions ip1 will always be allocated as a service for kubernetes API IP 10 will be

**19:55** · always allocated to the DNS and we are going to take a look into the DNS later okay other than that I don't think I've never seen actually happening uh you know allocating from 1 to 10 something in the middle I think it's reserved but uh other than that it would just do random allocations here uh yeah going to talk okay and here

### Kube-proxy implementation

**20:16** · uh uh as James say like we have this component CU proxy and I just wanted to show you like CU proxy actually injecting rules on the Node so uh as we can see here I have these NF tables uh had a talk just earlier today of like the maintainers of cube proxy showing that they have replaced IP tables for n tables for uh performance and we can see there is this table here saying like Okay so I I reach part uh 010 on Part 53

**20:43** · tcpr RPA I will send to DNS end points or this one to the uh to this back end and and so on okay cool uh let me see what's the six one and if I should because I okay uh if I show note part it's going to break because cuz I forgot to fix do you want to see it breaking shame okay sorry so just showing how note part works I think that we still have time yeah we're good Okay cool so

**21:08** · I'm am going to uh change my service here uh and make it as a note part right so now it's a not Port as we can see here I have a an external Port allocated this is random as well and depends on how you configure your API server usually it's from the 30,000 to the 32 something I don't know but you can extend the range it's in the darks yeah and and now I will call

**21:35** · instead of calling the my internal IP I'm I'm going to my node or something external right and I'm going to call the external IP and as I can see from any node IP that I call I can now get into my service okay so this again it's Q proxy injecting rules but as the

**21:50** · opposite saying like whenever you receive a traffic from the external of the cluster reaching this part you should on any of those notes you should send to any of those pods being backed by uh the service right there are some exceptions like you can control this Behavior say like just send to pods on the same node and so on but we are not going to go that down but it's a I'm happy to discuss about that cool uh okay

### Cluster DNS

**22:16** · so you keep doing your slides keep drawing the pretty pictures um so he already did you already um so we're going to talk about Q proxy again so Ricardo is talking over his slides like I do I have that problem it's fine uh so Q proxy again is another piece of software that runs on each a node and it maintains those networking rules that we're talking about so all those NF tables rules when a service change Q

**22:39** · proxy is responsible for changing those and it uses like I said the operating system packet filtering so IP tables or NF tables depending on your implementation it routes all of the traffic doesn't EF epbf yeah if you're using celium in Q proxy mode or Q proxy replacement mode um and that's responsible for the services to mapping so again you don't have to be responsible manually for changing all of those node IP node uh pod IPS we have

**23:07** · too many different types of IP addresses too many IPS and I don't think you have a specific qy proxy demo you just did that one I just did that okay you doing things out of order for me and we practiced it yesterday so you know too many parts so IP addresses are hard to remember we've got a bunch of different types as I'm already stumbling over them but what about names instead of IP addresses um we'll go ahead and you're going to go talk about services in quns yeah okay so uh again now we we do have

**23:35** · the uh service IP right and uh but imagine you have like Nam spaces and on all of those names spaces you want to reproduce an architecture so you want to have your back end and you want to have your front end and you are going to have the database maybe all of them on the same name space right so you have production and you have K and you want to reproduce that but you don't have the IP so you create a service IP right and

**23:59** · when you do that you will need to probably make your backend speak with your database and you want to make that reproducible so kubernetes instead of uh uh actually had this concept of the DNS of that we are going to show on the CNS

**24:15** · right that you can say okay so instead of calling 1096 whatever I'm going to call backand and when I call backhand CNS will go and say like okay so Ricardo is calling backand the namespace do service. cluster. loal and it's going to trans to us for the same service that is running on my same namespace right okay so I'm going to do you want to the button I can I can pess the button and you can talk yeah cool all we are we are fine in time yeah that was a

### DNS and CNI demo

**24:47** · good promise okay cool so let's take a look into the uh DNS and it working right so uh what's going on can you you broke it hold on okay so I have the pods I

**25:06** · have the PO IP is here I'm going to call it I have the services I can still do curl against the service right um I'm just proving you that I didn't change any from my last demo but now what happens if I call uh HTTP and backend which is POS it the service name right

**25:24** · so with that if I keep creating the service with the same name on different them spaces I have reproducible uh architectures reproducible blueprints right I don't need to keep doing adding on all of my environment uh uh uh variables or my configuration the service IP instead of the uh uh the name right and the way

**25:44** · that it happens um okay so I'm going to get the logs and I'm going to show you and then uh if you take a look we have this CNS pod running here it's installed by default you can replace but I wouldn't do that I I know I don't know why people would do that actually it works pretty fine uh and if I take a look into my DNS configuration of my pod what you're going to see it's that the name server do you remember when I say like the IP do10 is reserved to the DNS server right

**26:17** · so all of my pods by default unless you change something on the configuration on the Manifest they will have the ipth and they will have this search PA that says I will try to search do mpace SVC cluster. looc then cluster. looc and and and so on right so this is how DNS Works internally on the Pod so you call just the name the service name it will go to the DNS and say like hey give me this full name here and the card DNS will go to kubernetes API check what's the service AP and return you back that API

**26:50** · okay cool it's not and we're back to the beginning all of our pces are there we've got C DNS proxy cni and uh all of our pods connected um other opponents to consider so some of the things so this is again very basic intro to kubernetes networking something to consider is that all of these pods can talk to every other pod um by default so you would need something like a network policy that would deny um connectivity between

### Advanced networking concepts

**27:21** · these pods so from a security perspective please Implement Network policies and then one thing we didn't talk about um is ingress controllers and Gateway API controllers so from a again everything that we talked about just now was internal to the cluster and in 30 minutes we can't really um go through all of the noteboard services and all of the abstractions but this is um how you get traffic into your cluster from that perspective from external um we've got some resources here again we'll put these up there um

**27:53** · we've got the networking cinat book where we walk through all of that um in a much more uh detail description um we have the presentation survey here so please give us feedback tell us if we actually accomplished it or if we satisfied your morbid curiosity and the Sig networking Community Gateway API Community would ask if you are using Gateway API please take this survey and let us know how it's going that' be very helpful so we'll leave this up here for a few seconds and on time 30 minutes yay

**28:24** · any questions we have six minutes for questions are singing no tell me

**28:39** · why okay thanks everyone thank you folks sure okay sure let me we are just going to repeat your question then okay yes okay so the question is like you have several nodes and you have Cube procs and all of them and how they communicate with with each other so they don't right all of the cube proxies they go to the API server and they say like what kind of rule I need to to uh program here

**29:13** · right and then uh uh using that internal routing like pod top that you've seen you actually have your node speaking with the other pod on the other node if it's running on on the other node right but that's the same path that's the same R in that that it happens but you you have this extra hope if you get into a note that it's on a different uh uh

**29:39** · uh uh the IPS the nodes have their own CER range so you saw that sl24 that's assigned to that node yeah

**29:59** · yes yes but because we we do that split of like this SL 24 it will know like I'm calling actually something else on some some different place there's an underlying assumption that all of the nodes are routable to each other so that's when it has that route it knows to go talk to that node does that make that's why we showed the routing rules on the Node

**30:28** · uh it actually it actually gets the Endo slices is and understands you mean the IPS that it should program right okay so it go to the API server gets the endpoint slices object and it will know that like hey this service SL inpoint SLI has this IP addresses so those are the IPS that I need to program here yeah and then it Maps those to the IP tables and F tables rules so that's what Q prox is responsible for sorry

**31:03** · \[Music\] it depends so the question is like if the Pod exist on the same service if the Pod exist on the same note that you are calling from an Old Port right or eventually internally so it depends so there is this flag which is like internal uh traffic policy and external traffic policy that you set on the uh service that they will say like when the traffic reaches from the outside to the node I will just try to send to

**31:31** · the same to any place on the cluster or just to the same place right what can happen is actually uh first of all I don't know if there is some kind of prioritization for the pod on the same node when you are using the cluster model but if you use the local mode which is this one that we just try to use the local and you try to reach like

**31:50** · a host that doesn't have the Pod it will fail in that case it will just break so usually you use that when you have a load balancer in front of them so just one be answering or just the ones that have the yeah we didn't we didn't have time to go into external and internal traffic policies no it stays inside the no yes if it's external traffic equals to local it will stay inside the no okay

**32:29** · for same service for different pods yeah yeah so we didn't get a we didn't dive too much into I said like green labels so you can use label selectors with multiple services so you can have two Services going to the same

**32:54** · pods if they are the same pod I mean the same pod with different containers with the same port yes if you are trying to change to S different pods like you have back end front end and you want to use the same service then no you can't