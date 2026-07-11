const sampleQuestions = [
  "Can I expense a hotel minibar?",
  "Are hotel stays reimbursable?",
  "What happens if travel feels unsafe?"
];

const thinkingSteps = [
  {
    title: "Read question",
    detail: "Capture the policy question and prepare it for retrieval."
  },
  {
    title: "Embed query",
    detail: "Convert the question into searchable vector coordinates."
  },
  {
    title: "Retrieve chunks",
    detail: "Compare the query against policy chunks and rank the closest matches."
  },
  {
    title: "Select evidence",
    detail: "Use the strongest retrieved chunk as the grounding source."
  },
  {
    title: "Generate response",
    detail: "Draft a concise answer with the selected evidence attached."
  }
];

const stateKey = "policypilot-chat-history";
const historyEl = document.getElementById("chat-history");
const form = document.getElementById("question-form");
const questionInput = document.getElementById("question");
const sampleContainer = document.getElementById("sample-questions");
const clearButton = document.getElementById("clear-chat");
const exportButton = document.getElementById("export-chat");
const thinkingToggle = document.getElementById("toggle-thinking");
const thinkingToggleLabel = document.getElementById("thinking-toggle-label");
const turnCountEl = document.getElementById("turn-count");

const state = loadState();

renderThinkingToggle();
renderSamples();
renderHistory();

form.addEventListener("submit", async (event) => {
  event.preventDefault();

  const question = questionInput.value.trim();
  if (!question) return;

  setBusy(true);
  questionInput.value = "";
  appendUserMessage(question);
  saveState();

  const thinkingEnabled = state.thinkingEnabled === true;
  const loadingId = appendAssistantPlaceholder(thinkingEnabled);
  const thinkingTrace = thinkingEnabled ? runThinkingTrace(loadingId) : Promise.resolve([]);

  try {
    const [report, completedSteps] = await Promise.all([
      fetchAnswer(question),
      thinkingTrace
    ]);
    report.thinking_steps = completedSteps;
    replacePlaceholder(loadingId, report);
    state.messages.push({ role: "assistant", report });
    saveState();
  } catch (error) {
    const completedSteps = await thinkingTrace;
    const report = {
      answer: `Request failed: ${error.message}`,
      relevant_excerpt: "",
      sources: [],
      is_error: true,
      thinking_steps: completedSteps
    };
    replacePlaceholder(loadingId, report);
    state.messages.push({ role: "assistant", report });
    saveState();
  } finally {
    setBusy(false);
    questionInput.focus();
  }
});

questionInput.addEventListener("keydown", (event) => {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    form.requestSubmit();
  }
});

clearButton.addEventListener("click", () => {
  state.messages = [];
  saveState();
  renderHistory();
  questionInput.value = sampleQuestions[0];
  questionInput.focus();
});

exportButton.addEventListener("click", exportTranscript);

thinkingToggle.addEventListener("click", () => {
  state.thinkingEnabled = state.thinkingEnabled !== true;
  saveState();
  renderThinkingToggle();
});

historyEl.addEventListener("click", async (event) => {
  const button = event.target.closest("[data-copy-answer]");
  if (!button) return;

  await copyText(button.dataset.copyAnswer || "");
  const original = button.textContent;
  button.textContent = "Copied";
  setTimeout(() => {
    button.textContent = original;
  }, 1200);
});

function renderSamples() {
  sampleContainer.innerHTML = "";

  sampleQuestions.forEach((sample, index) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "button sample-button";
    button.innerHTML = `<span>Prompt ${index + 1}</span>${escapeHtml(sample)}`;
    button.addEventListener("click", () => {
      questionInput.value = sample;
      form.requestSubmit();
    });
    sampleContainer.appendChild(button);
  });
}

function renderHistory() {
  historyEl.innerHTML = "";

  if (state.messages.length === 0) {
    const intro = document.createElement("div");
    intro.className = "empty-state";
    intro.innerHTML = `
      <h3>Start with one question.</h3>
      <p>The chat stays clean. Answers are short, and evidence is available only when opened.</p>
    `;
    historyEl.appendChild(intro);
    updateTurnCount();
    return;
  }

  for (const message of state.messages) {
    if (message.role === "user") {
      historyEl.appendChild(createUserMessage(message.content));
    } else {
      historyEl.appendChild(createAssistantMessage(message.report));
    }
  }

  updateTurnCount();
  scrollToBottom();
}

function appendUserMessage(content) {
  clearEmptyState();
  state.messages.push({ role: "user", content });
  historyEl.appendChild(createUserMessage(content));
  updateTurnCount();
  scrollToBottom();
}

async function fetchAnswer(question) {
  const response = await fetch(`/api/answer?question=${encodeURIComponent(question)}`);
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  return response.json();
}

