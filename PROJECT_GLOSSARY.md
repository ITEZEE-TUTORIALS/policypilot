# PolicyPilot Project Glossary

This glossary combines the project data map, the Rust project anatomy, and the core Rust/RAG concepts in one place.

---

## Part 1: Project Data

### 1. Raw handbook pages

#### `data/sources/gitlab/global_travel_expense.html`
#### `data/sources/gitlab/travel_safety.html`

- What it is: the downloaded public GitLab handbook pages in their original HTML form.
- Why it matters: this is the source-of-truth material the demo is based on.
- Analogy: the **full library book** before any notes are taken.

---

### 2. Cleaned policy excerpts

#### `data/gitlab/global_travel_expense.md`
#### `data/gitlab/travel_safety.md`

- What it is: short markdown excerpts cleaned up for the demo.
- Why it matters: these are easier for the prototype to ingest and easier for the audience to read.
- Analogy: the **highlighted pages** with the useful lines marked in yellow.

---

### 3. Documents

#### `src/ingest.rs`

- What it is: the code that loads each policy source into a document record.
- Why it matters: documents are the first structured representation of source text.
- Analogy: the **file folder** that holds the pages before they get sorted.

---

### 4. Chunks

#### `src/chunk.rs`

- What it is: code that breaks each document into smaller sections.
- Why it matters: chunks are the units the retriever searches over.
- Analogy: **index cards** cut from a long report.

---

### 5. Embeddings

#### `src/embed.rs`

- What it is: code that turns text into vectors.
- Why it matters: embeddings let the system compare meaning instead of just exact words.
- Analogy: each chunk gets a **meaning fingerprint**.

---

### 6. Vector store

#### `src/store.rs`

- What it is: the in-memory store that keeps chunk vectors.
- Why it matters: this is where semantic search happens.
- Analogy: the **card catalog** in a library.

---

### 7. Retrieved matches

#### `src/retrieve.rs`

- What it is: the code that scores and returns the most relevant chunks.
- Why it matters: retrieval is the "find the best pages" step in RAG.
- Analogy: the **librarian’s shortlist** of the most useful pages.

---

### 8. RIG handoff

#### `src/rig.rs`

- What it is: the orchestration layer that turns retrieved evidence into a copy-paste AI prompt pack and cURL handoff.
- Why it matters: it shows how the demo bridges grounded retrieval to an external model without hiding the workflow.
- Analogy: the **dispatch desk** that packages the right evidence and sends it to the next specialist.

---

### 9. Final answer

#### `src/answer.rs`

- What it is: the code that turns retrieved evidence into a readable answer.
- Why it matters: this is the part the user actually sees.
- Analogy: the **briefing memo** written after the research is done.

---

### 10. Demo entry point

#### `src/main.rs`

- What it is: the entry point that connects the whole demo.
- Why it matters: it shows the full story from question to answer.
- Analogy: the **director’s cue sheet** for the presentation.

---

### 11. Presentation assets

#### `PRESENTATION_OUTLINE.md`
#### `SPEAKER_NOTES.md`
#### `ALTERNATE_TOPIC.md`

- What it is: the talk structure, narration, and alternate framing.
- Why it matters: these files help the presenter tell the story clearly.
- Analogy: the **script, stage directions, and backup set** for the show.

---

## Part 2: Rust Project Anatomy

### 1. Cargo manifest

#### `Cargo.toml`

- What it is: the project manifest that tells Cargo the package name, edition, dependencies, and metadata.
- Why it matters: this is the blueprint Cargo reads before it builds anything.
- Analogy: the **project passport** that says who you are and what tools you are allowed to bring.

---

### 2. Locked dependencies

#### `Cargo.lock`

- What it is: the exact dependency snapshot Cargo resolved for this project.
- Why it matters: it makes builds repeatable so the same code keeps using the same crate versions.
- Analogy: the **sealed parts list** for a machine order.

---

### 3. Build output root

#### `target/`

- What it is: Cargo’s output directory for compiled binaries, libraries, caches, and build metadata.
- Why it matters: this is where the results of `cargo build`, `cargo test`, and `cargo run` land.
- Analogy: the **workshop output shelf** where finished pieces are stored.

---

### 4. Debug build profile

#### `target/debug/`

- What it is: the default development build directory.
- Why it matters: debug builds are faster to compile and easier to inspect while you are iterating.
- Analogy: the **rehearsal stage** before the polished performance.

---

### 5. Release build profile

#### `target/release/`

- What it is: the optimized production build directory.
- Why it matters: release builds are slower to compile but faster to run.
- Analogy: the **final show version** after the rehearsal cut is polished.

---

### 6. Build-script outputs

#### `target/debug/build/`

- What it is: the output area for build scripts and generated helper artifacts.
- Why it matters: crates with `build.rs`, native libraries, or generated code often leave work here.
- Analogy: the **side room** where custom tools are prepared before the main assembly.

---

### 7. Compiled dependencies

#### `target/debug/deps/`

- What it is: the folder that holds compiled crate dependencies and hashed artifact copies.
- Why it matters: Cargo uses these artifacts to link your binary and avoid rebuilding everything from scratch.
- Analogy: the **labeled storage bins** holding the parts the assembler needs.

