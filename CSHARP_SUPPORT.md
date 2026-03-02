# C# Support Added to code-search-mcp

## Summary

C# language support has been successfully added to the code-search-mcp MCP server. The server can now parse and analyze C# (.cs) files using tree-sitter.

## Changes Made

### 1. Cargo.toml
- Added `tree-sitter-c-sharp = "0.23"` dependency

### 2. src/parser.rs
- Added `CSharp` variant to the `Lang` enum
- Added `.cs` file extension mapping in `Lang::from_path()`
- Added `csharp_queries` field to `ParseCache` struct
- Initialized C# queries in `ParseCache::new()`
- Added C# case to `queries()` method
- Added C# language support to the parser

### 3. src/queries.rs
- Added `csharp_queries()` function
- Added C# tree-sitter queries for:
  - **CSHARP_SYMBOLS**: Captures all top-level symbols including:
    - Classes, interfaces, structs, enums, records
    - Methods, constructors, properties, fields
    - Namespaces and delegates
  - **CSHARP_FUNCTION_BODY**: Captures method/constructor bodies and declarations
  - **CSHARP_CLASS_METHODS**: Captures methods within classes

### 4. src/tools/get_file_structure.rs
- Added `Lang::CSharp` case to the `impl_name_label` match expression
- Maps to `"class_name"` label (same as TypeScript)

### 5. README.md
- Updated "Supported Languages" section to include C#
- Updated "Dependencies" section to list tree-sitter-c-sharp

## Supported C# Constructs

The code-search-mcp now supports the following C# language constructs:

- **Classes** (`class_declaration`)
- **Interfaces** (`interface_declaration`)
- **Structs** (`struct_declaration`)
- **Enums** (`enum_declaration`)
- **Records** (`record_declaration`)
- **Namespaces** (`namespace_declaration`)
- **Methods** (`method_declaration`)
- **Constructors** (`constructor_declaration`)
- **Properties** (`property_declaration`)
- **Fields** (`field_declaration`)
- **Delegates** (`delegate_declaration`)

## Testing

A test C# file (`test.cs`) has been created with sample code to verify the functionality. The file includes:
- A namespace
- A class with properties, constructor, and methods
- An interface
- An enum

## Build Status

✅ Successfully built with `cargo build --release`

## Usage

The server now automatically detects and parses `.cs` files. All existing tools work with C# files:

1. **list_symbols** - Lists all C# symbols (classes, methods, properties, etc.)
2. **get_signature** - Gets C# method/property signatures
3. **get_definition** - Extracts full C# symbol definitions
4. **get_file_structure** - Shows hierarchical structure of C# files

## Example

```json
{
  "file": "GameCharacter.cs",
  "symbols": [
    {"name": "TestNamespace", "kind": "namespace_declaration", "line": 3},
    {"name": "GameCharacter", "kind": "class_declaration", "line": 5},
    {"name": "Name", "kind": "property_declaration", "line": 7},
    {"name": "Health", "kind": "property_declaration", "line": 8},
    {"name": "GameCharacter", "kind": "constructor_declaration", "line": 10},
    {"name": "TakeDamage", "kind": "method_declaration", "line": 16},
    {"name": "IsAlive", "kind": "method_declaration", "line": 24},
    {"name": "ISpellCaster", "kind": "interface_declaration", "line": 30},
    {"name": "CharacterClass", "kind": "enum_declaration", "line": 36}
  ]
}
```

## Next Steps

The C# support is fully functional and ready for use. To use it:

1. Build the project: `cargo build --release`
2. Configure the MCP server in your `.mcp.json` file
3. Use the server to analyze C# files in your projects