function appendAssistantPlaceholder(showThinking) {
  clearEmptyState();
  const id = `loading-${Date.now()}`;
  const node = document.createElement("div");
  node.className = "message message-assistant";
  node.dataset.placeholderId = id;

  if (showThinking) {
    node.innerHTML = `
      <div class="message-label">PolicyPilot</div>
      <div class="message-bubble">
        <div class="thinking-panel" data-thinking-panel>
          <div class="thinking-heading">
            <strong>Thinking through RAG</strong>
            <span data-thinking-status>Starting...</span>
          </div>
          <ol class="thinking-steps">
            ${thinkingSteps.map((step, index) => `
              <li data-step-index="${index}">
                <span>${escapeHtml(step.title)}</span>
                <small>${escapeHtml(step.detail)}</small>
              </li>
            `).join("")}
          </ol>
        </div>
      </div>
    `;
  } else {
    node.innerHTML = `
      <div class="message-label">PolicyPilot</div>
      <div class="message-bubble">
        <div class="typing" aria-label="Retrieving policy evidence">
          <span></span><span></span><span></span>
        </div>
      </div>
    `;
  }

  historyEl.appendChild(node);
  scrollToBottom();
  return id;
}

async function runThinkingTrace(placeholderId) {
  const completed = [];

  for (let index = 0; index < thinkingSteps.length; index += 1) {
    updateThinkingStep(placeholderId, index);
    completed.push(thinkingSteps[index]);
    await wait(560);
  }

  return completed;
}

function updateThinkingStep(placeholderId, activeIndex) {
  const node = historyEl.querySelector(`[data-placeholder-id="${placeholderId}"]`);
  if (!node) return;

  const status = node.querySelector("[data-thinking-status]");
  if (status) status.textContent = thinkingSteps[activeIndex].title;

  for (const item of node.querySelectorAll("[data-step-index]")) {
    const index = Number(item.dataset.stepIndex);
    item.classList.toggle("is-active", index === activeIndex);
    item.classList.toggle("is-done", index < activeIndex);
  }

  scrollToBottom();
}

function replacePlaceholder(id, report) {
  const node = historyEl.querySelector(`[data-placeholder-id="${id}"]`);
  if (!node) return;

  node.replaceWith(createAssistantMessage(report));
  updateTurnCount();
  scrollToBottom();
}

function createUserMessage(content) {
  const node = document.createElement("article");
  node.className = "message message-user";
  node.innerHTML = `
    <div class="message-label">You</div>
    <div class="message-bubble">
      <div class="message-content">${escapeHtml(content)}</div>
    </div>
  `;
  return node;
}

function createAssistantMessage(rawReport) {
  const report = rawReport || {};
  const answer = report.answer || "";
  const node = document.createElement("article");
  node.className = `message message-assistant${report.is_error ? " message-error" : ""}`;

  node.innerHTML = `
    <div class="message-label">PolicyPilot</div>
    <div class="message-bubble">
      <div class="assistant-card">
        <div class="answer-row">
          <p class="answer-text">${escapeHtml(answer)}</p>
          <button class="mini-button" type="button" data-copy-answer="${escapeAttribute(answer)}">Copy</button>
        </div>
        ${renderThinkingSummary(report)}
        ${renderEvidence(report)}
      </div>
    </div>
  `;
  return node;
}

function renderThinkingSummary(report) {
  if (!Array.isArray(report.thinking_steps) || report.thinking_steps.length === 0) return "";

  return `
    <details class="thinking-details">
      <summary>Thinking trace</summary>
      <ol class="thinking-summary">
        ${report.thinking_steps.map((step) => `
          <li>
            <strong>${escapeHtml(step.title)}</strong>
            <span>${escapeHtml(step.detail)}</span>
          </li>
        `).join("")}
      </ol>
    </details>
  `;
}

function renderEvidence(report) {
  if (report.is_error) return "";

  const excerpt = renderHighlightedExcerpt(report);
  const sourceCards = renderSources(report.sources);

  return `
    <details class="evidence-details">
      <summary>Evidence used</summary>
      <div class="evidence-body">
        <p class="excerpt">${excerpt}</p>
        <div class="source-list">${sourceCards}</div>
      </div>
    </details>
  `;
}

function renderHighlightedExcerpt(report) {
  const excerpt = report.relevant_excerpt || "No excerpt returned.";
  const lines = excerpt.split("\n");
  const bestIndex = findBestEvidenceLine(lines, report.question || "");

  return lines.map((line, index) => {
    const escaped = escapeHtml(line);
    if (index === bestIndex && line.trim()) {
      return `<strong class="used-policy-line">${escaped}</strong>`;
    }
    return escaped;
  }).join("\n");
}

function findBestEvidenceLine(lines, question) {
  const terms = expandedTerms(question);
  let bestIndex = -1;
  let bestScore = 0;

  lines.forEach((line, index) => {
    const cleaned = line.trim();
    if (!cleaned || cleaned.startsWith("#")) return;

    const lineTerms = extractTerms(cleaned);
    const overlap = [...terms].filter((term) => lineTerms.has(term)).length;
    const exactBoost = [...terms].some((term) => cleaned.toLowerCase().includes(term)) ? 0.25 : 0;
    const policyLineBoost = cleaned.startsWith("-") ? 0.2 : 0;
    const score = overlap + exactBoost + policyLineBoost;

    if (score > bestScore) {
      bestScore = score;
      bestIndex = index;
    }
  });

  if (bestIndex !== -1) return bestIndex;
  return lines.findIndex((line) => line.trim() && !line.trim().startsWith("#"));
}

