pub mod rpc;

// We don't require futures to be `Send + Sync` in a WASM context
// as it is pointless and complicated things in some cases.

#[cfg(not(target_arch = "wasm32"))]
#[path = "impls/native/mod.rs"]
#[macro_use]
pub mod impls;

#[cfg(target_arch = "wasm32")]
#[path = "impls/wasm32/mod.rs"]
#[macro_use]
pub mod impls;

pub use impls::*;
