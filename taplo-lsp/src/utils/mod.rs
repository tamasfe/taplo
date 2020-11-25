#[cfg(target_arch = "wasm32")]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub trait LspExt<T>: private::Sealed {
    fn into_lsp(self) -> T;
    fn from_lsp(val: T) -> Self;
}

impl private::Sealed for taplo::util::coords::Position {}
impl LspExt<lsp_types::Position> for taplo::util::coords::Position {
    fn into_lsp(self) -> lsp_types::Position {
        lsp_types::Position {
            line: self.line as u32,
            character: self.character as u32,
        }
    }

    fn from_lsp(val: lsp_types::Position) -> Self {
        Self {
            line: val.line as u64,
            character: val.character as u64,
        }
    }
}

impl private::Sealed for taplo::util::coords::Range {}
impl LspExt<lsp_types::Range> for taplo::util::coords::Range {
    fn into_lsp(self) -> lsp_types::Range {
        lsp_types::Range {
            start: self.start.into_lsp(),
            end: self.end.into_lsp(),
        }
    }

    fn from_lsp(val: lsp_types::Range) -> Self {
        Self {
            start: taplo::util::coords::Position::from_lsp(val.start),
            end: taplo::util::coords::Position::from_lsp(val.end),
        }
    }
}

mod private {
    pub trait Sealed {}
}
