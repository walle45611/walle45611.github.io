---
title: "Michael Kerrisk :: Understanding Linux user namespaces"
source: "https://www.youtube.com/watch?v=XgThPoL9mPE"
author:
  - "[[CoreCppIL]]"
published: 2023-09-04
created: 2026-09-23
description: "Presented at the Core C++ 2023 conference.User namespaces are at the heart of many interesting Linux technologies that allow isolation and sandboxing of applications, for example running containers"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=XgThPoL9mPE)

Presented at the Core C++ 2023 conference.  
  
User namespaces are at the heart of many interesting Linux technologies that allow isolation and sandboxing of applications, for example running containers without root privileges and sandboxes for web browser plug-ins. In this presentation, we’ll look in detail at user namespaces, building up a basic understanding of what a user namespace is and going on to questions such as: what does being "superuser inside a user namespace" allow you to do (and what does it not allow); and what is the relationship between user namespaces and other namespace types (PID, UTS, network, etc.)? We’ll also employ some simple shell commands to create and experiment with user namespaces in order to better understand how they work.  
  
\-----  
Michael Kerrisk is a trainer, author, and programmer who has a passion for investigating and explaining software systems. He is the author of "The Linux Programming Interface", a widely acclaimed book on Linux (and UNIX) system programming. He has been actively involved in the Linux development community since 2000, operating mainly in the area of testing, design review, and documentation of kernel-user-space interfaces and was for many years the maintainer of the Linux "man-pages" project, which provides the primary documentation for Linux system calls and C library functions. Michael is a New Zealander, living in Munich, Germany.

## Transcript

### Intro

**0:00** · foreign spaces the reason they're interesting is the the the Cornerstone of building unprivileged containers but they're also

**0:16** · uh the uh the building block the primary building block for a number of other interesting pieces of technology and I'll say a little bit more about some of those pieces later on a little bit about me um I was many years as a maintainer of the Linux manual pages project this provides about a thousand pages that document system calls and C library

**0:40** · functions I wrote a lot of those pages I also wrote a book um I do training okay I can do without that slide so before looking specifically at username spaces I want to look at the concept of namespaces generally and it's hard to nail down some sort of concise definition of namespaces but this is my poor attempt a namespace

### What are namespaces

**1:11** · wraps some Global resource to provide isolation of that resource there's a number of namespace types currently there's eight and Counting there's a new one appears every now and then the most recent one appeared in 2020 I think it was

**1:30** · each one of those namespace types isolates some kind of resource these are a few examples so UTS namespaces isolate hostname domain name Mount name spaces isolate the mount list this means different groups of processes can see different mounted file systems

**1:51** · network name spaces isolate Network infrastructure each container for example can have its own network infrastructure for each namespace type there can be multiple instances on our Linux system when the system is first booted up there's one instance of each namespace type this is called the initial namespaced instance

**2:17** · each process resides in one instance of each particular namespace type so of course there's eight namespace types each process resides in one instance of each one of those eight namespace types

**2:32** · for the processes that are inside a particular namespace instance they are sharing a view of a particular Global resource if that Global resource gets modified the change is visible to all of those processes in that namespace instance but it's not visible to processes that are in other namespace instances and this is kind of abstract so let's make it a bit more concrete a simple example UTS namespaces what UTS namespaces do is isolate two

**3:05** · system identifiers hostname and the domain name domain name here means the NIS domain name the Yellow Pages domain name as it was once called why could it be useful to isolate these identifiers for example this means that each container can have its own hostname which you could broadcast on DHCP and

**3:28** · therefore get assigned a unique IP address by DHCP the name of this namespace it has ancient Origins it comes from in the name of a structure that was defined way back when called the UTS name structure UTS name Unix time sharing system

**3:51** · so on any particular Linux system there might be multiple UTS namespace instances processes in a particular instance see a particular hostname and domain name but that hostname domain name is private to that instance it's invisible to processes in other namespace instances

