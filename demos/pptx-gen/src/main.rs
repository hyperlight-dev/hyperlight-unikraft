//! PowerPoint generator using Hyperlight-Unikraft sandboxed Python execution.

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use clap::Parser;
use hyperlight_unikraft::{parse_memory, run_vm_capture_output, VmConfig};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate PowerPoint presentations using Hyperlight-Unikraft")]
struct Args {
    /// Prompt describing the presentation
    #[arg(short, long)]
    prompt: String,

    /// Output file path
    #[arg(short, long, default_value = "presentation.pptx")]
    output: PathBuf,

    /// Path to Unikraft kernel
    #[arg(long, default_value = "assets/kernel")]
    kernel: PathBuf,

    /// Path to rootfs CPIO
    #[arg(long, default_value = "assets/rootfs.cpio")]
    rootfs: PathBuf,

    /// VM memory allocation
    #[arg(long, default_value = "2Gi")]
    memory: String,

    /// OpenAI model
    #[arg(long, default_value = "gpt-4o")]
    model: String,

    /// Print generated code without executing
    #[arg(long)]
    dry_run: bool,

    /// Show timing information
    #[arg(long)]
    timing: bool,
}

const SYSTEM_PROMPT: &str = r#"Generate Python code using python-pptx to create presentations.

Requirements:
1. Use python-pptx to create the presentation
2. Save to '/output.pptx'
3. Output the file as base64 with prefix "PPTX_BASE64:"

End with:
import base64
with open('/output.pptx', 'rb') as f:
    data = f.read()
print(f"PPTX_BASE64:{base64.b64encode(data).decode()}")

Output only Python code.
"#;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("hyperlight_pptx_gen=info".parse().unwrap())
                .add_directive("hyperlight_host=off".parse().unwrap())
                .add_directive(tracing::level_filters::LevelFilter::WARN.into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let args = Args::parse();

    info!("prompt: {}", args.prompt);

    info!("generating code...");
    let python_code = generate_python_code(&args.prompt, &args.model).await?;

    if args.dry_run {
        println!("\n--- Generated Code ---\n{}\n---", python_code);
        return Ok(());
    }

    debug!("code:\n{}", python_code);

    info!("executing in sandbox...");
    let start = std::time::Instant::now();
    let output = execute_in_sandbox(&python_code, &args.kernel, &args.rootfs, &args.memory, args.timing)?;
    let sandbox_time = start.elapsed();
    if args.timing {
        info!("sandbox execution: {:?}", sandbox_time);
    }

    info!("extracting pptx...");
    let pptx_data = extract_pptx_from_output(&output)?;

    std::fs::write(&args.output, &pptx_data)
        .with_context(|| format!("failed to write: {:?}", args.output))?;

    info!("saved: {:?} ({} bytes)", args.output, pptx_data.len());

    Ok(())
}

async fn generate_python_code(prompt: &str, model: &str) -> Result<String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .context("OPENAI_API_KEY not set")?;

    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

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
                    .content(format!("Create a PowerPoint presentation: {}", prompt))
                    .build()?,
            ),
        ])
        .temperature(0.7)
        .build()?;

    let response = client.chat().create(request).await?;

    let content = response
        .choices
        .first()
        .and_then(|c| c.message.content.as_ref())
        .context("no response from OpenAI")?;

    // Strip markdown code blocks if present
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

/// Prefix to patch zipfile timestamp issue (Unikraft time is 1970, ZIP needs >= 1980)
const ZIPFILE_PATCH: &str = r#"
# Patch zipfile to handle timestamps before 1980 (required for Unikraft)
import zipfile
_orig_ZipInfo_init = zipfile.ZipInfo.__init__
def _patched_ZipInfo_init(self, filename="NoName", date_time=None):
    if date_time is None or date_time[0] < 1980:
        date_time = (2024, 1, 1, 0, 0, 0)
    _orig_ZipInfo_init(self, filename, date_time)
zipfile.ZipInfo.__init__ = _patched_ZipInfo_init

"#;

