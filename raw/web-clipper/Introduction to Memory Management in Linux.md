---
title: "Introduction to Memory Management in Linux"
source: "https://www.youtube.com/watch?v=7aONIVSXiJ8"
author:
  - "[[The Linux Foundation]]"
published: 2017-04-05
created: 2026-09-29
description: "Introduction to Memory Management in Linux - Matt Porter, KonsulkoAll modern non-microcontroller CPUs contain a memory management unit and utilize the concept of virtual memory. This presentation wi"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=7aONIVSXiJ8)

Introduction to Memory Management in Linux - Matt Porter, Konsulko  
  
All modern non-microcontroller CPUs contain a memory management unit and utilize the concept of virtual memory. This presentation will describe the different types of virtual memory spaces and mappings used in the Linux kernel, the cases in which they are useful, how they are implemented in the kernel, and how they differ from user space memory. Concepts such as the hardware memory-management unit (MMU) and translation lookaside buffer (TLB) will be discussed, as well as software concepts like kernel page tables. User space concepts such as growable stacks, memory paging, memory mapping, page faults, exceptions, and other memory-related conditions will be covered as well.  
  
About Matt Porter  
  
Matt Porter is the CTO of Konsulko Group. At Konsulko, he works on design and development of software for the Linux kernel and other FOSS projects. Matt has contributed to a number of Linux related projects over his years of community involvement including the various part of the kernel, Debian, RapidIO, Beagleboard.org, and many others. Matt is currently working on GPGPU and eBPF hacks for Linux. Matt has spoken at previous Embedded Linux Conferences on the topics of userspace drivers, Android, Linux 6502 remote processors, kernel testing, and USB gadget configfs, and IoT frameworks.

## Transcript

### Intro

**0:00** · Welcome.

**0:01** · We got a lot of material, so we're going to blaze through this. Hold on. It's going to be quick.

**0:07** · So, my name is Matt Porter. I'm with Consalco Group. And if you're looking for Alan, I'll explain on the next slide why I'm not he and you can probably tell cuz if you know Alan, he's got hair down to here. So, This is a introduction to memory management and I stress introduction.

**0:29** · So, if you're experienced kernel person, you might not get as much out of it.

**0:34** · This is the This is the presentation that back when when the early adopters of embedded Linux back in 2000, 2001 were coming in from working on our tosses, I wish I had sat down and put something this this good together cuz these are the things that everybody needed to understand to really grasp their system.

**0:57** · All right.

### About the original author Alan O

**0:58** · So, just real quick about the original author, Alan Ott. He couldn't be here, unfortunately. Good friend of mine and he's a veteran embedded Linux developer.

**1:10** · He's a Linux architect at SoftIron.

**1:13** · You may have heard they're an ARM server company. But he put together all this material. He is a fellow instructor, LF training instructor. Also gives the the kernel internals class that contains a lot more than this material.

**1:29** · So, he did a really nice job on the slide deck and trusted me to present it as well. So, anyway, just want to give him the kudos for this awesome material. So, here we go.

**1:43** · Um We're going to talk about memory management from beginning to the end and it's going to be the intro as I said. So, we start at physical memory.

### Single Address Space

**1:57** · If you look at your your um low-end systems have a single address space and memory and peripherals are sharing that same space. Um, they're mapped into different parts of that single address space and um the all the processes and OS uh in in this type of system uh share the same memory space. There's no memory protection um like we you would often hear about.

**2:23** · We're going to get in all the details of this. Um, and you're running a process in that single address space, um processes can stomp on each other cuz they're all shared in there. You have to separate them manually.

**2:35** · Um, and quote user space or your user application can stomp on say your real-time executive that you're using to schedule.

**2:43** · Um So, examples of these would be um an 8080s 86, uh Cortex-M part, AVRs, all these low-end microcontrollers in the old um pre-MMU uh uh processors.

**3:00** · Um, so let's take a look. Um, so it it gives us I know a lot of us uh are not working on on x86, but it serves as a ubiquitous example. Uh if we look at a 32-bit uh x86 system, all right? Um, lots of legacy obviously, but it is a common ground. Um, we have um all these uh legacy areas and so forth.

**3:24** · Um You have hardware mapped between RAM areas.

**3:28** · Um, you can see that uh your PCI uh physical uh PCI area memory mapped IO is all in the high part, okay?

**3:38** · Um, so that gives you an idea physically uh what x86 looks like.

### Limitations

**3:43** · Now, um what's the limitation with um the single address space, right? Um you have portable C programs expect they kind of own the whole thing, right? They don't They don't know uh if you're you're trying to port several C programs into one space.

**4:02** · Uh you've got to go set the addresses.

