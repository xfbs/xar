#[cfg(test)]
mod tests;

mod error {
    use error_chain::error_chain;
    error_chain! {}
}

pub mod archive;
pub mod header;
pub mod toc;
pub use archive::Archive;
pub use header::Header;
pub use toc::Toc;
