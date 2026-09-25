---
title: "Container Networking From Scratch - Kristen Jacobs, Oracle"
source: "https://www.youtube.com/watch?v=6v_BDHIgOY8"
author:
  - "[[CNCF [Cloud Native Computing Foundation]]]"
published: 2018-12-16
created: 2026-09-22
description: "Join us for Kubernetes Forums Seoul, Sydney, Bengaluru and Delhi - learn more at kubecon.ioDon't miss KubeCon + CloudNativeCon 2020 events in Amsterdam March 30 - April 2, Shanghai July 28-30 and Bo"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=6v_BDHIgOY8)

Join us for Kubernetes Forums Seoul, Sydney, Bengaluru and Delhi - learn more at kubecon.io  
  
Don't miss KubeCon + CloudNativeCon 2020 events in Amsterdam March 30 - April 2, Shanghai July 28-30 and Boston November 17-20! Learn more at kubecon.io. The conference features presentations from developers and end users of Kubernetes, Prometheus, Envoy, and all of the other CNCF-hosted projects  
  
Container Networking From Scratch - Kristen Jacobs, Oracle  
  
Learn how to construct an overlay network across multiple hosts in just a few lines of bash! Containers in a system such as Kubernetes need to be able to communicate, and a common networking solution is to use an overlay network, for example, Flannel. In this talk we aim to 'demystify' container networking, and it's constituent elements such as Linux bridges, veth pairs, routing routes and TUN/TAP devices. Starting with defining a simple network namespace, we will work through networking between containers on the same machine (using the default docker model), up toward a full overlay network spanning multiple machines (as in Kubernetes). We will explain both how this works and why/when it is required, providing the necessary background for understanding and evaluating common existing Kubernetes networking solutions such as Flannel and Calico.  
  
To learn more: https://sched.co/GrWx

## Transcript

### Introduction

**0:02** · hello cool so um I work for Oracle and Oracle has a managed kubernetes offering and uh a little while ago I was given the the job of looking at replacing the networking layer which was flannel with some features of the the Oracle cloud

**0:17** · and uh that all sounded really good and I started digging into it and I soon realized that I didn't really understand how flannel worked and uh it kind of seemed wrong to replace one thing with another thing if you don't understand the thing you're replacing so uh I I kind of dug into it a bit deeper and after a while it became apparent that I didn't really understand any of this networking stuff at all so um yeah so

**0:39** · long story short went out a big rabbit hole learned some stuff but more importantly I kind of realized I really enjoyed this stuff so um I thought I'd write a talk and come and spread the network in love a little bit so yeah my name is Chris in the next uh 30 minutes or so I'm going to try and explain how a container on one machine can get can connect to a container on another machine and what the various mechanisms are to allow you to do that uh yeah so if you already know this stuff now is your time to leave uh cool so first of all we need to

### Agenda

**1:11** · know what we're aiming at so we're going to well go with the kubernetes model because it's Cube con and all so um basically for a network layer and kubernetes to to be compliant it needs to follow these three rules but what that really boils down to is every I every pod in the cluster has its own unique IP address and um each one noes

**1:30** · need to be able to talk to each other just using that IP address and there's no address translation in between and also uh pods need to be able to talk to nodes and nodes be able to talk to pods that's it so that's what we're trying to work towards and uh how are we going to do this so we are going to uh get there

**1:46** · in kind of four steps and for each of these steps uh I'm going to show a kind of rather badly handdrawn diagram and talk about that a bit and then I'm going to uh show some code in in bash and everybody loves bash so that's good and then I am going to run that code and then we can get in and sort of you know capture some packets and ping some interfaces and things and see how it all hangs together so the four steps are as follows first of all I'm going to look at uh the simplest possible thing just a node with a network names space and look at how that can connect to the node uh