**4:04** · Uh this this can live here and this can live in this segment so they don't stomp each other. So, it's kind of hard to do that. Um you've got to have special knowledge of your actual platform. Um need to know what your total RAM is.

**4:17** · Uh and uh you need as I'm saying, you need to separate those processes. You have to do all this work. Um and there's no protection, right? As we said, rogue programs can stomp all over things.

**4:28** · So, in comes virtual memory.

**4:31** · And this is where things get fun.

### What is Virtual Memory

**4:34** · Um so, what is it, right? Um this it's a mapping. It's a virtual mapping, hence the name virtual. So, um so, you map a virtual address, a fake address, um to that physical address, right? When we look back at that X86 map, that's all physical world. And if we can just think in virtual addresses, we can have any mapping we want. So, we map virtual addresses to physical RAM, but we also map virtual addresses to hardware devices, right?

**5:04** · So, PCI, GPU RAM, on SOC IP blocks, right?

**5:10** · Everything.

**5:13** · So, what's the advantage, right?

**5:15** · Described how in that flat memory model, the single address space, we have a situation where, you know, I got to tell something to run at this address and this address and this address up to n times and um actually have a nice memory map of where everything lives. It's not portable, right?

**5:33** · So, um when we have virtual memory, right? You have one process's RAM is inaccessible to the other uh processor. It's also invisible, right? So, you have built-in memory protection. And kernel RAM is not visible to user space directly. Um the nice thing you have is that memory can be moved, right? So, uh memory can be uh visible to different processes, but you have to actually um uh set up a mapping for that.

**6:00** · And the other nice thing is rather than in a single address space uh where you have uh all the memory sitting there uh and you have to manually share it and segment it, right? You can now do things like swapping memory out to disk cuz the addresses you're dealing with are just virtual.

**6:20** · Um the other thing um you can do with virtual memory is map that hardware, right? That we talked about can be mapped into your process address space, okay? We need help from a kernel to do that on behalf of of user space, right?

**6:36** · The other thing is we can take RAM memory and we can map it into multiple processes, right? And we're going to get into that more and that's the the shared um that would be a case where like a shared library, right? Where you're mapping it into multiple processes. And finally, with virtual memory, we get the ability to have read-write-execute permissions placed uh on those address accesses.

### Virtual Memory Details

**7:00** · All right, so we have two address spaces now, right? We've got the physical addresses we talked about and we saw that physical memory map X86 we used as example, and that's, you know, DMA, peripherals, whatever it maps out to in your physical world, right? Virtual addresses, right? And those are the ones that our actual software uses, right?

**7:21** · When we get to our machine code, whatever whatever architecture, that's our load-store accesses, right? Uh out to memory, and those are always using virtual addresses.

**7:32** · All right, so um looking at virtual memory, right? We have to do a mapping. This mapping is done in hardware. So, there's a piece of hardware that assists with these mappings, okay? Once we have it mapped, there's no penalty for accessing memory that way, all right?

**7:50** · The permissions are handled without a penalty.

**7:53** · So, this is all handled in hardware for us, and we're going to talk about what that hardware is that does this.

**7:59** · Um and of course we use the same CPU instructions, the same load stores, whether it's RAM or a piece of peripheral IO. Okay? Um so, in normal operation, you're always using virtual addresses.

### Memory Management Unit

**8:18** · All right. Um so, what magic does this?

**8:20** · It's the memory management unit, all right? And uh so, an MMU sits between the CPU core and the memory, all right?

**8:28** · It's often in a modern architecture, part of the physical CPU itself. If you look in like retro things, you'll find that MMUs were used to be a separate discrete part, right? And were interfaced and were part of that set um uh sold just like say a PMIC is often a an integral separate discrete piece of a of a architecture.

**8:54** · Uh so, um the one thing to keep in mind is that the RAM controller is a separate piece.

**9:02** · So, you got an MMU, the DDR controller is going to be a separate IP block, tightly coupled though.

**9:09** · Uh and what does an MMU do, right? Um what it does is it just does that magic of transparently handling the translation of those load store instructions into physical addresses, okay? So, we map the memory accesses, the virtual addresses to our system RAM, that physical address space we talked about, right? Same thing with peripheral hardware, no different from its point of view, right?

**9:35** · It handles permissions, and as I said, we got permissions with virtual memory, and if we have an invalid access to something, it's going to generate an exception, and with that exception, we can go do some interesting things.

### Translation Lookaside Buffer

**9:52** · And we'll talk about that in a bit.

**9:54** · Okay.

**9:55** · How an MMU works. There's an important piece of the MMU called the TLB, translation lookaside buffer, okay? And so, that's just a uh hardware buffer um that has a set of mappings, and those are your virtual to physical mappings.

