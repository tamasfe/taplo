# Architecture: taplo

## What this project is
taplo is a Rust-based project focused on providing language server protocol (LSP) support for TOML files. It includes a CLI tool for formatting and linting TOML files, as well as an LSP server for text editors.

## How it is laid out
The project is organized into several directories, each serving a specific purpose:
- `lsp-async-stub/`: Contains the LSP server implementation.
- `taplo-cli/`: Houses the command-line interface for formatting and linting TOML files.
- `taplo-common/`: Includes common utilities and data structures used across the project.
- `taplo-lsp/`: Contains the LSP-specific logic and handlers.
- `taplo-wasm/`: Likely contains WebAssembly-related code, though not detailed in the provided facts.
- `taplo/`: Includes benchmarking and example code.

## Key modules
| Module Name | Path | Commits | Bug Fixes |
|-------------|------|---------|-----------|
| format | taplo-cli/src/commands/format.rs | 19 | 6 |
| associations | taplo-common/src/schema/associations.rs | 16 | 7 |
| args | taplo-cli/src/args.rs | 14 | 1 |
| world | taplo-lsp/src/world.rs | 13 | 4 |
| node | taplo/src/dom/node.rs | 8 | 2 |
| queries | taplo-cli/src/commands/queries.rs | 8 | 0 |
| completion | taplo-lsp/src/handlers/completion.rs | 7 | 3 |
| hover | taplo-lsp/src/handlers/hover.rs | 7 | 3 |
| formatting | taplo-lsp/src/handlers/formatting.rs | 6 | 2 |
| query | taplo-lsp/src/query.rs | 6 | 1 |
| workspaces | taplo-lsp/src/handlers/workspaces.rs | 5 | 2 |
| cache | taplo-common/src/schema/cache.rs | 4 | 0 |
| diagnostics | taplo-lsp/src/diagnostics.rs | 4 | 0 |
| document_symbols | taplo-lsp/src/handlers/document_symbols.rs | 4 | 0 |
| documents | taplo-lsp/src/handlers/documents.rs | 4 | 2 |
| environment | taplo-common/src/environment.rs | 4 | 1 |
| lib | lsp-async-stub/src/lib.rs | 4 | 2 |
| lint | taplo-cli/src/commands/lint.rs | 4 | 1 |
| mod | taplo/src/util/mod.rs | 4 | 0 |
| listen | lsp-async-stub/src/listen.rs | 3 | 0 |
| lsp | taplo-cli/src/commands/lsp.rs | 3 | 0 |
| native | taplo-common/src/environment/native.rs | 3 | 1 |
| printing | taplo-cli/src/printing.rs | 3 | 1 |
| rename | taplo-lsp/src/handlers/rename.rs | 3 | 0 |
| schema | taplo-lsp/src/handlers/schema.rs | 3 | 1 |
| semantic_tokens | taplo-lsp/src/handlers/semantic_tokens.rs | 3 | 1 |
| configuration | taplo-lsp/src/handlers/configuration.rs | 2 | 1 |
| conversion | taplo-lsp/src/handlers/conversion.rs | 2 | 0 |
| convert | taplo-common/src/convert.rs | 2 | 0 |
| escape | taplo/src/util/escape.rs | 2 | 0 |
| folding_ranges | taplo-lsp/src/handlers/folding_ranges.rs | 2 | 0 |
| index | taplo/src/dom/index.rs | 2 | 1 |
| initialize | taplo-lsp/src/handlers/initialize.rs | 2 | 1 |
| links | taplo-lsp/src/handlers/links.rs | 2 | 0 |
| log | taplo-common/src/log.rs | 2 | 0 |
| profile | taplo/benches/profile.rs | 2 | 0 |
| rewrite | taplo/src/dom/rewrite.rs | 2 | 1 |
| serde | taplo/src/dom/serde.rs | 2 | 0 |
| shared | taplo/src/util/shared.rs | 2 | 1 |
| taplo | taplo/benches/taplo.rs | 2 | 0 |

## Public API surface
The project exposes several entry points:
- `taplo-cli/src/commands/format.rs`: Entry point for formatting TOML files.
- `taplo-cli/src/commands/lint.rs`: Entry point for linting TOML files.
- `taplo-lsp/src/handlers/completion.rs`: Handles code completion requests.
- `taplo-lsp/src/handlers/hover.rs`: Handles hover requests.

## Hotspots and gotchas
The following modules have the highest churn and the most bug-fix commits, indicating areas that may be more prone to issues:
- `format` (taplo-cli/src/commands/format