**2:19** · The Next Step we're going to stay on the one node but have two Network namespaces and look at how you can connect send packets between them next step move out to two nodes and then look at how the packets go between the two nodes containers but in this case just on the same L2 Network and then finally the kind of more General case is two nodes separated across different networks and we'll see how that works so let's get going so this is the um the simple case

### The Simple Case

**2:49** · so we've got the big diagram uh the big rectangle on the outside so that represents the node so that could be a bar M machine or it could be a VM kind of doesn't really matter that's got an interface here it's rather confusingly called EMP 0 S8 but it just treat it as e z it's only called that because that's what the virtual box uh called it uh it

**3:08** · has an IP address 1010 and then the little box in the middle which I've named labeled con while that represents the container so maybe we should just take a step back a bit so a container in Linux is is a process with a bunch of mechanisms Linux mechanisms to kind of isolate that and uh and they include c groups and Nam spaces and various security things but from the point of view of network connectivity the only thing that really matters is the network Nam space so um when I say container from now onwards really what I'm just talking about is Network namespaces here um yes so what

**3:44** · we need to do well actually first of all what is a network namespace so the way I see that is it's kind of separate instance of the kernel networking stack well another instance so that would involve three things so you got extra interfaces a separate list of interfaces you've got a separate list of IP uh IP

**4:02** · tables rules and a separate list of rooting rules so if you go into a network names space and type if have config you'll see one list of interfaces do that on the host on in the default namespace you'll see a different list so there are the kind of things we got to kind of play with to wire these things together so the first thing is how do we get connectivity between the network namespace and the host and the way we do this is we have a use a thing called a v pair so that is a kind of Linux Network

**4:29** · working construct I kind of visualize it as a sort of ethernet cable with a interface card at either end so it's a pointto point thing put packets one end they'll come out the other end and you get one of these ends and you put it inside the network namespace and the other end will leave in the default namespace on the host so uh that way we get connectivity between the two we're given the interface inside the container an IP address 17216 so that's a different network than the host and uh the final piece of the puzzle here is to set up some rooting rules so on the host

**5:00** · we have quite a simple rooting rule that's basically saying if you want to go to that IP address 17260 send it straight to that V one interface and then the rooting rule inside the container well there's only really one interface and there's nowhere else it can go so we just need one default rule which just Roots everything back out again and it kind of looks a little bit confusing because we're we're saying the Gateway is well we are the Gateway in this case uh but yeah it works this way so that is the setup so

**5:28** · what we're going to do now is try and um have a look at what this looks like in code you see this Okay cool so I'm running in a different uh a new virtual box a new virtual machine here and uh here's some code which will when we run it will set up this this model and uh I

**5:47** · say it's bash it's not really it's just lots of calls to the same command so here I'm using the IP Command and that is kind of well it's kind of the only one you really need to know in order to set up any of this networking stuff it kind of subsumes the the older if config and rout and things so I'm not going to

**6:04** · go through every line of this um uh this file because that'll take a while but I just wanted to give you a kind of flavor of what it might look like but basically we go through and and set up all the bits in that diagram so we create the namespace we create the V pair we put one end of the V pair into the namespace we enable we bring up the interface bring up the interface of the node set the loop back in there we don't really need to do that but done anyway uh and then we set up the roots both on the Node and in the namespace so let's give it a go see if

**6:33** · it works so now I'm going to run that so the first thing we can do is that'ss uh so there we're just listing the network namespaces and that's the one we just created so now let's have a look at the interface inside the

**6:54** · namespace so here I'm doing a netns exac so you can kind of think of this as a bit like Docker exec or Cube controlled exact you know kind of going into namespace and running a command in there uh and there you can see the bottom one is the V2 interface that's the one we created and then let's see if we can ping it from the

**7:18** · node cool so that kind of seems to work so that means we've got connectivity going both ways uh which is good and uh so yeah one question we could ask is is what is actually responding to this because normally when you think about containers you're thinking about a process which is in a container and that's the thing that's kind of you're talking to but in this case I haven't put any process I've just created a namespace uh so I'm sending ping requests icmp packets in and it's an it's the the kernel stack inside the