fn execute_in_sandbox(
    python_code: &str,
    kernel: &Path,
    rootfs: &Path,
    memory: &str,
    timing: bool,
) -> Result<String> {
    if !kernel.exists() {
        anyhow::bail!("kernel not found: {:?}. Run 'make assets'.", kernel);
    }
    if !rootfs.exists() {
        anyhow::bail!("rootfs not found: {:?}. Run 'make assets'.", rootfs);
    }

    // Prepend the zipfile patch to the generated code
    let patched_code = format!("{}{}", ZIPFILE_PATCH, python_code);

    let temp_dir = tempfile::tempdir()?;
    let script_path = temp_dir.path().join("generate_pptx.py");
    std::fs::write(&script_path, &patched_code)?;

    debug!("script: {:?}", script_path);

    let cpio_start = std::time::Instant::now();
    let (modified_rootfs, _rootfs_tmpdir) = inject_script_into_rootfs(rootfs, &script_path)?;
    if timing {
        info!("  cpio inject: {:?}", cpio_start.elapsed());
    }

    // Load rootfs into memory
    let rootfs_data = std::fs::read(&modified_rootfs)?;

    // Parse memory size
    let heap_size = parse_memory(memory)?;

    let config = VmConfig::default().with_heap_size(heap_size);

    let vm_start = std::time::Instant::now();
    let vm_output = run_vm_capture_output(
        kernel,
        Some(&rootfs_data),
        &["/generate_pptx.py".to_string()],
        config,
    )?;
    if timing {
        info!("  vm total: {:?}", vm_start.elapsed());
        info!("  sandbox setup: {:?}", vm_output.setup_time);
        info!("  sandbox.evolve: {:?}", vm_output.evolve_time);
    }

    debug!("output: {}", vm_output.output);

    Ok(vm_output.output)
}

fn inject_script_into_rootfs(original_rootfs: &Path, script_path: &Path) -> Result<(PathBuf, tempfile::TempDir)> {
    let temp_dir = tempfile::tempdir()?;
    let extract_dir = temp_dir.path().join("rootfs");
    let new_cpio = temp_dir.path().join("rootfs_with_script.cpio");

    // Convert to absolute path before cd
    let rootfs_abs = original_rootfs.canonicalize()
        .with_context(|| format!("failed to resolve: {:?}", original_rootfs))?;

    std::fs::create_dir_all(&extract_dir)?;

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && cpio -idm < {} 2>/dev/null",
            extract_dir.display(),
            rootfs_abs.display()
        ))
        .status()
        .context("cpio extract failed")?;

    if !status.success() {
        anyhow::bail!("cpio extract failed");
    }

    let dest_script = extract_dir.join("generate_pptx.py");
    std::fs::copy(script_path, &dest_script)?;

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && find . 2>/dev/null | cpio -o -H newc > {} 2>/dev/null",
            extract_dir.display(),
            new_cpio.display()
        ))
        .status()
        .context("cpio create failed")?;

    if !status.success() {
        anyhow::bail!("cpio create failed");
    }

    Ok((new_cpio, temp_dir))
}

fn extract_pptx_from_output(output: &str) -> Result<Vec<u8>> {
    const PREFIX: &str = "PPTX_BASE64:";

    // First try line-by-line
    for line in output.lines() {
        if let Some(base64_data) = line.strip_prefix(PREFIX) {
            let decoded = BASE64
                .decode(base64_data.trim())
                .context("base64 decode failed")?;
            return Ok(decoded);
        }
    }

    // Fallback: search for marker anywhere in output (handles missing newline)
    if let Some(start) = output.find(PREFIX) {
        let data_start = start + PREFIX.len();
        // Find end: next newline or "Kernel" or end of string
        let remaining = &output[data_start..];
        let end = remaining
            .find('\n')
            .or_else(|| remaining.find("Kernel"))
            .unwrap_or(remaining.len());
        let base64_data = &remaining[..end];

        let decoded = BASE64
            .decode(base64_data.trim())
            .context("base64 decode failed")?;
        return Ok(decoded);
    }

    // Show what we got so the user can diagnose Python errors
    let preview = if output.len() > 2000 {
        format!("{}...[truncated, {} bytes total]", &output[..2000], output.len())
    } else {
        output.to_string()
    };
    anyhow::bail!("PPTX_BASE64: marker not found in VM output:\n{}", preview)
}
