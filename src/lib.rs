pub (crate) const VERSION: &'static str = "0.0.0";

mod data;
mod http;
pub mod argument;

pub use self::http::download_to_writer;
pub use self::http::download_to_tx;