**7:48** · namespace that is responding with the the response the icmp responses so I guess if we wanted to set up a more kind of realistic example we might uh want to start a process inside there and talk to that but from the point of view of just investigating the connectivity we kind of don't need to so I'm just going to do it this way because it's easier um yeah so next step Oh wrong way is to move on

### The Single Node Case

**8:18** · to uh the case where we have two Nam spaces on a single node so here it kind of looks similar so we got the big node on the outside we've got each container there are separate Network names spaces uh given different IP addresses here so one is 0.2 one is 0.3 con 1 and con 2 uh

**8:36** · the V pairs are exactly the same uh the thing that's different in this case is that rectangular box in the middle and that's the way we achieve connectivity between them and this is a a Linux Bridge so again just like the eth pairs this is a kind of Linux networking construct which you can create with the IP Link command and so I've created one

**8:56** · here called it br0 and I've given it an IP address so so I guess you don't really need to give it an IP address uh if I didn't then you still be able to root packets between the two containers but there'll be no way off of that bridge onto the host and ultimately out onto other machines so by giving it an

**9:13** · IP address here it kind of becomes a gateway to that little Network that little subnet and this is kind of the way that Docker works by default if you just install Docker it creates a Docker zero Bridge which is exactly like this so the final piece of the puzzle here here the the two uh the roots that we have to set up so on the Node we've got

**9:33** · a sl24 range so I'm assigning all the IPS in that sl24 range to uh all of the containers that are hanging off the bridge uh that means I can have 254 I guess of them uh so anything in that range will get rooted to the bridge and then from inside the container we have uh so the bottom rule there is saying anything in that range just send it directly out the v21 interface so

**9:57** · that's a directly connected Network ly connected route uh and if it's not that then use the default route and the default route is saying use the bridge IP address as the Gateway so that would rout it out to the bridge and then when it gets out to the host well that would have to look at its rooting tables to kind of forward it onto wherever it's going to wherever it's destined for cool

### The Single Node Code

**10:21** · so let's move on so here we got uh rather like before we've got a um a kind of clean virtual machine so let's have a quick look at the code to set this up so as you might expect it's it's kind of similar to uh what it was last time but there's two of everything uh so again I'm not going to go through each step because it's basically the same as before uh the key Point here is the line

**10:48** · was that 26 so that's where we're creating the bridge so as I said you can use the IP Link command and you can um uh create a link call it br0 and it's of type Bridge and then we have to we assign a IP address to it a little bit lower on line 33 and then we enable the bridge and we're all good to

**11:12** · go so right so what can we do so we can look at the interfaces on the on the Node now and we can see the bridge is the one at the bottom here and it has the IP address 16.1 uh maybe let's get inside of one of the containers and see if we can ping the other

**11:39** · container so I am exec into con one which is the container on the left the network names space on the left uh and I'm going to Ping the one on the right so this is looking good it means we got connectivity both ways all good actually one thing worth mentioning here is that value of the the TTL so the the TTL is um time to live of the packet and

**12:08** · that gets it's just a number and it gets decremented each time a packet gets rooted uh it starts at 64 so in this case it hasn't been rooted at all because it's just come out one interface onto the bridge straight back in so there's been no routing going on it's just a single ethernet packet going around the bridge I only mention it now because in the next section we'll see this changing and might make more sense then uh also kind of proves that I'm not

**12:29** · yeah pulling the wool over your eyes or something uh oh and finally yeah we can just check out so from inside the container let's make sure we can talk to the node itself and again that's good so that means we've got icmp packets going in and out so we got connectivity both ways

**12:55** · right so let's move on to the third step oh uh and so now we're getting we're kind of doubling up again almost so this is the ca the key point about this case is both of these nodes are on the same uh layer 2 Network so they're just connected by a switch here so the node on the left has a 10 10 not on right 1020 they're in the same subnet um otherwise they're much the

