---
title: "Kubernetes Design Principles: Understand the Why - Saad Ali, Google"
source: "https://www.youtube.com/watch?v=ZuIQurh_kDk"
author:
  - "[[CNCF [Cloud Native Computing Foundation]]]"
published: 2018-12-16
created: 2026-09-23
description: "Join us for Kubernetes Forums Seoul, Sydney, Bengaluru and Delhi - learn more at kubecon.ioDon't miss KubeCon + CloudNativeCon 2020 events in Amsterdam March 30 - April 2, Shanghai July 28-30 and Bo"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=ZuIQurh_kDk)

Join us for Kubernetes Forums Seoul, Sydney, Bengaluru and Delhi - learn more at kubecon.io  
  
Don't miss KubeCon + CloudNativeCon 2020 events in Amsterdam March 30 - April 2, Shanghai July 28-30 and Boston November 17-20! Learn more at kubecon.io. The conference features presentations from developers and end users of Kubernetes, Prometheus, Envoy, and all of the other CNCF-hosted projects  
  
Kubernetes Design Principles: Understand the Why - Saad Ali, Google  
  
Kubernetes is quickly becoming indispensable for managing and deploying workloads on distributed systems across both cloud and on-prem environments. While most people are now familiar with how to use Kubernetes, few are aware of the “why” behind it? Why does the Kubernetes API look the way it does? Why do Kubernetes components only interact with each other through the Kubernetes API? Why is there a PersistentVolumeClaim object when you could easily reference a volume directly from a pod? To answer these questions and help you develop a deeper understanding of Kubernetes, this talk exposes the principles underpinning the design of Kubernetes.  
  
To learn more: https://sched.co/IrkE

## Transcript

### Intro

**0:00** · so my name is Sally I'm a software engineer at Google I started on the Google kubernetes team pretty early on way back in December of 2014 when the team was pretty small and it was before kubernetes 1.0 I was a young engineer

**0:15** · there was a lot that I didn't know and I'm not very smart so you know I didn't really pick up on the kubernetes ideas very quickly for me when I start to learn things learning all the little things how to do them doesn't really work for me what I need to do is try to understand why things work the way that they do and that kind of sticks better in my head and so I recognized some patterns that I've seen in kubernetes and that kind of helped me understand the API and understand how kubernetes works a lot better at least for me so I

**0:46** · was hoping to share some of that with you today for a lot of you this may be material that you're already familiar with but it's interesting just to call it out and recognize it and see it apply to future patterns so what's in it for me a deeper understanding of kubernetes again an important tool for learning is to understand the problem the why not

### What's in it for me?

**1:10** · just the what if you get a large collection of what's it's impossible to memorize everything but if you understand the fundamentals of how things work then it's easy to extrapolate and under you know come up with your own understanding of how things should work and that can lead you pretty far what is kubernetes I think

**1:30** · most people in this room probably probably already are familiar with that by a show of hands how many of you know what communities is all right good how many of you have deployed a pod in production all right good well just to recap and set the stage I

**1:47** · want to introduce what the problem space was that we were working with you know before kubernetes was a thing before docker was a thing folks were running distributed systems largely either on bare metal or in virtual machines when containerization started to take off it provided a way to be able to do consistent repeatable reliable deployments you could have multiple applications coexist on the same machines and not have to worry about their dependencies colliding for example

**2:17** · but you know containers are great but you have to figure out how to actually deploy those containers at scale and that's where kubernetes came in you didn't want to get into the business of actually writing your own system to be able to you know SSH into every machine and start a docker container and write some monitoring service to monitor those containers so kubernetes really filled that niche well let's take a step back

**2:44** · and talk about the very fundamental operations in kubernetes so how do you deploy a workload on kubernetes when I first came to kubernetes my understanding was kind of based on my previous knowledge which was very much a you know there is two entities and

**3:01** · there's a master and a slave and the master tells the slave what to do in this case you know you have a user a user wants to start a container the model in my head traditionally would be they pick a machine and they say start you know this container on that machine of course there's a set of problems with this approach what happens if that container crashes so this was the model that I was talking about before kubernetes where if you were SSH into

