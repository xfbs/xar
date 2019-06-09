#[cfg(test)]
mod tests;

mod error {
    use error_chain::error_chain;
    error_chain! {}
}

mod header;
mod archive;
pub use header::{Header};
