---
title: "Psikat: a tracker-like DAW"
description: "What do cats, 1980s, web development, a DAW and a tracker have in common?"
keywords: [music, webapp, tracker, daw, psikat, holoflash, michael palace, wasm, rust, browser daw]
author: Valeria Viana Gusmao
is_draft: true
created_at: 2026-04-19T16:15:24Z
updated_at: 2026-04-22T00:00:00Z
link: https://psikat.com/
cover:
    src: psikat_com.png
    alt: Pixel art cat - logo of psikat, followed by "Browser-based tracker-like sequencer DAW, v0.7.18-alpha, made by holoflash" over the interface of psikat with prominent piano keyboard at the bottom
---

>...I've always known that I've been missing out and the modern day just lacks that special charm...
> -- <cite>Palace, "Back to '85"</cite>

Ghostbusters were released in 1984, the first Back to the Future movie and Microsoft Windows - in 1985; and 1986 has brought us The Legend of Zelda and Castlevania. It was a different era, the one I too experienced only by watching the movies and playing games.

You might be wondering what do cats, 1980s and a musical sequencer have in common.
Well for starters, the song I quoted belongs to an artist known as Michael Palace,
the creator of psikat - the browser-based tracker-like sequencer- is holoflash
and both of them are the same person: Redas Jefisovas.

Why a successful musician decides to become a web developer is a story for another time, but it seems that neither of his passions ever fully took over because psikat has been born of them both.

## Why psikat?
"Psi - as in the greek letter Ψ (psi); kat - as in cat, because I love cats" - said Redas when I asked.

For at least 15 years, before getting into programming, the main way he interacted with computers was using the mouse and a handful of keyboard shortcuts. Dragging around audio clips in Logic Pro, moving around vector points in Adobe Illustrator - all the interfaces he needed to interact with were built around the mouse.

When he made the transition to programming, he was suddenly forced to put both hands on the keyboard and learn how to actually type properly to become productive, and add a lot more keyboard shortcuts to his repertoire.

Time went on, he learned how to do almost everything on the computer without ever touching the mouse. "This meant I could finally be one of those cool cats who can get by with just a laptop, and I got rid of my whole desk setup with external monitors, mice and keyboards." as he put it.

And once the carpal-tunnel-syndrome symptoms were gone, going back felt horrible. He discovered so called *music trackers*, that are in some cases completely keyboard based, with a look and a feel of a hex editor, but so very different from the software he used to create music up until now.

