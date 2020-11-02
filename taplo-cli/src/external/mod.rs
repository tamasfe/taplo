// #[cfg(target_arch = "wasm32")]
mod wasm;

// #[cfg(target_arch = "wasm32")]
pub use wasm::*;

// #[cfg(not(target_arch = "wasm32"))]
// mod native;

// #[cfg(not(target_arch = "wasm32"))]
// pub(crate) use native::*;