**13:22** · same so each one is same as the one before we got two containers in each connected with v pairs got a bridge on each all good so what we need to understand well what I'm aiming to get across here is how you can get packets from one container on one note to a container on a different node and the trick in this point is is really quite simple there's nothing to it it's it's just setting some rooting rules on each of the nodes so as they know where to root the packets for the other node so if you look at say the rooting rules on the left in the bottom left hand corner there we can see uh the key one is the

**13:55** · second one down so that's saying each for IP address which is destined for the containers on the right hand node send it as a next hop to the node itself and then the node will know how to rout it up into the bridge uh and likewise we have a corresponding rule on the other node

**14:14** · such that any of the containers in the 0.024 range send it the next top to the node on the left and that will know how to rout it up into the bridge so if you just have a your kubernetes cluster on on a single L2 Network then this becomes quite an easy way of getting connectivity that you don't need overlay Networks you don't need any of that magic and this is the way some of the the kubernetes uh plugins do it so there's a flannel has lots of different back ends and one of the back ends is a host Gateway back end and that's exactly what this does it just sets roots on the on the Node uh I think also Calico might behave

**14:46** · in a similar manner as well if you're all on the same uh L2 Network so if you have more than one node of course you might have you you'll have an entry per per node so you might end up with big rooting tables and you need some way to manage that so you'll need to some way to somewhere to store the range of ips on one node to the node itself and that could be ETD or it could be somewhere else but we'll get to that a bit

**15:13** · later so let's get back into our demo so now we've got two clean virtual machines one on the left corresponding to node on the left one on the right the other one so first let's have a quick look at the code to set up this

### Demo

**15:30** · uh all of this stuff is basically the same as what I um went through previously uh the key stuff is underneath the little comment down the bottom here and here we're setting the roots on each node to know how to root to the other node so I'm going to run this setup script on both VMS such that you get the roots going both ways um

**15:50** · yeah so it's kind of as simple as that the final thing we need to do is to enable IP forwarding on the Node so if I didn't do that Linux by default wouldn't forward packets out so if it received a packet on its e zero and it wasn't destined for the IP address of that interface it would just Chuck it away and that makes a lot of sense if you just got a laptop or something you're not acting as a router but in our case we are acting as a router because we're going to get packets coming into the E zero destined for one of the containers so the kernel needs to know to rout it onto the bridge so we have to enable IP

**16:24** · foring cool so if I run it on that side run it on that side so so first of all let's have a look at the roots on each side so on the left hand side here you can see the key root is the bottom one so that's saying any of the IPS in

**16:48** · the range 1.024 send it to the other node and there' be a corresponding route on this side sending it back the other way so that was what I saying that's the kind of trick to get connectivity when you're on the same L2 network uh let's have a quick look so now let's see if I can ping one of the containers on one node to the other

**17:11** · one so I'm executing into con one on the left hand node and I'm going to Ping con one on the right hand node cool so it looks like it's it's working we got connectivity both ways um

**17:29** · remember I said earlier about the TTL so in this case the the time to live has gone time to live has gone down by two which is kind of what you expect because it's been rooted twice it's been rooted once when you you can't really see it here so it's been rooted once when you come out of the on the Kernel on the left hand node and root it again going back onto the bridge on the right hand node hence the decrement of of of two if I were to do the same but instead ping the other node so not going into container but going to the node itself

**18:00** · what would happen well it works but now it's only gone down once which again is you know makes a lot of sense because it's only been rooted on the left hand node and not on the right hand node so now we get on to step four which

### Overlay Networks

**18:17** · is the the kind of the more complicated one and the one I kind of been building up to because it represents the kind of uh what I didn't understand in the first place about flannel and overlay networks and things so well I guess before we move on to that what what could we do so if we if these two nodes here were on separate L2 networks and that switch in the middle wasn't just one switch was the internet or other routers and all the kinds of things then this trick wouldn't wouldn't work any longer because it wouldn't the next top

