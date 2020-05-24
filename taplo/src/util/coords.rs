//! Utilities for mapping between offset:length and row:col character positions.

use rowan::{TextRange, TextSize};

pub use lsp_types::{Position, Range};

/// A mapper that translates offset:length positions to
/// row:col positions.
pub struct Mapper {
    lines: Vec<TextRange>,
    utf16: bool,
    utf16_mapping: Vec<u32>,
}

// TODO not all methods are UTF-16 aware.
impl Mapper {
    /// Creates a new Mapper that remembers where
    /// each line starts and ends.
    pub fn new(source: &str, utf16: bool) -> Self {
        let mut offset = 0;
        let mut total_len = 0;
        let mut lines = Vec::new();

        let mut total_len_utf16 = 0;
        let mut utf16_mapping = if utf16 {
            Vec::with_capacity(source.len() * 2)
        } else {
            Vec::new()
        };

        for c in source.chars() {
            total_len += c.len_utf8();
            if utf16 {
                utf16_mapping.push(total_len_utf16);
                total_len_utf16 += c.len_utf16() as u32;
            }

            if c == '\n' {
                lines.push(TextRange::new(
                    (offset as u32).into(),
                    (total_len as u32).into(),
                ));
                offset = total_len;
            }
        }
        utf16_mapping.push(total_len_utf16);

        if offset != total_len {
            lines.push(TextRange::new(
                (offset as u32).into(),
                (total_len as u32).into(),
            ));
        }

        Self {
            lines,
            utf16,
            utf16_mapping,
        }
    }

    pub fn lines(&self) -> &[TextRange] {
        &self.lines
    }

    pub fn offset(&self, position: Position) -> Option<TextSize> {
        self.lines
            .get(position.line as usize)
            .map(|l| (u32::from(l.start()) + position.character as u32).into())
    }

    pub fn text_range(&self, position: Range) -> Option<TextRange> {
        self.lines
            .get(position.start.line as usize)
            .map(|l| {
                TextRange::empty((u32::from(l.start()) + position.start.character as u32).into())
            })
            .and_then(|r| {
                self.lines.get(position.end.line as usize).map(|l| {
                    r.cover_offset((u32::from(l.start()) + position.end.character as u32).into())
                })
            })
    }

    pub fn position(&self, offset: TextSize) -> Option<Position> {
        for (line_idx, line_range) in self.lines.iter().enumerate() {
            if line_range.contains_inclusive(offset) {
                let mut p = Position {
                    line: line_idx as u64,
                    character: u32::from(offset - line_range.start()) as u64,
                };

                if self.utf16 {
                    p.character = self.utf16_mapping[p.character as usize] as u64;
                }

                return Some(p);
            }
        }

        None
    }

    pub fn range(&self, range: TextRange) -> Option<Range> {
        let mut r = Range::default();

        let mut start_found = false;
        let mut end_found = false;

        for (line_idx, line_range) in self.lines.iter().enumerate() {
            if line_range.contains_inclusive(range.start()) {
                r.start = Position {
                    line: line_idx as u64,
                    character: u32::from(range.start() - line_range.start()) as u64,
                };

                start_found = true;
            }

            if line_range.contains_inclusive(range.end()) {
                r.end = Position {
                    line: line_idx as u64,
                    character: u32::from(range.end() - line_range.start()) as u64,
                };

                end_found = true;
            }

            if start_found && end_found {
                if self.utf16 {
                    r.start.character = self.utf16_mapping[r.start.character as usize] as u64;
                    r.end.character = self.utf16_mapping[r.end.character as usize] as u64;
                }

                return Some(r);
            }
        }

        None
    }

    pub fn split_lines(&self, range: Range) -> Vec<Range> {
        if range.start.line == range.end.line {
            return vec![range];
        }

        let mut lines = Vec::with_capacity((range.end.line - range.start.line) as usize);

        let l = self.lines[range.start.line as usize];

        lines.push(Range {
            start: range.start,
            end: Position {
                line: range.start.line,
                character: (u32::from(l.end()) - u32::from(l.start())) as u64,
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
                    character: (u32::from(l.end()) - u32::from(l.start())) as u64,
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
            end: self
                .lines
                .last()
                .map(|l| Position {
                    line: ((self.lines.len()) as u64),
                    character: if self.utf16 {
                        (self.utf16_mapping[u32::from(l.start()) as usize]
                            .checked_sub(self.utf16_mapping[u32::from(l.end()) as usize])
                            .unwrap_or_default()) as u64
                    } else {
                        u32::from(l.start() - l.end()) as u64
                    },
                })
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

/// Ranges are relative to others' starts, not ends.
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