**10:11** · It'll have permissions for that space, okay? And um there's a there's a granularity in which these mappings are kept, and we're going to talk about that uh in a moment. And um the the interesting thing is that, you know, TLB design is uh very architecture-specific, very part-specific, performance-sensitive, and so you'll see a wide variance in how TLBs are designed, um how um

**10:41** · uh mappings are placed in, if it's software-done or it's hardware-assisted, um that type of thing, and also uh capabilities, how many uh slots they have.

**10:52** · All right. So, this is a quick little diagram of what a system looks like if you're having trouble visualizing it, where it sits. You see the MMU between the memory controller, right? And the CPU. You see that TLB on the side with some entries, okay?

**11:10** · All right. So, as I was saying, with that TLB, the MMU takes a look at that buffer, right? Is there already a mapping in there when it accesses a virtual address?

**11:20** · And then it can look that up, and if it doesn't uh doesn't find one, then it's going to generate the page fault, interrupt the CPU, okay? Now, if the address is in the TLB, but you let's say you're doing a write access, but it's only set for read permission, it's also going to generate an exception.

**11:43** · And that'll come back into play as we get into how we use those things in Linux. So, in Linux, a page fault, right? So, you have a CPU exception generated, okay? And um this happens when you access that invalid virtual address. What makes it invalid, right? It's not in the TLB, okay?

**12:09** · And you have three cases. So, first virtual address just isn't mapped, okay? For the process that's requesting it.

**12:18** · Second, you don't have the right permissions, right? And third would be that it's a valid virtual address, but it's currently swapped out. And that one's a software condition. So, let's dive into we're going to dive into each of those, but first we're going to get into kernel virtual memory side of things, okay?

### Kemel Virtual Memory

**12:39** · Um So, we use virtual addresses both in the kernel and user space.

**12:46** · But the way that we use them, how things are mapped are quite a bit different.

**12:50** · So, in the kernel, we use them obviously, and but we have this split in how we treat our virtual addresses. Um and the upper part of our virtual memory map is for the kernel and the lower part for user space. And usually when we teach people about this, it's harder to think with 64-bit addresses, so we go back to 32-bit.

**13:15** · And we affectionately call the default spot that it's split between user space and kernel space is C Brazilian, that's at that 3-gig location. That is a default.

**13:30** · Um so, this is what it looks like. So, you saw that hugely complex physical memory map of uh x86 32-bit architecture, and lo and behold, here's the virtual memory map.

**13:43** · We've got 3 gig for user space, right?

**13:46** · Config page offset controls where that split is set at, right? And so, every process gets its own 3 GB in that system. It has that whole view. So, you remember to go back to that single address space. If you had multiple processes, you had to go link them in all these different spots and manage your processes very manually. In this world, when we link applications, they all end up at the same place, right? And the kernel just has this 1 gig in our 32-bit case.

**14:18** · Okay. So, as I said, um that config page offset controls that. A lot of architectures, um if you have specific needs, you might uh fiddle with that a bit. That sometimes happens in in embedded stuff.

**14:32** · Um and um the uh um on 64-bit, we don't have this situation where there's ever a possible need to do that essentially. Um on ARM 64, we're at um 8 bazillion there. Um x86 64, the split's at a at a different location.

**14:50** · Um but uh you know, given RAM sizes and so forth, uh it's effectively something that's that's not worried about uh in a 32-bit system um where that page offset is is uh has an effect on how we deal with large memory systems, which we'll talk about in a moment.

**15:11** · Um so, uh there's three kinds of virtual addresses uh in Linux. And uh LDD um uh defines these these best. You can you can look at that and the way the way we define them are and and some people use some different terminology historically, but in the kernel side we have kernel logical addresses, kernel kernel virtual addresses, and then we have those user space virtual addresses. Okay.

**15:41** · There's another special case, but most people don't speak about them exactly that way, either physical or bus addresses, but you can look at LDD3, the link was in there for a little bit more information. So, kernel logical addresses, um that's the what people consider the normal address space that they're normally dealing with. What you get back from kmalloc is a kernel logical address. Okay. They have a fixed offset. Okay.

**16:11** · And so, um you see a magic number there. That's the config page offset value, and that would map to that. Now, that that physical address is specific to one architecture. That could be wherever your base of RAM is. And it does get more complicated in in various other segmented memory systems, but this is an introduction, so we keep it simple.

**16:38** · Um so, because this is a very simple mapping, logical mapping, the conversion is really easy to do. So, visually it looks like this.

**16:48** · Your kernel logical addresses are at page offset, point down assuming your physical RAM and that physical memory map is starts at zero. Boom, you've got this very simple logical thing. And accordingly, you have a very simple set of macros that can convert when it when it's a kernel linear or logical address.

**17:10** · Okay. Now, the next thing that that's interesting when when we have a small memory system, right? So, we'll we'll we'll call them small or large and this is really specific to our 32-bit example, okay?

