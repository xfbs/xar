#[cfg(test)]
mod tests;

mod error {
    use error_chain::error_chain;
    error_chain! {
    }
}

mod header;
mod archive;
mod toc;
pub use header::{Header};
pub use toc::Toc;
pub use archive::Archive;