---

### 8. Fingerprints

#### `target/debug/.fingerprint/`

- What it is: metadata that tracks what changed for each crate and artifact.
- Why it matters: Cargo uses fingerprints to decide what must be rebuilt.
- Analogy: the **inspection tags** attached to each package.

---

### 9. Incremental compilation cache

#### `target/debug/incremental/`

- What it is: the cache that stores partial compilation state for faster rebuilds.
- Why it matters: incremental compilation speeds up the edit-build-run loop during development.
- Analogy: a **memory of the last assembly step** so the machine does not start from zero every time.

---

## Part 3: Core Concepts

### 1. RAG

- What it is: Retrieval-Augmented Generation, the pattern of finding source text first and then generating an answer from it.
- Why it matters: it keeps the assistant grounded in facts instead of guessing.
- Analogy: **asking a librarian to fetch the right book before you write the summary**.

---

### 2. RIG

- What it is: the Rust AI orchestration layer used to organize retrieval, prompting, and tool flow.
- Why it matters: it keeps the moving parts coordinated.
- Analogy: **the conductor in an orchestra**.

---

### 3. Rust

- What it is: the systems language used to build the prototype.
- Why it matters: Rust gives the project speed, safety, and a strong structure.
- Analogy: **a precision machine shop** instead of a loose workbench.

---

### 4. Ownership

- What it is: the rule that each value has one clear owner at a time.
- Why it matters: ownership is what lets Rust manage memory safely without a garbage collector.
- Analogy: **one passport holder at a time**. The passport can be handed over, but it always belongs to exactly one person.

---

### 5. Borrowing

- What it is: temporary access to a value without taking ownership away.
- Why it matters: borrowing lets code use data safely without copying everything.
- Analogy: **checking out a library book for a limited time**.

---

### 6. References

- What it is: the actual borrowed pointer to data, either shared or mutable.
- Why it matters: references are how Rust lets you read or edit data without moving it.
- Analogy: **a bookmark pointing to the page you are allowed to use**.

---

### 7. Move

- What it is: transferring ownership from one variable to another.
- Why it matters: moves prevent dangling pointers and double frees.
- Analogy: **handing over the house keys**. Once the keys are handed over, the old holder no longer owns access.

---

### 8. Clone

- What it is: making a second owned copy of a value.
- Why it matters: cloning is useful when you really need two independent owners.
- Analogy: **photocopying a document so two people can keep a copy**.

---

### 9. Option

- What it is: Rust’s type for values that may be present or absent.
- Why it matters: it forces the code to handle missing data explicitly.
- Analogy: **a lunchbox that might have a sandwich or might be empty, but never silently pretends to have food**.

---

### 10. Struct

- What it is: a custom data shape made from named fields.
- Why it matters: structs keep related data organized and readable.
- Analogy: **a labeled filing tray with named compartments**.

---

### 11. Vec

- What it is: Rust’s growable list type.
- Why it matters: the demo uses vectors for chunks, documents, and search results.
- Analogy: **a row of expandable storage bins**.

---

### 12. String

- What it is: owned, growable text.
- Why it matters: policy text and answers are stored safely as owned strings.
- Analogy: **a notebook you own and can keep writing in**.

---

### 13. Slice

- What it is: a borrowed view into part of a collection.
- Why it matters: slices let you work with part of a string or list without copying it.
- Analogy: **a window cut out of a larger map**.

---

### 14. Result

- What it is: Rust’s type for success or failure.
- Why it matters: it makes errors part of the design, not an afterthought.
- Analogy: **a receipt that clearly says whether the checkout succeeded or failed**.

---

### 15. Chunking

- What it is: splitting long policy text into smaller searchable pieces.
- Why it matters: retrieval works better when the text is broken into manageable units.
- Analogy: **cutting a long report into index cards**.

---

### 16. Embeddings

- What it is: numeric representations of text meaning.
- Why it matters: embeddings let the system compare ideas instead of just exact words.
- Analogy: **a meaning fingerprint**.

---

### 17. Vector Store

- What it is: the memory layer that stores embeddings for search.
- Why it matters: this is the catalog the retriever searches through.
- Analogy: **a library card catalog**.

---

### 18. Retrieval

- What it is: the step that finds the most relevant chunks for a user question.
- Why it matters: it decides what evidence the model gets to see.
- Analogy: **the librarian’s shortlist**.

---

### 19. Final Answer

- What it is: the response generated from the retrieved evidence.
- Why it matters: this is what the user reads and trusts.
- Analogy: **a briefing memo written after the research is done**.

---

## One-line model

Source pages become cleaned excerpts, excerpts become chunks, chunks become embeddings, embeddings live in a vector store, retrieval finds the best matches, and the answer is assembled from those matches. RAG asks the librarian, RIG keeps the workflow organized, Rust keeps the machinery reliable, Cargo keeps the build reproducible, `target/` holds the generated artifacts, and ownership/borrowing plus `Option`/`Struct`/`Vec` keep the data shapes explicit and safe.
