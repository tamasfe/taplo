Performs "thin local LTO" which performs "thin" LTO on the local
crate only across its [codegen units](https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units). No LTO is performed
if codegen units is 1 or [opt-level](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level) is 0.