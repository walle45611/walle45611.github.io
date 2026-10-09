---
title: "Containerlab - a Modern way to Deploy Networking Topologies for Labs, CI and Testing - Roman Dodin"
source: "https://www.youtube.com/watch?v=snQTlFahY1c"
author:
  - "[[The Linux Foundation]]"
published: 2021-11-12
created: 2026-10-07
description: "Containerlab - a Modern way to Deploy Networking Topologies for Labs, CI and Testing - Roman Dodin, Nokia"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=snQTlFahY1c)

Containerlab - a Modern way to Deploy Networking Topologies for Labs, CI and Testing - Roman Dodin, Nokia

## Transcript

### Introduction

**0:05** · hi and welcome to the talk container lab a modern way to deploy networking topologies for labs ci and testing my name is roman i'm a systems engineer with nokia where i primarily work on network automation with model driven angle i do love open source communities and try to contribute to them as much as possible if you want to talk to me you can look me up on linkedin or ping me on twitter my twitter handle is ntdbps

**0:32** · so let's talk about labs they are not any more a nice to have feature they are really a necessity each networking team should have it might be that the labs are the only way to tell that things work as they should or they don't as they shouldn't

### Current Lab Challenges

**0:48** · so being able to run those labs with virtual networking topologies is really important let's recap what options do we currently have to run those labs what advantages do they have and maybe identify some disadvantages as well of course it makes sense to start with the elephant in the room the network emulation software that has been specifically built to answer the need to run networking labs projects like even gmg and s3 are the prime examples here

**1:18** · they are proven we've been running them in the labs for years we know how they work they are free and available to download for everybody although some premium features of evg might require some extra bucks they have a nice ui so you can really create a topology by drag and dropping uh nodes on the pane and inter interconnecting links with your mouse so that is really nice to have on the flip side though these projects

**1:46** · they are really vm centered that means they have they have weak container support and i will talk later about why i think container support is really important for the network emulation of today and tomorrow they are also quite heavy and not fully open so if we need to install even g or gns3 we need to allocate a server or a

**2:07** · bare metal machine and we need to keep it allocated for as long as we need to run the labs so we cannot really deploy and destroy the labs on demand with the resources dynamically allocated the nice ui that these tools have can also be seen as a disadvantage even for example if you want to generate your topology then you would need to have a text file that defines the topology or if you want to push your topology to the git repository then as well the ui might stand in your

**2:38** · way what else do we have we have a cluster of projects which are basically vm orchestrators projects like openstack for example they are very common in medium and large enterprises they used to to run vms right and it is

**2:55** · quite tempting to reuse those orchestrators to run networking labs but there are the problems coming to use those projects as the network emulation software it requires quite a lot of integrational effort you need to know how to work with the primitives that these projects expose they of course steal your vm centric because they are vm orchestrators right

**3:20** · sometimes it might be even challenging to have a clean data path for example if your network topology uses linux bridges in the data path it is really probably even impossible to have lacp frames flowing or even have some some simple things like lldp these projects they do not have a specific ui nor specific cli that can

**3:44** · deploy labs or networking primitives for you they do have ui and cli for the generic purposes like work with the snapshots with images deploy instances etc but those ui and clies they are not

**3:59** · specifically built to run network topologies and of course we can have some custom automation solutions those bespoke scripts of thousands of lines of bash that can trigger or deploy libvert vms or create containers they can really answer the

**4:17** · needs that you have they they can be a silver bullet the only problem is that they only tailor it for you so being able to reuse those scripts in the customers environment or in the partners environment or even in the different departments environment might be really challenging as you see i mentioned weak containers support as a

**4:38** · downside for both network emulation software and vm orchestrators let me explain why moving from vms to containers can bring some substantial benefits to the networking labs first of all by making containers the first class citizens of our networking labs we welcome all the containerized network operating systems such as nokia sr linux arista cos juniper crpd and others

### Benefits of Containers