**4:17** · so as I said a picture makes this a little clearer we've got then this idea three different UTS namespaces here each namespace contains some member processors the black circles and within say this namespace instance here these processes are seeing a certain hostname if that hostname is changed there that change will be visible to the other processes in that UTS namespace instance

**4:45** · but it won't be visible to the processes in the other namespace instances we've got commands or backup setting there are system calls for buildings infrastructure we've got we've got commands that are layered on top of those system calls and I want to talk about those commands a little bit because I'm going to use them in some demonstrations

**5:13** · first of all in each proc PID directory okay prop good information about a process with a certain PID there is an NS subdirectory NS of course for namespace and inside that subdirectory there's a bunch of symbolic links these symbolic links have names that are suggestive there's one symbolic link for each type of namespace

**5:43** · if you look at the contents of these symbolic links they're rather unusual looking strings they're not path names rather they're strings of the form namespace type colon and then in square brackets a number now this number is actually an inode number it comes from a

**6:01** · an internal file system that is used to support the implementation of namespaces inside the kernel you can't see this file system in user space but internally it's mounted by the kernel and each namespace instance is represented as a file in this internal file system and what you're seeing here is the inode number for a particular namespace instance and the point is each one of these numbers is unique per namespace instance so one use of these files they

**6:33** · have many uses but one one use is if you see two processors where the inode number inside these files is the same you know those two processes are members of the same namespace instance

**6:52** · we've got some commands for for working with namespaces there's a command called unshare unshare lets you create new namespace you specify which name spaces you want to create with the options and then you can say that you want to run a command inside that new namespace or inside those new name spaces the command if you leave it out defaults to being a shell there's another command NS enter this lets us step into an existing namespace

**7:24** · and execute a shell command again we give options to say which name space do we want to step into and we can give a command or if we leave out the command the default command is a shell each one of these commands takes options that say what kind of name spaces do we want to work with you'll see this consistency across the two commands the option there is for these for each name spaces they're the same option letters

**7:52** · so let's try a demonstration um creating a UTS namespace so two shells here they're both in the initial UTS namespace and if I look at my hostname here I see a certain hostname bien if I run the same command down below of course I

**8:23** · see the the same hostname because these two shells are in the same namespace instance now another command that I want to use a little bit and I'm going to copy this command is

**8:40** · this command let's look at the symbolic link for this UTS namespace I'll just copy that command I see a certain string there if I run the same command down below I see the same string this is telling us these two processes these two shell processors are in the same UTS namespace now if up above I then create a new UTS namespace

**9:17** · I say unshared Su credit new UTS namespace pseudo because to create most kinds of namespaces you need to have a privilege uh the Privileges as a so-called capability capsis admin and I'll just say I'm going to have a shell layer

**9:52** · okay now if I type hostname here I still see the same hostname and the reason is when a new UTS namespace is created it inherits a copy of the hostname but what I could do now is say hostname

**10:12** · langvid and okay the host name has been changed but down below I've still got a shell here in the initial UTS namespace and if I type hostname I still see BN

**10:32** · if I use that read link command in the top window and in the bottom window whoops I see two different numbers these this is telling us these two shells are in different UTS name spaces but we already saw that because we saw different host names for the two shells

**10:58** · now I want to get the PID of that shell up above the shell that's in the new new UTS namespace now down below I can say sudo NES enter Dash \[Music\] t5137-u step into the same UTS namespace as the process with this PID which of course is the shell on the Apple window

**11:41** · now if I type hostname I see long vid because I've stepped into that other UTS namespace and if I run that read link command I see the same string as up above this new shell is in that new UTS namespace

**12:05** · okay all righty so I'm getting closer to talking about username spaces but I still need a little bit of background and this is the notion of capabilities

### Capabilities

**12:21** · the traditional Unix privilege model divides users into two categories normal users who are subject to a lot of rules and restrictions and super user who gets to bypass a lot of rules and restrictions the traditional way of giving a process

