//! Utilities for mapping between offset:length bytes and col:row character positions.

use rowan::{TextRange, TextSize};

use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
pub struct Position {
    /// Line position in a document (could be zero-based or one-based based on the usage).
    pub line: u64,
    /// Character offset on a line in a document (could be zero-based or one-based based on the usage).
    pub character: u64,
}

impl Position {
    pub fn new(line: u64, character: u64) -> Self {
        Position { line, character }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

/// Offset in characters instead of bytes.
/// It is u64 because lsp_types uses u64.
pub type CharacterOffset = u64;

/// Inclusive offset range in characters instead of bytes.
#[derive(Debug, Clone, Copy)]
pub struct CharacterRange(u64, u64);

/// A mapper that translates offset:length bytes to
/// 1-based line:row characters.
#[derive(Debug, Clone)]
pub struct Mapper {
    /// Mapping offsets to positions.
    offset_to_position: BTreeMap<TextSize, Position>,

    /// Mapping positions to offsets.
    position_to_offset: BTreeMap<Position, TextSize>,

    /// Line count.
    lines: usize,

    /// Ending position.
    end: Position,
}

impl Mapper {
    /// Creates a new Mapper that remembers where
    /// each line starts and ends.
    ///
    /// Uses UTF-16 character sizes for positions.
    pub fn new_utf16(source: &str, one_based: bool) -> Self {
        Self::new_impl(source, true, if one_based { 1 } else { 0 })
    }

    /// Uses UTF-8 character sizes for positions.
    pub fn new_utf8(source: &str, one_based: bool) -> Self {
        Self::new_impl(source, false, if one_based { 1 } else { 0 })
    }

    pub fn offset(&self, position: Position) -> Option<TextSize> {
        self.position_to_offset.get(&position).copied()
    }

    pub fn text_range(&self, range: Range) -> Option<TextRange> {
        self.offset(range.start)
            .and_then(|start| self.offset(range.end).map(|end| TextRange::new(start, end)))
    }

    pub fn position(&self, offset: TextSize) -> Option<Position> {
        self.offset_to_position.get(&offset).copied()
    }

    pub fn range(&self, range: TextRange) -> Option<Range> {
        self.position(range.start())
            .and_then(|start| self.position(range.end()).map(|end| Range { start, end }))
    }

    pub fn mappings(&self) -> (&BTreeMap<TextSize, Position>, &BTreeMap<Position, TextSize>) {
        (&self.offset_to_position, &self.position_to_offset)
    }

    pub fn line_count(&self) -> usize {
        self.lines
    }

    pub fn all_range(&self) -> Range {
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: self.end,
        }
    }

    fn new_impl(source: &str, utf16: bool, base: u64) -> Self {
        let mut offset_to_position = BTreeMap::new();
        let mut position_to_offset = BTreeMap::new();

        let mut line: u64 = base;
        let mut character: u64 = base;
        let mut last_offset = 0;

        for c in source.chars() {
            let new_offset = last_offset + c.len_utf8();

            let character_size = if utf16 { c.len_utf16() } else { 1 };

            offset_to_position.extend(
                (last_offset..new_offset)
                    .map(|b| (TextSize::from(b as u32), Position { line, character })),
            );

            position_to_offset.extend(
                (last_offset..new_offset)
                    .map(|b| (Position { line, character }, TextSize::from(b as u32))),
            );

            last_offset = new_offset;

            character += character_size as u64;
            if c == '\n' {
                // LF is at the start of each line.
                line += 1;
                character = base;
            }
        }

        // Last imaginary character.
        offset_to_position.insert(
            TextSize::from(last_offset as u32),
            Position { line, character },
        );
        position_to_offset.insert(
            Position { line, character },
            TextSize::from(last_offset as u32),
        );

        Self {
            offset_to_position,
            position_to_offset,
            lines: line as usize,
            end: Position { line, character },
        }
    }
}

/// This trait is used for splitting a range into multiple
/// single-line ranges.
///
/// This is needed because in VSCode semantic tokens
/// could not span across multiple lines.
pub trait SplitLines {
    fn is_single_line(&self) -> bool;
    fn split_lines(self, mapper: &Mapper) -> Vec<Range>;
}
impl SplitLines for Range {
    fn split_lines(self, _mapper: &Mapper) -> Vec<Range> {
        unimplemented!()
    }

    fn is_single_line(&self) -> bool {
        self.start.line == self.end.line
    }
}

pub fn relative_position(position: Position, to: Position) -> Position {
    if position.line == to.line {
        Position {
            line: 0,
            character: position.character - to.character,
        }
    } else {
        Position {
            line: position.line - to.line,
            character: position.character,
        }
    }
}

/// Ranges are relative start to start, not end to start.
pub fn relative_range(range: Range, to: Range) -> Range {
    let line_diff = range.end.line - range.start.line;
    let start = relative_position(range.start, to.start);

    let end = if line_diff == 0 {
        Position {
            line: start.line,
            character: start.character + range.end.character - range.start.character,
        }
    } else {
        Position {
            line: start.line + line_diff,
            character: range.end.character,
        }
    };

    Range { start, end }
}

#[cfg(test)]
#[test]
fn test_mapper() {
    let s1 = r#"
line-2
line-3"#;

    let mapper = Mapper::new_utf16(s1, false);

    assert!(s1.len() == mapper.mappings().0.len() - 1);

    assert!(
        mapper.position(0.into()).unwrap()
            == Position {
                line: 0,
                character: 0
            }
    );

    assert!(
        mapper
            .position(TextSize::from(s1.len() as u32 - 1 as u32))
            .unwrap()
            == Position {
                line: 2,
                character: 5
            }
    )
}
