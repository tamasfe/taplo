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
    pub fn in_table_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let header_syntax =
                    match before.syntax.ancestors().find(|s| s.kind() == TABLE_HEADER) {
                        Some(h) => h,
                        None => return false,
                    };

                if !after.syntax.ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let bracket_start = match header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_START {
                        t.into_token()
                    } else {
                        None
                    }
                }) {
                    Some(t) => t,
                    None => return false,
                };

                let bracket_end = match header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) {
                    Some(t) => t,
                    None => return false,
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
        }
    }

    pub fn in_table_array_header(&self) -> bool {
        match (&self.before, &self.after) {
            (Some(before), Some(after)) => {
                let header_syntax = match before
                    .syntax
                    .ancestors()
                    .find(|s| s.kind() == TABLE_ARRAY_HEADER)
                {
                    Some(h) => h,
                    None => return false,
                };

                if !after.syntax.ancestors().any(|a| a == header_syntax) {
                    return false;
                }

                let bracket_start = match header_syntax
                    .children_with_tokens()
                    .filter_map(|t| {
                        if t.kind() == BRACKET_START {
                            t.into_token()
                        } else {
                            None
                        }
                    })
                    .nth(1)
                {
                    Some(t) => t,
                    None => return false,
                };

                let bracket_end = match header_syntax.children_with_tokens().find_map(|t| {
                    if t.kind() == BRACKET_END {
                        t.into_token()
                    } else {
                        None
                    }
                }) {
                    Some(t) => t,
                    None => return false,
                };

                (before.syntax == bracket_start
                    || before.syntax.text_range().start() >= bracket_start.text_range().end())
                    && (after.syntax == bracket_end
                        || after.syntax.text_range().end() <= bracket_end.text_range().start())
            }
            _ => false,
        }
    }

    pub fn header_key(&self) -> Option<SyntaxNode> {
        match (&self.before, &self.after) {
            (Some(before), _) => {
                let header_syntax = match before
                    .syntax
                    .ancestors()
                    .find(|s| matches!(s.kind(), TABLE_ARRAY_HEADER | TABLE_HEADER))
                {
                    Some(h) => h,
                    None => return None,
                };

                header_syntax.descendants().find(|n| n.kind() == KEY)
            }
            _ => None,
        }
    }

    pub fn entry_key(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let keys = match syntax
            .ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == KEY))
        {
            Some(keys) => keys,
            None => return None,
        };

        Some(keys)
    }

    pub fn entry_value(&self) -> Option<SyntaxNode> {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return None,
        };

        let value = match syntax
            .ancestors()
            .find(|n| n.kind() == ENTRY)
            .and_then(|entry| entry.children().find(|c| c.kind() == VALUE))
        {
            Some(value) => value,
            None => return None,
        };

        Some(value)
    }

    pub fn parent_table_or_array_table(&self, root: &Node) -> (Keys, Node) {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
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

        let last_header = match last_header {
            Some(h) => h,
            None => return (Keys::empty(), root.clone()),
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

    pub fn in_entry_keys(&self) -> bool {
        self.entry_key()
            .map(|k| k.text_range().contains(self.offset))
            .unwrap_or(false)
    }

    pub fn entry_has_eq(&self) -> bool {
        let key_syntax = match self.entry_key() {
            Some(p) => p,
            None => return false,
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

    pub fn in_entry_value(&self) -> bool {
        let in_value = self
            .entry_value()
            // We are inside the value even if the cursor is right after it.
            .map(|k| k.text_range().contains_inclusive(self.offset))
            .unwrap_or(false);

        if in_value {
            return true;
        }

        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
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

    pub fn is_inline(&self) -> bool {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
            Some(p) => &p.syntax,
            None => return false,
        };

        syntax
            .ancestors()
            .any(|a| matches!(a.kind(), INLINE_TABLE | ARRAY))
    }

    pub fn in_inline_table(&self) -> bool {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
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

    pub fn in_array(&self) -> bool {
        let syntax = match self.before.as_ref().or_else(|| self.after.as_ref()) {
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
            .map(|keys| Keys::from_syntax(keys.into()))
            .unwrap_or_else(Keys::empty)
    }

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
    let last_key = match keys
        .iter()
        .filter_map(KeyOrIndex::as_key)
        .last()
        .map(Key::text_ranges)
    {
        Some(k) => k,
        None => {
            return join_ranges(node.text_ranges());
        }
    };

    join_ranges(last_key.chain(node.text_ranges()))
}
