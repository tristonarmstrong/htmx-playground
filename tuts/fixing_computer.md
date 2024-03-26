# How to save your penguin when your system wont boot after removing or changing a drive

## So first off, what was my problem?
I had setup my 1TB HDD in `/etc/fstab` and everything was perfect. I could boot my system and BLAM, there it was all nice and mounted. Little did i know, this would come to bite me.

## What happened? 
I was just playing around and testing things when i set up my drive with fstab. I had no idea what that even meant. I knew i planned to eventually dual boot windows on this thing, just didnt know when. Well, today was the day. I reformatted the drive as NTFS and attempted to load up windows on the little fella. Unfortunately that failed misserably, probably due to a bad drive tbh. I said f**k-it and went to boot back into arch. i was tired of fucking with it. I wasted enough time.

I booted...

... wah... what is this?.. 

i get hit with a message saying that arch cant boot, arch cant sign in, and arch is fucked. It tells me to press enter to continue. That didnt do jack diddly squat. What do i do now?

## The solution
I reeeeeeally didnt wanna have to format my usb stick again, but it seems i had no choice. I KNOW that pesky fstab file is the culprit. And, i KNOW i need to change it. I need to delete that dasterdly line i added to auto boot that 1TB drive of mine.

I caved, loaded up arch onto my flashdrive. Slammed it in that sexy blue usb port. And booted. 

what do i do? .... lets see... Oh thats right, i need to mount my drive.

lets go ahead and do that...

clickity clack on the K-B-D...
```bash
fdisk -l
# shows my drives n shit
# pics my drive (nvme0n1)
# partition (p2) 465.3G
```
so thats my drive. I need a place to mount it.
```bash
mkdir /repair # drive mount point 
```
lets mount it 
```bash
mount /dev/nvme0n1p2 /repair
cd /repair
```

THANK GOD it mounted without issues.. Why would it give me issues? hell if i know at this point.

I go into that `/etc/fstab` file, and open it with vim... 
```bash
vim /etc/fstab
#... empty file
```
WTF??? oh come on..... oh wait.. duuuh relative path - me stupid
```bash
vim ./etc/fstab
#... lots of stuff 
```
I see the line.. the line causing me all this pain...
I go to the line in question.... and perform the all mighty `dd` 

i save.. `:w`
it saves.. YES!!, i say
i quit.. `:q`
i reboot.. `reboot`
i wait..

## The results
YES IM IN! Im finally booted back into my beautiful ass KDE Plasma environement. Never thought id see her gorgeous face again!

This was scary for 1 main reason... Im no linux guru. I barely know what im doing half the time. And i break my stuff a lot and usually end up having to reinstall everything.. I did not want to go through that again lol

im sure ill be back with another
__end note__
