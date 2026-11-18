# mod.format_rows

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\formatter\mod.rs` (lines 1233-1308)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn format_rows<R, S>(
    align_range: Range<usize>,
    separator_range: Range<usize>,
    rows: &[R],
    newline: &str,
    separator: &str,
) -> String
where
    R: AsRef<[S]>,
    S: AsRef<str>,
{
    let mut out = String::new();

    // We currently don't support vertical alignment of complex data.
    let can_align = rows
        .iter()
        .flat_map(|r| r.as_ref().iter())
        .all(|s| !s.as_ref().contains('\n'));

    let diff_widths = |range: Range<usize>, row: &R| -> usize {
        let mut max_width = 0_usize;

        for row in rows {
            let row_len = row.as_ref().len();

            let range =
                cmp::min(range.start, row_len.saturating_sub(1))..cmp::min(range.end, row_len);

            max_width = cmp::max(
                max_width,
                row.as_ref()[range]
                    .iter()
                    .map(|s| s.as_ref().chars().count())
                    .sum(),
            );
        }

        let row_width = row.as_ref()[range]
            .iter()
            .map(|s| s.as_ref().chars().count())
            .sum::<usize>();

        max_width - row_width
    };

    for (row_idx, row) in rows.iter().enumerate() {
        if row_idx != 0 {
            out += newline;
        }

        let mut last_align_idx = 0_usize;

        for (item_idx, item) in row.as_ref().iter().enumerate() {
            if item_idx > separator_range.start
                && item_idx <= separator_range.end.saturating_add(1)
                && item_idx < row.as_ref().len()
            {
                out += separator;
            }

            out += item.as_ref();

            if can_align
                && align_range.start <= item_idx
                && align_range.end > item_idx
                && item_idx < row.as_ref().len() - 1
            {
                let diff = diff_widths(last_align_idx..item_idx + 1, row);
                out.extend(repeat(" ").take(diff));
                last_align_idx = item_idx + 1;
            }
        }
    }

    out
}
```

## Docstring

_No docstring provided._

## Source

```
fn format_rows<R, S>(
    align_range: Range<usize>,
    separator_range: Range<usize>,
    rows: &[R],
    newline: &str,
    separator: &str,
) -> String
where
    R: AsRef<[S]>,
    S: AsRef<str>,
{
    let mut out = String::new();

    // We currently don't support vertical alignment of complex data.
    let can_align = rows
        .iter()
        .flat_map(|r| r.as_ref().iter())
        .all(|s| !s.as_ref().contains('\n'));

    let diff_widths = |range: Range<usize>, row: &R| -> usize {
        let mut max_width = 0_usize;

        for row in rows {
            let row_len = row.as_ref().len();

            let range =
                cmp::min(range.start, row_len.saturating_sub(1))..cmp::min(range.end, row_len);

            max_width = cmp::max(
                max_width,
                row.as_ref()[range]
                    .iter()
                    .map(|s| s.as_ref().chars().count())
                    .sum(),
            );
        }

        let row_width = row.as_ref()[range]
            .iter()
            .map(|s| s.as_ref().chars().count())
            .sum::<usize>();

        max_width - row_width
    };

    for (row_idx, row) in rows.iter().enumerate() {
        if row_idx != 0 {
            out += newline;
        }

        let mut last_align_idx = 0_usize;

        for (item_idx, item) in row.as_ref().iter().enumerate() {
            if item_idx > separator_range.start
                && item_idx <= separator_range.end.saturating_add(1)
                && item_idx < row.as_ref().len()
            {
                out += separator;
            }

            out += item.as_ref();

            if can_align
                && align_range.start <= item_idx
                && align_range.end > item_idx
                && item_idx < row.as_ref().len() - 1
            {
                let diff = diff_widths(last_align_idx..item_idx + 1, row);
                out.extend(repeat(" ").take(diff));
                last_align_idx = item_idx + 1;
            }
        }
    }

    out
}
```

## Git History

- Commits: 17
- Authors: 14
- Bug fixes: 5
- Features: 4

## Callers

None

## Callees

None
