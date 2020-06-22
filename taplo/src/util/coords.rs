//! Utilities for mapping between offset:length bytes and col:row character positions.

use rowan::{TextRange, TextSize};

pub use lsp_types::{Position, Range};
use std::iter;

/// Offset in characters instead of bytes.
/// It is u64 because lsp_types uses u64.
pub type CharacterOffset = u64;

/// Offset range in characters instead of bytes.
pub type CharacterRange = std::ops::Range<CharacterOffset>;

/// A mapper that translates offset:length bytes to
/// col:row characters.
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

        let mut total_chars = 0;

        let mut lines = Vec::with_capacity(512); // a guess
        let mut mapping = Vec::with_capacity(source.len() * 4); // We assume the worst case

        for (i, c) in source.chars().enumerate() {
            mapping.extend(iter::repeat(i as u64).take(c.len_utf8()));

            if c == '\n' {
                lines.push(line_start_char..i as u64 + 1);
                line_start_char = i as u64 + 1;
            }

            total_chars = i as u64;
        }

        if line_start_char < total_chars + 1 {
            lines.push(line_start_char..total_chars as u64 + 1);
        } else {
            // last empty line
            let last_mapping = mapping.last().copied().unwrap_or_default() + 1;
            lines.push(last_mapping..last_mapping + 1);
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
        self.lines().get(position.line as usize).map(|l| {
            let idx = (l.start + position.character)
                .checked_sub(1)
                .unwrap_or_default();

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
                .unwrap_or(TextSize::from(self.mapping.len() as u32)) // Last empty line
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
                .find(|(_, line)| line.start <= *c && line.end > *c)
                .map(|(line_idx, line)| Position {
                    line: line_idx as u64,
                    character: *c - line.start,
                })
        })
    }

    /// A convenience method for finding range endings.
    /// Since ranges are exclusive, one must be subtracted for endings.
    pub fn position_end(&self, offset: TextSize) -> Option<Position> {
        self.position(offset.checked_sub(TextSize::from(1)).unwrap_or_default())
    }

    pub fn range(&self, range: TextRange) -> Option<Range> {
        // Special case for a 0-length range
        if range.start() == range.end() {
            return self
                .mapping
                .get(u32::from(range.start()) as usize)
                .or_else(|| self.mapping.last())
                .and_then(|c| {
                    self.lines().iter().enumerate().find_map(|(i, l)| {
                        if l.start <= *c && l.end >= *c {
                            Some(Range {
                                start: Position {
                                    line: i as u64,
                                    character: (*c).checked_sub(1).unwrap_or_default(),
                                },
                                end: Position {
                                    line: i as u64,
                                    character: *c,
                                },
                            })
                        } else {
                            None
                        }
                    })
                });
        }

        self.position(range.start()).and_then(|start| {
            self.position_end(range.end()).map(|mut end| {
                end.character += 1;
                Range { start, end }
            })
        })
    }

    pub fn split_lines(&self, range: Range) -> Vec<Range> {
        if range.start.line == range.end.line {
            return vec![range];
        }

        let mut lines = Vec::with_capacity((range.end.line - range.start.line) as usize);

        let start_line = self.lines[range.start.line as usize].clone();

        lines.push(Range {
            start: range.start,
            end: Position {
                line: range.start.line,
                character: start_line.end - start_line.start,
            },
        });

        for i in (range.start.line + 1)..range.end.line {
            let l = self.lines[i as usize].clone();
            lines.push(Range {
                start: Position {
                    line: i,
                    character: 0,
                },
                end: Position {
                    line: i,
                    character: l.end - l.start,
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
            character: self
                .lines
                .last()
                .map(|l| l.end - l.start)
                .unwrap_or_default(),
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
