use std::path::PathBuf;
use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{schemars, tool, tool_handler, tool_router, ServerHandler};

use crate::parser::ParseCache;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FileParam {
    #[schemars(description = "Relative file path from project root (e.g. \"server/src/lib.rs\")")]
    pub file: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FileAndNameParam {
    #[schemars(description = "Relative file path from project root (e.g. \"server/src/lib.rs\")")]
    pub file: String,
    #[schemars(description = "Symbol name to search for (e.g. \"compute_movement_speed\")")]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CodeSearchServer {
    project_root: PathBuf,
    cache: Arc<ParseCache>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl CodeSearchServer {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            cache: Arc::new(ParseCache::new()),
            tool_router: Self::tool_router(),
        }
    }

    fn resolve_path(&self, file: &str) -> PathBuf {
        self.project_root.join(file)
    }

    #[tool(
        name = "list_symbols",
        description = "List all symbols (functions, structs, enums, traits, etc.) in a file with their line numbers and types. Much more efficient than reading the entire file."
    )]
    fn list_symbols(&self, Parameters(params): Parameters<FileParam>) -> String {
        let path = self.resolve_path(&params.file);
        match crate::tools::list_symbols::list_symbols(&self.cache, &path) {
            Ok(symbols) => serde_json::to_string_pretty(&symbols).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(
        name = "get_signature",
        description = "Get the signature of a function or declaration without its body. Returns the function signature, struct definition header, etc. ~95% fewer tokens than reading the file."
    )]
    fn get_signature(&self, Parameters(params): Parameters<FileAndNameParam>) -> String {
        let path = self.resolve_path(&params.file);
        match crate::tools::get_signature::get_signature(&self.cache, &path, &params.name) {
            Ok(Some(sig)) => sig,
            Ok(None) => format!("Symbol '{}' not found in {}", params.name, params.file),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(
        name = "get_definition",
        description = "Get the full source code of a specific named symbol (function, struct, enum, trait, impl, const, etc.). Returns only that symbol's code, not the surrounding file."
    )]
    fn get_definition(&self, Parameters(params): Parameters<FileAndNameParam>) -> String {
        let path = self.resolve_path(&params.file);
        match crate::tools::get_definition::get_definition(&self.cache, &path, &params.name) {
            Ok(Some(def)) => def,
            Ok(None) => format!("Symbol '{}' not found in {}", params.name, params.file),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(
        name = "get_file_structure",
        description = "Get a hierarchical overview of a file's structure showing modules, impl blocks, functions, structs, etc. with nesting. ~85% fewer tokens than reading the file."
    )]
    fn get_file_structure(&self, Parameters(params): Parameters<FileParam>) -> String {
        let path = self.resolve_path(&params.file);
        match crate::tools::get_file_structure::get_file_structure(&self.cache, &path) {
            Ok(structure) => serde_json::to_string_pretty(&structure).unwrap_or_default(),
            Err(e) => format!("Error: {e}"),
        }
    }
}

#[tool_handler]
impl ServerHandler for CodeSearchServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Code search server providing semantic code intelligence via tree-sitter. \
                 Use list_symbols to discover what's in a file, get_signature for function \
                 signatures without bodies, get_definition for full symbol source code, \
                 and get_file_structure for hierarchical file overviews."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
