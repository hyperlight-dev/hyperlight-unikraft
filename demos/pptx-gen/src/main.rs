//! PowerPoint generator using hluk sandboxed Python execution.
//!
//! `python-pptx` code runs inside a Hyperlight micro-VM (no host filesystem
//! except one mounted output directory, no network, ephemeral) and writes the
//! finished `.pptx` straight to the mounted folder.
//!
//! The code can come from an LLM (set `OPENAI_API_KEY`) or, by default, from a
//! small built-in template so the demo runs with no key and no network.

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use hyperlight_unikraft::{Exec, Mount, SandboxBuilder, run};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(version, about = "Generate PowerPoint presentations with hluk")]
struct Args {
    /// Prompt describing the presentation.
    #[arg(short, long)]
    prompt: String,

    /// Output file path (written on the host).
    #[arg(short, long, default_value = "presentation.pptx")]
    output: PathBuf,

    /// python-pptx rootfs CPIO (build with `just rootfs`).
    #[arg(long, default_value = "../../build-elfloader/pptx-rootfs.cpio")]
    rootfs: PathBuf,

    /// Guest scratch memory, MiB.
    #[arg(long, default_value_t = 512)]
    scratch_mb: usize,

    /// OpenAI model (used only when OPENAI_API_KEY is set).
    #[arg(long, default_value = "gpt-4o")]
    model: String,

    /// Force the built-in template even if OPENAI_API_KEY is set.
    #[arg(long)]
    no_llm: bool,

    /// Print the generated code without running it.
    #[arg(long)]
    dry_run: bool,
}

const SYSTEM_PROMPT: &str = r#"Generate Python code using python-pptx to create a presentation.

Requirements:
1. Use python-pptx to build the presentation as a Presentation object.
2. Do NOT call .save() and do NOT print anything — the host saves the file.
3. Do not read any files.

Output only Python code.
"#;

/// Runs after the generated code: save the last python-pptx Presentation object
/// it built to the mounted output path. Using a private name (`_HL_OUT`) means
/// generated code can't clobber the destination, and searching globals() means
/// it doesn't matter what the code named its Presentation variable.
const SAVE_EPILOGUE: &str = r#"
from pptx.presentation import Presentation as _HL_P
_hl_decks = [v for v in list(globals().values()) if isinstance(v, _HL_P)]
if not _hl_decks:
    raise SystemExit("generated code created no python-pptx Presentation object")
_hl_decks[-1].save(_HL_OUT)
"#;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("hyperlight_pptx_gen=info".parse().unwrap())
                .add_directive(tracing::level_filters::LevelFilter::WARN.into()),
        )
        .init();

    dotenvy::dotenv().ok();
    let args = Args::parse();

    let output = std::path::absolute(&args.output)
        .with_context(|| format!("invalid output path {:?}", args.output))?;
    let out_dir = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let out_name = output
        .file_name()
        .context("output path has no file name")?
        .to_string_lossy()
        .into_owned();
    std::fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create {out_dir:?}"))?;

    let use_llm = !args.no_llm && std::env::var_os("OPENAI_API_KEY").is_some();
    info!("prompt: {}", args.prompt);

    let body = if use_llm {
        info!("generating code with {} ...", args.model);
        generate_python_code(&args.prompt, &args.model).await?
    } else {
        info!("no OPENAI_API_KEY (or --no-llm): using the built-in template");
        template_code(&args.prompt)
    };

    // The guest writes to the mounted output directory. The destination is a
    // private name the generated code never sees, and the epilogue saves the
    // Presentation the code built.
    let program = format!("_HL_OUT = \"/out/{out_name}\"\n{body}\n{SAVE_EPILOGUE}");

    if args.dry_run {
        println!("\n--- Generated Code ---\n{program}\n---");
        return Ok(());
    }

    info!("executing in the micro-VM...");
    execute_in_sandbox(&program, &args.rootfs, args.scratch_mb, &out_dir)?;

    let written = out_dir.join(&out_name);
    let size = std::fs::metadata(&written)
        .with_context(|| format!("the guest did not write {written:?}"))?
        .len();
    info!("saved: {written:?} ({size} bytes)");
    Ok(())
}

/// A deterministic python-pptx program built from the prompt — a title slide
/// plus a content slide. No LLM, no network.
fn template_code(prompt: &str) -> String {
    // `{:?}` yields a quoted, escaped string literal that Python accepts.
    let title = format!("{prompt:?}");
    format!(
        r#"from pptx import Presentation

prs = Presentation()

cover = prs.slides.add_slide(prs.slide_layouts[0])
cover.shapes.title.text = {title}
cover.placeholders[1].text = "Generated inside a Hyperlight micro-VM"

body = prs.slides.add_slide(prs.slide_layouts[1])
body.shapes.title.text = "About"
tf = body.placeholders[1].text_frame
tf.text = {title}
for line in ("Runs untrusted code in a hardware-isolated VM",
             "No host filesystem access beyond the mounted output folder",
             "No network, ephemeral"):
    p = tf.add_paragraph()
    p.text = line
    p.level = 1
"#
    )
}

async fn generate_python_code(prompt: &str, model: &str) -> Result<String> {
    use async_openai::{
        config::OpenAIConfig,
        types::{
            ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        },
        Client,
    };

    let api_key = std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY not set")?;
    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(SYSTEM_PROMPT)
                    .build()?,
            ),
            ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(format!("Create a PowerPoint presentation: {prompt}"))
                    .build()?,
            ),
        ])
        .temperature(0.7_f32)
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response
        .choices
        .first()
        .and_then(|c| c.message.content.as_ref())
        .context("no response from OpenAI")?;

    // Strip a markdown code fence if the model wrapped the code in one.
    let code = content
        .trim()
        .strip_prefix("```python")
        .or_else(|| content.trim().strip_prefix("```"))
        .unwrap_or(content)
        .strip_suffix("```")
        .unwrap_or(content)
        .trim()
        .to_string();
    Ok(code)
}

fn execute_in_sandbox(
    program: &str,
    rootfs: &PathBuf,
    scratch_mb: usize,
    out_dir: &PathBuf,
) -> Result<()> {
    if !rootfs.exists() {
        bail!("rootfs not found: {rootfs:?}. Build it with `just rootfs`.");
    }
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs.clone())
        .scratch_mb(scratch_mb)
        .mount(Mount::rw(out_dir, "/out"))
        .boot()
        .map_err(|e| anyhow!("boot sandbox: {e}"))?;
    run(&mut sandbox, Exec::Code(program.to_string())).map_err(|e| anyhow!("guest run: {e}"))?;
    Ok(())
}
