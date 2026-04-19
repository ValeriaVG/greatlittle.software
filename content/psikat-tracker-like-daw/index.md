---
title: psikat-tracker-like-daw
description:
keywords: []
is_draft: true
created_at: 2026-04-19T16:15:24Z
link: https://psikat.com/
---
First, just a minor detail - it’s just psikat  (all lowercase, because it feels cooler somehow :P).
And a tiny explanation about the name it self:

psi - as in the greek letter Ψ (psi);  kat - as in cat, because I love cats.

One of the main distinguishing aspect of tracker programs and an element that is central to psikat - is that the notes/commands are arranged and read vertically, as opposed to horizontally in more mainstream and modern music software. The Ψ (psi) letter is often used to denote the Y-axis in Greek mathematics .
——

•⁠ ⁠What motivated you to build it? (you told me a bit about the nostalgic look and sound, but I don't want to mess up some details)
—
First a little backstory.
For at least 15 years, before getting into programming, the main way I interacted with computers was using the mouse and a handful of keyboard shortcuts. Dragging around audio clips in Logic Pro, moving around vector points in Adobe Illustrator - all the interfaces I needed to interact with were built around the mouse.

When I made the transition to programming, I was suddenly forced to put both hands on the keyboard and learn how to actually type properly to become productive, and add a lot more keyboard shortcuts to my repertoire.

As time went on, I learned how to do almost everything on the computer without ever touching the mouse.
This meant I could finally be one of those cool cats who can get by with just a laptop, and I got rid of my whole desk setup with external monitors, mice and keyboards.

By the type the carpal-tunnel-syndrome symptoms of using the Magic Mouse for years were gone, going back to the aforementioned mice based interfaces felt horrible. This is where I discovered music trackers that are in some cases completely keyboard based, and most of them feel and look like a code editor - but so far removed from the way I’ve learned to produce music that I’d need a few years to become comfortable using them.

So to finally answer the question I wanted a music program that is somewhere in-between a tracker like FastTracker 2 and a traditional DAW (what I’m used to) like Logic Pro X, and there was nothing out there that worked exactly the way I want it to. So I decided to start building my own

•⁠ ⁠What was easy and what was unexpectedly hard in the process of building it?
—
The easy part was getting started. My starting point was building a basic tracker that kinda works like FastTracker 2 and can playback .xm format files. This format has been around long enough to be fully documented and iterated on, so it was trivial to reimplement. Especially with the help of LLM’s.
Once I had that in place I could start to think about how I want to start bridging the gap between modern DAWs and trackers, starting from the tracker side of it.

The unexpectedly hard part was that I felt pressure to respect the legacy of the .xm format and trackers in general and felt that if I want to label psikat as a tracker, I’d have to jump through to a lot of hoops to satisfy everyone that’s looking for a tracker.

That’s when I changed the description to tracker-like, deleted all of the code and started from scratch. I would have to make my own decisions to make this thing my own and I can’t be weighed down by trying to support all legacy formats people expect.
•⁠ ⁠Building alone means you're the only one making all the decisions, from tech stack to color palette to marketing. What decisions were non-negotiable for you?
—
All decisions are non-negotiable at this point. I’m almost the only person using the program right now and I first need to make sure that it does what I need it to do. I have thus far only shared the program with other developers and am waiting to share it with musicians, to avoid colouring my own opinions too much.
Once all the base features are in place, I’ll start to be more concerned about making it useful for others too. But only to a certain point.

The inherent difficulty of making music in a tracker IS the main feature for me. If you’re coming from a traditional DAW, you can be sure that it will force you to break out of patterns you’re stuck in and make you see music from a completely different perspective. Once more people start using it, I will face the new challenge of trying to accommodate users who will undoubtedly have different needs while avoiding feature-creep and ruining the program for myself. Let’s see how that went next time we speak!
•⁠ ⁠How did AI fit into your process, if at all?
—
With AI I’m able to iterate fast and try out many different ideas and avoid becoming a victim to the sunk cost fallacy. For example, at some point I started to doubt that I could achieve the performance I wanted using wasm in the browser so I had Claude rebuild the whole thing in SolidJs.

After doing some research I discovered the thing that would make the Rust version work as intended and just as quickly deleted the whole javascript version and went back to wasm.

The speed it took to try out a completely different stack didn’t leave me feeling like I had wasted my time but just helped settle a difficult question and remove doubts so I can continue to focus on what works.

Also, working with audio requires a level of mathematic prowess that I simply don’t possess but it’s an already solved problem that I’m not that excited about re-solving. AI helps a lot with that, so I can focus more on the essentials. How does it perform? How does it feel to use? Can I make music with it and does it sound any good?
•⁠ ⁠What do you wish you'd done differently with the knowledge you have now?
—
Honestly, I only wish I started this project earlier. People often advise against building a DAW ( https://www.youtube.com/watch?v=GMlnh6_9aTc&t=6s ) with good merit. It’s a very very complex piece of software to build. And based on what people online were saying, I was too scared to even try to start.
But you can trick yourself into building a DAW, if you don’t start by building a DAW.

I started by building an audio file browser for the terminal. That was my only goal. Then once that goal was achieved it was followed by more equally small goals. psikat is not what you would consider a DAW yet (doesn’t have audio recording capabilities), but a few more goals later - it just might be. Just google “how to eat an elephant”.

•⁠ ⁠What's your wildest dream for the app?
---
If using psikat can help at least one other person be creative and express themselves in their own voice, I’ve accomplished everything I could ever dream of.
---
Add link to discord
