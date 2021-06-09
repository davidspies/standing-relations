#![feature(map_first_last, type_alias_impl_trait)]

mod convenience;
mod core;
mod feedback;

pub use self::convenience::Collection;
pub use self::core::*;
pub use self::feedback::{Feedback, FeedbackContext, Interrupter};

#[cfg(test)]
mod tests;