**3:30** · machines and starting your container that container died or that node died or you know for some reason that SSH connection dropped for a second and you weren't able to actually get that container started what happens you have to write a lot of custom recovery logic to monitor your service monitor your application and then you have to take recovery action so if you know the service isn't behaving as you expect it to you'd have to write custom logic to recover it and so you're writing a lot of custom logic just to keep your application running so this is where the

### Principle #1

**4:05** · first principle of kubernetes comes in and that is that the kubernetes api is declarative rather than imperative so in kubernetes you do not tell the system start this container on this machine instead used to define the desired state you say that I want to have a container running please make it so this is the difference between for example a pilot flying a an airplane

### Declarative APIs

**4:33** · themselves versus setting an autopilot when they're flying the plane themselves they're constantly providing input they're monitoring their trying to figure out where it should go when they set an autopilot the computer takes over and there's control systems that monitor and make sure that whatever the requested altitude is maintained so you can think of the declarative API in a similar way it's a way to say this is what I want my intended state to look like please make it so and the difference is basically this is what kubernetes is all about instead of

**5:04** · having to provide a set of instructions and monitor things and then provide more instructions you just say make this happen and that's the basis of kubernetes so let's get a little bit more concrete the way that you create or

**5:21** · do anything in the kubernetes api is that you create an API object that is persisted on the cube api server until you delete it and all the components within kubernetes will work in parallel to drive towards that state so for example when you want to get a workload running you can use one of the basic primitives the replica set which many of you are probably familiar with that says I want to create a container maybe an

**5:47** · engine X container and I want one replica that running across my system once that object is created the system kind of figures out what needs to be done next and schedules the workload appropriately and gets it started you're pretty much hands-off at that point you don't need to go in and monitor the status of your workloads that all kubernetes will take care of it and what

### Why declarative over imperative?

**6:12** · is the benefit of this approach this declarative approach and the primary benefit is automatic recovery meaning if something happens to your application if the application crashes if the node crashes kubernetes will automatically take care of recovering that application for you moving it around so now let's

### How to deploy a workload?

**6:35** · revisit this or deep dive into this a little bit more so what act Tooley happens when you create a replica object on the kubernetes api server you say I want a pod of you know a certain definition created on my cluster how

**6:54** · does the node figure out that it's supposed to be running this does the master tell it like how does all of that work my natural inclination would be that since you have a master the master would figure out which node this particular workload would be scheduled on and then it would just call out to that node to say please start this container and that's if you look at a lot of traditional you know server client setups this is the way that things work you may you issue in a network RPC and the caller tells the

**7:27** · server or the client what to do I'm sorry this the client tells the server what to do and the server will execute that action in return either a success or a failure so in this case that was an option the kubernetes master api server could have been built in a way where it could call out to the node that it selected and say please start a container but there were the same set of

**7:51** · problems that we discussed with a user manually telling you know a specific machine to start a container apply here as well right think about what happens if a container dies or if the node itself dies or if

**8:10** · the node is unable to accept that command at the moment that the master decided to send it that command how would it recover from that if we had an imperative API where the master was calling into the nodes to tell the nodes what to do that means the master would have to be infinitely more complicated the master

**8:31** · would have to begin to store the state of every single component that it was responsible for it would have to play catch-up every time it discovered that these components weren't doing what they were expected to so first it's figuring out what they're supposed to do understand that they're not doing what they're supposed to do and then issue follow-up commands to fix that so that means that every time uber Nettie's is extended every time a new component is added this master control plane grows larger and larger and larger and if kubernetes had been

**9:01** · built that way honestly I don't think it would have been very successful it would have made the master very complex brittle difficult to extend so this is where the second principle of kubernetes comes in and that is that the kubernetes control plane is transparent and that there are no hidden internal api's this

### Principle #2

**9:20** · is extremely powerful for multiple reasons what does it mean actually what it means is that that same declarative API that we exposed to the end users is the same API that all the internal kubernetes components use to interact with each other and so the benefits of that declarative API then apply to the components themselves so whereas before

**9:44** · the if we had followed the model that I was suggesting earlier where the master would call out to individual nodes to tell them what to do if we follow that model you would imagine a master would have to provide a exact set of instructions to say the node should do something the node would execute those instructions and return a response and

