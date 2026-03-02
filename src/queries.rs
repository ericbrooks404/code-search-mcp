use tree_sitter::Query;
use tree_sitter_language::LanguageFn;

/// Pre-compiled query sets for a language.
pub struct LangQueries {
    pub symbols: Query,
    pub function_body: Query,
    pub impl_methods: Query,
}

pub fn rust_queries(lang: LanguageFn) -> LangQueries {
    let ts_lang = lang.into();
    LangQueries {
        symbols: Query::new(&ts_lang, RUST_SYMBOLS).expect("invalid rust symbols query"),
        function_body: Query::new(&ts_lang, RUST_FUNCTION_BODY)
            .expect("invalid rust function_body query"),
        impl_methods: Query::new(&ts_lang, RUST_IMPL_METHODS)
            .expect("invalid rust impl_methods query"),
    }
}

pub fn typescript_queries(lang: LanguageFn) -> LangQueries {
    let ts_lang = lang.into();
    LangQueries {
        symbols: Query::new(&ts_lang, TS_SYMBOLS).expect("invalid ts symbols query"),
        function_body: Query::new(&ts_lang, TS_FUNCTION_BODY)
            .expect("invalid ts function_body query"),
        impl_methods: Query::new(&ts_lang, TS_CLASS_METHODS)
            .expect("invalid ts class_methods query"),
    }
}

pub fn csharp_queries(lang: LanguageFn) -> LangQueries {
    let ts_lang = lang.into();
    LangQueries {
        symbols: Query::new(&ts_lang, CSHARP_SYMBOLS).expect("invalid csharp symbols query"),
        function_body: Query::new(&ts_lang, CSHARP_FUNCTION_BODY)
            .expect("invalid csharp function_body query"),
        impl_methods: Query::new(&ts_lang, CSHARP_CLASS_METHODS)
            .expect("invalid csharp class_methods query"),
    }
}

// ── Rust queries ──

const RUST_SYMBOLS: &str = r#"[
  (function_item name: (identifier) @name) @def
  (struct_item name: (type_identifier) @name) @def
  (enum_item name: (type_identifier) @name) @def
  (trait_item name: (type_identifier) @name) @def
  (impl_item trait: (type_identifier)? @trait_name type: (type_identifier) @name) @def
  (type_item name: (type_identifier) @name) @def
  (const_item name: (identifier) @name) @def
  (static_item name: (identifier) @name) @def
  (mod_item name: (identifier) @name) @def
]"#;

const RUST_FUNCTION_BODY: &str = r#"[
  (function_item name: (identifier) @name body: (block) @body) @def
  (const_item name: (identifier) @name) @def
  (static_item name: (identifier) @name) @def
  (struct_item name: (type_identifier) @name) @def
  (enum_item name: (type_identifier) @name) @def
  (trait_item name: (type_identifier) @name) @def
  (impl_item type: (type_identifier) @name) @def
  (type_item name: (type_identifier) @name) @def
  (mod_item name: (identifier) @name) @def
]"#;

const RUST_IMPL_METHODS: &str = r#"
(impl_item
  type: (type_identifier) @impl_name
  body: (declaration_list
    (function_item name: (identifier) @method_name) @method_def))
"#;

// ── TypeScript queries ──

const TS_SYMBOLS: &str = r#"[
  (function_declaration name: (identifier) @name) @def
  (class_declaration name: (type_identifier) @name) @def
  (interface_declaration name: (type_identifier) @name) @def
  (type_alias_declaration name: (type_identifier) @name) @def
  (enum_declaration name: (identifier) @name) @def
  (lexical_declaration
    (variable_declarator name: (identifier) @name)) @def
  (export_statement
    declaration: (function_declaration name: (identifier) @name)) @def
  (export_statement
    declaration: (class_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (interface_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (type_alias_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (enum_declaration name: (identifier) @name)) @def
  (export_statement
    declaration: (lexical_declaration
      (variable_declarator name: (identifier) @name))) @def
]"#;

const TS_FUNCTION_BODY: &str = r#"[
  (function_declaration name: (identifier) @name body: (statement_block) @body) @def
  (class_declaration name: (type_identifier) @name) @def
  (interface_declaration name: (type_identifier) @name) @def
  (type_alias_declaration name: (type_identifier) @name) @def
  (enum_declaration name: (identifier) @name) @def
  (lexical_declaration
    (variable_declarator name: (identifier) @name)) @def
  (export_statement
    declaration: (function_declaration name: (identifier) @name body: (statement_block) @body)) @def
  (export_statement
    declaration: (class_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (interface_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (type_alias_declaration name: (type_identifier) @name)) @def
  (export_statement
    declaration: (enum_declaration name: (identifier) @name)) @def
  (export_statement
    declaration: (lexical_declaration
      (variable_declarator name: (identifier) @name))) @def
]"#;

const TS_CLASS_METHODS: &str = r#"
(class_declaration
  name: (type_identifier) @class_name
  body: (class_body
    (method_definition name: (property_identifier) @method_name) @method_def))
"#;

// ── C# queries ──

const CSHARP_SYMBOLS: &str = r#"[
  (class_declaration name: (identifier) @name) @def
  (interface_declaration name: (identifier) @name) @def
  (struct_declaration name: (identifier) @name) @def
  (enum_declaration name: (identifier) @name) @def
  (record_declaration name: (identifier) @name) @def
  (namespace_declaration name: (identifier) @name) @def
  (method_declaration name: (identifier) @name) @def
  (constructor_declaration name: (identifier) @name) @def
  (property_declaration name: (identifier) @name) @def
  (field_declaration
    (variable_declaration
      (variable_declarator name: (identifier) @name))) @def
  (delegate_declaration name: (identifier) @name) @def
]"#;

const CSHARP_FUNCTION_BODY: &str = r#"[
  (method_declaration name: (identifier) @name body: (block)? @body) @def
  (constructor_declaration name: (identifier) @name body: (block)? @body) @def
  (class_declaration name: (identifier) @name) @def
  (interface_declaration name: (identifier) @name) @def
  (struct_declaration name: (identifier) @name) @def
  (enum_declaration name: (identifier) @name) @def
  (record_declaration name: (identifier) @name) @def
  (property_declaration name: (identifier) @name) @def
  (field_declaration
    (variable_declaration
      (variable_declarator name: (identifier) @name))) @def
  (delegate_declaration name: (identifier) @name) @def
]"#;

const CSHARP_CLASS_METHODS: &str = r#"
(class_declaration
  name: (identifier) @class_name
  body: (declaration_list
    (method_declaration name: (identifier) @method_name) @method_def))
"#;
