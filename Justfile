build:
    cd crates/taplo-cli && cargo build -F lsp -Fcargo_toml

run:
    cd crates/taplo-cli && cargo run -F lsp -Fcargo_toml lsp tcp

install:
    cd crates/taplo-cli && cargo install -F lsp -Fcargo_toml --path .
