use thiserror::Error;
use crate::syntax::SyntaxElement;
use super::node::Key;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("the syntax was not expected here: {syntax:#?}")]
    UnexpectedSyntax { syntax: SyntaxElement },
    #[error("the string contains invalid escape sequence(s)")]
    InvalidEscapeSequence { string: SyntaxElement },
    #[error("conflicting keys")]
    ConflictingKeys { key: Key, other: Key },
    #[error("expected table")]
    ExpectedTable { not_table: Key, required_by: Key },
    #[error("expected array of tables")]
    ExpectedArrayOfTables { not_array_of_tables: Key, required_by: Key },
    #[error("{0}")]
    Query(QueryError)
}

#[derive(Debug, Clone, Error)]
pub enum QueryError {
    #[error("the key or index was not found: {key}")]
    NotFound { key: String }
}