**10:03** · then the master would have to have some sort of way to monitor those nodes and then issue you know commands to rectify anything that went wrong instead what happens is that the master basically defines a desired state for a node to be in once the master figures out that a workload should be scheduled on a particular node it will essentially

**10:24** · state that in the API server so the master just talks to the API server and then all the components including the node work independently to drive themselves towards that state so let's take a look at what that looks like more concretely so the way to think about it

**10:40** · is that the kubernetes api server is the center of everything in the kubernetes world this is what you interact with but it's also what every other component in kubernetes interacts with so basically what happens is that all the nodes when they first come up will monitor the kubernetes api server to try and figure out what are they supposed to be doing so instead of having the decision being centralized and being sent out every component is responsible for its own health and keeping itself running so whenever a component comes up it goes

**11:11** · to the API server to figure out what it should be doing the benefits of this approach are that if a component crashes for some reason and it comes back up it can easily recover just by looking at the API server and figuring out what it's supposed to be doing this pattern is called level triggered instead of edge triggered you can imagine a system

**11:33** · that's based on events to be edge triggered where you issue an event and then the system will do something but if the system is down for some reason and didn't receive that event you're responsible for sending that event again versus level triggered meaning you set this state and then if there's a

**11:51** · momentary issue with any of the system systems components whenever the system comes back up it can just look at what the current state of the signal is and be able to operate off of that so this is the way that the kubernetes API works again and let's just walk through that so you as a end user operate against the

**12:12** · same kubernetes api you declare that you want a workload running you define what that should look like and then your hands off now there's a object that exists in the API server that defines what should be happening and all the other components are watching the kubernetes api server going oh something needs to happen here I need to make sure that I do something the scheduler is

**12:37** · just another component in kubernetes that all it does is monitor the kubernetes api server looking for unscheduled pods unscheduled pods are just API objects that are of type pod and don't have the node field filled in so if it finds one of those it starts executing its logic to figure out what the best placement for this particular pot is going to be once it makes that determination instead of calling out to that node it just updates the API object

**13:06** · to say hey this object has now officially been scheduled to a particular node and then the node that it gets scheduled to is also watching for pod objects but it's filtering those pod objects too pods that match the same name as itself so it's a very simple concept but very

**13:24** · powerful and once a node recognizes that there is a pod that should be scheduled to itself it can look locally and see if that the containers for that pot are running or not and start the process of getting those things running and as long as that as that API object exists the

**13:42** · the node agent the cubelet can basically ensure that it's remains in that state so if that application crashes for some reason and the pod indicates that it should be restarted the the the node the cubelet agent can automatically do that

**13:59** · so then when you're when you decide that you don't want that workload you simply remove the API object from the master and then the components see that there's a difference there's something running in the system and that doesn't correspond to the desired state set by the user so all the components will work to drive towards the desired state which in this case is please stop work running this workload it should not be running

### Why No hidden internal APIs?

**14:28** · so what are the benefits of this no hidden internal API principle I think I talked a little bit about this already components that are level triggered instead of edge trigger tend to be more robust especially for distributed systems where you have to expect that you're gonna have so many components

**14:50** · that make up your system something is always going to be failing and so you have to design your system for reliability and tolerate be able to tolerate failure if any one of those components so if your system did was edge triggered it would have to be much more complicated in order to have one component constantly figure out what the state of everything should be and figure out where what it should be doing instead if you distribute that responsibility to every single component

**15:17** · to figure out what that component itself should be doing it makes the system more reliable but also more extensible and there is also no more single point of failure so in the previous example if the master was calling out to everything if the master goes down your cluster is down in with the with a model

**15:38** · that kubernetes has with a kubernetes api server acting as a central point if the kubernetes api server goes down all the components continue to operate on whatever the last date was that they saw and when the kubernetes api server comes up again they can start operating on whatever the new state is and if any one of those components goes down the other components can continue to function independently of that so the there is no single point of failure and the system overall is more robust and it also makes