**17:27** · So, less than a gig of RAM, it's technically less than that really when when you look at where the split's at.

**17:34** · You have those kernel logical addresses, right? Starting at the page offset and then going through the end of memory.

**17:41** · So, if you had 512 MB, okay? Then you would have C up through D all boxes, right? Would be that kernel logical area. So, it's very simple when you have that small amount of memory. That's where it gets mapped into kernel logical space.

**18:00** · Okay?

**18:01** · Um So, things that it it includes in logical space, like we already said, allocations with Kmalloc, get free pages, all of the all of those allocators, and kernel stacks. Um And the key thing here is and we haven't talked about how swapping works yet, but logical memory can never be swapped out, okay?

**18:25** · Um There is as we said, there's that fixed mapping.

**18:30** · We saw how simple the mac the macros were for that, okay? But what's because of that all everything in that kernel logical area, it's all physically contiguous. So, that's important because we need that for DMA. So, that's that's why you'll see Kmalloc and those those types of allocations used with DMAable buffers, okay?

**18:54** · Okay, then it gets complicated if you're on a large memory system, something more than a gig of RAM nominally, right? We run out of space, right? Our page offset was at C, but so how are we going to map that all into kernel? We can't, okay? Um so, there is we we run out of room there, and then on top of it, we have to have the space for use by vmalloc memory, which is our kernel virtual address range, and we need to keep that.

**19:25** · So, we're going to talk about that in a moment, okay?

**19:31** · Once we go above that gig of RAM, nominally it's actually less, then we have stuff mapped into the kernel virtual memory area, and so that's the the high mem support.

**19:45** · Again, note that when we're taking the 32-bit model, we have that problem. With 64-bit, we don't really effectively have that problem until we're going to get ginormous amounts of RAM on the system.

**19:56** · Doesn't seem like it's going to happen tomorrow in that space, okay?

**20:01** · All right. So, kernel logical mapping, right? We saw page offset, and then in this large mem system, right? We don't have enough room for that. So, the additional RAM is going to be mapped separately in that kernel virtual area.

**20:23** · So, you don't have that logical mapping.

**20:25** · So, those things you can't you can't use any of those simple macros on them at all.

**20:30** · Okay? So, how do we call that? Low memory is that directly mapped set, right? And then the high memory there is um is you know, not physically contiguous.

**20:44** · It gets mapped in on demand. We only have that situation on 32-bit.

**20:50** · And um the the key thing on the low memory as you saw, and you can see it visually, you go back to the these these sections that you have that one-to-one mapping there, right? But the rest of it you do not.

**21:06** · Okay.

**21:07** · Um So, let's talk a little bit more about kernel virtual addresses. So, the easy part was that logical set, very simple, right?

**21:16** · So, kernel virtual addresses I usually call it vmalloc space, like a lot of people. And um so, keep in mind uh of that. And that's that area above that logical range of it's managed dynamically.

**21:32** · And uh so, those are used for non-contiguous mappings. So, what's the practical case for that? Um insmod, right? You load a module, um memory's allocated. It needs to be virtually contiguous for that module, but doesn't need to be physically contiguous, right? So, what you'll find is the module does vmalloc, it's got that virtually contiguous area, and uh but the actual physical backing RAM could be scattered anywhere, okay?

**22:02** · Um the other piece is memory mapped IO.

**22:06** · You're using ioremap and friends, we'll get into that, right? Um and uh that also ends up in that space.

**22:15** · So, all right. So, quick look at that. We've got logical addresses, physical RAM.

**22:23** · Ooh.

**22:28** · Yeah. And so, that that's what that looks like. And then you've got your virtual address space up there, right?

**22:33** · Your modules are getting stalled, your ioremap's, all of that.

**22:39** · All right.

**22:41** · So, keep in mind as we said that the key there is it's non-contiguous, right? You can't can't can't rely on it for DMA at all. That's the main point here.

**22:54** · All right.

**22:56** · Over that.

**22:57** · Um Yeah, so this is this is reiterating probably maybe too much um is is this emphasis that on the 32-bit machine, right? We have a very constrained space um in that in the the the um the logical address space if we have um you know, 768 MB RAM, okay? So, there's less space for for kernel virtual addresses.

**23:25** · So, um those are tunable and you just don't deal with this problem on a 64-bit system.

**23:34** · All right, so let's jump into the meat of user virtual addresses cuz this is where it gets more complicated. Um so, our user virtual addresses, right?

### User Virtual Addresses

**23:43** · That's what our applications or our processes are are are mapped into, okay?

**23:48** · They're all below page offset, if you remember that memory map we saw below uh the 3 gig mark in in our modeled um uh 32-bit system, right? And each process has its own mapping.

