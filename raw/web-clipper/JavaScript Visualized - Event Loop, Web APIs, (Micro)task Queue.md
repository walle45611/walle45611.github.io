---
title: "JavaScript Visualized - Event Loop, Web APIs, (Micro)task Queue"
source: "https://www.youtube.com/watch?v=eiC58R16hb8&t=139s"
author:
  - "[[Lydia Hallie]]"
published: 2024-04-04
created: 2026-09-21
description: "Learn how the browser event loop, task queue, microtask queue, and Web APIs work together to enable non-blocking, asynchronous JavaScript. - https://www.patreon.com/LydiaHallie- https://buymeacoffe"
tags:
  - "clippings"
---
![](https://www.youtube.com/watch?v=eiC58R16hb8)

Learn how the browser event loop, task queue, microtask queue, and Web APIs work together to enable non-blocking, asynchronous JavaScript.  
  
\- https://www.patreon.com/LydiaHallie  
\- https://buymeacoffee.com/lydiahallie  
\- https://twitter.com/lydiahallie  
\- https://www.linkedin.com/in/lydia-hallie/  
\- https://instagram.com/theavocoder  
  
Timestamps:  
0:00 Intro  
0:32 Call Stack  
1:18 Single-threaded Problem  
2:01 Web APIs  
2:47 Callback-based APIs  
4:04 Task Queue  
4:39 setTimeout  
6:22 Microtask Queue  
8:40 Promisifying Callbacks  
8:57 Challenge  
10:52 Recap  
11:58 Outro

## Transcript

### Intro

**0:00** · the event Loop is a pretty notorious Topic in JavaScript but when we zoom out it's just a tiny component within the JavaScript runtime we also have the call stack we have web apis we have the task q microtask q and then eventually the event Loop well to be a little more technically accurate here we actually have the JavaScript engine in which we have the call stack and then also the memory Heap but to keep my slides a little organized I'll just be showing

**0:24** · the call stack here all these components together allow us to use asynchronous tasks in a non-blocking way in JavaScript and this is important because JavaScript itself is single threaded we're only working with a single call stack so the call stack manages the execution of our program so if we have

### Call Stack

**0:40** · the following script we first have console log one so a new execution context is created pushed onto the call stack which is then evaluated and logs one then we have console log two same story execution context is created pushed onto the call stack is evaluated in logs two then on line 14 we invoke another function the log three and four and within this function body we invoke yet another function log three and within log three we invoke another function log the console log three

**1:07** · eventually it logs three then on the second line within the log three and four function body we console log four four gets logged and now the log three and four execution context is popped off the call stack as well now something important to remember here again is that JavaScript can handle a single task at a

### Single-threaded Problem

**1:22** · time so if we for example had this long running task in which we have a pretty heavy computation it takes a while before for JavaScript can continue with the rest of our program so the console log long task done is only logged after a couple seconds and this is not what we want CU in the meantime our entire program is frozen so we want to avoid these long running tasks but in a real

**1:45** · life application we often have to use these long running tasks like maybe a network request or anything based on user input timers so what happens then like is our entire cost stch just blocked until we get the data back no because we're actually using web API in those cases and web apis provide a set of interfaces that allow us to interact with the browser's features this includes functionality that we often use like the document object model fetch set timeout and so many more the browser is

### Web APIs

**2:13** · a very powerful platform with a lot of features some of these features are required like the rendering engine or the networking stack but we also have access to some cooler ones like device ones sensors cameras geolocation and so on okay cool but what this does have to do with with long running tasks well some of these web apis allow us to offload long running tasks to the browser so when we invoke such an API we're kind of just initiating that offloading and web apis that expose

**2:43** · these asynchronous capabilities are either callback based or promise based so first let's focus on the Callback based apis and I'm just going to use the geolocation API because it's fun I could have used any other callback based API but let's say that we want to get the user's location and for this we can use the get current position method exposed by the geolocation API and this receives two callbacks first we have the success call back in case everything goes well and the user allows us to get the location and we actually get it from the browser or the error call back in case

### Callback-based APIs

**3:12** · anything goes wrong so let's see what happens when we actually use this in our script so first the get current position invocation gets added to the call stack however this is just to register those call backs and initiate that async task after doing that it can get popped off the call stack immediately so it doesn't wait for any data now in the background the browser starts some kind of process

**3:34** · that eventually shows the user popup now of course we don't know when the user is going to interact with this popup but that's not a problem because this is not happening on the call stack so our entire website is still responsive in case other tasks need to run instead now finally the user clicks on allow so the API receives the data from the browser and uses the success call back to handle this result however it can't just push that call back back onto the call stack this could disrupt an already running task and just create very unpredictable Behavior so instead the call back gets

### Task Queue

**4:05** · pushed to the task CU which is also called the Callback queue for this exact reason the task CU holds web API callbacks and event handlers to be able to get executed at some point later in the future and this is where we finally get to the event loop it's the event Loop's responsibility to check if the call stack is empty and if that's the case so if nothing is running and then gets the first available task from the task queue and moves this to the call St where it's executed so now finally we handle the results and the user's location is logged to the console

**4:37** · another very popular callback based web API is set timeout and set timeout also receives a call back and a delay so let's see how that works so first we encounter a set timeout and this again gets added to the call stack but all it does again is register that call back and also to delay with the timer's API and in the background the browser will actually handle that timer then we have another set timeout and again it registers the call back and the delay

### setTimeout

**5:02** · now our timers are still running and we have a console log end of script this just gets added to the call stack and logs and of script Nothing asynchronous here now after 100 milliseconds the browser is like hey 100 milliseconds expired so now the Callback moves onto the task Q there's nothing on the call stack right now so this moves onto the call stack where eventually it logs 100 milliseconds now 2,000 milliseconds are up again same story The call back is pushed onto the testq call stack is empty so it moves onto the call stack where it logs 2,000 milliseconds so it's

**5:35** · just very important to remember that when you have a set timeout and a delay it's not the delay until it gets moved onto the call stack no it's the delay until it gets moved to the task Q so this means that the delay that we specify might not actually be the delay

**5:50** · to execution because if the call stack was still very full with other tasks and this could run for many more seconds the call back would still have to wait in the task cu until the call stack is empty so just something to to keep in mind so long story short the call backs provided by web apis are pushed onto the

**6:08** · test queue when the asynchronous task completes so what about the promise-based ones if you haven't checked out my promises video yet I highly recommend you watch it because I'll just assume some basic promise knowledge uh while explaining this entire flow whenever we work with promises we're working with the microtask Q the microtask queue is a

### Microtask Queue

**6:27** · special queue dedicated to then catch finally call backs uh a function body execution after await the Q microtest call back and the new mutation Observer call back so only those callbacks or those function body parts get pushed onto the microtask CU so it's very specific however the event Loop prioritizes the microtask Q so whenever the call stack is empty the event Loop first ensures that the microtask Q is entirely empty so it gets all the tasks

**6:55** · from the microtask Q moves them onto the call stack where they get executed and only then will it move to the task que and after each task in the task CU it again checks the microtask Q and A popular promise based web API fetch so let's see what happens behind the scenes when we invoke fetch so whenever we call fetch it's added to the call step this is just responsible for creating a promise object which by default is pending the result is undefined and we

**7:20** · don't have any promise reactions just yet it also initiates that background Network request that's handled by the browser then we move on to the next line we have the den Handler and this creates a promise reaction record where we have res console log res the server still hasn't responded by the way but we got to line four so there we have a synchronous console log end of script so now end of script is logged to the console and then finally the server

**7:43** · returns some data so now the promised data set to fulfilled the promised results is now the response object with the data that we got from the server and the promised reaction Handler is now also pushed to the microtask Q right because it's a then call back and that gets pushed to the microtask Q the call stack is empty so the event Loop checks the microtask Q moves this to the call stack where it eventually logs the result that we got from the server something to keep in mind with microtasks is that a microtask can also

**8:11** · schedule another microtask and this means that the event Loop is just constantly handling the microtask and it can never actually get to the task CU it would just have to wait indefinitely so we're kind of creating an infinite Loop an infinite microtask Loop um freezing

**8:27** · our entire program I believe in node you can set like Mex tick depth or something like that which prevents this exact thing from happening but just make sure that you don't accidentally end up doing that and we can also promisify uh a callback based API so for example we can rep the get current position with a new promise Constructor and for the success call back and the error call back we

### Promisifying Callbacks

**8:48** · just pass resolve and reject so this can be a pretty nice solution just to improve the readability within your codebase a bit all right A little quiz to see if you uh kind of understand it so we have a promised resolve with a den Handler we have a set timeout we have a Q microtask in which we have another q microtask and then we have a console log five it's up to you to see what gets logged so pause the video now and let's see if you got it right and the right answer is

### Challenge

**9:18** · 51342 so let's see why first we have the promise resolve and this just creates a new promise object that's instantly resolved then on the next line we have the den Handler the promise is already resolved so in the background it does create that promised reaction but the Handler is immediately pushed to the microtask Q then we have set timeout

**9:36** · which is responsible for initiating that timer so the call back and the delay get passed to the API and in the background the browser start some sort of timer then we have q microtask so the call is added to the call stack and this cues that call back to the microtask que then we have the synchronous cons Log 5 so this gets push to the call stack and logs five and in the meantime the 10 milliseconds are up so the call back from set timeout is pushed to the task Q

**10:01** · cuz this was a call back based API so task Q our script is done the call stack is empty so the event Loop checks the microtas Q and there we have the promise Handler call back and this eventually calls console log one so one is logged to the console then we have the Q microtask callback and within this callback we call console log three so three is locked to the console then we call another q microtask and this cues

**10:23** · another microtask with it with its call back to the microtask que however the event Loop has to to ensure that the microtask Q is entirely empty before moving on to the task Q so that call back is immediately moved onto the call stack again and logs four now finally the call stack is empty and the microtask Q is empty so the first available task from the task Q is moved onto the call stack and this eventually logs two so now we have

**10:51** · 51342 so let's just recap what we've covered so far so JavaScript is single threaded it can only handle one task at a time we can use web apis to interact with the features leveraged by the browser and some of these apis allow us to initiate Asing tasks in the background so the function call that initiates an asnc task like that is still added to the call stack but this is just to hand it off to the browser the actual async task is handled in the

### Recap

**11:19** · background so it does not block the call stack the task CU is used by callback based web apis to enue the Callback once the asynchronous task has completed then we have the microtask Q which is only used by promise handlers the async function bodies after await the Q microtask Q callbacks and the new mutation Observer callbacks this queue has priority over the task q and the event Loop ensures that this queue is entirely empty before moving on to the task q and after handling each task from

**11:51** · the task Q the event loop again checks the microtask Q to ensure that nothing has been added in the meantime you often come across asynchronous JavaScript and if you aren't entirely sure why things execute a certain way it might just be very discouraging but I hope that my explanation for the task q and the micro tasq and the event Loop kind of helped you understand why certain parts of our code execute at a certain time of course

### Outro

**12:15** · as always if you have any specific questions feel free to reach out but I also highly recommend that you kind of just play around with it yourself like try using set timeout try using Q microtask just to get a better sense of like oh yeah okay I understand why this runs at this time time and why this doesn't execute stuff like that good luck and have fun coding