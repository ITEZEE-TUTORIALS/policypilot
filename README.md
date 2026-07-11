# PolicyPilot
### A Rust RAG Demo for Internal Policy Questions

> Presentation title: **PolicyPilot: An Open-Book Policy Assistant in Rust**

PolicyPilot is a beginner-friendly RAG project built around a simple internal knowledge problem: people constantly ask policy questions that already have answers in handbooks, travel guides, or expense rules.

Instead of asking a model to remember every rule, PolicyPilot looks up the right policy first, then drafts a grounded answer.

This repository includes:

- a compileable Rust prototype,
- a browser-based UI demo,
- markdown policy documents for retrieval,
- downloaded GitLab handbook source pages under `data/sources/gitlab/`,
- a merged project glossary with data and concept analogies,
- a slide-by-slide deck outline,
- a single runner script for launching the app,
- and a presentation-friendly alternate framing if you want to compare audiences.

---

## Why this topic works

- Everyone understands the pain of searching policy docs.
- The questions are concrete and easy to follow.
- The RAG pipeline is visible without being overwhelming.
- Rust gives the demo a clean, production-minded feel.
- RIG can be introduced as the orchestration layer without stealing the spotlight.

### Good audience example

Imagine a user asking:

> "Can I expense a hotel minibar?"

PolicyPilot should retrieve the relevant policy text, then answer with the correct rule instead of guessing.

---

## RAG in plain English

Think of RAG like asking a librarian instead of a fortune teller:

- The policy docs are the library.
- Chunking is turning long documents into searchable pages.
- Embeddings are the meaning fingerprints on each page.
- The vector store is the catalog.
- Retrieval is the librarian finding the most relevant pages.
- The LLM is the writer who turns those pages into a clean answer.

So the model is not making things up from memory. It is checking the source first.

---

## What PolicyPilot demonstrates

1. Load policy documents from markdown files.
2. Split the docs into chunks.
3. Embed each chunk into a numeric vector.
4. Store the vectors in an in-memory vector store.
5. Search by semantic similarity.
6. Generate an answer from the retrieved context.
7. Keep the response grounded with source references.

---

## Why Rust is a strong fit

Rust is a good fit for RAG demos because it gives you:

- speed,
- memory safety,
- clear types,
- easy parallelism,
- and a single binary that is simple to ship.

For a presentation, that matters. The demo feels reliable instead of fragile.

### Simple analogy

If Python is a flexible workshop, Rust is a precision machine shop.

That is useful when your pipeline has multiple moving parts and you want fewer surprises.

---

## Where RIG fits

If you use RIG, think of it as the conductor.

- Rust provides the instruments.
- RIG coordinates the steps.
- Retrieval, prompting, and tool use stay organized.

That makes it easier to explain the system without hiding the engineering.

---

## Architecture overview

```text
              +-------------------+
              | Policy Documents  |
              +---------+---------+
                        |
                        v
              +-------------------+
              | Parse + Chunk     |
              +---------+---------+
                        |
                        v
              +-------------------+
              | Embedding Model   |
              +---------+---------+
                        |
                        v
              +-------------------+
              | Vector Store      |
              +---------+---------+
                        |
                        v
User Question -> Retrieval -> Answer Builder -> Final Response
```

### In one sentence

RAG is the process of finding the right facts before generating the answer.

---

## Demo story arc

Use this sequence during the presentation:

1. Start with the policy question.
2. Explain why guessing is risky.
3. Introduce RAG as open-book AI.
4. Walk through ingestion, chunking, embeddings, retrieval, and generation.
5. Show how Rust keeps the system dependable.
6. Show where RIG can organize the workflow.
7. End with a live grounded answer.

---

## Suggested project scope

Keep the first version focused:

- Input: policy and handbook docs.
- Output: answers with citations or source references.
- Interface: CLI or lightweight web API.
- Retrieval: semantic top-k search.
- Generation: short answers with supporting excerpts.

That is enough to demonstrate the idea without getting bloated.

---

## What is in this repo

- [src/main.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/main.rs) wires the demo together.
- [src/web.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/web.rs) serves the browser UI.
- [src/app.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/app.rs) runs the shared answer pipeline.
- [src/ingest.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/ingest.rs) loads the policy documents.
- [src/chunk.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/chunk.rs) splits docs into chunks.
- [src/embed.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/embed.rs) turns text into vectors.
- [src/store.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/store.rs) keeps the in-memory vector store.
- [src/retrieve.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/retrieve.rs) searches for the best matches.
- [src/answer.rs](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/src/answer.rs) formats the grounded response.
- [data/sources/gitlab/global_travel_expense.html](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/data/sources/gitlab/global_travel_expense.html) is the downloaded handbook page.
- [data/sources/gitlab/travel_safety.html](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/data/sources/gitlab/travel_safety.html) is the downloaded travel safety page.
- [data/gitlab/global_travel_expense.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/data/gitlab/global_travel_expense.md) is the cleaned policy excerpt used by the demo.
- [data/gitlab/travel_safety.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/data/gitlab/travel_safety.md) is the second source excerpt used by the demo.
- [PROJECT_GLOSSARY.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/PROJECT_GLOSSARY.md) explains both the data artifacts and the core Rust and RAG concepts with analogies.
- [PRESENTATION_OUTLINE.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/PRESENTATION_OUTLINE.md) is the slide deck draft.
- [SPEAKER_NOTES.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/SPEAKER_NOTES.md) is the presenter cheat sheet.
- [ALTERNATE_TOPIC.md](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/ALTERNATE_TOPIC.md) gives a contrasting manual-support framing.
- [run.sh](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/run.sh) launches the whole app.
- [assets/ui/index.html](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/assets/ui/index.html) is the static UI shell.
- [assets/ui/styles.css](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/assets/ui/styles.css) holds the simplified single-screen chat styling.
- [assets/ui/app.js](/Users/lakefront/Desktop/Dev/Rust/rust-bigdata-manager/assets/ui/app.js) drives chat history, retrieval requests, compact evidence, optional thinking trace, copy, and transcript export.

---

## Recommended repo structure

```text
policypilot/
├─ README.md
├─ SPEAKER_NOTES.md
├─ PRESENTATION_OUTLINE.md
├─ ALTERNATE_TOPIC.md
├─ PROJECT_GLOSSARY.md
├─ run.sh
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  ├─ app.rs
│  ├─ web.rs
│  ├─ ingest.rs
│  ├─ chunk.rs
│  ├─ embed.rs
│  ├─ store.rs
│  ├─ retrieve.rs
│  └─ answer.rs
├─ data/
│  ├─ gitlab/
│  └─ sources/gitlab/
└─ assets/
```

---

## One-line summary

PolicyPilot is a clean Rust RAG demo that shows how policy docs can become reliable, cited answers.

## Run The UI

```bash
./run.sh
```

Open `http://127.0.0.1:7878` in a browser to test the demo.

The UI is a single-screen chat demo. It keeps attention on the conversation, supports sample prompts, keeps evidence collapsed inside each answer, offers an optional Thinking toggle to show the RAG flow, copies answers, and exports a transcript for presenter review.
# policypilot