**18:44** · wouldn't be on the same network so one thing you could do is add those routing rules to all the routers in between and maybe if you control all those router in between then well maybe that'd be okay but I suspect that's probably not what most people can do another thing you might be able to do is well it depends where you're running if you're running in a cloud environment and and the cloud provides some sort of root rule capability so I think Amazon and Google do this you can assign IP ranges to

**19:11** · nodes and then you just do it in the cloud and that's basically what what it would do for you then so again instead of using any kind of more complicated overlay you could just assign these root ranges and allow the cloud to root root it for you but let's assume we can't do that either so what are we left with so so we are left with using well one option is to use an overlay

**19:44** · Network so in the um example here we basically got a similar setup two different nodes same container same Bridges same everything key Point there's a router in between so we can't pull the same trick that we did in the last step the one thing that's different is we got this ton zero interface so this is a wow this is another yeah this is a bit that made me understand the kind of how you can set up these kind of virtual networks so a ton zero interface is something you can create using the IP tool or a ton interface and if you just

**20:13** · create one it will show up in if config as an interface but there's nothing behind it so normally when you have a network interface there's some sort of Hardware or some virtual Nick or something but in this case there's nothing behind it and uh so it doesn't seem very useful but what you can do is put a process behind that and that process when you send a packet to the ton device the process will get that packet the raw IP packet and it can do whatever it wants with it so it could uh

**20:37** · print it a standard out or it could you know send it to the printer and physically print out if it want to um but what we could do is have that process behind it uh wrap it in say a UDP packet and send it to a node and that's exactly what happens in a in an overlay Network so you don't need so the two nodes don't need to know about the separate IP ranges of the containers they just need to be able to connect via their node addresses and um yeah so

**21:03** · we'll go into this in a bit more detail on the next Slide the last thing we need to do here is explain these rooting rules so on the left hand side we're saying everything for the all the containers on my node just send it to the bridge the rule beneath it saying everything for the containers on the other node send it to the tund device and likewise we've got corresponding roots on the on the other side

### Packet Routing

**21:28** · so let's drill in a little bit and have a look at how a packet actually makes it from one container in the top left hand corner all the way around to a container in the top right hand corner and um yeah so the packet comes out of the container goes onto the bridge uh it comes out of the bridge the kernel will then rout it to the ton device and we want to set up a process which sits behind that and it knows

**21:53** · because it can see the IP address it's got the rule IP packet it knows which node to send it to so like I said before it might look up in a database like etcd or something and it looks at that mapping and then it knows where to for it on to so in this case we we we're created we wrapping in a UDP packet sending it to the other node on Port 9000 so it goes out of e Zer goes

**22:12** · through whatever Network there is in between comes back in E zero on the the right hand node there's a process sitting there which is listening on 9000 which gets that unwraps it it's just got the rooll IP address then it sends it back into the tund device and when it comes out of that the kernel will then notice that as the original packet and Route it up into the bridge and hopefully to its

**22:34** · destination yeah ah there's one thing as well um this was when I did this talk once before someone asked this question and I thought it was a really good question they basically said but isn't UDP unreliable and you kind of kind of stumped me there but it kind of doesn't matter in this case because we're getting our reliability at a higher level so it's the TCP stuff on top

**22:55** · inside that's what would actually do the retries if this failed you can kind of think of the the UDP connection as a bit like just an Ethernet like send it on the wire that's not reliable either but it doesn't matter because the retries are handed by handled by the the layer above so UDP is okay in this case and this is exactly how well the UDP flannel back end Works which we'll get to again in a minute

**23:21** · so right I guess you're getting it now it's a similar setup to before let's have a look at the code so again all this stuff is exact exactly identical to before before if we pop down we can see the stuff specific to uh this step so as before we have to enable IP forwarding but the key thing here is we are using socat to set up this tunnel