**12:48** · privileges is to create a set user ID root program to create a set user ID root program you take a binary you change its ownership to root and then you turn on the set uid but the way you do that you say chmod U plus s now when you do that if a process executes this program the processor's user ID changes to being the same as the user ID of the binary in other words uid 0. as a consequence the

**13:21** · process gets the full power of super users super user and therefore the program can do the things that super user can do this is obviously a powerful technique it's also dangerous because if the program gets compromised then the attacker has access to the full power of Superuser

**13:45** · so this is this this this this coarse granularity of the privilege model either you have no special powers or you have all the powers of Superuser and what that means is if you have a process that has all the powers of Superuser and it gets compromised then there's reading no limit on the damage that the attacker can do on the system capabilities are an attempt to mitigate

**14:14** · that situation and the idea is let's break the power of Superuser into smaller pieces as things are currently there are 41 different smaller pieces that documented in a certain manual page called capabilities just a flavor of what we have here of

**14:35** · here's three examples out of those 41 there's a capably called CAP DAC override this says you can bypass file permissions checks um capsis time says you can make arbitrary changes to the system clock

**14:51** · cap says admin unfortunately lets you do way too many things but that's a story for another day the idea then is instead of having set uid route programs which are very powerful instead you can create binaries that have one or perhaps maybe two if necessary capabilities attached that let the program do some particular privileged operation

**15:23** · the point here is that the binary is a weaker binary if it gets compromised the attacker has less power and hopefully the attacker can do less damage because the thing that is compromised is weaker

**15:44** · so just to summarize then we've got this idea that processors can have capabilities some subset of the power of root programs can have capabilities attached and when a process executes that program the process gets those capabilities and the whole point behind this idea is having processes with some

**16:06** · some subset of capabilities or binaries that have some cables attached these things are less dangerous than traditional super user processes and set uid root binaries

**16:21** · okay username spaces at last what's going on here the idea here is that we can have per namespace mappings of user IDs and group IDs in other words

**16:39** · inside a username space a process might have certain credentials certain user IDs certain group ID but outside the namespace it has different credentials and the really interesting example or the interesting use case here is a process that has uid 0 inside the username space but has an unprivileged user ID outside the namespace this process has super user powers

**17:10** · but only in that username space now we're going to see what that means but the point is this is this process does have some kind of special powers we need to know some things user namespaces have a hierarchical relationship each username space has a parent username space which has a parent username space and so on going all the way back to the initial username space which obviously doesn't have a parent

**17:40** · the way that that parental relationship gets established is when the username space is created the the parent of a news namespace is the username space of the process that created this news namespace

**17:58** · the reason that this parental relationship is important is it determines how capabilities work inside namespaces I probably won't have time to talk about that but I have got some n slides that go into a little bit of detail there

**18:18** · when a new username space is created the first process in that username space gets all capabilities all 41 capabilities in other words it gets super user Powers the kernel just does this the kernel simply says first process in a username space shall have all capabilities in that username space

**18:43** · but only inside that username space so I mentioned this idea that a process has certain credentials inside the namespace but has different credentials outside the namespace the way that that is set up is by creating uid and GID mappings

### Mappings

**19:11** · okay thank you alrighty um the way these mappings are set up is by writing strings to a couple of files called proc Purdy uid map and proc PID gidmap now there's a lot of rules about

**19:32** · how these files need to be updated rules because of security how these files are updated when they can be updated who can do the updates way too many rules for me to talk about here but you can read about them in a certain manual page the lines inside these files look like this lines that say an ID inside the namespace maps to an ID outside the namespace and then a length so that you

**20:04** · can say that a certain range of IDs maps to arrange outside the namespace and so common example that you see is a mapping like this that says uid 0

**20:20** · outside the names sorry inside the namespace maps to some unprivaged ID outside the namespace in this case I've said the length of the mapping is one only one uid is being mapped I could have for example had the number 10 there and that would have been saying that uids 0 through to nine inside the namespace mapped to 1000 through to 1009 outside the namespace but here I've just mapped one user ID

