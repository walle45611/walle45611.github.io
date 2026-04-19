---
title: How to sync Obsidian 4 FREE · Syncthing vs Autosync
source: https://www.youtube.com/watch?v=t3cy132eeUU
author:
  - "[[DJ Lensing]]"
published: 2022-09-22
created: 2026-04-20
description: "👉 My NEW Skillshare Class is online! Digital Efficiency: Prepare for Productivity with Weekly Upkeeps → https://skl.sh/3ECRC3k | It’s FREE within the trial, enjoy!00:00 Intro · Obsidian Sync Free0"
tags:
  - clippings
  - youtube
---
![](https://www.youtube.com/watch?v=t3cy132eeUU)

👉 My NEW Skillshare Class is online! Digital Efficiency: Prepare for Productivity with Weekly Upkeeps → https://skl.sh/3ECRC3k | It’s FREE within the trial, enjoy!  
  
00:00 Intro · Obsidian Sync Free  
04:07 Syncing Obsidian 101  
06:57 Setting up Autosync for Dropbox, Google Drive, OneDrive  
13:13 Setting up Syncthing for Obsidian  
20:24 Final thoughts on Dropsync, Syncthing, Obsidian  
  
This video is about how to sync Obsidian for free between your pc or desktop computer and your Android device, using the Obsidian app and Autosync for Google Drive, for Dropbox or for OneDrive, also known as DriveSync, Dropsync or OneSync — or Syncthing.  
  
There are other alternatives to do this, such as Foldersync. And there’s is no one solution for all scenarios. What suits you best is a matter of your financial situation, technical environment and personal preference. 🧑‍💻  
  
🔊 Music by @Kijugo: https://www.youtube.com/c/Kijugo — big thanks for letting creators use your work! 🙏  
  
🔗 Related links  
  
Tutorial on How To Backup Obsidian (@FromSergio): https://youtu.be/wkZVrDaBzz0  
Tutorial on How To Setup Foldersync (@Dre Techsoft): https://youtu.be/zIkZCCDWPdU  
Eleanor Konik on Syncing between Devices (Obsidian Forum): https://forum.obsidian.md/t/meta-post-syncing-between-devices/20983  
Jakob Borg on Syncing and Syncthing (Medium): https://medium.com/@jakobborg  
Obsidian Community on Discord (Obsidian Members Group): https://obsidian.md/community  
Obsidian Forum Discussion on Syncing and SD Card trouble: https://forum.obsidian.md/t/mobile-syncing-vault-with-dropsync-on-android/20714/4  
Official Obsidian Help Vault: https://help.obsidian.md/  
Official Obsidian Website: https://obsidian.md  
Syncthing’s documentation: https://docs.syncthing.net/  
  
David via Social Media: Say hello! 👋  
Facebook: https://www.facebook.com/dajolensing  
Instagram: https://www.instagram.com/dajolens/  
Twitter: https://twitter.com/dajolens  
Skillshare: https://www.skillshare.com/user/dajolens  
  
→ #obsidian #googledrive #syncthing

## Transcript

### Intro · Obsidian Sync Free

**0:00** · Whichever tool you end up using, before setting up syncing, backup your files!

**0:04** · More on that in a minute. Just didn't want you to skip this advice. It’s important.

**0:09** · That being said, this video is about how to sync your Obsidian files for free between your pc or desktop computer — this one — or your Laptop, and your Android device.

**0:19** · Using the Obsidian mobile app and Autosync for Google Drive, for Dropbox or for OneDrive, also known as DriveSync, Dropsync or OneSync — or Syncthing.

**0:30** · There are other free or freemium alternatives to do this, such as Foldersync.

**0:34** · And there’s is no one solution for all scenarios.

**0:38** · What suits you best is a matter of your financial situation, technical environment and personal preference, really.

**0:45** · After watching this somewhat longer video than I usually do on this channel.

**0:49** · you will have a clearer idea of how DriveSync, Dropsync, Onesync and Syncthing work in combination with the Obsidian mobile app, whether one of these is an adequate solution for you — and whether you liked this video and want to subscribe to see more, or if this just wasn’t helpful at all.

**1:06** · Speaking of... right upfront: Do the apps discussed here work for iPhone or iPad as well?

**1:13** · To be honest, no idea, I haven’t tested it.

**1:15** · As far as I know, syncing notes on your iPhone or iPad does NOT work with Syncthing for example, but with iCloud Drive.

**1:24** · If you know more about it or have been able to test it lately, then feel free to let us know in the comment section below.

**1:30** · I’m sure it would be greatly appreciated, thanks!

**1:33** · Now, the knowledge base and notetaking app Obsidian is free for personal use.

**1:38** · But the encrypted sync built into Obsidian is a paid service, which currently costs $8 per month, if paid annually.

**1:45** · I think it’s worth it, since it’s convenient, officially supported, pretty fast and it comes with one year of version history.

**1:53** · This time machine like feature turns out to be quite useful as a temporary backup.

**1:58** · But it’s not a real backup. You should have a real backup.

**2:01** · However, Syncthing also offers a version history, even longer than one year if needed.

**2:06** · So, on that note: If you’re just starting to use Obsidian and haven’t yet settled on it, you might be reluctant to pay for syncing.

**2:13** · Also, if money is just tight and the alternatives work fine, without paying a dime.

**2:17** · It only costs you some time.

**2:19** · Wow, did that rhyme? How sublime.

**2:22** · Yeah, enough coffee for me today.

**2:24** · But of course, your time is tight too.

**2:26** · Therefore, there are chapter markers so you can jump to what’s most relevant to you.

**2:32** · For overview, check out the table of contents in the description.

**2:35** · Right below the link to my Skillshare profile, where you can find courses of mine and watch them for free as part of the Skillshare trial.

**2:43** · Ok, before we continue, three important warnings.

**2:46** · First, in case this has not yet been understood: syncing is not the same as creating a backup.

**2:52** · How to backup Obsidian, that is a different topic.

**2:55** · But I'll put some links in the description on how to backup Obsidian files.

**2:59** · Second, create a backup.

**3:01** · It doesn’t have to be the ultimate backup solution, yet.

**3:04** · Still, make a copy of your files and put it somewhere safe, before setting up syncing.

**3:10** · Things can go wrong. Shit can get lost.

**3:12** · Don’t want you to cry.

**3:13** · Third, do not mix syncing options.

**3:16** · Because that’s when things will go wrong, eventually.

**3:20** · To quote Eleanor Konik: Bi-directional syncing of a vault with two different services (i.e. Obsidian Sync &amp; iCloud, or iCloud &amp; Dropbox, or git + obsidian sync) is a recipe for race conditions and disaster.

**3:34** · This quote is from a Meta Post on Syncing between Devices in the Obsidian forum.

**3:40** · It’s widely referred to and I highly recommend to check this out.

**3:44** · Other good places for all things related to syncing and more are the Obsidian Members Group, OMG, on Discord — full of bright minds who are really into Obsidian (not just occasionally, like me), and the official Obsidian Help Vault.

**3:58** · Links to all this can also be found in the video description below.

**4:03** · So...

**4:05** · Still here?

**4:06** · How nice! Let’s continue.

### Syncing Obsidian 101

**4:08** · Syncing Obsidian 101 Once you use Obsidian for taking notes or writing in general or whatever, at some point you want to use it anywhere.

**4:18** · That includes, of course, a mobile device such as your phone.

**4:22** · The Obsidian mobile app is made for that.

**4:24** · It’s a great – and I would argue: essential – addition to the desktop version of Obsidian.

**4:29** · The Obsidian mobile app let’s you easily add and review files on the go once it’s all set up.

**4:36** · But therefore you have to deal with the topic of how to sync your Obsidian files.

**4:40** · Syncing files means keeping them up to date across different devices.

**4:44** · Then you can switch these devices, but continue working on your files.

**4:48** · Since Obsidian is based on simple markdown files, syncing Obsidian for free between PC and a notebook is fairly easy.

**4:56** · In most cases you can do that with your cloud service, be it Dropbox, Google Drive, iDrive, OneDrive, pCloud...

**5:03** · Quick tip: You can not only sync your markdown files like this, but also your Obsidian plugins, settings and themes, as they are saved in a hidden folder called .obsidian right within your vault.

**5:15** · To make hidden files and folders visible, press shift + cmd + . for Mac.

**5:20** · For Windows: I don’t know, but Google will.

**5:22** · Although all of these cloud services offer mobile apps, syncing Obsidian files with these does not work that easily.

**5:30** · The reasons for this are in detail beyond my expertise and better explained elsewhere.

**5:36** · Apparently it has something to do with app sandboxing, among other things.

**5:42** · As a not very, very tech-savvy person (I am more into old paper books, obviously), I would explain it like this: Syncing depends heavily on communication between applications and devices and stuff.

**5:55** · That’s what makes syncing inherently complicated, just as communication in general is.

**6:01** · Think of everyday situations: You say A, your partner understands B — and all of a sudden you're getting a divorce, arguing about the custody of your kids.

**6:08** · Communication is messy and always will be.

**6:11** · It needs Fingerspitzengefühl, you know, empathy, sensitivity and all that jazz.

**6:15** · And still, there is simply no way to rid communication of all potential for conflict, to eradicate every ambiguity and misunderstanding.

**6:25** · The same goes for syncing.

**6:27** · There is no such thing as error-free, totally seamless syncing.

**6:32** · As one of the Obsidian developers put it in the early days: »yeah syncing is never easy« So much for lowering your expectations.

**6:40** · Whichever solution you choose, it will never be all smooth all the time.

**6:45** · Even with the official Obsidian Sync.

**6:47** · But for now, let’s finally talk about what you're here for: Syncing Obsidian for free, at first using Autosync for Google Drive and then using Syncthing.

### Setting up Autosync for Dropbox, Google Drive, OneDrive

**6:57** · Setting up Autosync for Dropbox, Google Drive and OneDrive Czech this out: Autosync for Google Drive (also called DriveSync), for Dropbox (called Dropsync) and for OneDrive (called OneSync) are all from the same developer and work very similar.

**7:14** · I used Dropsync for several month and it was all in all a good experience.

**7:18** · But for a change I'll show how it works using Autosync for Google Drive (or DriveSync) as an example.

**7:24** · As I said, the apps are very similar, so much so that this Quick Start Guide should help either way, even if you want to use Dropsync or OneSync.

**7:32** · I assume you have Obsidian and the Cloud Service of your choice, Dropbox, Google Drive or OneDrive, already installed, both on your desktop computer or laptop and on your Android device — and that your Obsidian Vault is already placed in your Cloud folder.

**7:48** · That’s like step zero.

**7:50** · This seems like a good opportunity to remind you again to please back up your files.

**7:55** · Beware that once sync is set up, deletions will also be synced.

**8:00** · Therefore, if you haven't already, save another copy of your vault somewhere safe.

**8:05** · Like, right now.

**8:07** · Ok, open Obsidian and — from the start screen — select "Open folder as vault", then find the copy of your vault in your cloud folder and open it.

**8:17** · Obsidian will now remember that your vault is in the cloud.

**8:21** · If you do this on your PC and your laptop, that's already enough to sync Obsidian between those devices.

**8:27** · But the mobile versions of Dropbox, Google Drive and OneDrive, they don't create a local copy that would be accessible for the Obsidian mobile app.

**8:35** · Hence, we need a workaround and that's where Autosync for Dropbox, Google Drive or OneDrive comes in.

**8:41** · Step 1: Find the right Autosync app in the Google Playstore.

**8:46** · For this example it’s Autosync for Google Drive.

**8:49** · Note that there’s also an app called Autosync — File Sync &amp; Backup, again, from the same developer, as a universal file sync and backup tool for any cloud storages and NAS devices.

**9:01** · But this one has a trial period after which you would have to purchase to continue using the app.

**9:06** · Autosync for GoogleDrive, Dropsync and OneSync on the other are free to use, but they come with ads.

**9:13** · You can upgrade to remove these ads and add some premium features, which we don’t need for now.

**9:19** · But for the record: The upgrades are very affordable one-time payments.

**9:23** · You practically buy the developers one or two Starbucks coffees, a nice gesture, you might want to consider.

**9:30** · Install the app of your choice, open it and agree to any terms. To sync data the app needs access to your device storage, which you have to be ok with as well.

**9:39** · Therefore, enable »All files access« for Autosync.

**9:43** · Step 2: Connect Autosync to your cloud storage.

**9:46** · Press the button, choose an account, click ok and done.

**9:50** · Google Drive connected, in this case.

**9:52** · If you want to disconnect, go to Settings in the upper right corner, click Google Drive account, press the three dots on the right side and remove your account.

**10:02** · Step 3: Choose what to sync.

**10:05** · Again, press the button.

**10:07** · Then there are three options, from which we choose the second one: Let me create my own folder pair.

**10:14** · Here’s where the magic happens.

**10:16** · First, assigning a folder pair name is optional.

**10:19** · I’ll call it Obsidian.

**10:20** · Second and required: Select a remote folder.

**10:25** · In Google Drive I go to My files, to Documents and there I placed only one folder, my Obsidian Vault, which is named after the princess Ariadne you might know from Greek mythology.

**10:36** · Third and also required: Select a local folder on your phone or your mobile device.

**10:42** · You can choose between internal storage and, if available, SD card.

**10:46** · I read on the Obsidian forum that Android has made changes to make SD cards more secure in 2021, and as a result only certain apps have access to SD cards — so this might cause issues.

**10:58** · Not sure though if this situation is still up to date.

**11:01** · Either way, in the setting of DriveSync, Dropsync and OneSync you can manage the SD card write access if that’s relevant to you.

**11:10** · I choose the internal storage anyway, go to Documents and there I’ve already created a folder called Ariadne as well and that’s it.

**11:18** · As Sync method you want to enable Two-way to really make this a syncing solution and not, for example, a backup solution — which is something different that you should totally have.

**11:28** · Can’t stress this enough...

**11:30** · Lastly, uncheck the checkbox for "Exclude hidden files", so that your Obsidian plugins, settings and themes are synced as well.

**11:38** · Et voilà, you have synced folders.

**11:41** · Quick example: On the desktop version of Obsidian I create a new file in my vault, give it a name, write something in it and then take a look in Obsidian on my mobile device.

**11:51** · After a few seconds, the file appears, great, it works.

**11:55** · I add something in the mobile version — and a little later the change appears on the desktop.

**12:00** · A small delay remains, though.

**12:02** · At the time, I had made it a habit to constantly open Dropsync and sync it manually to speed up the process.

**12:10** · Not ideal, but that’s how workarounds are, right? Not. Ideal.

**12:14** · The official Obsidian Sync works faster and more conflict-free, as you would expect.

**12:21** · A common problem that I ran into when syncing Obsidian files via Dropbox and Dropsync has been file names: There's a list of characters that are not allowed and Dropbox also has a problem with emojis in filenames.

**12:36** · Not that I would use emojis in filenames but a lot of people do, so...

**12:41** · Consider it a warning.

**12:42** · So much for Autosync for Dropbox, Google Drive and OneDrive.

**12:46** · Other alternatives to sync Obsidian files for free are Foldersync and the much more recommended Syncthing.

**12:53** · I didn’t try Foldersync myself but there are great introductions to Foldersync online.

**12:58** · And I’ll put a link to one of them in the description, right below the like and subscribe buttons, as we are more than halfway through and you might want to press at least one of these buttons by now.

**13:11** · Next up: Syncthing.

### Setting up Syncthing for Obsidian

**13:13** · Setting up Syncthing for Obsidian When it comes to syncing Obsidian for free, many people on the internet recommend Syncthing as the best solution.

**13:22** · Syncthing is a synchronization application that is free to use, encrypted and open-source.

**13:28** · It let’s you sync files without third parties such as Dropbox or Google Drive being involved.

**13:34** · Syncthing is around for almost 10 years, so it’s well established.

**13:38** · Here’s how to set it up, in short.

**13:40** · For more details check out the official and very comprehensive documentation of Syncthing.

**13:44** · It’s linked below.

**13:46** · Step 1: Download Syncthing via syncthing.net on your desktop or laptop device — let’s call it: your local device — and install it.

**13:55** · For Windows, the desktop application is called SyncTrayzor.

**13:58** · When you start the program for the first time, it will create a Default Folder called Sync in your home directory.

**14:05** · Then download and install Syncthing also on your Android device — your remote device.

**14:11** · You can find the Syncthing mobile app on Google’s Playstore.

**14:15** · Step 2: Open the mobile app and click through the first steps.

**14:19** · Syncthing needs to access your storage to do file synchronization, so you have to grant permission to that.

**14:26** · Syncthing can also be configured to synchronize on selected WiFi networks only.

**14:32** · If you want to use this feature, you have to grant permission to access the location of your phone device in the background.

**14:38** · This is optional. Let's continue.

**14:41** · If you’re not connected to any Wi-Fi you’ll see a warning, that Syncthing is disabled.

**14:47** · You can change that by connecting to Wi-Fi or change the setting so Syncthing can run on mobile data as well.

**14:55** · There might be another warning for battery optimization, which you might want to turn off for Syncthing so that Android won’t stop synchronization after some time.

**15:04** · To do that, just add Syncthing to your device’s whitelist.

**15:08** · Here we are, ready to put this app to use.

**15:11** · Step 3: Add your local device — your pc, your desktop computer or laptop — to the Syncthing mobile app of your remote device.

**15:19** · Therefore, go to DEVICES and click on the plus icon in the top right corner.

**15:24** · Now, to add a device you need the Device ID, the identification number for the other device that you want to sync your files with — which is the local device, the first device you installed Syncthing on in Step 1.

**15:38** · Switch to that device, open Syncthing and go to Actions and Show ID.

**15:43** · That will open up a window with the ID and a quick response code, better known as QR Code.

**15:49** · You can copy the ID manually or scan the QR code by clicking on the QR code icon in the mobile app and allow Syncthing to take a picture.

**15:58** · You can name the device if you want and click on the checkmark icon in the top right corner.

**16:04** · On your desktop device, when you close the Device ID window you’ll be informed that a new device wants to connect.

**16:10** · Click add device, change the name if you want, save and done.

**16:15** · Congrats, you are connected.

**16:17** · On your desktop version of Syncthing you can now see your remote device right beneath the section of this device.

**16:24** · Step 4: Set up folders for syncing on both devices.

**16:28** · On your desktop device you’ll find a Default Folder, unshared so far.

**16:33** · This is the folder called Sync that Syncthing created in your home directory.

**16:38** · You can edit and change the folder label, which is not the same as the folder name on your computer.

**16:44** · I’ll label it Obsidian, because that’s want I want to use it for.

**16:49** · There’s the default folder’s path, which you can’t change here when the folder does already exist.

**16:55** · But this is the place where you define the location for a new folder, when you decide to add a folder.

**17:01** · If you — for example — already have an existing Obsidian Vault as a folder on your device, then you can copy the path to this folder and paste it here.

**17:11** · Keep in mind that your Obsidian Vault should not be in a cloud when doing this, to avoid a mix of multiple sync options.

**17:19** · To copy a path on Windows, just right click on the folder and choose copy as path.

**17:24** · On Mac, right click the folder and when the menu pops up, hold down the option key.

**17:30** · That reveals the copy as path name option.

**17:33** · Paste it to Syncthing.

**17:35** · Otherwise, move (or copy) your Obsidian Vault into the Default Folder — or create a new one in there to be your future vault.

**17:43** · I’ve done that, so in the folder called Sync created by Syncthing, I created a new folder called Ariadne to be my Obsidian Vault for this demonstration.

**17:54** · Next go to Sharing and select your mobile device.

**17:57** · If untrusted, you can enter an encryption password.

**18:00** · Click save et voilà, your folder is now up to date.

**18:04** · On your mobile device you’ll see a notification, that your desktop device wants to share a folder.

**18:09** · If you click accept you can now create a folder on your mobile device.

**18:14** · The mobile folder's label can be different from the desktop folder’s label.

**18:18** · Again, this label is not the same as the folder name.

**18:21** · Now click Directory to choose where on your mobile device this folder should be put.

**18:26** · Per default it gives me a path it gives me a path starting from my SD Card.

**18:30** · But I want it to be on my internal storage, to have less trouble with accessibility.

**18:35** · Again, not sure if this really might cause issues.

**18:38** · Anyhow, I go to my internal storage and create a folder called Sync — could be called different though, whatever you want.

**18:45** · I allow Syncthing to access files in this folder and the this path will be shown as Directory.

**18:52** · Further down you can see the folder type, which is by default Send &amp; Receive.

**18:56** · You can change that now or later under advanced settings to Send Only or Receive Only, so that syncing is a one-way street.

**19:06** · But for most use cases with Obsidian you probably want to have both.

**19:10** · Hit the checkmark and done.

**19:13** · Since the folders are now linked and synced, the Obsidian Vault that I created on the local device also shows up on my remote device.

**19:21** · Well, to make it an Obsidian Vault I have to start Obsidian and open the folder as a vault — on both devices.

**19:28** · Now when I create a new file on the remote device and write something, then after a few seconds it is also displayed on the local device.

**19:39** · And if I add something there, I see it — with the magic of syncing — appearing on the remote device as well.

**19:47** · This is what Syncthing with Obsidian looks like in action.

**19:51** · Speaking of the magic: How does this even work?

**19:54** · Like, syncing in general and with Syncthing specifically?

**19:58** · For that I recommend a couple of Medium articles written by Jakob Borg, the original author of Syncthing.

**20:05** · They are linked below and for laymen like me an interesting, short introduction.

**20:10** · And again, for more details, for exmaple on how to ignore certain files — if you don’t want your Obsidian settings and themes and plugins to sync — or on how versioning works in Syncthing, check out the official documentation.

### Final thoughts on Dropsync, Syncthing, Obsidian

**20:24** · To sync Obsidian files for free, the easiest solution for me was Dropsync.

**20:29** · I still think it’s a bit easier to set up than Syncthing.

**20:33** · Or maybe it’s just that Dropsync’s UI looks a bit more familiar to me than Syncthing’s GUI or Graphical User Interface does.

**20:42** · And since Dropsync has always worked well for me, there was no need to try other alternatives.

**20:48** · But there was curiosity.

**20:51** · And after getting to know Syncthing, setting it up and trying it for a while, I gotta admit: This is a really great software.

**20:57** · Not only for syncing Obsidian files, but so much more, like backing up your Android device for example.

**21:03** · And I like that it’s open source like Bitwarden.

**21:06** · Not that I would ever understand the code when looking into it, but well, it feels good that I could.

**21:12** · Now, compared to Syncthing, the official Obsidian Sync doesn't seem to have much more to offer.

**21:17** · So why pay for it?

**21:18** · Which I would still recommend, by the way, if you have the money to spare.

**21:22** · Is the price, for a sync service, a bit high? Yes.

**21:26** · But is the app Obsidian itself, being free for personal use, kind of underpriced?

**21:32** · Hell yes.

**21:33** · And still, the official Obsidian Sync is the easiest to set up.

**21:37** · As someone who uses Obsidian for several hours every day now I find it more than ok to pay for syncing.

**21:44** · Especially considering that Obsidian itself, in the Obsidian Help Vault, is happy to point out free alternatives.

**21:52** · I think that is a great attitude.

**21:54** · Ok, that’s it for today.

**21:56** · Class dismissed.

**21:58** · No, wait!

**21:59** · On a personal note: Since my last video here on YouTube, my subscriber count has multiplied, not doubled, not tripled, but increased sixfold, which really blew me away.

**22:10** · Therefore: A big hello to everyone new here – I’m glad to have you!

**22:15** · Take a look around, I'm sure there are already a few videos you might enjoy.

**22:19** · And for more, check my Skillshare profile.

**22:22** · I'm currently working on a course that includes Obsidian use cases as well.

**22:27** · So, if you'd like to suggest specific Obsidian topics or questions to be covered in that course or in future videos, just post them in the comment section below.

**22:37** · And to stay up to date, follow me on Instagram and Twitter and subscribe to my newsletter.

**22:42** · Thanks for watching and auf Wiedersehen, see you soon!

**22:49** · But there was curiosity and that’s quite enough for most things that I do in my spare time. However...