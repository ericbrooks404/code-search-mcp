use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use dashmap::DashMap;
use tree_sitter::{Parser, Tree};

use crate::queries::{self, LangQueries};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Rust,
    TypeScript,
}

impl Lang {
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "rs" => Some(Lang::Rust),
            "ts" | "tsx" => Some(Lang::TypeScript),
            _ => None,
        }
    }
}

struct CachedParse {
    tree: Tree,
    source: Vec<u8>,
    mtime: SystemTime,
}

impl std::fmt::Debug for ParseCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseCache")
            .field("cached_files", &self.cache.len())
            .finish()
    }
}

pub struct ParseCache {
    cache: DashMap<PathBuf, CachedParse>,
    rust_queries: LangQueries,
    ts_queries: LangQueries,
}

impl ParseCache {
    pub fn new() -> Self {
        Self {
            cache: DashMap::new(),
            rust_queries: queries::rust_queries(tree_sitter_rust::LANGUAGE),
            ts_queries: queries::typescript_queries(tree_sitter_typescript::LANGUAGE_TYPESCRIPT),
        }
    }

    pub fn queries(&self, lang: Lang) -> &LangQueries {
        match lang {
            Lang::Rust => &self.rust_queries,
            Lang::TypeScript => &self.ts_queries,
        }
    }

    /// Parse (or return cached) file. Returns (tree, source bytes).
    pub fn parse(&self, path: &Path) -> Result<(Tree, Vec<u8>)> {
        let mtime = std::fs::metadata(path)
            .and_then(|m| m.modified())
            .context("failed to stat file")?;

        // Check cache
        if let Some(entry) = self.cache.get(path) {
            if entry.mtime == mtime {
                return Ok((entry.tree.clone(), entry.source.clone()));
            }
        }

        let source = std::fs::read(path).context("failed to read file")?;
        let lang = Lang::from_path(path).context("unsupported file type")?;

        let mut parser = Parser::new();
        let ts_lang = match lang {
            Lang::Rust => tree_sitter_rust::LANGUAGE.into(),
            Lang::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        };
        parser.set_language(&ts_lang).context("failed to set language")?;

        let tree = parser
            .parse(&source, None)
            .context("tree-sitter parse failed")?;

        let result = (tree.clone(), source.clone());

        self.cache.insert(
            path.to_path_buf(),
            CachedParse {
                tree,
                source,
                mtime,
            },
        );

        Ok(result)
    }
}