**20:54** · okay so let's just try another example okay so first of all I'm doing all of this as an unprivileged user uid 1000 gid1000.

### Example

**21:24** · and what I'm going to do is create a new username space Dash capital u Dash R says I want to create a namespace with root mappings root mappings means my user ID or my group ID mapped to zero

**21:46** · inside the username space and then I'll run a bash shell and shell prompt has changed in an interesting way the reason for that is if I now say ID this process has uid 0 gid0 inside the username space if I look at the proc PID uid map file

**22:15** · here there was a mapping credit that says zero maps to one thousand for length one okay zero inside the netspace maps to one thousand uid outside the namespace for length one there's a similar looking GID map as well

**22:36** · those mappings were set up by that Dash R option now if I look for

**22:55** · cap that'll do cap in slash proc slash dollar dollar status I'm looking in the prop PID status file because I want to see the capabilities of this process and there's a couple of fields in that file that show me this processor's capabilities and particular lines that I'm looking at here these lines here the these are hexadecimal bit masks those Exodus masks

**23:27** · represent 41 bits for the 41 different cables and what this is telling me is this process has all so-called permitted and effective capabilities so it has all the powers of Superuser of course none of us is very easy and very good at reading hexadecimal bit masks there is a shell command that we can use instead called get pcaps and you

**23:52** · give it a PID as an argument and it gives you a human readable representation of the capabilities it's a little hard to interpret still you need to know the rules what of what this notation means but the equals EP here is saying this process has all effective and permitted capabilities now this shell here is a certain PR I can already see the PID there of course but what if outside

**24:23** · the shell down below is in the initial username space how does that process look in the initial username space inside it's inside these namespace it's got uid 0 gid0 but let's look from outside and the uid the GID all right might as well the PID of that process five three five six outside the process looks like it has uid 1000 gid1000.

**24:58** · this is consistent with the fact that outside the namespace this is an unprivileged process okay now one more thing I've got super user Powers inside that username space maybe I can do some super usury super usury things maybe I can change the hostname let's try that

**25:25** · first of all I see a certain hostname there bien let's try and change that hostname I can't do it okay we need to know a little bit more

**25:45** · okay so user namespaces and capabilities I mentioned already the kernel gives the first process in a new username space a full set of capabilities but those capabilities can only be exercised on

### User namespaces capabilities

**26:03** · objects that are governed by the username Space by that new username space of course what does that mean well we've learned a few things already there are a number of different namespace types each of those namespace types governs some type of resource UTS namespaces govern hostname and domain name Mountain Land spaces govern Mount points network name spaces govern Network infrastructure and so on

### Namespace types

**26:36** · a new piece of information non-user name space is owned by some particular username space there's an ownership relationship between user namespaces and non-user name spaces the way that name that ownership relationship is established is when a new non-user name space is created the username space that owns that new namespace is the username space of the process that created the new non-user

**27:09** · name space process tries to do operations on some kind of global resource what the kernel checks is does the process have the necessary permissions the necessary privileges in the username

**27:32** · space that owns the non-user name space that governs that resource again a picture is going to help but first of all we need a command the picture I'm about to show you is what we would get if we did this command what this command is saying is create a new username space with root mappings and at the same time create a new UTS namespace and run some

### Ownership relationship

**28:02** · program that program of course is the one being run inside this process now this process well we're back up second to begin with on the system there was the initial username space and there was an initial UTS namespace

**28:25** · and an initial Network namespace initial Mountain space initial PID namespace and so on the effect of the command was to create a new username space this newsnatespace is a child of the initial username space and because a new UTS namespace was created at the same time okay we have a new UTS namespace there that new UTS namespace is owned by the new username space

**29:03** · okay now this process here it was set up with the root mapping its user ID inside the namespace is zero but outside the namespace its user ID is some unprovised ID let's say one thousand and because of the magic that the kernel does the kernel said this process has all permitted and effective capabilities it has the powers of Superuser