**5:03** · next to that by using containers we are inherently going light and fast containers are typically lighter than vms so we can have a bigger topology which will consume less resources and will be very fast to deploy also by using containers we are bridging the gap between i.t workloads and networking topologies because we can leverage the same familiar features of containers such as bin mounts port exposure

**5:28** · environment variables and so on when your lab is defined in a declarative fashion in a text file it makes it really git friendly you can push it to a git repository and use all the familiar git features such as collaboration pr reviews

**5:44** · versioning and so on using the git for storing your networking lab topologies again bridging the gap between the normal it workloads and the networking labs one of my most loved features of using containers for network topologies is the new way we work with network operating images so

**6:02** · before we used to work with cucao images and deploy vms out of it and we needed to upload these images to dropbox or onedrive or ftp to share it with somebody now when using containers we're inherently using container images and container images are pushed to container registers right so now you can version your images and push it to container registry and everybody else can pull it if they have the certain rights and authorization and by running networking labs as the containers you make it really easy to integrate those topologies in your ci cd pipelines

**6:34** · really it is just a bunch of containers and links between them so you deploy them pretty much as any it workload that you have in your ci cd pipelines it is fast to deploy it is lightweight and thus it's it is really easy to integrate network topologies to ci cd pipelines that you might have and that is where container lab comes into picture it tries to bring all the benefits that we just briefly discussed and provide a cli for setting networking

### Introducing Containerlab

**7:01** · labs with container based nodes it can be used to apply complex topologies like you see on the left hand side or it can also be used to create some small topologies that you need for your ad hoc testing or dev integrations or similar now we have a container lab documentation site that you see below in this slide that probably will answer all

**7:22** · the questions that you might have around this project it's quite a comprehensive documentation portal so please do check this out and although we invested a lot in the documentation i really believe that learning by doing is the best way forward so i'd like to bring you on a journey where we will first install

### Demo Journey Overview

**7:38** · container lab then we will get to know what is the topology definition syntax that containerlab uses to deploy and define the labs we then of course will deploy the lab and and manage it then we will see how to access and configure the nodes that will that will be part of our lab we will talk about the configuration persistency because it's quite important to understand how to work with configuration how to save it how to retrieve it and how to make your nodes start with a predefined configuration and the last step would be to package our lab and push it to a git repository

**8:11** · the lab that i chose for this demo is a three node lab topology that demonstrates a route reflection use case we have three bgp speakers here the go bgp linux container will inject around 192 168 101 32

**8:26** · towards the containerized network operating system arista cos a risto will act as a route reflector and will reflect this route towards another containerized network operating system nokia s or linux sr linux will successfully receive this route in its routing table and we would like to verify that you can see the addressing information and the timeline of this lab on the left hand side installing container lab is super easy all you need to do is type this one command highlighted in blue and contain a lab binary will get installed on your

### Installation

**8:55** · operating system we do support three major operating systems such as linux windows with vsl 2 support and mac os for other installation options you can check the documentation site we where we outline all the other options available the only thing you need to have on your system is the docker installed the rest

**9:14** · is packaged into the binary of container lab itself so let's install containerlab on our system first we go to the container lab documentation site where we navigate to the installation section to get the installation command we choose the auto installation script and we go to our system we paste it here

**9:34** · we type enter and in three seconds we have our container lab already installed it is really easy as that with installation step out of the way we can see how container lab really works it all starts with writing a topology definition file the topology file contains your links nodes and some parameters that your lab needs to have containerlab then takes this file and deploys the real nodes and and wires the links between them let's starting this topology definition file we will create a file named rr.clap.yaml

### Topology Definition

**10:06** · where rr stands for route reflection and this file is written with yaml syntax so we will follow the basic yaml rules first we need to to give our lab a name and that would be rr then we will have to define the topology container which will have a nodes subcontainer

**10:24** · and the nodes subcontainer will host all the nodes that we will need to deploy in our lab now if you remember we'll have three nodes and i will start just with the first one with nokia as or linux node so each node must have a name and that is basically an arbitrary string that you give to your node to distinct it from other nodes the mandatory parameter for every node is its kind kind basically tells container lab what node this is is it an sr linux

