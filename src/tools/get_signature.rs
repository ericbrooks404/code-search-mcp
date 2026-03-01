use std::path::Path;

use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCursor;

use crate::parser::{Lang, ParseCache};

pub fn get_signature(cache: &ParseCache, path: &Path, symbol_name: &str) -> Result<Option<String>> {
    let lang = Lang::from_path(path)
        .ok_or_else(|| anyhow::anyhow!("unsupported file type: {}", path.display()))?;
    let (tree, source) = cache.parse(path)?;
    let queries = cache.queries(lang);

    let name_idx = queries
        .function_body
        .capture_index_for_name("name")
        .expect("query missing @name capture");
    let def_idx = queries
        .function_body
        .capture_index_for_name("def")
        .expect("query missing @def capture");
    let body_idx = queries.function_body.capture_index_for_name("body");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&queries.function_body, tree.root_node(), source.as_slice());

    while let Some(m) = matches.next() {
        let mut found_name = String::new();
        let mut def_start = 0;
        let mut def_end = 0;
        let mut body_start: Option<usize> = None;

        for cap in m.captures {
            if cap.index == name_idx {
                found_name = cap.node.utf8_text(&source).unwrap_or("").to_string();
            } else if cap.index == def_idx {
                def_start = cap.node.start_byte();
                def_end = cap.node.end_byte();
            } else if Some(cap.index) == body_idx {
                body_start = Some(cap.node.start_byte());
            }
        }

        if found_name == symbol_name {
            let sig_end = body_start.unwrap_or(def_end);
            let signature = std::str::from_utf8(&source[def_start..sig_end])
                .unwrap_or("")
                .trim_end()
                .to_string();
            return Ok(Some(signature));
        }
    }

    Ok(None)
}