**24:01** · What I mean by mapping is its own view of virtual address space, right? A thread shares mappings and things get a little bit more complicated with clone because there's a lot of options and you can choose how much you're sharing and so forth. Um but that's beyond intro level.

**24:19** · Um and uh so, one of the key things is um kernel virtual logical addresses, right?

**24:27** · They have that fixed mapping, okay? User processes are fully using the MMU and um the only time um the only time that you actually use RAM is when when um when you're actually touching it. We'll get into that, right? The memory isn't contiguous. It's a lot like that vmalloc space in kernel. You can't rely on anything being contiguous just because it looks that way from the virtual address, right? And um the nice thing is you can swap out, remember chronological uh virtual addresses.

**24:58** · It's not swappable memory and the memory can be moved around on you. So, that's virtual world.

**25:07** · Um All right. So, um What What does this fundamentally mean?

**25:13** · Since things can be moved around on you, it can be swapped out, you can't use it for DMA, right? Can't Can't allocate You can't malloc memory and then try to DMA to that virtual address. All right. It's not not going to be a a stable backing behind it, all right? Now, how does this work? Every process has its own memory map. You can go look at struct mm, right? There's pointers to that in your task struct for your process.

**25:38** · And uh that's where that whole mapping is kept of those um um pages. We'll talk about pages in a moment. Um every time you do a context switch, that memory map gets changed and that's where that overhead comes, right? Your context switching overhead, where you have to go change that mapping.

**25:58** · Okay?

**25:59** · Um So, again, back to our map here.

**26:02** · Um we've got this this view of the 32-bit world and um every time we change the process, this whole set of mappings in here into the space is going to change.

**26:14** · So, back to the MMU, right? Um so, we use that to to manage those virtual address mappings and so, I already hinted at page. So, how is this done? It works on the granularity of a unit called a page, okay? Um and some architectures, people always hear 4K.

### The MMU

**26:35** · Um some architectures, most architectures, they're configurable. Um there's um some advanced features, some very large page types. Um we're not going to get into that um today, but here's some common ones, right? 4K.

**26:49** · Um 4K or 64K in ARM 64.

**26:54** · Um like I say, we're not going to talk about huge pages. That'd be a more advanced topic, but um let's just assume 4K for um the stock since that's what's most uh most architectures are defaulting to.

**27:06** · So, that's our unit of memory that the MMU can work with, right? Um we're lined on that page size anytime we do any allocations or mappings, all right? And then, we have this concrete concept, which is the page frame.

**27:22** · Okay? And that's page size, page aligned unit of memory, a physical memory. So, anytime we say page frame, that means in that physical memory map, okay? And when we talk about a page, that's the unit for virtual addressing purposes that the MMU's dealing with, okay? And so, you'll see that abbreviation PFN throughout the memory management code. That's your page frame number, right? Referring to that page frame physical unit.

**27:53** · Okay. So, MMU operates on pages, right?

**27:57** · Memory map for a process is going to have this huge list of mappings, right?

**28:02** · Big space, a bunch of scattered page frames all over the place, right? A range of multiple pages. And so, what does the TLB need to know, right?

**28:13** · The TLB when when actually gets loaded with a mapping, right? A virtual address, the physical address, so page, page frame, right? And then a set of permissions, right? Read, write, execute.

**28:27** · Back to our view of that, right? Just as a reminder.

**28:31** · All right. Um So, as we were we we we touched on earlier, um if we ac- access a region of memory, right? That we don't have mapped, we're going to get a page fault exception, okay? And this is normal, right? These are good things. We want this page fault exception, okay? And I mentioned that TLB is very in size. You know, some of the embedded stuff, they have 16 entries in it. It's not much when we know that our page size is 4K, right? And so, that's got a lot of churn in it, okay?

**29:02** · And so, when we context switch, we have a lot of page faults as we start touching virtual addresses that aren't mapped, right? So, your process gets swapped or context switched in, you start executing code, it's touching, you get page faults, that exception because we don't have a mapping, right? And um we also have a a concept of lazy allocation we'll we'll talk about in detail here.

### Basic TLB Mappings

**29:32** · Um All right, so this is what it looks like visually. In between, you've got your virtual address, it's hitting the TLB, and then it's able to touch those physical page frames, right?

**29:45** · Through those mappings that are set up, right? So, mapped page ranges, right? So, contiguous set say the text for your uh application, your process, um some data area that's mapped, and those are going through the TLB to access actual backing page frames for that area. And then you'll have some unmapped space that maybe hasn't been executed yet.

**30:11** · Notice that's the the allocated frames on that side that's going through there.

**30:21** · All right.

**30:22** · Um So, just just as I mentioned with kernel virtual addresses and that vmalloc space, it's not guaranteed to be contiguous, okay? In in user space virtual addresses, right? So, don't rely on that. We already said that's why you can't use them for DMA, right? And one of the reasons for that is it it makes it much easier to allocate memory.

