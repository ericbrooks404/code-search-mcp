use std::path::Path;

use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::QueryCursor;

use crate::parser::{Lang, ParseCache};
use crate::types::{FileStructure, StructureItem};

pub fn get_file_structure(cache: &ParseCache, path: &Path) -> Result<FileStructure> {
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

    let mut items: Vec<StructureItem> = Vec::new();

    while let Some(m) = matches.next() {
        let mut name = String::new();
        let mut kind = String::new();
        let mut line = 0;

        for cap in m.captures {
            if cap.index == name_idx {
                name = cap.node.utf8_text(&source).unwrap_or("").to_string();
            } else if cap.index == def_idx {
                kind = cap.node.kind().to_string();
                line = cap.node.start_position().row + 1;
            }
        }

        if name.is_empty() {
            continue;
        }

        // Deduplicate
        if items.iter().any(|i| i.name == name && i.kind == kind && i.line == line) {
            continue;
        }

        // For impl blocks and classes, find child methods
        let children = if kind == "impl_item" || kind == "class_declaration" {
            get_methods(cache, &source, &tree, lang, &name)
        } else {
            Vec::new()
        };

        items.push(StructureItem {
            name,
            kind,
            line,
            children,
        });
    }

    Ok(FileStructure {
        file: path.display().to_string(),
        items,
    })
}

fn get_methods(
    cache: &ParseCache,
    source: &[u8],
    tree: &tree_sitter::Tree,
    lang: Lang,
    parent_name: &str,
) -> Vec<StructureItem> {
    let queries = cache.queries(lang);

    let impl_name_label = match lang {
        Lang::Rust => "impl_name",
        Lang::TypeScript => "class_name",
    };
    let impl_name_idx = queries.impl_methods.capture_index_for_name(impl_name_label);
    let method_name_idx = queries.impl_methods.capture_index_for_name("method_name");
    let method_def_idx = queries.impl_methods.capture_index_for_name("method_def");

    let (impl_name_idx, method_name_idx, method_def_idx) =
        match (impl_name_idx, method_name_idx, method_def_idx) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => return Vec::new(),
        };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&queries.impl_methods, tree.root_node(), source);

    let mut methods = Vec::new();
    while let Some(m) = matches.next() {
        let mut impl_name = String::new();
        let mut method_name = String::new();
        let mut method_line = 0;
        let mut method_kind = String::new();

        for cap in m.captures {
            if cap.index == impl_name_idx {
                impl_name = cap.node.utf8_text(source).unwrap_or("").to_string();
            } else if cap.index == method_name_idx {
                method_name = cap.node.utf8_text(source).unwrap_or("").to_string();
            } else if cap.index == method_def_idx {
                method_line = cap.node.start_position().row + 1;
                method_kind = cap.node.kind().to_string();
            }
        }

        if impl_name == parent_name && !method_name.is_empty() {
            methods.push(StructureItem {
                name: method_name,
                kind: method_kind,
                line: method_line,
                children: Vec::new(),
            });
        }
    }

    methods
}
