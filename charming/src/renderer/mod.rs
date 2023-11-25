#[cfg(feature = "html")]
#[cfg_attr(docsrs, doc(cfg(feature = "html")))]
pub mod html_renderer;
#[cfg(feature = "ssr")]
#[cfg_attr(docsrs, doc(cfg(feature = "ssr")))]
pub mod image_renderer;
#[cfg(feature = "wasm")]
#[cfg_attr(docsrs, doc(cfg(feature = "wasm")))]
pub mod wasm_renderer;

#[cfg(feature = "html")]
#[cfg_attr(docsrs, doc(cfg(feature = "html")))]
pub use html_renderer::*;
#[cfg(feature = "ssr")]
#[cfg_attr(docsrs, doc(cfg(feature = "ssr")))]
pub use image_renderer::*;
#[cfg(feature = "wasm")]
#[cfg_attr(docsrs, doc(cfg(feature = "wasm")))]
pub use wasm_renderer::*;
