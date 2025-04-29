//! Cursor queries of a TOML document.

use taplo::{
    dom::{
        node::{DomNode, Key},
        FromSyntax, KeyOrIndex, Keys, Node,
    },
    rowan::{Direction, TextRange, TextSize},
    syntax::{SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::join_ranges,
};

#[derive(Debug, Default)]
pub struct Query {
    /// The offset the query was made for.
    pub offset: TextSize,
    /// Before the cursor.
    pub before: Option<PositionInfo>,
    /// After the cursor.
    pub after: Option<PositionInfo>,
}

impl Query {
    /// Query a DOM root with the given cursor offset.
    /// Returns [`None`] if the position is out of range.
    ///
    /// # Panics
    ///
    /// Panics if the DOM was not entirely constructed from a syntax tree (e.g. if a node has no associated syntax element).
    /// Also panics if the given DOM node is not root.
    ///
    /// Also the given offset must be within the tree.
    #[must_use]
    pub fn at(root: &Node, offset: TextSize) -> Self {
        let syntax = root.syntax().cloned().unwrap().into_node().unwrap();

        Query {
            offset,
            before: offset
                .checked_sub(TextSize::from(1))
                .and_then(|offset| Self::position_info_at(root, &syntax, offset)),
            after: if offset >= syntax.text_range().end() {
                None
            } else {
                Self::position_info_at(root, &syntax, offset)
            },
        }
    }

    fn position_info_at(
        root: &Node,
        syntax: &SyntaxNode,
        offset: TextSize,
    ) -> Option<PositionInfo> {
        let syntax = match syntax.token_at_offset(offset) {
            taplo::rowan::TokenAtOffset::None => return None,
            taplo::rowan::TokenAtOffset::Single(s) => s,
            taplo::rowan::TokenAtOffset::Between(_, right) => right,
        };

        Some(PositionInfo {
            syntax,
            dom_node: root
                .flat_iter()
                .filter(|(k, n)| full_range(k, n).contains(offset))
                .max_by_key(|(k, _)| k.len()),
        })
    }
}

impl Query {
    #[must_use]
    pub fn in_table_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let Some(header_syntax) = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| s.kind() == TABLE_HEADER)
                else {
                    return false;
                };

                if !after.syntax.parent_ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let Some(bracket_start) = header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_START {
                        t.into_token()
                    } else {
                        None
                    }
                }) else {
                    return false;
                };

                let Some(bracket_end) = header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) else {
                    return false;
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
        }
    }

    #[must_use]
    pub fn in_table_array_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let Some(header_syntax) = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| s.kind() == TABLE_ARRAY_HEADER)
                else {
                    return false;
                };

                if !after.syntax.parent_ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let Some(bracket_start) = header_syntax
                    .children_with_tokens()
                    .filter_map(|t| {
                        if t.kind() == BRACKET_START {
                            t.into_token()
                        } else {
                            None
                        }
                    })
                    .nth(1)
                else {
                    return false;
                };

                let Some(bracket_end) = header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) else {
                    return false;
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
        }
    }

    #[must_use]
    pub fn header_key(&self) -> Option<SyntaxNode> {
        match (&self.before, &self.after) {
            (Some(before), _) => {
                let header_syntax = before
                    .syntax
                    .parent_ancestors()
                    .find(|s| matches!(s.kind(), TABLE_ARRAY_HEADER | TABLE_HEADER))?;

                header_syntax.descendants().find(|n| n.kind() == KEY)
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn entry_key(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let keys = syntax
            .parent_ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == KEY))?;

        Some(keys)
    }

    #[must_use]
    pub fn entry_value(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let value = syntax
            .parent_ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == VALUE))?;

        Some(value)
    }

    #[must_use]
    pub fn parent_table_or_array_table(&self, root: &Node) -> (Keys, Node) {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(s) => s.syntax.clone(),
            None => return (Keys::empty(), root.clone()),
        };

        let last_header = root
            .syntax()
            .unwrap()
            .as_node()
            .unwrap()
            .descendants()
            .skip(1)
            .filter(|n| matches!(n.kind(), TABLE_HEADER | TABLE_ARRAY_HEADER))
            .take_while(|n| n.text_range().end() <= syntax.text_range().end())
            .last();

        let Some(last_header) = last_header else {
            return (Keys::empty(), root.clone());
        };

        let keys = Keys::from_syntax(
            last_header
                .descendants()
                .find(|n| n.kind() == KEY)
                .unwrap()
                .into(),
        );
        let node = root.path(&keys).unwrap();

        (keys, node)
    }

    #[must_use]
    pub fn empty_line(&self) -> bool {
        let before_syntax = match self.before.as_ref() {
            Some(s) => &s.syntax,
            None => return true,
        };

        match &self.after {
            Some(after) => {
                if matches!(after.syntax.kind(), WHITESPACE | NEWLINE) {
                    let new_line_after = after
                        .syntax
                        .siblings_with_tokens(Direction::Next)
                        .find_map(|s| match s.kind() {
                            NEWLINE => Some(true),
                            WHITESPACE | COMMENT => None,
                            _ => Some(false),
                        })
                        .unwrap_or(true);

                    if !new_line_after {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            None => {}
        }

        before_syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                NEWLINE => Some(true),
                WHITESPACE | COMMENT => None,
                _ => Some(false),
            })
            .unwrap_or(true)
    }

    #[must_use]
    pub fn in_entry_keys(&self) -> bool {
        self.entry_key()
            .is_some_and(|k| k.text_range().contains(self.offset))
    }

    #[must_use]
    pub fn entry_has_eq(&self) -> bool {
        let Some(key_syntax) = self.entry_key() else {
            return false;
        };

        key_syntax
            .siblings(Direction::Next)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE => None,
                _ => Some(false),
            })
            .unwrap_or(false)
    }

    #[must_use]
    pub fn in_entry_value(&self) -> bool {
        let in_value = self
            .entry_value()
            // We are inside the value even if the cursor is right after it.
            .is_some_and(|k| k.text_range().contains_inclusive(self.offset));

        if in_value {
            return true;
        }

        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .siblings_with_tokens(Direction::Prev)
            .find_map(|s| match s.kind() {
                EQ => Some(true),
                WHITESPACE | COMMENT | NEWLINE => None,
                _ => Some(false),
            })
            .unwrap_or(false)
    }

    #[must_use]
    pub fn is_single_quote_value(&self) -> bool {
        self.entry_value().is_some_and(|v| {
            v.descendants_with_tokens()
                .any(|t| matches!(t.kind(), STRING_LITERAL | MULTI_LINE_STRING_LITERAL))
        })
    }

    #[must_use]
    pub fn is_inline(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .parent_ancestors()
            .any(|a| matches!(a.kind(), INLINE_TABLE | ARRAY))
    }

    #[must_use]
    pub fn in_inline_table(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != INLINE_TABLE {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACE_END {
                            Some(self.offset <= t.text_range().start())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true)
            }
            None => false,
        }
    }

    #[must_use]
    pub fn in_array(&self) -> bool {
        let syntax = match self.before.as_ref().or(self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        match syntax.parent() {
            Some(parent) => {
                if parent.kind() != ARRAY {
                    return false;
                }

                parent
                    .children_with_tokens()
                    .find_map(|t| {
                        if t.kind() == BRACKET_END {
                            Some(self.offset <= t.text_range().start())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true)
            }
            None => false,
        }
    }

    pub fn entry_keys(&self) -> Keys {
        self.entry_key()
            .map_or_else(Keys::empty, |keys| Keys::from_syntax(keys.into()))
    }

    pub fn header_keys(&self) -> Keys {
        self.header_key()
            .map_or_else(Keys::empty, |keys| Keys::from_syntax(keys.into()))
    }

    #[must_use]
    pub fn dom_node(&self) -> Option<&(Keys, Node)> {
        self.before
            .as_ref()
            .and_then(|p| p.dom_node.as_ref())
            .or_else(|| self.after.as_ref().and_then(|p| p.dom_node.as_ref()))
    }
}

/// Transform the lookup keys to account for arrays of tables and arrays.
///
/// It appends an index after each array so that we get the item type
/// during lookups.
#[must_use]
pub fn lookup_keys(root: Node, keys: &Keys) -> Keys {
    let mut node = root;
    let mut new_keys = Keys::empty();

    for key in keys.iter().cloned() {
        node = node.get(&key);
        new_keys = new_keys.join(key);
        if let Some(arr) = node.as_array() {
            new_keys = new_keys.join(arr.items().read().len().saturating_sub(1));
        }
    }

    new_keys
}

#[derive(Debug, Clone)]
pub struct PositionInfo {
    /// The narrowest syntax element that contains the position.
    pub syntax: SyntaxToken,
    /// The narrowest node that covers the position.
    pub dom_node: Option<(Keys, Node)>,
}

fn full_range(keys: &Keys, node: &Node) -> TextRange {
    let Some(last_key) = keys
        .iter()
        .filter_map(KeyOrIndex::as_key)
        .next_back()
        .map(Key::text_ranges)
    else {
        return join_ranges(node.text_ranges(true));
    };

    join_ranges(last_key.chain(node.text_ranges(true)))
}