function renderSources(sources) {
  if (!Array.isArray(sources) || sources.length === 0) {
    return `<div class="source-card">No sources returned.</div>`;
  }

  return sources.slice(0, 2).map((source, index) => {
    const score = Number(source.score);
    const scoreText = Number.isFinite(score) ? score.toFixed(3) : "n/a";
    const section = source.section ? ` | ${escapeHtml(source.section)}` : "";

    return `
      <div class="source-card${index === 0 ? " source-card-used" : ""}">
        <span class="source-title">${escapeHtml(source.title)}</span>
        <span>${escapeHtml(source.document_id)}${section} | similarity ${scoreText}</span>
      </div>
    `;
  }).join("");
}

function renderThinkingToggle() {
  const isEnabled = state.thinkingEnabled === true;
  thinkingToggle.setAttribute("aria-pressed", String(isEnabled));
  thinkingToggleLabel.textContent = isEnabled ? "Thinking On" : "Show Thinking";
  thinkingToggle.classList.toggle("is-on", isEnabled);
}

function updateTurnCount() {
  const turns = state.messages.filter((message) => message.role === "user").length;
  turnCountEl.textContent = String(turns);
}

function setBusy(isBusy) {
  questionInput.disabled = isBusy;
  form.querySelector("button[type='submit']").disabled = isBusy;
}

function clearEmptyState() {
  const empty = historyEl.querySelector(".empty-state");
  if (empty) empty.remove();
}

function exportTranscript() {
  const lines = state.messages.map((message) => {
    if (message.role === "user") return `You: ${message.content}`;

    const report = message.report || {};
    const sources = Array.isArray(report.sources)
      ? report.sources.map((source) => `- ${source.title} (${source.document_id})`).join("\n")
      : "No sources";

    return [
      `PolicyPilot: ${report.answer || ""}`,
      report.relevant_excerpt ? `Evidence: ${report.relevant_excerpt}` : null,
      `Sources:\n${sources}`
    ].filter(Boolean).join("\n");
  });

  const transcript = [
    "PolicyPilot Chat Transcript",
    new Date().toLocaleString(),
    "",
    lines.length ? lines.join("\n\n---\n\n") : "No messages yet."
  ].join("\n");

  const blob = new Blob([transcript], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = "policypilot-transcript.txt";
  document.body.appendChild(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
}

async function copyText(text) {
  if (navigator.clipboard && window.isSecureContext) {
    await navigator.clipboard.writeText(text);
    return;
  }

  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.style.position = "fixed";
  textarea.style.opacity = "0";
  document.body.appendChild(textarea);
  textarea.select();
  document.execCommand("copy");
  textarea.remove();
}

function loadState() {
  try {
    const raw = localStorage.getItem(stateKey);
    if (!raw) return { messages: [], thinkingEnabled: false };

    const parsed = JSON.parse(raw);
    if (!parsed || !Array.isArray(parsed.messages)) return { messages: [], thinkingEnabled: false };

    return {
      messages: parsed.messages,
      thinkingEnabled: parsed.thinkingEnabled === true
    };
  } catch {
    return { messages: [], thinkingEnabled: false };
  }
}

function saveState() {
  localStorage.setItem(stateKey, JSON.stringify(state));
}

function scrollToBottom() {
  historyEl.scrollTop = historyEl.scrollHeight;
}

function wait(milliseconds) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

function extractTerms(value) {
  const stopWords = new Set([
    "a", "an", "and", "are", "as", "at", "be", "because", "can", "do", "does",
    "for", "from", "happens", "how", "i", "if", "in", "is", "it", "of", "on",
    "or", "should", "the", "to", "what", "when", "where", "while", "with"
  ]);

  return new Set(
    String(value)
      .toLowerCase()
      .replace(/[^a-z0-9\s-]/g, " ")
      .split(/\s+/)
      .map((term) => normalizeTerm(term))
      .filter((term) => term.length > 2 && !stopWords.has(term))
  );
}

function expandedTerms(value) {
  const terms = extractTerms(value);
  const additions = [];

  for (const term of terms) {
    switch (term) {
      case "drink":
      case "drinks":
      case "beverage":
      case "beverages":
        additions.push("alcohol", "purchase");
        break;
      case "buy":
      case "bought":
      case "purchase":
      case "purchased":
        additions.push("expense", "purchase");
        break;
      case "expense":
      case "expenses":
        additions.push("reimbursement");
        break;
      case "reimbursable":
      case "reimbursement":
        additions.push("expense");
        break;
      case "unsafe":
      case "safety":
      case "security":
        additions.push("travel", "security", "safety");
        break;
      case "minibar":
        additions.push("incidentals", "personal");
        break;
      default:
        break;
    }
  }

  additions.forEach((term) => terms.add(term));
  return terms;
}

function normalizeTerm(term) {
  return term
    .replace(/ies$/, "y")
    .replace(/ing$/, "")
    .replace(/ed$/, "")
    .replace(/s$/, "");
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function escapeAttribute(value) {
  return escapeHtml(value).replaceAll("\n", "&#10;");
}
