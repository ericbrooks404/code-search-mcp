use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: String,
    pub line: usize,
    pub end_line: usize,
}

#[derive(Debug, Serialize)]
pub struct StructureItem {
    pub name: String,
    pub kind: String,
    pub line: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<StructureItem>,
}

#[derive(Debug, Serialize)]
pub struct FileStructure {
    pub file: String,
    pub items: Vec<StructureItem>,
}
