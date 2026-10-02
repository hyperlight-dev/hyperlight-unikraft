// The page's behavior: install tabs, terminal recordings, the startup race
// (from the CI history beside this page on gh-pages), the density grid and
// the template browser (data/templates.json).

const $ = (sel, root = document) => root.querySelector(sel);
const $$ = (sel, root = document) => [...root.querySelectorAll(sel)];
const reducedMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;

function escapeHtml(s) {
  return s.replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
}

function ms(v) {
  if (v >= 1000) return `${(v / 1000).toFixed(v >= 10000 ? 1 : 2)} s`;
  if (v >= 100) return `${Math.round(v)} ms`;
  if (v >= 10) return `${v.toFixed(1)} ms`;
  return `${v.toFixed(v < 1 ? 2 : 1)} ms`;
}

// ── Tabs and copy buttons ────────────────────────────────────────────

for (const box of $$("[data-tabs]")) {
  const tabs = $$('[role="tab"]', box);
  const select = tab => {
    for (const t of tabs) {
      const on = t === tab;
      t.setAttribute("aria-selected", on);
      t.tabIndex = on ? 0 : -1;
      document.getElementById(t.getAttribute("aria-controls")).hidden = !on;
    }
  };
  tabs.forEach((tab, i) => {
    tab.addEventListener("click", () => select(tab));
    tab.addEventListener("keydown", e => {
      const step = { ArrowRight: 1, ArrowLeft: -1 }[e.key];
      if (!step) return;
      const next = tabs[(i + step + tabs.length) % tabs.length];
      select(next);
      next.focus();
    });
  });
}

document.addEventListener("click", async e => {
  const btn = e.target.closest(".copy");
  if (!btn) return;
  const text = btn.previousElementSibling.textContent.trim();
  try {
    await navigator.clipboard.writeText(text);
    btn.textContent = "Copied";
  } catch {
    btn.textContent = "Select and copy";
  }
  setTimeout(() => (btn.textContent = "Copy"), 1600);
});

// ── Terminal recordings ──────────────────────────────────────────────

function mountPlayer(el) {
  if (el.dataset.mounted) return;
  el.dataset.mounted = "1";
  const src = el.dataset.cast;
  if (!window.AsciinemaPlayer) {
    el.innerHTML = `<p class="player-fallback">The player did not load. <a href="${src}">Download the recording</a> and play it with <code>asciinema play</code>.</p>`;
    return;
  }
  AsciinemaPlayer.create(src, el, {
    theme: "hluk",
    fit: "width",
    idleTimeLimit: 1.5,
    autoPlay: !reducedMotion,
    preload: true,
    poster: "npt:0:30",
    terminalFontFamily: '"JetBrains Mono", ui-monospace, monospace',
    terminalFontSize: "small",
    controls: true,
  });
}

const playerObserver = new IntersectionObserver(entries => {
  for (const entry of entries) {
    if (entry.isIntersecting) {
      mountPlayer(entry.target);
      playerObserver.unobserve(entry.target);
    }
  }
}, { rootMargin: "0px 0px -15% 0px" });
$$(".player").forEach(el => playerObserver.observe(el));

// ── Startup: CI results, raced in slow motion ───────────────────────

// The race plays this many times slower than real time: CPython boots in
// about 200 ms, which at real speed is over before anyone sees it.
const SLOWDOWN = 10;

async function loadCi(os, image) {
  const r = await fetch(`dev/bench/${os}/${image}/data.js`);
  if (!r.ok) throw new Error(`${r.status} for ${os}/${image}`);
  // github-action-benchmark writes a script assigning a global; the JSON
  // starts at the first brace.
  const text = await r.text();
  const data = JSON.parse(text.slice(text.indexOf("{")));
  const series = Object.values(data.entries)[0];
  return series[series.length - 1];
}

const lanes = $$(".lane").map(el => ({ el, fill: $(".fill", el), value: $(".lane-value", el), metric: el.dataset.metric }));
let raceFrame = 0;

function race() {
  cancelAnimationFrame(raceFrame);
  const longest = Math.max(...lanes.map(l => l.target));
  const finish = l => {
    l.fill.style.width = `${(l.target / longest) * 100}%`;
    l.value.textContent = ms(l.target);
    l.el.classList.add("done");
  };
  for (const l of lanes) l.el.classList.remove("done");
  if (reducedMotion) {
    lanes.forEach(finish);
    return;
  }
  const t0 = performance.now();
  const tick = now => {
    const t = (now - t0) / SLOWDOWN;
    for (const l of lanes) {
      if (l.el.classList.contains("done")) continue;
      if (t >= l.target) {
        finish(l);
      } else {
        l.fill.style.width = `${(t / longest) * 100}%`;
        l.value.textContent = ms(t);
      }
    }
    if (t < longest) raceFrame = requestAnimationFrame(tick);
  };
  raceFrame = requestAnimationFrame(tick);
}

