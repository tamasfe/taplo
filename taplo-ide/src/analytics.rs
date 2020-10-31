use crate::Document;
use dom::RootNode;
use lsp_types::Position;
use rowan::{TextRange, TextSize};
use taplo::{
    dom::{self, NodeSyntax},
    syntax::{SyntaxKind, SyntaxNode},
    util::SyntaxExt,
};