**29:33** · on the other and this process of course is a member of the new UTS Nan space and it's a member of the new username space but of course it's a member of each one of the other kinds of namespace as well which network namespaces is it a member of well when we ran that on Shake command we didn't say create a new network namespace therefore this process

**29:56** · is still in the initial Network namespace it's in the initial Mount layer space it's initial um PID namespace and so on now suppose the process tries to change

**30:15** · the hostname the necessary capability to do that is the one called capsus admin in that case what the kernel asks is we're changing the hostname which UTS namespace is this process of emerald and the answer is it's a member of the studio space and then the kernel says which user namespace owns that UTS namespace and the answer

**30:44** · is this one and then the kernel says well does the process have caps as admin in that namespace in that username space the answer is yes and so it is allowed to change the hostname by contrast suppose this process tried to do a privileged Network operation perhaps bring a network device up or down

**31:11** · because this is a network operation the kernel says well which network namespace is this process member of and the answer is it's a member of the initial Network namespace and then the kernel says well which username space owns that Network namespace and the answer is the initial username space and then the kernel says well what capabilities in particular the necessary capability is catnet cat net admin what

**31:40** · capabilities does that process have in that username space and the answer is this process has all capabilities but only in its own username space it doesn't have those capabilities in the initial username space and therefore it can't change the uh you can't bring the network device up or down

**32:08** · if we generalize this to sort of the container situation things are like this where for our container there's a new username space and that username space owns an instance of each one of the other kinds of namespace I haven't shown all the namespace zero don't have space on the slide but the news namespace that for the container it owns a pi DNA space it owns a UTS namespace it owns a mount name space it owns a network name space and so on and there are processes inside

**32:39** · that container including for example a an internet process with pid1 okay now there are apis to discover

### Apis

**32:59** · what is set up on your system you can discover the parental relationship between user name spaces you can discover the ownership relationship between non-user name spaces and username spaces these apis they're documented in this manual page here I'm not going to try and talk about these apis but I've got a program that uses those apis to give us a certain

**33:22** · kind of visualization of our namespace setup and I'll demonstrate that program in a moment and my program's fairly simple if you want to see a much better program someone not far down the road from me literally speaking um did it better and you can find that produce on GitHub gives you a better visualization okay so let's try an example

**33:55** · let's start with fresh shells so what I'm going to do here again is say create a a username space with root mappings and a new UTS namespace at the same time

**34:19** · and I should have said something by the way that I didn't say earlier on notice this time I didn't use sudo and the reason is to create username spaces doesn't require any capabilities and a process a new process and a new used a

**34:41** · new username space gets a full set of capabilities which means in effect you can create in other kinds of namespaces at the same time and therefore I didn't need to use sudo here now I've got a process that's in a new username space and a new UTS namespace now let's look at the hostname okay it's BN but let's try changing the hostname

**35:09** · that was successful the reason is this process is in a new UTS namespace that is owned by the username space where the process resides and this process has all capabilities

**35:24** · you can see it there get this process has all capabilities in in its username space that's what equal equals EP is telling us okay now on the other hand just to remind us of the syntax here um let's do a privileged networking operation where I say I P link set Dev Allo

**35:56** · up question let's try it down little the loopback device is already up okay I have all all powers of Superuser here inside this new username space and therefore can I take the loopback device down

**36:16** · I can't do this because this process is a member of the initial Network namespace and the initial network namespace is owned by the initial username space and this shell doesn't have any cake Blues in the initial username space

**36:36** · okay now just my window there are a little just to give you an idea of what's going on virtually I'll use that program of mine and what I'm going to do is just go where I have that program

**37:02** · oops say if I just run this program like this it'll show me all the namespaces on the system and all the processes that are members of all of those namespaces this is too much information instead I'm going to say let's restrict what's being displayed here to

**37:24** · the process that is in the top window which is in some new namespaces and compare it to another process the process is the initial namespace perhaps the shell down below so the PID of the process up the top that's five four nine one the PID of the shell down below that's dollar dollar this will show me all the namespaces of those two processes but even that's too much information because there's a lot of namespaces there's eight different types so I'm going to just restrict the output a