**30:43** · If you get into how the internal memory allocators uh work and and uh think about how how fragmented things get, this allows you to go put together a large allocation with a lot of scattered page frames, right? And um and almost everything you do it doesn't require physically contiguous um page frames backing your code.

**31:10** · All right. Um so uh as as we were saying, when we looked at that that virtual address and and we said the the virtual address space, say in 32-bit in that 3 gig area, we said one of the cool things was that each process gets his own address space. So, what does that mean? You hear that all the time.

**31:31** · It means that when you look at the virtual address space and say you look at that task struct and that MM, you're going to see mappings that have that same virtual address, but they're pointing to all different physical memory addresses all over the place. So, if they're in there at the same uh if you have things uh scheduled um running next to each other, they're using the same virtual address, right?

**31:55** · They get scheduled in, but it's mapped, right? To a different page frame each time. So, um but they don't have to know about that backing.

**32:05** · And so, here's an example.

**32:08** · Process one with this set of of of uh virtual addresses map through all these different page frames, right? In the blue.

**32:18** · And then process two has got these same virtual addresses and he's he's touching completely different page frames, right?

**32:26** · Just visually representing that, okay?

**32:29** · Um and now we get into shared memory, right? Uh we all we need to for IPC purposes, shared memory is a common concept, a POSIX concept. Um normal concept in most OS's.

**32:43** · And um and so uh shared memory using an MMU, and we saw how we can have the same virtual address with different page frames, right? We can have different virtual addresses pointing to the same page frames. Is essentially how shared memory works, right? So, simply map the same physical frame to different processes, right? The virtual addresses don't have to be the same.

**33:09** · And uh now you have shared memory, right? Two different processes, completely different virtual addresses, but they're touching that same page frame as they get context switched in.

**33:19** · Okay? And how does that look?

**33:21** · We got the shared physical frame down there in green, right? We've got this virtual address mapping to it. It's touching that shared frame, right? This is a 4K shared memory space, and then this completely different virtual mapping in the other process pointing to the same frame. Boom, we've got shared memory.

**33:43** · Um Now, um So, that was the case with with different virtual addresses, okay? Um The mmap system call you may be familiar with, right? You can get at a specific uh um uh address um to share uh the memory.

**34:01** · So, um that's uh that's a different case, okay? And uh it can fail.

### Lazy Allocation

**34:10** · All right, let's talk about lazy allocation.

**34:13** · Um So, one of the things you will notice when you uh work on a Linux system or classic Unix system is that um the kernel's not going to allocate uh memory uh directly.

**34:28** · Well, yeah, you saw your your call actually come back successfully, right?

**34:33** · You got virtual memory, but it didn't actually allocate the physical memory, those page frames that back it, right?

**34:40** · And that's what we call lazy allocation.

**34:43** · So, this is an optimization, right? The kernel's going to wait until you actually need to use that memory. So, if you're allocating a 4 MB chunk of memory for your database, and you haven't touched any yet, it didn't really allocate anything for you, right?

**35:00** · If you if you never use it, you never touch it, it never allocates anything.

**35:06** · All right. So, how does this work? Um so, when we we request that memory, it just creates this record of the request in the page tables. We'll talk about page tables in a moment. Returns the process, and so you've got that virtual memory set aside in the user space process, okay? Once we touch it, our old friend, the page fault, comes into play, right?

**35:30** · We already learned that we're going to get an exception, right? Cuz there's there's no mapping there, right? Or it's only set to read permissions, right? And uh we're going to go do the uh page fault handler. So, um kernel's going to use page tables, see that the mapping's valid in this case in a lazy allocation, right?

**35:52** · Allocated virtual address space, but it's not yet mapped in the TLB, okay? Um at that point, it's going to allocate those page frames, a page frame, a series of it, whatever the request uh needs to be satisfied with, okay? And um then it's going to update the TLB. It's architecture specific how that happens, of course, with that mapping, and then he comes back from the exception handler, and the user space program continues.

**36:19** · So, you your malloc got you that virtual address space and returned quickly, but when you went to touch the memory, all of this happened behind the scenes, right? The first time you went to dereference that pointer and update it with a value.

**36:35** · So, that's what's happening behind the scenes, okay?

**36:38** · But, you're not aware of that. Key point here, right? Um but, you will see it if you're running uh benchmarking and you see that lag, right? It's appreciable, right? And you can use you can use tracing tools and see how that's uh happening um uh visually.

**36:57** · Uh The other thing, if if you have uh time sensitive um things here, right? You know that uh you have a fast path, um you can go uh preallocate that. You may have used Mlock um or the family of Mlock calls. Um that will go ahead and preallocate these things, so you don't have that lazy allocations situation.

