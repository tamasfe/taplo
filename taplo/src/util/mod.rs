pub mod coords;
mod escape;

pub use escape::check_escape;
pub use escape::unescape;

pub(crate) mod allowed_chars {
    pub(crate) fn comment(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && (c >= '\u{0000}' && c <= '\u{0008}'
                    || c >= '\u{000A}' && c <= '\u{001F}'
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn multi_line_string(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t'
                && c != '\n'
                && c != '\r'
                && (c >= '\u{0000}' && c <= '\u{0008}'
                    || c >= '\u{000A}' && c <= '\u{001F}'
                    || c == '\u{007F}')
            {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }

    pub(crate) fn multi_line_string_literal(s: &str) -> Result<(), Vec<usize>> {
        let mut err_indices = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c != '\t' && c != '\n' && c != '\r' && c.is_control() {
                err_indices.push(i);
            }
        }

        if err_indices.is_empty() {
            Ok(())
        } else {
            Err(err_indices)
        }
    }
}

pub trait StringExt {
    fn remove_prefix<'a>(&'a self, p: &str) -> &'a str;
    fn remove_suffix<'a>(&'a self, p: &str) -> &'a str;
}
impl StringExt for &str {
    fn remove_prefix<'a>(&'a self, p: &str) -> &'a str {
        if self.starts_with(p) {
            &self[p.len()..]
        } else {
            self
        }
    }

    fn remove_suffix<'a>(&'a self, p: &str) -> &'a str {
        if self.ends_with(p) {
            &self[..self.len() - p.len()]
        } else {
            self
        }
    }
}