**23:48** · between the two the two nodes so if you not come across socat before I only come across it recently when I was looking at this because I fully expected to have to write a little process behind this just to set this demo up and do that UDP stuff and I come across socat and is amazing so this tool sets up a bidirectional route between two or two endpoints and those endpoints could be TCP or UDP or standard in or stood out or ton devices and um yeah if you just

**24:13** · type man socat it'll blow your mind it's amazing it does loads of stuff so there's a lot going on in this so remember we're going to be running this on both sides and uh so just this one line is saying set up a ton device give it an IP address bring that interface up behind that tund device I want the ud process which is supp listening on Port

**24:34** · 9000 so any packets that come into it it will receive them and send them to the tund device and it bro is sending stuff out on Port 90002 to the other node so any packets that come from the tund device it will send them out e Zer onto the other node and because we're running this same thing on both sides we get connectivity between the two oh and finally there's a yes so

**24:56** · there's a couple of little um I say or other things that you need to do to get this to set up so when you start dealing with overlay networks you have to kind of worry about the MTU so this is the maximum transition unit and uh that b yeah so we've got to account for the 8

**25:11** · byes of UDP header hence I'm setting it here to 1492 uh bumping it down from 1500 which is what it was before uh if we didn't do this it would probably still work but if you just got a packet which was just above that bit it will get fragmented and that's well in this case it probably wouldn't matter but in the general scheme you don't want to do that um so yeah it's just something you might have to be wary of if you're setting this stuff up yourself finally we've got this um stuff about disabling reverse path filtering and um so this is a little

**25:41** · little more subtle so Linux by default if you send a packet out of one interface and it receives a response in on a different interface it will just drop that it will consider it as kind of suspicious which kind of makes a lot of sense really in general but in this case when we're sending a packet from say one node to a container on the other node

**25:59** · the packet will go across the ton device going up to the container but on the way back it will just come straight out of e zero and back into the node hence the packet's going out of one interface coming in on a different interface uh so unless we disable this in this case anyway this demo won't work I guess there's other ways you could work around this maybe you can do some sort of um more complicated routing stuff using kind of source based routing to ensure that the packet goes over the thund device no matter whether it's destined for the node or the container

**26:29** · but yeah in this case I've chose to do it this way so let's have we go running this stuff so run it on both oh no it's typical it didn't work let have another guy

**26:59** · let's try one more time before I start resorting to uh videos or something y it's looking good so uh what was I can yeah so first of all let's see if it you know if it's connected see if it actually works so as before I'm going to exec into the container on the left hand node and try and ping the container on the right hand node

**27:35** · cool so that's working and uh as before you can see the TL's gone down by two which is what you would expect it's been rooted on both nodes and um and if I ping the node itself then it should be down to 63 so there yeah so that proves we got connectivity so if we draw in a little deeper now and actually look at what's going on with the packet as it traverses through the through the through the various interfaces so on the one side

**28:01** · I'm going to run this little script which just pings continuously so let's set that one going on the other side I've got this little script running which uh basically um given an argument of the interface name it will uh do a t-sh it use t-shark to kind of sniff the packets on that interface and t-shark if you haven't come across it is like the terminal version of wire shark and it's a bit like TCP dump it's it's great great for this kind of stuff uh so first things

**28:29** · let's try and capture the packet coming in sort of through the front door so this is the EMP 0 S8 cool so as you can see the packet is coming in through the front door but the source and the IP addresses here are of the nodes itself so this here you can see the encapsulation in in in process

**28:50** · so you don't see anything about the IP source and destinations the 172 ones of the actual containers that's invisible and that stuff is all held Within the the little data section at the end so you can see it's an Ethernet packet wrapped in IP packet within the UDP packet and then the data which would be the IP packet of the container itself so now if we stop that and kind of drill in one level deeper and we go to the ton

