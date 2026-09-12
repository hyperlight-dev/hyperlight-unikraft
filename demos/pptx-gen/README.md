# PPTX generator demo

Generates a PowerPoint file by running `python-pptx` code inside a Hyperlight micro-VM with `hluk`. The code can come from an LLM, but by default it comes from a small built-in template, so the demo runs with **no API key and no network**.

```
prompt ──▶ python-pptx code ──▶ Hyperlight micro-VM ──▶ /out/<file>.pptx
           (LLM or built-in)     (only the output folder is mounted;
                                   no other host FS, no network, ephemeral)
```

The generated code is untrusted, so it runs in a hardware-isolated VM. The one thing it can touch on the host is the mounted output directory, where it writes the finished file — no base64 round-trip needed.

## Prerequisites

- The `python-shell` base rootfs: `just build-rootfs python-shell` (from the repo root)
- Optional: an OpenAI API key, to generate the slide code with an LLM instead of the built-in template

## Quick start

You can pass any prompt string. What happens depends on whether a key is set:

- **No key (built-in template):** the prompt is used verbatim as the title of a fixed two-slide deck (a title slide + a generic "About" slide). It does not interpret the prompt — "5 slides", "about cloud security" etc. are not acted on; the point is to exercise the sandbox without a network call.
- **With a key (LLM):** the prompt is sent to the model, which writes `python-pptx` code, so you actually get a deck about the topic with the requested structure.

```bash
cd demos/pptx-gen

# No key — a 2-slide template titled with your prompt:
just run "My first sandboxed deck"
# -> presentation.pptx

# With an LLM (put OPENAI_API_KEY in .env; see .env.example) — real slides:
just run "Create a 5-slide presentation about cloud security"
```

Options:

```bash
./target/release/hyperlight-pptx-gen --help
#   -p/--prompt   the description
#   -o/--output   where to write the .pptx (default presentation.pptx)
#   --no-llm      force the built-in template even if a key is set
#   --dry-run     print the generated Python without running it
#   --model       OpenAI model (default gpt-4o), --rootfs, --scratch-mb
```

## How it works

1. The CLI produces `python-pptx` code — from OpenAI when `OPENAI_API_KEY` is set, otherwise from a built-in template built from the prompt.
2. `execute_in_sandbox` boots the python-pptx rootfs with `hluk`'s library API (`SandboxBuilder::from_initrd(...).boot()` → `run(Exec::Code(...))`), mounting the output directory at `/out`. The code is passed inline; nothing else is written to the guest disk image.
3. The code only *builds* a `Presentation`. A fixed epilogue then saves the last Presentation object to a private path under `/out`, so the generated code can't redirect the output or has to know the destination.
4. Because `/out` is a real host mount, the `.pptx` appears directly on the host.
