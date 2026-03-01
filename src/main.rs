mod parser;
mod queries;
mod server;
mod tools;
mod types;

use std::path::PathBuf;

use clap::Parser;
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "code-search-mcp", about = "Tree-sitter powered code search MCP server")]
struct Args {
    /// Project root directory for resolving relative file paths
    #[arg(long, default_value = ".")]
    project_root: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("code_search_mcp=info".parse()?),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let args = Args::parse();

    let project_root = args.project_root.canonicalize().unwrap_or(args.project_root);
    tracing::info!(
        "Starting code-search-mcp with project root: {}",
        project_root.display()
    );

    let server = server::CodeSearchServer::new(project_root);

    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .inspect_err(|e| tracing::error!("Serve error: {e}"))?;

    service.waiting().await?;

    Ok(())
}