So he needed something in-between Fast Tracker 2:
!["Dead Lock" by Elwood playing on FastTracker II"](https://upload.wikimedia.org/wikipedia/en/4/4b/FastTracker_2_screenshot.png)

And Logic Pro X he was used to:
!["A screenshot of Logic Pro X running on macOS Mojave"](https://upload.wikimedia.org/wikipedia/en/e/e2/Logic_Pro_X_screenshot.png)

He couldn't find one, so he built psikat and the rest is history!

## If only it'd be that easy

Getting started, as Redas shared, wasn't hard: the plan was to start with the basic tracker and then bridge the gap towards the DAW (Digital Audio Workstation). The first goal was to build a "tracker that kinda works like FastTracker 2 and can playback .xm format files". This format has been around long enough to be fully documented and iterated on, so it was trivial to reimplement. Especially with the help of LLM’s.

A downside of an established format is that it usually comes with a certain legacy. And thus he felt that if he would label psikat as a tracker, he’d have to jump through to a lot of hoops to satisfy everyone that’s looking for a "tracker".

As a result he decided to narrow his focus or in his own words: "That’s when I changed the description to tracker-like, deleted all of the code and started from scratch. I would have to make my own decisions to make this thing my own and I can’t be weighed down by trying to support all legacy formats people expect."

## One-Person-Software

The decision to build just for yourself might sound counterproductive: how would you market it, how would the app get users if it made only for one person?!

But if you had been to the rabbit hole of marketing books, you would have discovered that a common advice is to build for a very specific person in mind. They call it ICP (Ideal Customer Profile) and no, it has nothing to do with the age, or geografical location, or gender. The best and most useful description of an ideal customer comes from knowing their beliefs and needs.

When you are your own ideal customer that's gotta make it a lot easier and if noone ever use the software you make - at least you get a custom made software out of it! I think there's another side effect to this "selfish" software-for-the-sake-of-software process: when you don't think about how much money you'd make out of it or how you'd be advertisting it - your software becomes an art piece, a reflection of your own values, thoughts and perspective.

And when it comes to psikat - it became a reflection of its creator's view on the art in general and music in particular.

One of the main distinguishing aspect of tracker programs and an element that is central to psikat - is that the notes/commands are arranged and read vertically, as opposed to horizontally in more mainstream and modern music software. Hence the Ψ (psi) letter, that is often used to denote the Y-axis in Greek mathematics.

So why would someone who used to write music left-to-right suddenly would decide to turn things a quarter down?

Redas says that the inherent difficulty of making music in a tracker __is__ the main feature for him: "If you’re coming from a traditional DAW, you can be sure that it will force you to break out of patterns you’re stuck in and make you see music from a completely different perspective."

## Back to the future with LLMs

Both Michael Palace music and holoflash interfaces have this old-school feeling to it; he compares trackers to vim and DAW to VSCode; so, naturally, I had to ask about his relationship with AI. What place does modern day tools take in his work?

"With AI I’m able to iterate fast and try out many different ideas and avoid becoming a victim to the sunk cost fallacy" - he said, "For example, at some point I started to doubt that I could achieve the performance I wanted using wasm in the browser so I had Claude rebuild the whole thing in SolidJs."

And after doing some research he discovered "the thing that would make the Rust version work as intended" and just as quickly deleted the whole javascript version and went back to wasm.

Redas shared, that the speed it took to try out a completely different stack didn’t leave him feeling like he had wasted his time, but instead helped to try different approaches, remove any doubts and continue with what works.

"Also, working with audio requires a level of mathematic prowess that I simply don’t possess" - he adds, "but it’s an already solved problem that I’m not that excited about re-solving.". AI  allowed him to focus more on the essentials, not the implementation details.

And his essentials are: How does it perform? How does it feel to use? Can I make music with it and does it sound any good?

## Don't build a DAW

Redas shared that his only regret was that he hasn't built psikat sooner:

> People often [advise against building a DAW](https://www.youtube.com/watch?v=GMlnh6_9aTc&t=6s) and with good merit. It’s a very very complex piece of software to build. And based on what people online were saying, I was too scared to even try to start. But you can trick yourself into building a DAW, if you don’t start by building a DAW.

He started by building an audio file browser for the terminal, then one small goal after another psikat came to be as it is today. It's not what you would consider a DAW yet (it lacks recording capabilities), but you can mix a song right there in your browser, like this one, I called it "psikat jam":

<audio controls="1" controlslist="nodownload nofullscreen noremoteplayback" src="psikat_jam.wav" loop><a href="psikat_jam.wav">Click to play</a></audio>

I imagine this could be a soundtrack to something like nyan cat, but way cooler:
!["Nyan cat shooting through the stars leaving a rainbow trail](https://gist.githubusercontent.com/s-shivangi/7b54ec766cf446cafeb83882b590174d/raw/8957088c2e31dba6d72ce86c615cb3c7bb7f0b0c/nyan-cat.gif)

I don't think I fully explored what psikat has to offer: my relationship with my own music is complicated. And if this is what the app can do today, in its alpha version, I can only imagine what v1.0 would do!

I bet it would be intentionally unconventional for a good reason.
Or in the words of its unconventional creator Redas aka Michael Palace aka holoflash:
> If using psikat can help at least one other person be creative and express themselves in their own voice, I’ve accomplished everything I could ever dream of.

I encourage you to try it out at [psikat.com](https://psikat.com) and join [Discord](https://discord.gg/2a7jghQ2W).
It is a great little software.
