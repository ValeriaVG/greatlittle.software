---
title: "Crit: Your feedback loop with the agent"
description: Is there a middle ground between vibe-coding and writing code entirely by hand?
category: devtools
keywords: [ai, agent, devtools]
is_draft: true
created_at: 2026-06-12T14:52:31Z
cover: 
  src: screenshot.png
  alt: "Code Review, But for AI. — crit.md promotional image showing a code review interface with a 'Quality Improvement Plan' document analyzing 21 pull requests. The interface displays an executive summary about automated tooling and CLAUDE.md agent prompt rules. Tagline: 'The workflow your agent is missing' and 'Your feedback loop with the agent."
product:
  name: Crit
  cover: cover.png
  blurb: Your feedback loop with the agent
  actions:
    - label: Try Crit
      url: https://crit.md
    - label: Star on GitHub
      url: https://github.com/tomasz-tomczyk/crit
---
> I want it to be lean - it should work with YOU. It works with your IDE, it works with your issue tracker, it doesn't try to change how you work, it just makes it easy to provide feedback and iterate on the output.
> -- <cite>Tomasz Tomczyk</cite>

Just some years ago we, software developers, were fighting holywars on React vs. Vue vs. Angular, JavaScript vs TypeScript, Chrome vs Firefox and countless others. I can't say the battles have died out, but nothing, in my opinion, compares to the epic confrontation of AI vs no-AI camps.

On one side of the fence we have people who claim to be the evolution, intentionally atrophying their ability to produce code by hand and using their AI provider bills as a measuring stick. On the other, we have people who deny AI advancements and brand anyone using LLMs a heretic. This article is for neither. It is for the folks in between, who, just like me, are trying to find a footing in the new reality.

And I think Tomasz Tomczyk, creator of Crit, have a good amount of insight to share on the matter.

## LGTM = TLDR

"I use AI to generate plans a lot - and reviewing those in the terminal was painful," shared Tomasz. "It's hard leaving granular comments (e.g. 'for X workflow, change A & B...') - it would take a lot of scrolling up and down, copy pasting, writing comments. Accidentally hitting enter would send it before I was actually finished. Speaking to peers, they had similar experiences - and often resolve to approving plans without reading them thoroughly and seeing subpar results."

Indeed, AI generates a lot of code and fast, definitely a lot faster than a human can read, let alone understand.
I too often resorted to blind vibecoding (and then throwing the whole project away) just because reviewing that amount of slop was too much work. And yet writing code entirely by hand felt like refusing to accept autocomplete and linters and going back to the embarrassing early days when I thought that real developers should only use Notepad.

Instead of jumping between two extremes, Tomasz seem to have been determined to find the middle ground. On February 16, 2026, he pushed this commit:
> Single-binary Go CLI for reviewing markdown files with inline comments.
Browser-based UI with GitHub PR-style commenting, syntax highlighting,
mermaid diagram support, dark/light themes, and real-time .review.md output.

![crit.md code review interface showing notification-plan.md with a design decisions table. The table compares options for Queue (Redis Streams chosen over SQS, RabbitMQ), Delivery guarantee, Template engine, and Storage. User @Tomasz comments 'Just use SQS - we're in AWS' on line 20. Sidebar shows document contents including Notification Service, Design Decisions, Database Schema, API, and Worker Design sections](./planreviewer.png)

And that was the beginning of Crit, though for a few hours it has been called "PlanReviewer".

## "Vibecoded" go-to tool

Crit quickly became a go-to tool for Tomasz himself, his friends and colleagues, and started to spread. At the moment of writing this article, Crit repository has almost 500 stars on GitHub, just four months after its conception.

It's worth noting that Tomasz, while having over two decades of engineering experience under the belt, in his daily work wields Elixir, not Go. In the interview he gave to the "Cup o' Go" podcast hosts he honestly said that he went with Go because that's what the coding agent recommended for CLI. He didn't know Go at all. Yes, his tool was so good, that he got invited to talk about it on a podcast dedicated to Go developers regardless.

So what's the secret?

In its creator's own words: "Once this workflow became my day to day driver, I wanted to mimic it for other parts of my job: reviewing code, reviewing running site. Turns out it's a very transferable UX/DX, commenting on plans, code diffs, dev site. Point and click, leave a comment, iterate. It became fun again and allowed me to improve what the output looked like without making me hate the process."

![crit.md preview interface showing preview.html with a feature comparison table and '03 Lifecycle of an invite' diagram. The table lists org admin capabilities like 'Remove a member' with YES/NO checkboxes across two columns. Below, a five-stage invite lifecycle flow shows: Issued → Delivered → Waiting → Accepted → Expired. Right sidebar displays 1 open comment from @Tomasz Tomczyk: 'make org settings its own line.' Browser toolbar shows Mobile/Tablet/Desktop/Fit view options.](./htmlreviewer.png)

From my point of view, Crit was able to offer developers something very valuable: the way to go back to the flow - a magical state of ultra productivity and creative joy that has been fractured for many ever since the advent of AI agents.

## AI != Easy

Writing code has never been easier, but agents can get you only this far. To quote Tomasz: "AI-driven development gets you 80% of the way very quickly and it looks impressive, but that last 20% is extremely crucial for good UX." It's one thing to spin up a prototype - a refined product requires a lot of iterations with or without AI.

And while Tomasz trusted code generation and stack choices to the agents, product philosophy and vision belonged entirely to the human behind the wheel:

> The main thing I'm being pushy on is the DX: I don't want to remember many commands (/crit is a context-aware skill) and I want it to be lean - it should work with YOU. It works with your IDE, it works with your issue tracker, it doesn't try to change how you work, it just makes it easy to provide feedback and iterate on the output.

![crit.md diff view of test-plan-copy.md showing API security change: replacing 'No authentication required on the internal network' with 'Requires X-Internal-Token header; requests without it are rejected with 401.' JSON payload example for POST /notifications/send endpoint visible below.](./diff-split.png)

Crit supports pretty much any agent you can think of and for the outliers, like myself, it has an easy step-by-step tutorial to how to port it to anything else.

## The Future

When I asked Tomasz about what he would like to achieve with the project, he said that even if the project stays as it is - he wouldn't mind it because the fact that his creation is used daily and loved is already a huge success. Of course, it doesn't mean that he can't dream big! 

Looking back, Tomasz says that he'd consider more elaborate frameworks if he'd do it all over again: "What started as a simple app for reviewing just plans now functions in 4 different review modes and with a multiplayer option on the web. My agents are still telling me it's fine though!"

And they are absolutely right! But even they can't tell what the future holds. Tomasz dreams of Crit becoming the GitHub of plans: a place where teams are sharing, reviewing and iterating on product requirement documents, architecture decision records, specifications and so on.

"For the local app," he adds, "I'd be interested in exploring a world where it's more of a kanban style work orchestrator, kinda like https://www.conductor.build/, but it'd be a huge scope increase so not sure about it!"

I admire Tomasz's ability to keep himself grounded in reality and to not accept the status quo. 
Give Crit a try, see if it fits into your workflow.
It's a great little software.