**10:52** · container is it cos container is it a basic linux container you give this input you provide this information with kind and of course since we are working with containers we need to specify the container image that this node will use to start the image is really the same

**11:09** · string parameter that you would use in docker run command or in the kubernetes pod specification now that we know how to define a single node let's take a step further and define all the three nodes that our lab must have as you see on the left hand side we defined three nodes the sro linux that one we defined on the step before then we added cus node which is the arista cos container and we also added our go bgp node notice how cus go bgp and sr linux nodes

**11:38** · all have different kinds because these are three distinctive containers with three different rules to start them hence the different kinds they all use of course they use different images and that is also reflected here now our logical view at this moment

**11:55** · represents three nodes deployed but there are no links whatsoever to interconnect the nodes of your lab you need to work with links we need to specify a links section in the topology file which consists of list of end points the endpoints element in its turn defines the beginning of the link and the end of the link so putting things into the context if we need to connect sr linux interface e11 to cus interface

**12:22** · eth2 we will need to create the end point that says my fo my first and is sr linux e11 and my remote and is cos eth1 same goes for another link in our topology between cus interface eth2 and go bgp interface eth1

**12:40** · the cool part about container lab is that those 17 lines that we defined here is everything we need to start the lab so to deploy a lap there is a command container lab deploy where you supply the topology file that you created and then container lab will immediately get to the deployment process and it will show you the summary table once it's finished but instead of looking at the slides let's let's really go and look at this live so i have created the once 2021

### Deploying the Lab

**13:12** · directory where i have my topology file rr.clap.yaml so if we look at that file we will see that it basically the same i just copy pasted it from the slide right it's exactly the same content we have three nodes we have two links and that's

**13:31** · all we need one important thing here that i would like to articulate specifically is that those images they need to be available to contain a lab in order to deploy the labs so how do you get those images right for nokia srl linux it's really easy you can pull the image directly from the public github container registry without any registration or licensing agreements

**13:56** · with arista cos it's a bit different because you still need to have an account with arista site but you can pull the archive from this side freely then you can load this archive as a docker image and you're done go bgp container is really just a basic linux container with gobi gp installed hence the kind linux here i wanted to show you that i have the images already pre-pulled so i have nokia server linux container pulled from the public github container registry and i

**14:29** · have downloaded cus star archive and loaded it as a container image but i do not have go bgp container for example the reason is that i wanted to show you that container lab can pull the images on the fly when you start to deploy the lab so let's try and deploy this lab i will use container lab deploy dash t and we'll specify the path to my topology file and i hit enter so see right now container app actually

**15:00** · pulling the network multi-tool container image and since it's in the github container registry it can do that because my linux machine has access to internet a few seconds later once the all the images are pulled or present in the local image store container lab proceeds with creating the containers and creating the wires between them now the last step is to actually wait till arista cos node render online and

**15:26** · container lab will configure the management address for this particular image okay 40 seconds later since the deployment started container lab shows us the summary table in the summary table we see that we have three containers or lab nodes created the name consists of three elements the prefix

**15:47** · syllab the lab name rr and the node name all these parts except for the prefix are coming from the topology we named our lab rr and we named cus node as cios we have the container id we have the images that we used to to spin up these containers we see the kinds but most importantly we have the ip addresses that were assigned to the management interfaces of those containers and this brings me to my next section which is how do we actually get

**16:20** · access to those nodes there are two ways to get access to the nodes first you can execute the process inside the container typically you would do this with docker exact command for example to execute the srcli process which is a cli application inside the sr linux container you can do docker exec

### Accessing Nodes

**16:40** · it the name of the container and then the name of the process but we also can connect to the ssh server that runs inside the containers because these containers are really a networking operating systems they do run ssh servers inside them so we can use the ssh client and connect to those nodes immediately we can use the lab container