**16:13** · kubernetes very extensible and this is I think the big reason that kubernetes has become as large as it is is because it's very simple to add on to it because all the components within kubernetes use the same API that you used to interact with it you could swap out any one of these components with your own implementation so for example if you wanted to write your own scheduler all you need to do is write a binary that is able to interact with the kubernetes api server and fill

**16:42** · in the node field of a pod object that is what a scheduler does it it looks at pod objects it runs some algorithm to figure out where it should be run and it updates the API server so you could write a custom schedule or for kubernetes pretty easily of course writing a custom scheduler is not an easy task of course but the the idea is that kubernetes makes it possible and this is true for all the components that exist within the kubernetes ecosystem ok

**17:15** · now let's talk about some of the interesting information that exists on the kubernetes api server we have the idea of secrets you know if you have some sensitive information you should not be putting those inside a few containers obviously that information

**17:35** · should somehow be injected inside your container at runtime so that these sensitive things aren't going to be leaked along with your container and the facility that kubernetes provides to do this is the secret API object in kubernetes it's just another

**17:51** · API object and you can declare a secret give it a name and then provide set of key value fields similarly there is a way to be able to provide configuration information to your application so for example your application may have some set of startup parameters for a particular configuration or whatever

**18:10** · other configuration information you may have and downward api is another kubernetes api that allows you to fetch basically name namespace UID pod information about the pod that this workload is running in all of this information can be very valuable to the application itself so how does a kubernetes application get access to this information we talked about the

### Fetching Kube API Data

**18:38** · fact that this kubernetes api is you know it's the same api that's used by the internal components which means that if you write an application that needs to use a secret you could basically have that application call out to the kubernetes api server to fetch that secret and use it within your application you can also have a call out to the kubernetes api server to fetch information about the pod that it's running in or any configuration information from the config map api objects this is possible but it's not

**19:12** · necessarily the best solution because of kubernetes principle number three which is meet the users where they are what we want to do with kubernetes is make it easy for folks that already have applications running to transition into kubernetes if we required folks to actually modify their application in order to work within kubernetes that would be a non-starter for a lot of folks I you can you're all probably

### Principle #3

**19:42** · aware of some legacy application which hasn't been touched in a very long time it knows how to consume secrets from a volume or an environment variable but nobody's going to go in and modify that to have a new kubernetes api right so what we did was in addition to being

**19:59** · able to call out to the kubernetes api server there is the ability for you to consume secrets config maps and downward api objects as files within the container or as environment variables so as part of your pod definition you can specify please expose to my containers

**20:17** · this set of secrets or config map information and kubernetes will automatically make sure that that information is either mounted in as a file into that container or as an environment variable and so your application doesn't have to be modified as long as it knows how to read a file or read from an environment variable so

### Why meet the user where they are

**20:41** · why did we do this and of course the reason is to minimize the hurdles for developing applications to deploy on kubernetes we want to make it as seamless and easy to transition to kubernetes as possible again I keep mentioning that you know there are these are the reasons why kubernetes has been successful and I think one of the reasons is because it made it easy to transition some of the existing applications to kubernetes ok next up

### Remote Storage

**21:12** · let's talk about remote storage this is an area that I am very familiar with I'm the storage stake lead and the idea here is that you know you containers alone

**21:27** · are not sufficient because they are ephemeral if you start a container and you write anything inside of that container that file system gets basically deleted as soon as that container is terminated if you're running any sort of stateful application you need some way to be able to persist state beyond the life of an individual pod and the facility that kubernetes provides to do this our kubernetes volume plugins we have a number of volume plugins that allow you to plug into remote storage systems so if you're

**21:58** · running in a cloud environment you could be using for example GC persistent disks or Amazon EBS block volumes you could use an NFS share of different options exist one way to be able to use them is in your pod definition to directly reference the type of volume plug-in that you want this of course isn't the exact gamal that you're going to use but the idea is that inside your pod configuration file

**22:26** · you specify the volume that you're going to use so in this case I would say that I want to use a GC persistent disk named PD one so as soon as I define that on the API server kubernetes now needs to figure out how to make that volume available to the container and how does it do that so first up we have our lovely scheduler that'll kick in and try to figure out where this volume of this workload should be scheduled it realizes