### Page Tables

**37:20** · Um So, as we said, getting into page tables, TLB entries could be uh TLB The entries in the TLB can be a limited resource, right? We can't just map the whole world of our address space in there, right? Um so, um we have a lot more mappings in that struct mm for our process than we have TLB entries. So, the kernel's got to track all that. So, it has a set of data structures uh we call the page tables, okay? And uh you can look in struct mm and vm\_area\_struct to see how um those are done.

**37:51** · And um but, it's essentially a hierarchy that leads you down to that 4K page, right? And the associated mapping to page frame number and the permissions, right? So, everything lines up with what needs to get loaded into the TLBs.

**38:09** · Okay? And also has metadata in addition to that about uh is it valid or not and so forth and some other housekeeping uh um flags as well. Okay? Uh So, um when we have something in the page table in the TLB, we so we have a valid mapping, right? And you touch it, the hardware, since there's nothing in the TLB yet, is going to generate that page fault, right? CPU doesn't have the knowledge, CPU the MMU, right? Uh only a kernel does.

**38:43** · All right. So, our page fault handler runs, right? It's going to traverse these page tables, find that mapping for the virtual address, right? Page granularity, select and remove an existing TLB entry, create a new one with our address and the correct permissions and so forth, and come back to the user space process.

**39:05** · Okay?

**39:07** · All right. Swapping.

**39:09** · And good. All right. Um so, swapping, we're used to our systems, we deal with our desktop systems, our development systems, um where we have a lot of swapping out to our disk when we're doing heavy builds, right? We're running low on RAM.

**39:29** · And um you know, how this works is the MMU is the thing that enables this, okay? And um so, um you're going to run out of that 16 gig RAM you have under these heavy builds, and uh you're going to context switch, and it needs more memory, and it's going to take those page frames that were backed, and it's going to take the contents of those, and it's going to push them out to your storage, right?

**39:54** · And then, when you need that data back, and you've been context switched back in, it's going to read that back off that slow storage, and bring it back in. That's the big picture, right? So, low-level details, right?

**40:12** · It's going to do that on a frame-based basis, right?

**40:15** · It's It's to copy a frame to the disk, remove the TLB entry, and then that frame is free to be backed for another process, right?

**40:27** · So, when we need it again, right? CPU generates page fault, right?

**40:32** · Common theme here, right?

**40:35** · We we we flush that entry out of the TLB, right? So, now it's going to generate a page fault, and then when we we hit that page fault, process sleeps, we copy that frame from the disk into an unused frame, and we update that page table entry, and then wake the process back up.

**40:57** · Okay?

**40:58** · So, it's going to be slow process, right? We got to go out to that block IO, we're throttled by that bandwidth now.

**41:06** · So, when we restore the page to RAM, okay?

**41:09** · We're not necessarily getting the same page frame. So, again, we have this virtual dance going on here, right? Um there is no persistence or affinity to that original physical page frame. So, you need to get rid of this notion that paid you know, physical addresses matter.

**41:25** · Okay? Um you will use the same virtual address though, right? Cuz those mappings stay the same in user space, right? So, you don't know the difference. So, your code's executing along, you yield the processor, it gets swapped out, you contact switch back in, it could it'll it'll redo that mapping, same virtual address, and your code continues on at the same virtual address, but a completely different backing as it the freight page frame contents gets copied back in and then mapped in, okay?

**41:59** · Again, this is that low-level detail why we said we can't use user space virtual addresses for DMA. We have no persistence of the physical backing that the the engines and the peripheral hardware need.

**42:14** · All right, so what does this look like visually?

**42:17** · Um we've got this frame that was selected um by the kernel to be swapped out to our disk. We've got this wonderful trash can looking cylindrical disk thing here and um we copy that frame out to the swap media.

**42:33** · We invalidate the TLB entry, page table entry is invalid now.

**42:39** · Right?

**42:40** · Okay? And now there's there's no entry there. So that that frame's freed up. So now you can free it back into the allocator pool, but the the data's preserved out there on disk. That's in your swap partition, right? All right.

**42:54** · Now we go we get we get context switched back in, right? We're back and running, same process. We try to access that same virtual address we were just running when we got so rudely taken off the CPU and we get the page fault thing. We've been through the page fault dance before and we just rock on through that. We get copied from the swap this cylindrical simple disk thing and uh um put back in into that page frame that we got allocated.

**43:28** · Create the TLB entry. Oops.

**43:31** · Got to add one more animation. Yeah. And uh then we return to user space.

**43:37** · Now we can access that virtual address we got the same data we had before we got swapped out. All right. So I'm actually running this on time behind, so I win. All right.

**43:54** · It's 95 slides.

