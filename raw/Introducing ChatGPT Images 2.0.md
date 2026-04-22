---
title: "Introducing ChatGPT Images 2.0"
source: "https://openai.com/index/introducing-chatgpt-images-2-0/#textmode"
author:
published:
created: 2026-04-22
description: "ChatGPT Images 2.0 introduces a state-of-the-art image generation model with improved text rendering, multilingual support, and advanced visual reasoning."
tags:
  - "clippings"
---
A new era of image generation

<iframe src="https://player.vimeo.com/video/1184900273?h=40b6d658c6&amp;amp%3Bbadge=0&amp;amp%3Bautopause=0&amp;amp%3Bplayer_id=0&amp;amp%3Bapp_id=58479&amp;controls=0&amp;autopause=0" width="100%" height="100%" title="Images 2.0_FINAL from OpenAI on Vimeo" allow="autoplay; fullscreen; picture-in-picture; clipboard-write"></iframe>

Images are a language, not decoration. A good image does what a good sentence does—it selects, arranges, and reveals. It can explain a mechanism, stage a mood, test an idea, or make an argument.

A year ago, we released ChatGPT Images, showing that images created by AI can be both beautiful and useful. ChatGPT Images 2.0 is the next step: a state-of-the-art model that can take on complex visual tasks and produce precise, immediately usable visuals.

This model is a step change in detailed instruction following, placing and relating objects accurately, and rendering dense text, with the ability to generate across aspect ratios. Its sense of composition and visual taste means results feel less AI-generated and more intentionally designed. It’s accurate across languages and uses its expanded visual and world knowledge to fill in the gaps for you, so you get smarter images with less prompting.

To extend the model’s capabilities for the most complex tasks, Images 2.0 is our first image model with thinking capabilities. When a thinking or pro model is selected in ChatGPT, Images 2.0 can search the web for real-time information, create multiple distinct images from one prompt, and double-check its own outputs. With thinking, the model can take on even more of the heavy lifting between idea and image, especially when accuracy, up-to-date information, consistency, and visual cohesion matter most.

With both the intelligence of OpenAI’s reasoning models and a vast understanding of the visual world, this model moves image generation from rendering to strategic design, from a tool to a visual system, helping people turn ideas into outputs they can understand, share, teach with, and build from. It’s available starting today to all users in ChatGPT, Codex, and the API.

### Greater precision and control

Images 2.0 brings an unprecedented level of specificity and fidelity to image creation. It can not only conceptualize more sophisticated images, it actually brings that vision to life effectively, able to follow instructions, preserve requested details, and render the fine-grained elements that often break image models: small text, iconography, UI elements, dense compositions, and subtle stylistic constraints, and at up to 2K resolution in the API. Instead of getting something vaguely in the neighborhood of what you meant, you get something you can actually use.

a screenshot of chatgpt, in a browser, in macosx. the user types "draw me a dog" chatgpt draws an ascii dog the front window is chatgpt, but the desktop is quite messy with lots of random windows open (e.g. a terminal). they're all in the background

