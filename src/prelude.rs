//! Crate prelude

// Re-export the crate Error.
pub use crate::utils::error::Error;

// Alias Result to be the crate Result.
// pub type TResult<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
// pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;