**37:55** · little bit more and say just show me some of the names some of the namespaces let's say the UTS namespace the network namespace and then perhaps just the mounting space as well

**38:14** · I need to do this with sudo because it relies on me reading the contents of proc PID NS simlink files that belong potentially to other processes and in order to do that I need to have privilege because what this program does is scan all the prop good NS files on

**38:34** · the system including ones that belong to other users all righty so it's a crude visualization but some things indentation is Meaningful indentation either indicates parental relationship so this username space here is a child of the initial username space or

**38:59** · indentation represents ownership and this is telling us for instance the initial Mount namespace here is owned by the initial username space the initial network name space is owned by the initial username space the initial UTS namespace is owned by the initial username space and of course there's one other namespace here this is the new UTS

**39:20** · namespace and it's owned by the new username space that's what the indentation is telling us and we see some pids here's the PID of my shell of above it's a member of the new UTS namespace it's a member of the new username space and it's a member of the initial Network namespace and the member of the initial Mount name space and so on so you can

**39:47** · get these kinds of visualizations to discover the shape of the namespaces on your system so you understand what's going on okay alrighty I'm nearly done

### Use cases

**40:04** · Okay so what can we do with the stuff why is it interesting well the sort of the the the motivating force for a lot of this work was unprivileged containers the idea that without being super user we can fire up a container and you know do the container thing but without being super user and Docker and Alexi and so on they're

**40:29** · using this this idea of username spaces to have unprivileged containers but there are plenty of other use cases as well browsers are using this infrastructure to do uh sandboxing of the renderer process historically the way that this was done was with set user ID root helper programs but set user ID root helper programs are dangerous things because maybe they could get compromised locally by using username spaces instead we have

**41:04** · um we don't need set user ID root programs so things are more secure because we don't have set user ID root programs as a point of attack this is an interesting use case you can have a username space with a uid map that looks like this unproofed ID maps to the same ID outside the namespace for length one inside this username space there is no uid 0.

**41:32** · that's an interesting kind of guarantee you know the process inside this namespace can't get super user Powers by switching to uid 0 because uid 0 doesn't exist in this namespace

**41:51** · um perhaps some of you are aware of tools like for example fire jail fire jail is a sort of generalized sandboxing application it's built using name spaces as well as other pieces of infrastructure like control groups and setcom and you can do generalized sandboxing of applications and fire jail one of the nice things about fire gel is when you install it it comes with a bunch of pre-created profiles for many

**42:19** · common applications so straight out of the box you can use fire gel to sandbox your favorite application to hopefully improve security a little bit that's your goal um flat pack or or snap for example they're also using this kind of infrastructure flat pack and snap these are of course tools for packaging applications where an application is packaged with all of its dependencies and this means that the end user can

**42:52** · just deploy the application without going through any special installation steps on their local system all they need to do is install flat pack and download the pre-created package then it just runs out of the box without needing to install any additional dependencies and there are plenty of other applications as well

**43:15** · um if you're looking for more information I wrote a series of articles about name spaces a few few years ago on lwn there's some manual pages that I wrote um something that someone else wrote which I think is rather interesting to read Linux containers in 500 lines of

**43:30** · code it's this page is an incredibly detailed annotation of what steps do we need to do to isolate a process in the fashion of a container how do we get a process isolated in the way that Docker isolates a process for example and the interesting point here is it doesn't take that much code tools like Docker are big not because of the code that is needed to do the isolation but all the infrastructure that goes along all the orchestration

**44:01** · that goes along with docker so this is a very interesting page to read and I am actually done and if there are any questions please I don't think it does so the question related to in NSS did you say yeah to

### Questions

**44:20** · related to NSS models I don't think it does I think this is an orthogonal piece but also I'm not very knowledgeable in this area either of NSS modules but I'm reasonably sure that this is just an orthogonal concept