**22:57** · that node one has availability so it's going to schedule Tod a to node one simply by updating the pod configuration API to add a field that says it's been scheduled to node one now there's a second component this is the attached detached controller the attached detached controller like every other component in kubernetes is monitoring the kubernetes api and acting on a set of API objects in this case it is

**23:27** · looking for pods that have been scheduled to a node and are also referencing a remote volume when it finds one of these pods what it does is figure out if that if that volume is available on this on the node that it has been scheduled to so for example in this case pod a has been scheduled to node one but GCE PD one is not available on node one so what this attached detached controller would do is take action to

**23:56** · call out to the GCP back-end to say please attach gcpd one to node one there's a step that I'm skipping here which is it now actually updates the kubernetes api server to say that the volume has been attached in a prior code

**24:16** · it didn't use to do that but with the current iteration with CSI one point and which is coming out in kubernetes 1.13 there's a new object called the volume attachment object which represents the state of a volume attachment and that's used to communicate back and forth and that's through the the kubernetes api server so

**24:37** · once a volume is attached to the node the cubelet basically is just operating on API objects and it notices that there should be a pod running and this pod is referencing a GC persistent disk so it will monitor the node object to figure

**24:55** · out whether that volume has been attached to that node or not once it has been attached it'll go ahead and proceed with making that volume available inside the container so when your container starts on this machine that remote volume is now available inside your container and anything that you write to that directory will be persisted to that disk if your volume dies for any reason

**25:19** · it gets rescheduled to a different machine the attached detached controller will move that volume to the other machine and your persist your storage is available anything that you wrote is persisted whenever that container comes up on any other machine so this pattern is very powerful and it uses a lot of what we've been talking about but it's not something that you should be doing you should never reference a particular type of storage directly inside your pod and let me talk about why that is the

**25:52** · case the problem is that when you reference a particular type of storage inside your pod your pod definition is no longer no longer portable so if I reference a GC persistent disk directly inline in my pod definition if I were to take that pod definition and move it to AWS or on-prem where that GC persistent disk storage just doesn't exist my pod is not going to be able to start running so in order to fix this problem

**26:26** · we created an interface called the PVC and the PV a persistent volume claim and a persistent volume the purpose of these objects is to decouple the request for storage from the actual implementation of storage so now in your

**26:44** · pod definition instead of referencing a GC persistent disk what you would reference is a claim a persistent volume claim and a persistent volume claim is simply another kubernetes api object this kubernetes api object contains generic information about the type of storage that you want it could include things like the capacity of the storage you want the access mode whether it's readwrite read-only information like

**27:12** · that so it's a very generic way for you to describe the requirements that this application needs for storage and then there is a controller that exists called the persistent volume controller that will try to match your request for storage with what storage is available in this case you have two options one is

**27:31** · that as a cluster administrator you could create PV objects ahead of time that represent the storage that's available on your cluster and then the persistent volume controller will bind any PVCs that it sees with an available piece of storage whenever the pvc object is created so what this means is that if you were to transition into another environment your pod and PVC objects the objects that are user-facing are now portable so as soon

**28:02** · as you drop it onto a new cluster as long as there are some PV available on that machine that can fulfill your PV C they'll get bound and you'll have persistent storage available to you this goes a step further with the storage class which allows automatic provisioning of storage so if the PVS

**28:21** · have don't pre exist and there's a storage class defined on your system when a PV C is created the storage class defines how to create new storage from scratch and that storage class is specific to that cluster environment and so as long as a storage class with the same name exists in both environments some sort of storage will be able to be created for you work load to be able to for for you to use and the application specific API

**28:52** · objects that you have remained portable across environments and this is basically the fourth principle of kubernetes it's about workload portability and I think this is the most critical piece of kubernetes the way

### Why Workload Portability?

**29:09** · that I think about it is like an operating system if you went back before operating systems or a thing folks were writing applications directly for specific hardware and those applications were not portable across different machines and an operating systems basically said don't worry about the underlying hardware that's available on this particular machine right to this interface that I provide you and as long as this operating system is running on one of these environments your application will work and that allowed

