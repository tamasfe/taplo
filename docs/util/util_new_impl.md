# util.new_impl

- **Type:** function
- **Language:** rust
- **File:** `\crates\lsp-async-stub\src\util.rs` (lines 114-163)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Callers

None

## Callees

None