**44:36** · so the question is do you have to create multiple processes in order to get multiple name spaces and the answer is no you can actually the underlying system calls allow you to create multiple name spaces at the same time there's a flag so I can use with the system calls that says what kind of namespaces do you want to create and you can specify multiple Flags to create multiple namespaces now the rule is that

**45:00** · for most kind of name spaces a capability is required capsus admin username spaces are exceptional no capability is required this means an unpreviewed user can create a username space but the point is that the process that is created as a result of that step

**45:16** · is itself privileged and what this means is and it has capsus admin for example and what this means is you can combine all of the flags with cap with um with the flag that creates a new username space and you create a username space and other namespaces at the same time while being an unprivileged user because the username space step gives the resulting process all capabilities

**45:44** · you can do it in one step so I mean I I could have for instance if I go back here and here I said create a new username space and a new UTS namespace I could have said create a new network namespace as well and now there oh there's the commands in

### Examples

**46:04** · my history um now I could in that new network namespace now I'm able to take the net the loopback device down and that's because that loopback device belongs to the network namespace that is owned by this new username space okay so there are three new name spaces there in play the new username space the new UTS namespace new network namespace

**46:26** · there was another question back here I'll I'll say that SC Linux policies can deny privileged operations for sure but SC Linux policies as far as I know are not namespaced what that means is you can't have different SE Linux policies for different username spaces as far as I know does can who see the device can I think what you're asking is

**46:57** · yeah so what I think you know what you perhaps are asking is you know can a process down here which is in the initial Network namespace see that Network device in the new network namespace no what we could do is step into that network name space and then see the device but from the outside that device is invisible there was a question here in the middle

**47:24** · so the question related to file permissions and how that works in this model I think the simple way of thinking about that is suppose you know we've got a process up here that has you know uid has uid 0 does that mean that I can do

**47:40** · things like creating a file in the root directory because I have uid 0. well when it comes to doing file change what happens is the processes credentials are mapped back to what they would be equivalent to in the initial username space and initial using ancest this process has uid 1000 so it you know it couldn't do those sorts of things in in say the root directory does that answer your question yeah there was a question at the back I think yes please you'll have to yell at me

**48:11** · um the way it works the underlying system call is called set NS oh sorry you have set an S um process can only move itself into another namespace one process can't move another process into a namespace but a process can move itself into a different namespace assuming it has the necessary privileges yes I I could have I chose here to set up a uid map that said your unprovised

**48:40** · ID okay if my time is up I'm purchase ID maps to zero inside the namespace but I could have mapped something else and I could have had a process that is simply unprivileged inside the username space correct yes correct even if it was privileged in the original namespace it would be in a new name space where it potentially didn't have privileges didn't have any capabilities I'll take one more question

**49:11** · a process can move itself to a new namespace oh no a process can move itself to another existing namespace yes so the question is suppose we have a namespace set up like this where there's a a uid map that Maps unprovised ID down

**49:28** · to zero inside this username space suppose a from down here we created the process that stepped into that username space how do we determine what credentials that process is going to have when it steps into the namespace that's determined by the mapping that was already created and there is already a mapping that says you know can't slash

**49:52** · process dollar dollar slash uid map there is that map now here down below I've got a process that has uid 1000 and I need to know the PID of that shell of above if I now sit down here sudo in fact I don't need to do that in our Center say I want to step into the same user namespace and the same let's say UTS namespace and the same network namespace as the target PID 6404

**50:28** · don't think I need to use sudo there oh no this uh okay yeah now I will take studio for reasons I don't want to try and explain right now

**50:49** · okay that's don't ignore that's just scenario from my bash startup script um now if I now look at the idea your credentials that process it has uid zero GI zero and the reason is because it had one thousand outside namespace and the mapping said inside the native space it would have uid 0. perhaps an interesting

**51:12** · counter example though oh I can't do that from here actually yeah no that'll be enough that'll be enough thank you for your time \[Applause\]