**29:42** · application ecosystems to thrive in the distributed systems space we haven't really had anything equivalent for a very very long time basically everybody was writing custom applications for custom cluster implementations and what kubernetes has done is it's acting like an operating system for distributed systems it basically has southward

**30:08** · facing API is that it'll hook into different clusters and it has the kubernetes api that it exposes to end users and as long as application developers deploy against that kubernetes api as long as kubernetes exists on that cluster their application just works it doesn't need to be modified it can be moved around from cluster to cluster to cluster it

**30:33** · basically excuse me a decouples distributed system application development from cluster implementation it acts like a true abstraction layer so in review the four principles that I talked about were the cube API server being declarative rather than imperative I want to get some audience participation can someone tell me why that is important sorry say that again self-healing yes able to rollback extensible so no hidden

**31:07** · internal api's immutable that is an excellent point so act should include that in the presentation as well his point was that it's immutable regardless of the the great thing about this kind of configuration that kubernetes has is you can actually check it in two source code and it's repeatable you can deploy

### Kubernetes Principles Introduced

**31:39** · you can you know how to create a application from scratch as long as you deployed this set of llamó files on any cluster that has kubernetes you're going to have your application running so it is immutable let's talk about meeting the users where they are why is that important Yussef adoption yes and then finally workload portability why does that matter cloud agnostic cluster agnostic that's

**32:13** · right it's a it's a portability layer it's an abstraction layer there is a principle of being able to separation of concerns basically if you design your system with a separation of concerns and you isolate different components with very distinct of basically contracts

**32:36** · between them it becomes much more easier to test much easier to upgrade and maintain overall and kubernetes basically acts as a separation of concern abstraction layer that says don't worry about anything below this layer I'll take care of that you worry about the application layer that is all that I have if you have any questions uh please yes you know Lightning talks they talk about custom definitions CRT idrd isomorphic

**33:14** · to good question so the question is about CR DS custom resource definitions and how they relate to pot objects or any

**33:39** · other API object that currently exists as a built-in type in the kubernetes api so for those of you who aren't familiar with what CR DS r CR DS are a way to be able to write your own custom kubernetes api object type so for example you are probably familiar with the node API object or the pod API object but if you wanted to create your own API object the CRD allows you a very easy way to do that you define a new CRD of your type

**34:06** · and you define the schema for what that looks like and then you can create your own objects you could call it you know foo or blah or whatever you want against the kubernetes api server and then you could write your own custom controllers that use those CRS and C RDS

**34:23** · to basically act extend kubernetes and so the question is how they relate to these built-in types I like to repeat what Tim Hawkins said about this which is that if we had C or D Zonday 0 for kubernetes there would be no built-in types the idea is that the kubernetes

**34:44** · api machinery should be distilled down such that it can be used as just API machinery for any project not just kubernetes so we're starting to see it be used for things like Sto and other higher-level projects and there should be no built in API types meaning that if we started from day 0 pods nodes everything else would also be a CR D

**35:18** · yes well persistent volume claim is similar to a pod when we first came up with a persistent volume claim and PV it was kind of modeled after a pod in a node where a node represents the cluster resource that's available and a pod represents kind of a request for using that resource so a PV represents the resource the storage resource that's available on that cluster and a PV C represents a user's desire to use that resource and they kind of bind together

**36:01** · it's a weakness of what system I missed the first part a declarative system strong consistency guarantees in what regard

**36:27** · right yes so so the comment is that one of the drawbacks of a declarative system is that you can't offer consistency guarantees and this is absolutely true which is why kubernetes is an eventually consistent system it will try essentially forever to drive towards the state that you specify and this works

**36:54** · well for a lot of the use cases that we have in kubernetes especially in terms of getting workloads running and keeping them running but it is definitely a challenge when we have patterns that don't necessarily fit into this model well so for example on the storage side of things we're looking at the ability to take a snapshot a snapshot is a very point in time time constrained operation

**37:20** · and when you have something like that it is a little bit difficult to model in a declarative API if you're interested you can come to the storage sake and we can talk about how we're trying to make that work there are issues with it but we have been able to get something working any other questions all right thank you very much \[Applause\]