use std::path::Path;

use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCursor;

use crate::parser::{Lang, ParseCache};
use crate::types::SymbolInfo;

pub fn list_symbols(cache: &ParseCache, path: &Path) -> Result<Vec<SymbolInfo>> {
    let lang = Lang::from_path(path)
        .ok_or_else(|| anyhow::anyhow!("unsupported file type: {}", path.display()))?;
    let (tree, source) = cache.parse(path)?;
    let queries = cache.queries(lang);

    let name_idx = queries
        .symbols
        .capture_index_for_name("name")
        .expect("query missing @name capture");
    let def_idx = queries
        .symbols
        .capture_index_for_name("def")
        .expect("query missing @def capture");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&queries.symbols, tree.root_node(), source.as_slice());

    let mut symbols = Vec::new();
    while let Some(m) = matches.next() {
        let mut name = String::new();
        let mut kind = String::new();
        let mut line = 0;
        let mut end_line = 0;

        for cap in m.captures {
            if cap.index == name_idx {
                name = cap.node.utf8_text(&source).unwrap_or("").to_string();
            } else if cap.index == def_idx {
                kind = cap.node.kind().to_string();
                line = cap.node.start_position().row + 1;
                end_line = cap.node.end_position().row + 1;
            }
        }

        if !name.is_empty() {
            symbols.push(SymbolInfo {
                name,
                kind,
                line,
                end_line,
            });
        }
    }

    Ok(symbols)
}