**17:02** · name such as clap or rcos or we can use the address that comes from the summary table that container lab shows us at the end of the deployment process let's now connect to the ssh inside this linux container i will use ssh admin which is the default username for sr linux container and i will copy paste the node name here as is as that i get access to sr linux

**17:31** · cli so i can now configure the node and save its configuration for future use to connect to the arista cos i can use the the other trick i can use the exact process so if i use docker exec it i will copy paste the name of the container and i will call the cli application which is the cli processor inside inside arista cs and just like that i get access to the cos cli

### Node Configuration

**18:02** · let's talk about how we can configure nodes once the lab is deployed we already saw that we can get access to the cli of the nodes and that is probably the most obvious and simple way to configure something on the network in operating systems but apart from the cli we also try to enable all the programmatic interfaces on the nodes that container lab support such as gmi netconf etc so you will be

**18:25** · able to use those interfaces if you deploy them with containerlab we can also use something that is coming from the dockerland configuration mount we can mount the files from the host to container and if the operating system expects to see the configuration file by a certain path we can use those bin mounts and that will make network operating system to read these configurations we also can work of course with third-party configuration management tools such as scrapply ansible narnier

**18:57** · etc because your nodes are really accessible to the host you can use those tools without any issues and soon we will have an embedded configuration engine inside container lab so you will be able to generate configuration for many nodes that contain a lab support using the variables that you will define as part of your topology

### Route Reflection Demo

**19:18** · all right now when we have our lab deployed we can go and jump right away into the configuration of the bgp and route reflection but before we do that it's really handy to refresh ourselves which nodes do we actually have in our lab and to do that you can use the container lab inspect command when it is provided with all flag it will list all the all the labs that are currently deployed so we have just one

**19:42** · lab which consists of these three nodes i will start with configuring cus first i will copy paste its name i will ssh into it and i will just paste the config that i have so this configuration is basically configuring the link ip addresses and configures the pgp i save this configuration and i will proceed with sr linux config same way i ssh into it

**20:14** · and i will copy paste the prepared config for sr linux i will save this configuration and what i would like to do now i would like to test if my bgp pairing between sro linux and cos is up and running so for that i will install a watch on the show command network instance default protocols pgp

**20:38** · there is a single neighbor that i have that is the link i p of the cus i want to see if i get any ipv4 outs so right now you see that the sr linux reports that it has no routes received from the cus neighbor although the neighbor is up and running we just happen to not have any routes okay so now it's time to configure the code bgp that's our

**21:10** · injector of the route in the router reflection topology so to connect to the go bgp i can also use the container name but now go bgp container doesn't have any ssh server running so what i will do

**21:27** · is that i will leverage the execution of the process inside the container i will paste it name here and i will just execute the bash shell now when we are in the bash shell we can start working with go bgp and actually inject inject the route again i will copy paste the objp configuration here in just a second so as you see google bgb configuration is a bit more involved because you need to create the objp yaml file that b2b

**21:54** · will use to actually configure itself and then we will create the announcement with by adding the address with certain attributes that i chose to use here so if we save this command and we execute it now we will see that in five seconds go bhp will start to announce a route now if i switch back to sr linux

**22:20** · you do see that the watch reported that we now have a single route received from the cus so just like that we deployed a lab of three nodes really fast it doesn't really consume much resources at all compared to the vms and we configured the use case for route reflection demo but it is a bit too too involved we we adapt quite a lot of commands on the cli

**22:45** · can we do this in a more like declarative form where we would have to specify the resulting configurations and the nodes would pick it up and yes we can do that containing lab actually has a state directory where it keeps all the state information for the nodes of a certain lab this state directory is called lab directory and it is named sealab dash

### Saving Configurations

**23:08** · lab name so for the case of the route reflection lab container lab creates a lab called c lab dash rr so in this lab we will have the state that is kept for the nodes of this lab and we can use this lab directory to get the resulting configs out but before we can do that we can also actually save

