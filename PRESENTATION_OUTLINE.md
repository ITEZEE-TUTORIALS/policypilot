# PolicyPilot Visual Deck

## Creative Direction

- Theme: editorial, modern, and quietly technical
- Mood: calm confidence, not "AI demo" neon
- Palette:
  - Ink: `#101827`
  - Paper: `#F7F4EE`
  - Sage: `#7A9B8F`
  - Copper: `#B06A4A`
  - Slate: `#5E6B78`
- Type pairing:
  - Headings: a refined serif such as `IBM Plex Serif` or `Fraunces`
  - Body: a clean sans such as `Inter` or `Source Sans 3`
- Visual language:
  - thin rules,
  - large margins,
  - cards with gentle shadow,
  - one strong accent color per slide,
  - simple diagrams instead of dense bullet lists

## Slide 1: Title

Layout:
- Full-bleed paper background with an ink-colored title block
- Small accent line in sage
- Right side: a simple illustrated workflow ribbon

On-slide:
- PolicyPilot
- An Open-Book Policy Assistant in Rust
- Internal policy questions, answered from source

Speaker notes:
- Open with the promise that the assistant reads before it answers.
- Set the tone as practical and trustworthy rather than flashy.

Visual cue:
- A clean document icon feeding into a speech bubble

---

## Slide 2: The Problem

Layout:
- Split screen
- Left: stacked policy pages
- Right: a user trapped in search results

On-slide:
- Policy docs are long
- Answers are buried
- Guessing creates risk

Speaker notes:
- Show that the problem is not intelligence, it is retrieval friction.
- Make the pain feel familiar and non-technical.

Visual cue:
- A page stack with highlighted lines that are hard to find

---

## Slide 3: RAG in One Sentence

Layout:
- Centered statement on a clean card
- Supporting line below
- Minimal iconography

On-slide:
- Look it up first.
- Answer second.
- Stay grounded in source material.

Speaker notes:
- Use the librarian analogy.
- Keep this slide simple and memorable.

Visual cue:
- Library shelf icon on one side, answer bubble on the other

---

## Slide 4: The Pipeline

Layout:
- Horizontal flow diagram across the slide
- Each stage inside a rounded card
- Thin connector arrows between stages

On-slide:
- Ingest
- Chunk
- Embed
- Store
- Retrieve
- Answer

Speaker notes:
- Walk left to right.
- Each stage should feel like a handoff, not a black box.

Visual cue:
- A conveyor-belt style pipeline with distinct nodes

---

## Slide 5: Where Rust Helps

Layout:
- Two-column layout
- Left: benefits list
- Right: a "machine shop" metaphor illustration

On-slide:
- Memory safety
- Speed
- Maintainability
- Easy deployment

Speaker notes:
- Rust is the part that makes the demo feel engineered instead of improvised.
- Mention the single-binary advantage as a practical detail.

Visual cue:
- Precision tools, not generic AI sparkles

---

## Slide 6: Where RIG Helps

Layout:
- Orchestrator motif
- Center: conductor figure or baton-style diagram
- Surrounding nodes: retrieval, prompt, tools, answer

On-slide:
- Orchestrates the flow
- Keeps prompts and retrieval organized
- Makes the architecture easier to explain

Speaker notes:
- RIG is the coordination layer.
- It does not replace the pipeline, it gives it structure.

Visual cue:
- Conductor baton connecting the workflow nodes

---

## Slide 7: Demo Question

Layout:
- Large quote bubble at center
- Smaller policy card underneath
- Side note showing the retrieval match

On-slide:
- Open the browser UI
- Ask: "Can I expense a hotel minibar?"
- Show the retrieved policy text and grounded answer
- Keep the conversation visible as a chat history

Speaker notes:
- Pause after the question to build anticipation.
- Let the audience see the source before the answer appears.
- Keep the browser visible while you point to the answer, excerpt, and sources.
- Use the growing chat history to show that the assistant is answering in context, not as a one-off lookup.

Visual cue:
- A highlighted excerpt from the expense policy

---

## Slide 8: Grounded Answer

Layout:
- Top: final answer in a prominent card
- Bottom: source excerpt with a highlight
- Side margin: source metadata tags

On-slide:
- Final answer
- Relevant excerpt
- Source reference

Speaker notes:
- Point to the excerpt first.
- Then point to the answer and show that it is justified.

Visual cue:
- Highlighted policy sentence with a small citation tag

---

## Slide 9: Why It Matters

Layout:
- Three large metric-style cards
- One word or short phrase in each

On-slide:
- Less hallucination
- More trust
- Faster answers

Speaker notes:
- Keep this practical.
- The benefit is not abstract AI progress, it is better day-to-day decisions.

Visual cue:
- Clean checkmarks, not flashy charts

---

## Slide 10: Closing

Layout:
- Full-width closing statement
- Small footer with project name and thesis

On-slide:
- PolicyPilot turns policy docs into reliable answers.
- Rust keeps the system solid.
- RAG keeps the answer grounded.

Speaker notes:
- End with the contrast: memory versus source material.
- Strong final line: "We are not teaching the model to know more. We are teaching it to look first."
