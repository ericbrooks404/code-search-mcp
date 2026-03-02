# Code Search MCP Server

A high-performance Model Context Protocol (MCP) server that provides semantic code intelligence using tree-sitter parsing. This tool enables efficient code exploration and analysis without reading entire files.

## Features

The server provides four powerful tools for code analysis:

### 1. `list_symbols`
Lists all symbols (functions, structs, enums, traits, etc.) in a file with their line numbers and types.

**Much more efficient than reading the entire file** - only returns symbol metadata.

**Parameters:**
- `file` - Relative file path from project root (e.g., "server/src/lib.rs")

**Example output:**
```json
[
  {
    "name": "compute_movement_speed",
    "kind": "function",
    "line": 42
  },
  {
    "name": "Player",
    "kind": "struct",
    "line": 15
  }
]
```

### 2. `get_signature`
Gets the signature of a function or declaration without its body. Returns function signatures, struct definition headers, etc.

**~95% fewer tokens than reading the file** - perfect for understanding APIs without implementation details.

**Parameters:**
- `file` - Relative file path from project root
- `name` - Symbol name to search for (e.g., "compute_movement_speed")

**Example output:**
```rust
pub fn compute_movement_speed(player: &Player, terrain: TerrainType) -> f32
```

### 3. `get_definition`
Gets the full source code of a specific named symbol (function, struct, enum, trait, impl, const, etc.).

**Returns only that symbol's code, not the surrounding file** - ideal for focused code review.

**Parameters:**
- `file` - Relative file path from project root
- `name` - Symbol name to search for

**Example output:**
```rust
pub fn compute_movement_speed(player: &Player, terrain: TerrainType) -> f32 {
    let base_speed = player.stats.speed;
    let terrain_modifier = terrain.speed_modifier();
    base_speed * terrain_modifier
}
```

### 4. `get_file_structure`
Gets a hierarchical overview of a file's structure showing modules, impl blocks, functions, structs, etc. with nesting.

**~85% fewer tokens than reading the file** - great for understanding file organization.

**Parameters:**
- `file` - Relative file path from project root

**Example output:**
```json
{
  "modules": [
    {
      "name": "combat",
      "items": [
        {"kind": "struct", "name": "Weapon"},
        {"kind": "function", "name": "calculate_damage"}
      ]
    }
  ]
}
```

## Installation

### Build from source

```bash
cd tools/code-search-mcp
cargo build --release
```

The compiled binary will be at `target/release/code-search-mcp`.

## Usage

### As an MCP Server

Configure the server in your MCP client configuration (e.g., `.mcp.json`):

```json
{
  "mcpServers": {
    "code-search": {
      "type": "stdio",
      "command": "/path/to/code-search-mcp/target/release/code-search-mcp",
      "args": [
        "--project-root",
        "/path/to/your/project"
      ],
      "env": {}
    }
  }
}
```

### Command Line Arguments

- `--project-root <PATH>` - Project root directory for resolving relative file paths (default: ".")

### Standalone Usage

```bash
# Use current directory as project root
./code-search-mcp

# Specify a project root
./code-search-mcp --project-root /path/to/project
```

## Supported Languages

Currently supports:
- **Rust** (.rs files)
- **TypeScript/JavaScript** (.ts, .tsx files)
- **C#** (.cs files)

The tool uses tree-sitter parsers for accurate semantic understanding of code structure.

## Performance Benefits

| Tool | Token Reduction | Use Case |
|------|----------------|----------|
| `list_symbols` | ~99% | Discover what's in a file |
| `get_signature` | ~95% | Understand function APIs |
| `get_file_structure` | ~85% | Grasp file organization |
| `get_definition` | Varies | Extract specific symbols |

Instead of reading entire files (thousands of tokens), you can get exactly the information you need with minimal token usage.

## Architecture

- **Parser**: Tree-sitter based parsing with file caching for performance
- **Server**: MCP-compliant stdio-based server using the `rmcp` library
- **Queries**: Tree-sitter queries for semantic code analysis
- **Caching**: Automatic parse tree caching to avoid redundant parsing

## Dependencies

- `rmcp` - MCP server framework
- `tree-sitter` - Parser generator tool and incremental parsing library
- `tree-sitter-rust` - Rust grammar for tree-sitter
- `tree-sitter-typescript` - TypeScript/JavaScript grammar
- `tree-sitter-c-sharp` - C# grammar for tree-sitter
- `tokio` - Async runtime
- `dashmap` - Concurrent hash map for caching

## Development

### Running Tests

```bash
cargo test
```

### Adding Language Support

To add support for a new language:

1. Add the tree-sitter grammar dependency to `Cargo.toml`
2. Update the parser to handle the new file extension
3. Add tree-sitter queries for the language in `src/queries.rs`
4. Update the tool implementations to handle the new language constructs

## License

See the project root for license information.
