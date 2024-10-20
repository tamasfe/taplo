build:
    cd crates/taplo-cli && cargo build -F lsp -Fcargo_toml

run:
    cd crates/taplo-cli && cargo run -F lsp -Fcargo_toml lsp tcp
