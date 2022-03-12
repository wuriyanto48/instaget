pub (crate) const VERSION: &'static str = "1.0.2";

mod data;
mod http;
pub mod argument;

pub use self::http::download_to_writer;
pub use self::http::download_to_tx;