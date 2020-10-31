//! Utilities for mapping between offset:length bytes and col:row character positions.

use rowan::{TextRange, TextSize};

pub use lsp_types::{Position, Range};
use std::iter;

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
    /// These are characters, not byte offsets.
    lines: Vec<CharacterRange>,

    /// A character position mapped to each byte offset.
    /// If there was a single character that is 3 bytes long,
    /// then this will contain 3 elements that are zero each.
    mapping: Vec<CharacterOffset>,
}

impl Mapper {
    /// Creates a new Mapper that remembers where
    /// each line starts and ends.
    pub fn new(source: &str) -> Self {
        let mut line_start_char = 0;

        let mut chars_count = 0;

        let mut lines = Vec::with_capacity(512); // a guess
        let mut mapping = Vec::with_capacity(source.len() * 4); // We assume the worst case

        for (i, c) in source.chars().enumerate() {
            // 1-based char offset.
            let char_offset = (i + 1) as u64;
            mapping.extend(iter::repeat(char_offset).take(c.len_utf8()));

            if c == '\n' {
                // LF is at the start of each line.
                lines.push(CharacterRange(line_start_char + 1, char_offset));
                line_start_char = char_offset;
            }

            chars_count = i as u64 + 1;
        }

        if mapping.is_empty() {
            mapping.push(1);
        }

        if line_start_char <= chars_count {
            lines.push(CharacterRange(line_start_char + 1, chars_count + 1));
        }

        Self { lines, mapping }
    }

    pub fn lines(&self) -> &[CharacterRange] {
        &self.lines
    }

    pub fn mapping(&self) -> &[CharacterOffset] {
        &self.mapping
    }

    pub fn offset(&self, position: Position) -> Option<TextSize> {
        self.lines()
            .get(position.line.checked_sub(1).expect("lines must be 1-based") as usize)
            .and_then(|l| {
                let idx = l.0
                    + position
                        .character
                        .checked_sub(1)
                        .expect("characters must be 1-based");

                self.mapping
                    .iter()
                    .enumerate()
                    .find_map(|(i, p)| {
                        if *p == idx {
                            Some(TextSize::from(i as u32))
                        } else {
                            None
                        }
                    })
                    .or_else(|| Some(((self.mapping.len() - 1) as u32).into()))
            })
    }

    pub fn text_range(&self, range: Range) -> Option<TextRange> {
        self.offset(range.start)
            .and_then(|start| self.offset(range.end).map(|end| TextRange::new(start, end)))
    }

    pub fn position(&self, offset: TextSize) -> Option<Position> {
        self.mapping.get(u32::from(offset) as usize).and_then(|c| {
            self.lines
                .iter()
                .enumerate()
                .find(|(_, line)| line.0 <= *c && line.1 >= *c)
                .map(|(line_idx, line)| Position {
                    line: line_idx as u64,
                    character: *c - line.0,
                })
        })
    }

    pub fn range(&self, range: TextRange) -> Option<Range> {
        self.position(range.start())
            .and_then(|start| self.position(range.end()).map(|end| Range { start, end }))
    }

    pub fn split_lines(&self, range: Range) -> Vec<Range> {
        if range.start.line == range.end.line {
            return vec![range];
        }

        let mut lines = Vec::with_capacity((range.end.line - range.start.line) as usize);

        let start_line = self.lines[range.start.line as usize];

        lines.push(Range {
            start: range.start,
            end: Position {
                line: range.start.line,
                character: start_line.1 - start_line.0,
            },
        });

        for i in (range.start.line + 1)..range.end.line {
            let l = self.lines[i as usize];
            lines.push(Range {
                start: Position {
                    line: i,
                    character: 0,
                },
                end: Position {
                    line: i,
                    character: l.1 - l.0,
                },
            })
        }

        lines.push(Range {
            start: Position {
                line: range.end.line,
                character: 0,
            },
            end: range.end,
        });

        lines
    }

    pub fn all_range(&self) -> Range {
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: self.end(),
        }
    }

    pub fn end(&self) -> Position {
        Position {
            line: (self.lines.len() as u64).checked_sub(1).unwrap_or_default(),
            character: self.lines.last().map(|l| l.1 - l.0).unwrap_or_default(),
        }
    }
}

/// This trait is used for splitting a range into multiple
/// single-line ranges.
///
/// This was originally needed because in VSCode semantic tokens
/// could not span across multiple lines.
pub trait SplitLines {
    fn is_single_line(&self) -> bool;
    fn split_lines(self, mapper: &Mapper) -> Vec<Range>;
}
impl SplitLines for Range {
    fn split_lines(self, mapper: &Mapper) -> Vec<Range> {
        mapper.split_lines(self)
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
