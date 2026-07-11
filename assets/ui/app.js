const sampleQuestions = [
  "Can I expense a hotel minibar?",
  "Are hotel stays reimbursable?",
  "What happens if travel feels unsafe?"
];

const stateKey = "policypilot-chat-history";
const historyEl = document.getElementById("chat-history");
const form = document.getElementById("question-form");
const questionInput = document.getElementById("question");
const sampleContainer = document.getElementById("sample-questions");
const clearButton = document.getElementById("clear-chat");

const state = loadState();

renderSamples();
renderHistory();

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const question = questionInput.value.trim();
  if (!question) return;

  questionInput.value = question;
  appendUserMessage(question);
  saveState();

  const loadingId = appendAssistantPlaceholder();
  try {
    const response = await fetch(`/api/answer?question=${encodeURIComponent(question)}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const report = await response.json();
    replacePlaceholder(loadingId, report);
    state.messages.push({ role: "assistant", report });
    saveState();
  } catch (error) {
    replacePlaceholder(loadingId, {
      answer: `Request failed: ${error.message}`,
      relevant_excerpt: "",
      sources: [],
      top_match_title: null
    });
  }
});

clearButton.addEventListener("click", () => {
  state.messages = [];
  saveState();
  renderHistory();
});

function renderSamples() {
  sampleContainer.innerHTML = "";
  for (const sample of sampleQuestions) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "btn btn-secondary";
    button.textContent = sample;
    button.addEventListener("click", () => {
      questionInput.value = sample;
      form.requestSubmit();
    });
    sampleContainer.appendChild(button);
  }
}

function renderHistory() {
  historyEl.innerHTML = "";

  if (state.messages.length === 0) {
    const intro = document.createElement("div");
    intro.className = "message assistant";
    intro.innerHTML = `
      <span class="label">Assistant</span>
      <div class="body">Ask a question to begin. The UI will keep the conversation visible so you can walk the audience through the retrieval step.</div>
    `;
    historyEl.appendChild(intro);
    return;
  }

  for (const message of state.messages) {
    if (message.role === "user") {
      historyEl.appendChild(createUserMessage(message.content));
    } else {
      historyEl.appendChild(createAssistantMessage(message.report));
    }
  }
}

function appendUserMessage(content) {
  state.messages.push({ role: "user", content });
  const node = createUserMessage(content);
  historyEl.appendChild(node);
  scrollToBottom();
}

function appendAssistantPlaceholder() {
  const id = `loading-${Date.now()}`;
  const node = document.createElement("div");
  node.className = "message assistant";
  node.dataset.placeholderId = id;
  node.innerHTML = `
    <span class="label">Assistant</span>
    <div class="body">Retrieving relevant policy text...</div>
  `;
  historyEl.appendChild(node);
  scrollToBottom();
  return id;
}

function replacePlaceholder(id, report) {
  const node = historyEl.querySelector(`[data-placeholder-id="${id}"]`);
  if (!node) return;
  const replacement = createAssistantMessage(report);
  node.replaceWith(replacement);
  scrollToBottom();
}

function createUserMessage(content) {
  const node = document.createElement("div");
  node.className = "message user";
  node.innerHTML = `
    <span class="label">You</span>
    <div class="body">${escapeHtml(content)}</div>
  `;
  return node;
}

function createAssistantMessage(report) {
  const node = document.createElement("div");
  node.className = "message assistant";

  const sources = Array.isArray(report.sources) && report.sources.length > 0
    ? report.sources.map((source) => `
        <div class="source">
          <div><strong>${escapeHtml(source.title)}</strong></div>
          <div class="meta">${escapeHtml(source.document_id)}${source.section ? `, ${escapeHtml(source.section)}` : ""} | score ${Number(source.score).toFixed(3)}</div>
        </div>
      `).join("")
    : `<div class="source">No sources returned.</div>`;

  const topTitle = report.top_match_title ? `<div class="meta">Top match: ${escapeHtml(report.top_match_title)}</div>` : "";

  node.innerHTML = `
    <span class="label">Assistant</span>
    <div class="body">
      <div class="answer-box"><strong>${escapeHtml(report.answer || "")}</strong></div>
      ${report.relevant_excerpt ? `<div class="excerpt">${escapeHtml(report.relevant_excerpt)}</div>` : ""}
      ${topTitle}
      <div class="sources">${sources}</div>
    </div>
  `;
  return node;
}

function loadState() {
  try {
    const raw = localStorage.getItem(stateKey);
    if (!raw) return { messages: [] };
    const parsed = JSON.parse(raw);
    if (!parsed || !Array.isArray(parsed.messages)) return { messages: [] };
    return parsed;
  } catch {
    return { messages: [] };
  }
}

function saveState() {
  localStorage.setItem(stateKey, JSON.stringify(state));
}

function scrollToBottom() {
  historyEl.scrollTop = historyEl.scrollHeight;
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