**29:18** · device now you can see the packets have been unwrapped and you can actually see the source and destination IPS of the actual containers itself and uh likewise you can see just like I said it's a raw IP packet and inside of that you can see the icmp packet which is the the Ping and then we can just drill in one more

**29:40** · step we should see now we're capturing the packets on the bridge on the right hand node and you can see it's the same it's the same Source an IP uh packet so of the containers themselves uh but you can see now it's been ripped uh wrapped in an Ethernet packet so it's been wrapped in Ethernet packet sent onto the bridge which again is is what we would

**30:00** · expect so that is basically kind of the whole overlay Network just kind of you know patched together in a few lines of bash there uh so what's this well let's have a quick recap first and see what we've done so we've gone through these

### Recap

**30:17** · four steps and so the first step was the single Network namespace and the key point there was uh if you want to connect name spaces to nodes you can use V Pairs and then the second step if you have more than one network namespace on the same node where use these pairs along with a a bridge uh the third step

**30:34** · was the case where we had multiple nodes but they were on the same L2 Network and that was a kind of easy one where you can just set up some routing rules to just directly hop to the node the destination node and then the fourth step is what we just did and the key point there was you you can use a ton device to create the overlay Network and uh a couple of like key takeaways at least for me understanding the different types of routing rules that was kind of my you know my kind of aha moment for understanding this stuff and uh and ton

**31:01** · devices well they're know one of the ways you can well it allows all this kind of virtual magic to to work and in terms of tools well you've got IP for setting all up you've got socat for just creating bir directional streams for testing and then for debugging stuff your TCP dump and t-shark uh they are your friends so finally uh I just want

**31:21** · to kind of bring all this back to sort of real life I guess and try and relate it back to some existing stuff that exists out there uh so one of the common um uh Network Solutions for kubernetes is is flannel and flannel has a bunch of different back ends and uh they work in different ways so one of the back ends is a host Gateway back end which exactly corresponds to step three so that's when you have the all the nodes on the same R2 Network one of the back ends is a UDP back end which is basically what step four does uh so although it doesn't do

### Common Network Solutions

**31:52** · it with socat and things um it in essence it's the same but that wouldn't be the one that you would typically use in in production or anything that's more like a kind of almost like an educational backend as far as I can tell maybe debugging uh so what it would really use is vxlan and vxlan is is an overlay Network it's a UDP thing but it's implemented in the kernel and it's I guess it's more efficient and things and then we also have these Cloud specific backends so they set roots in the cloud which I talked about earlier so one for Amazon one for GCE and um on

**32:20** · the other thing I want to characterize in these different um Network Solutions is where they store their uh node to IP range mappings because they all do it slightly differently so in the case of flannel well flannel just stores it in

**32:36** · SCD uh there's Calico that's a a popular one um there's I believe you well all of these things are so configurable but you can set it up such that there's no overlay for just L2 stuff so it uses a step three next toop routing uh for cross Network stuff it can use another type of overlay Network which is ipip

**32:56** · encapsulation uh but I I'm sure you can configure it to use other things too um and in terms of it's no to pod subnet mappings I believe that's done via bgp so you run bgp agents on your node and they kind of Gossip this around uh weave is the final one or another one in terms of connectivity similar to flannel so it uses vxlan which is the UDP overlay uh but the difference being it doesn't use SD I believe it has its pod subnet to node mappings distributed peerto peer

**33:26** · somehow so that's basically all I've got to say so all of these scripts and stuff you can uh just go on to this um uh get her page and um and grab them and FID around and and send us some comments if you know that' be great uh yeah any \[Applause\]

**33:54** · questions got about about one minute I think so no

**34:15** · y I guess it's sort of well it is simulating like an L2 switch so it's it's kind of using ethernet package just as the way a normal physical switch would don't understand why they do that why would they oh okay I see I think they're probably yeah you probably don't have to you could do it other ways I'm sure it's just the way it works here cool