**43:56** · Um So user space um we've got several ways. So So we've been through that whole stack, all the major pieces of how everything's happening in the background. Now now let's see how this maps into, you know, our APIs we have in user space, right? And So, we have several ways that we allocate memory, right? We've got all our family of Alec things and I've referenced them a couple times verbally, right?

**44:24** · We know that we can Mmap to directly allocate and map pages. We often see that to map some peripheral IO if we're hacking around not doing proper kernel drivers. We have break and Sbreak where we can modify the heap size, right?

**44:42** · So, first off, Mmap, right?

**44:46** · One way that we allocate a bunch of memory from user space, right?

**44:51** · You'll see it if you if you if you live the world of running S trace on things, you see lots of Mmap happening, right?

**44:59** · When files are getting uh um uh opened and so forth. Uh so, if you use map anonymous, you get you get allocated normal memory.

**45:11** · The shared flag allows us to share that memory with other processes.

**45:17** · All right, so break. Why is it called break? Sets the top of the program break, legacy terminology, right? And um so uh effectively, you increase the heap size with that, as we're saying, okay?

**45:34** · Now, um lazy allocation, going back to our whole lazy allocation technique, okay? Um we have a situation with with uh if we look at Mmap.c and do break, um that it's implemented a lot like Mmap, all right?

**45:50** · So, it goes in, it modifies page tables.

**45:54** · We talked about how that happened, right? Um where we modify the page tables and then we wait for a page fault, okay? And uh the other thing you can do is you can pre-fault we talked about with mlock, right? And not have that issue where with with accessing the memory you have this long lag relatively long lag where it actually has to allocate that big big chunk of page frames for you, right?

**46:22** · So you can you can take that cost up front with mlock and then have relatively deterministic behavior once you're actually accessing the memory.

### High-Level Implementation

**46:36** · Um the implementations of malloc and calloc are the same thing.

**46:43** · They're going to use break or mmap depending on how big the allocation is.

**46:48** · And that's going to happen behind the scenes, right? And if you if you are astute, you can modify that behavior with mallopt. You can set the threshold parameter to say where where one kicks in or not.

**47:03** · That's often used in system tuning. Okay? And then finally a stack.

**47:10** · If a process goes beyond a stack, right?

**47:13** · CPU's also going to trigger a page fault, okay?

**47:17** · One of the special things the page fault handler does in this case, right? Is it's going to detect that you got an address just behind behind the stack. It knows where that's at, right? And then it can allocate a new page, right? So it'll allocate another PFN go into the page tables, map that in, drop it in the TLB.

**47:38** · And remember PFN could be anywhere. It's not physically contiguous. It's just virtually contiguous. So it's faulted in execution continues on and it's able to you know drop stuff on that segment of the stack.

**47:55** · You can see how that works in do page fault.

**47:57** · That's the arm version.

### Summary

**48:00** · And um so quick summary, like I said, introduction. So if you're already a kernel expert, you probably know all that, but we went through physical memory, right? We looked at uh stock um you know, x86 familiar memory map. We talked about virtual memory, three types, right?

**48:20** · Kernel logical, kernel virtual, user virtual.

**48:24** · Which ones are contiguous or not, right?

**48:27** · We use kernel logical for DMA. Um we went through user space addressing, how uh processes will not have contiguous uh physical memory, and how swapping, page faults work to do lazy allocation, and so forth. Um like I said, we covered swapping, and then how those user space uh APIs map on to all of that.

**48:52** · So that's it for the intro. I've got 1 minute for questions.

**49:07** · Yes, we're in the back.

**49:28** · Okay. So So the first part first part of the question, let me address that. So the question was, well, if the kernel always has the mappings, right? And you're talking about that kernel logical mapping that has, why do we have to wait for this expensive mapping to user space? And that that So to answer that, and I hopefully I'm answering the right question, um the reason for that is those those kernel logical mappings, if we just use those, it would be just like that single address system without an MMU.

**49:56** · And I can tell you that there's there's systems that in the '90s that had MMUs that running our tosses like VXWorks, they would map with the MMU just flat address space because they had to have the MMU on for performance reasons, but you were you you don't without without having your own process space, right? You would have to link everything in its own address space and everything.

**50:21** · So, kernel logical addresses are nice and linear and easy to think about, but uh you have to do these remapping uh for user space to have that nice world that we enjoy of that protected per user process address space where you just write a program, link it, and it'll run in any context, right?

**50:45** · If we had all one mapping of the kernel logical spaces be just that single address space, you'd have to link your program at zero and one bazillion, two bazillion, and manage them not stomping on each other as you allocated the memory.

**51:02** · I hope that answers the first part of it. And I'm out of time, but we can talk about the second one.

**51:11** · Yep.

**51:11** · Sorry, 95 slides, so.