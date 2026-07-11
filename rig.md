# RIG in PolicyPilot

RIG is the orchestration layer in this project.

If RAG is the act of finding the right policy text before answering, RIG is the layer that packages that evidence into a clean, repeatable handoff.

In PolicyPilot, that means RIG does not replace retrieval or answer generation. It coordinates them.

## Why RIG matters here

- It keeps the demo easy to explain.
- It separates grounded retrieval from prompt construction.
- It turns the retrieved evidence into a copy-paste prompt sample and a cURL example.
- It gives the UI a clean `Use AI` panel without hard-coding prompt logic in the browser.
- It supports a fallback to the full policy bundle when retrieval confidence is low.

That makes RIG useful both technically and narratively. It gives the project a second layer beyond RAG: not just "find the evidence," but "prepare the evidence for the next model or tool."

## The orchestration flow

```text
User question
  -> load policy docs
  -> chunk documents
  -> embed the question
  -> retrieve matching chunks
  -> draft a grounded answer
  -> build the RIG handoff
  -> show prompt sample + cURL in the UI
```

### What each step does

- `src/app.rs` runs the shared pipeline.
- `src/answer.rs` picks the best grounded policy line and writes the response.
- `src/rig.rs` decides whether to use a focused excerpt or the full policy bundle.
- `src/rig.rs` also builds the system prompt, user prompt, combined prompt text, and cURL command.
- `src/web.rs` returns that orchestration payload through the API.
- `assets/ui/app.js` renders the `Use AI` accordion so the user can copy the prompt or the cURL block.

## Simple analogy

Think of the project like a newsroom:

- RAG is the reporter who gathers the facts.
- Rust is the press room that keeps the workflow reliable.
- RIG is the assignment editor who packages the right story, source notes, and handoff instructions.

Without RIG, the demo would still answer questions. With RIG, the demo shows how a grounded answer becomes a portable workflow.

## Value to the project

RIG adds three things that are worth presenting:

1. It makes the system feel complete, not just functional.
2. It shows how a Rust app can orchestrate AI work without hiding the steps.
3. It gives the audience a concrete artifact they can copy into another tool.

That last point matters in a presentation. The audience can see the exact evidence used, the prompt that would be sent, and the cURL command that would call an API directly.

## Low-confidence behavior

PolicyPilot uses a simple rule:

- If retrieval is strong, the prompt pack stays focused on the best excerpt.
- If retrieval is weak, the prompt pack expands to the full policy bundle.

That is the practical value of orchestration. RIG is not just formatting text. It is making a judgment about how much context the next stage should receive.

## Files involved

- [`src/answer.rs`](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/answer.rs)
- [`src/rig.rs`](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/rig.rs)
- [`src/web.rs`](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/web.rs)
- [`assets/ui/app.js`](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/assets/ui/app.js)

## One-line summary

RIG in PolicyPilot is the layer that turns retrieved policy evidence into a reusable AI handoff, so the demo shows not only grounded retrieval but also how the next model or tool would be driven.