**23:29** · the running configuration to the startup configuration for that we created container lab save command which executes an appropriate command for every supported operating system which will save the running configuration to the startup file and we can get this startup file from the lab directory let me show you how it works so if we go to our container host i need to

### Lab Directory Analysis

**23:53** · switch to the directory container lab save dash t and then our topology now contain lab will perform the safe configuration both for the cus and sr linux because it knows how to do that and now if i go to the lab directory which is clap dot rr i will have two subdirectories for cus and sr linux so if i drill to the cus directory there is

**24:23** · a flash directory created by cus and then there is a startup config file so if you see that actually the startup configuration or the the aristo configuration of this node and it also already has all the configuration that we provided through the cli such as bgp and link addresses for the interface addresses so now what we can do we can actually take those files and save them to to our

**24:50** · directory with one purpose we want to use these files as a startup configuration next time we deploy a lab so once we know how to save the configuration and it's and extract it from the lab directory what we can actually do is that we can modify our topology file and specify that we would like our nodes to start with some certain startup configuration so using the startup config parameter for the node we can specify the file that is present

### Persistent Configuration

**25:20** · on our container host that will be used by container lab to deploy a node with a certain startup configuration for the go bgp node which is a linux kind we cannot use that but we can use the binds instruction and we can say that the shell script that we have on our container host we would like to be mounted to the container namespace and then container will be able to use that script now what

**25:45** · does it give us it actually creates a topology file that has all the things already embedded in it and that's one of the very nice benefits of using startup configuration within container lab topology now anyone who will pull this lab will have those files as part of the repository will be able to deploy the lab with all the configuration already done and they will be able to demonstrate this use case or test it in ci without really doing any

**26:13** · configuration over the cli and that is exactly what i did here i created a repository on github which is named once 2021 so this repository as you see it contains all the files that we've previously extracted from the running nodes and it explains how to deploy the

### Infrastructure as Code

**26:32** · lab how to execute the use case and how to verify the operations now you can pull this repository and with just two commands you can perform the full use case that is really valuable and answers the infrastructure's code claim so far we have been working with containerized network operating systems in our previous lab but container lab supports not only containerized systems but also the traditional vm-based network oss currently we count 9 different vendors

### Supported Vendors

**27:00** · and 15 different network operating systems that container lab supports and the split between containerized and vm based network operating systems is currently at 60 40 ratio this slide shows which vendors and their corresponding kinds are supported by contain lab as you see here we make a distinction between the containerized network operating system and the traditional vm based ones to help you get started with your favorite network operating system you can go to container lab documentation site and choose the lab examples section

### Comparison and Conclusion

**27:33** · in this section you will find the lab examples for each operating system that we support so now if we put together the traditional software projects for lab emulation and container lab head to head we can see what are the differences between them so container lab focuses on

**27:49** · the using your labs as a code and working with them as as you would work with the code it allows you to create your labs and push them to git repositories and then unlock all the features of collaboration that git offers it also can enable repeatable labs because you now work with containerized images and your lab is actually an artifact that you can store somewhere it has a very small footprint and light and fast to deploy which also

**28:17** · play quite an important role in your ci cd pipelines at the same time it has fewer support for some network operating systems we do support the major ones but we are not trying to get every other network operating system to contain lab although we are open to external contributions and probably one of the most important distinctive features is that container lab is ui less we do not have any ui

**28:42** · because we think the way you should work with container lab labs is by using the textual files and basically work with it as you would work with the code okay so if container app sounds interesting and you feel like you can use it in your environment i think the first step for you would be to explore containerlab.com.dev documentation site because it has a lot to offer there are much more

### Q&A and Next Steps

**29:06** · things in container lab that i didn't cover here so you can try to create a lab with a network operating system of your choice and see how it plays out if there is any missing feature or you have a nice idea please do go to github issues or discussions and we can talk to you there if you want to have a more real-time communication we have a discord server for container lab specifically so do go

**29:29** · and join it and if container lab will prove to be useful to you you can always thank us by starting the repository on github with that i thank you for listening and see you next time