const raceReady = loadCi("linux", "python").then(entry => {
  const values = Object.fromEntries(entry.benches.map(b => [b.name, b.value]));
  for (const l of lanes) l.target = values[l.metric];
  // The bars are hidden from screen readers; this says the same once.
  $('[data-bind="race-summary"]').textContent = lanes
    .map(l => `${$(".lane-name", l.el).textContent}: ${ms(l.target)}.`).join(" ");
  const sha = entry.commit.id.slice(0, 7);
  const date = new Date(entry.commit.timestamp).toISOString().slice(0, 10);
  $('[data-bind="ci-provenance"]').innerHTML = `The python image running <code>print("ok")</code>, from CI on the latest push to <code>main</code> (<a href="${entry.commit.url}">${sha}</a>, ${date}) on a GitHub-hosted <code>ubuntu-latest</code> runner. <a href="dev/bench/linux/python/">Every commit's results</a>.`;
}).catch(() => {
  $('[data-bind="ci-provenance"]').innerHTML = `The CI results did not load. They are charted at <a href="dev/bench/linux/python/">dev/bench/linux/python</a>.`;
  $(".replay").hidden = true;
  throw new Error("no CI results");
});

new IntersectionObserver((entries, obs) => {
  if (!entries.some(e => e.isIntersecting)) return;
  obs.disconnect();
  raceReady.then(race, () => {});
}, { threshold: 0.4 }).observe($(".race"));
$(".replay").addEventListener("click", () => raceReady.then(race, () => {}));

// ── Density ──────────────────────────────────────────────────────────

// One square per sandbox in the measurement.
$(".cells").innerHTML = "<i></i>".repeat(1000);

// ── Templates ────────────────────────────────────────────────────────

// Templates merged since the latest release: `hluk init` from a release
// doesn't know them yet. Empty this when the next release ships.
const UNRELEASED = { java: "New since the latest release: install with HLUK_VERSION=dev to use it now." };

function filesHtml(files) {
  return files.map(f => `<div class="file"><h3>${escapeHtml(f.path)}</h3><pre class="code"><code>${escapeHtml(f.text.trimEnd())}</code></pre></div>`).join("");
}

fetch("data/templates.json").then(r => {
  if (!r.ok) throw new Error(`${r.status}`);
  return r.json();
}).then(templates => {
  const byName = Object.fromEntries(templates.map(t => [t.name, t]));
  for (const el of $$("[data-template-files]")) {
    const t = byName[el.dataset.templateFiles];
    if (t) el.innerHTML = filesHtml(t.files.filter(f => f.path !== "server.py"));
  }

  const order = templates.filter(t => !t.name.startsWith("http-"))
    .sort((a, b) => (a.tier ?? 9) - (b.tier ?? 9) || a.name.localeCompare(b.name));
  const picker = $(".lang-picker");
  const panel = $(".lang-panel");
  picker.innerHTML = order.map((t, i) => `<button role="tab" id="lang-${t.name}" aria-controls="lang-panel" aria-selected="${i === 0}" tabindex="${i === 0 ? 0 : -1}" data-name="${t.name}">${t.name}<small>tier ${t.tier}</small></button>`).join("");
  const buttons = $$("button", picker);
  const select = btn => {
    const t = byName[btn.dataset.name];
    for (const b of buttons) {
      b.setAttribute("aria-selected", b === btn);
      b.tabIndex = b === btn ? 0 : -1;
    }
    panel.setAttribute("aria-labelledby", btn.id);
    const note = UNRELEASED[t.name] ? `<span class="note">${escapeHtml(UNRELEASED[t.name])}</span>` : "";
    $(".lang-desc", panel).innerHTML = `${escapeHtml(t.description.replace(" -- ", ": "))}. Runs on the <code>${t.runtime}</code> image.${note}`;
    $(".cmd code", panel).textContent = `hluk init hello --template ${t.name}`;
    $(".files", panel).innerHTML = filesHtml(t.files);
  };
  buttons.forEach((b, i) => {
    b.addEventListener("click", () => select(b));
    b.addEventListener("keydown", e => {
      const step = { ArrowDown: 1, ArrowRight: 1, ArrowUp: -1, ArrowLeft: -1 }[e.key];
      if (!step) return;
      e.preventDefault();
      const next = buttons[(i + step + buttons.length) % buttons.length];
      select(next);
      next.focus();
    });
  });
  select(buttons[0]);
}).catch(() => {
  const link = '<a href="https://github.com/hyperlight-dev/hyperlight-unikraft/tree/main/templates">templates/</a>';
  $(".lang-layout").innerHTML = `<p class="provenance">The templates did not load. They are in ${link}.</p>`;
  for (const el of $$("[data-template-files]")) el.innerHTML = `<p class="provenance">See ${link}.</p>`;
});