![images-2-ascii](https://images.ctfassets.net/kftzwdyauwt9/2NrxFDyv49oLAbaFtaLzD8/ca488767d16a07fbe4151ae905717458/screenshot.png?w=1920&q=90&fm=webp "images-2-ascii")

### Stronger across languages

To date, our image generation models have been more consistent in English and other Latin-script languages, but less precise beyond that, especially when text was complex or dense.

Images 2.0 moves beyond that barrier with stronger multilingual understanding and significant gains in non-Latin text rendering, particularly in Japanese, Korean, Chinese, Hindi, and Bengali. It can produce images with non-English text that’s not only rendered correctly but with language that flows coherently.

That includes not just translating a label or two, but generating visually coherent outputs where language is part of the design itself, from posters and explainers to diagrams and comics. This makes the model more globally useful and helps people create visuals that work in the languages they actually use.

Make a sample page of a colorized Japanese shonen adventure manga. The page should vividly depict our main character found a magical quill. The name of the quill is called the Quill of GPT Image. Make it dramatic. The magical quill has strong power sealed inside it.

Additional instructions: Aspect ratio: Portrait 1440x2560. The pen should have an OpenAI logo on it. The language throughout the manga should be Japanese. Think carefully first to make this a good story with good split of manga panels. The page should appear as a photo of a physical page, not a digital page.

![japanese](https://images.ctfassets.net/kftzwdyauwt9/3qjs0ayPyz6j281GtxjEaH/ad25ae8c0b33164f65604070a02e2725/japanese.png?w=1200&q=90&fm=webp "japanese")

### Stylistic sophistication and realism

Images 2.0 also shows significantly improved fidelity across a wide range of visual styles. It is better able to capture the defining characteristics of photos—including the tiny flaws that add realism—as well as cinematic stills, pixel art, manga, and other distinctive visual languages, with greater consistency in texture, lighting, composition, and fine detail.

As a result, the model can produce outputs that more faithfully reflect the style requested, rather than approximating it. This is especially useful for game prototyping, storyboarding, marketing creative, and creating assets in a particular medium or genre.

#### Photorealism

A photorealistic candid travel scene of a person standing at a coastal roadside turnout on an overcast morning, shot on 35mm film. Natural imperfect framing, visible grain, ambient light, muted colors, wind in clothing and hair, cinematic realism, and the feeling of a lived-in documentary photograph.

![images-2-candid-people-1](https://images.ctfassets.net/kftzwdyauwt9/6xvpGcclxpKI8LxTBvsrRB/fa483614f27623d3bee1bd3fdb35bc67/images-2-candid-people-1.png?w=1920&q=90&fm=webp "images-2-candid-people-1")

A photorealistic snapshot portrait of two friends outside a venue at night, shot on a compact point-and-shoot camera with direct flash. Close subject distance, crisp foreground detail, deep shadow falloff, slightly raw spontaneous energy, nightlife atmosphere, and the unmistakable look of an early-2000s flash photograph.

![images-2-candid-people-2](https://images.ctfassets.net/kftzwdyauwt9/4dwQqstABTMhoGAGJFrf3Q/4e9f462c9223275697ae7903a5fc6ec8/images-2-candid-people-2.png?w=1920&q=90&fm=webp "images-2-candid-people-2")

#### Styles

a page of a comic book in the style of Japanese Seinen manga

![images-2-Seinen](https://images.ctfassets.net/kftzwdyauwt9/481LFZ4eKWcrfwFGogeFzD/45da93bffa9ba79636cb81d4d14726df/shonenfinal.png?w=1200&q=90&fm=webp "images-2-Seinen")

### Flexible aspect ratios

The new model also gives you more flexibility in how those images are delivered. With support for aspect ratios as wide as 3:1 and as tall as 1:3, Images 2.0 can generate outputs that are ready to fit the formats you need, from wide banners and presentation slides to posters, mobile screens, bookmarks, and social graphics. Ask for the aspect ratio you want in the prompt, or select from preset options to regenerate any image in new dimensions.

"japanese-manga-style disassembly" of a basketball dunk shoot motion like a time lapse. Tell the most story through visuals rather than text. 3:1 utlrawide aspect ratio. prefer light background rather than dark. do not use japanese

![images-2-manga-style disassembly](https://images.ctfassets.net/kftzwdyauwt9/67ga0IwEhNItMLL3zpROQw/e57b9fd02749e9d815871ac9ca79cbc3/images-2-manga-style_disassembly.png?w=1920&q=90&fm=webp "images-2-manga-style disassembly")

### Real-world intelligence

Images 2.0 brings a more up-to-date understanding of the world into image creation, with a knowledge cutoff of December 2025, for more relevant and contextually accurate outputs. This is especially important for artifacts like explainers, educational graphics, and visual summaries, where correctness and clarity matter just as much as aesthetics.

Its intelligence allows it to expertly handle tasks end-to-end: synthesizing information, writing the story, and laying it out with clean structure, intentional whitespace, and strong visual flow.

cantor's diagonalization proof, infographic

![images-2-cantor](https://images.ctfassets.net/kftzwdyauwt9/6467MdNRAwtJKPXL775AYs/4b80480b239e35d4f901f03ea6c68c4e/images-2-cantor.png?w=1200&q=90&fm=webp "images-2-cantor")

### A visual thought partner

When a **thinking model** is selected in ChatGPT, the model takes more time and does more agentically behind the scenes to thoroughly understand and execute the task. It can use the web to find relevant information, transform uploaded materials into clear visual explainers, and reason through the structure of the image before generating. In this mode, Images 2.0 acts more like a **visual thought partner,** helping carry a project from rough concept to finished asset with significantly less work on your part.

With thinking, it can also produce multiple distinct images at once, a first for image generation in ChatGPT. That opens up workflows that were previously cumbersome: a sequence of manga pages, a set of redesign directions for every room in a house, a family of poster concepts, or a collection of social graphics in different aspect ratios and languages.

Instead of prompting one image at a time and stitching the project together yourself, you can ask for a coherent set of **up to eight outputs in one go** with character and object continuity, that sequentially build on one another.

Make an advertisement promoting my new matcha shop called 'kizuki' opening in brooklyn heights. have a nice sunlight image of a strawberry matcha (iced) and a streetwear aesthetic w japanese minimalism. make sure to include multiple aspect ratio outputs so i can use it on twitter, IG stories, IG feed, and linkedin.

![imagegen call 000 image 000](https://images.ctfassets.net/kftzwdyauwt9/5EfRow9gTQXv2IQXl7iSca/76a78c9e4dcb80459d08a92733f82691/imagegen_call_000_image_000.png?w=1200&q=90&fm=webp "imagegen call 000 image 000") ![imagegen call 001 image 002](https://images.ctfassets.net/kftzwdyauwt9/58nJ56zIF7MdCHlmWe1oPF/89ed5c8d4c0b6c5cf142203393f7267b/imagegen_call_001_image_002.png?w=1200&q=90&fm=webp "imagegen call 001 image 002") ![imagegen call 001 image 000](https://images.ctfassets.net/kftzwdyauwt9/1eCfPBhIOe2WY3E1EN5KSm/d3461b33adbc754b2576fd38ac110211/imagegen_call_001_image_000.png?w=1200&q=90&fm=webp "imagegen call 001 image 000") ![imagegen call 001 image 001](https://images.ctfassets.net/kftzwdyauwt9/13OzIXVn60jxMA1eIXRY63/f77db9f979f2dbc9e89163ccfd839c80/imagegen_call_001_image_001.png?w=1200&q=90&fm=webp "imagegen call 001 image 001")

### Using image generation in Codex

Images in Codex brings visual creation into one workspace for creating, iterating, and shipping apps, slide decks, and other work, making Codex more useful for broader tasks across design, marketing, product, sales, and learning & development.

For example, you can generate multiple UI directions, concepts, and prototypes, compare options quickly, and then turn the strongest ideas into live products or website experiences without leaving the Codex app. You can create images in Codex with your ChatGPT subscription without creating a separate API key.

### Build images into your product with gpt-image-2 in the API

Developers and businesses can bring these same capabilities into the products they’re developing to the API through **gpt-image-2** —adding high-quality image generation and editing to the workflows they already serve.

With stronger text rendering, multilingual generation, improved instruction following, and support for more output formats and aspect ratios, the API makes it easier to build image workflows for real business use cases: localized advertising, infographics, explainers, educational content, design tools, creative platforms, and web creation products.

Here’s what customers are already saying about using gpt-image-2 in production workflows, from visual storytelling and design software to website creation and creative automation:

##### “What surprised us most was the detail GPT Image 2 added. It introduced elements we hadn’t considered, like a “viral on TikTok” sticker—a smart creative choice designed to build hype. The model wasn’t just rendering images. It was interpreting briefs, understanding audiences, and making creative decisions behind the scenes. We’ve been measuring AI on technical outputs. The real shift is creative reasoning and design taste—and that shift just happened.”

— Dwayne Koh, Creative Strategist, Canva

---

A fashion celebrity is launching a new makeup glow lip balm. The target audience is 14 to 30. Imagine an ad for this audience. Must be trendy and appeal to the age group. Looks like a professional photo shoot. Model must be cool, hip, and stylish. Make the design clean aesthetics. Make it look premium luxury.

![canva](https://images.ctfassets.net/kftzwdyauwt9/58EmRlebR2Dj0097IfRkqw/cce9ce9b7db230cf54f07baecb4aa74d/canva.png?w=1920&q=90&fm=webp "canva")

### Limitations

ChatGPT Images 2.0 is a major step forward, but it is not perfect. It can still struggle with tasks that require a complete and coherent physical world model, origami guides, puzzles like Rubik’s Cubes, and details that need to appear correctly on hidden, angled, or reversed surfaces, very dense or repetitive visual details, like fine grains of sand, may also test the limits of the model. Labels and diagrams may still need review for accuracy, especially when they rely on precise arrows or part labels. We see these limitations as important frontiers for future work.

In the API, outputs over 2K are currently in beta and may produce inconsistent results in some cases.

### Pricing & availability

ChatGPT Images 2.0 is available starting today to all ChatGPT and Codex users. Advanced outputs with thinking are available to ChatGPT Plus,Pro, and Business users.

The gpt-image-2 model is available in the API; [pricing ⁠](https://openai.com/api/pricing/) varies depending on the selected quality and resolution of the image.

We’re building image generation to be useful, creative, and safe. That means taking an end-to-end approach to safety: working to prevent harmful outputs, implementing robust safeguards, and continuously strengthening our protections as capabilities and risks evolve. You can read more about our approach in the system card [here ⁠](https://deploymentsafety.openai.com/chatgpt-images-2-0).

OpenAI