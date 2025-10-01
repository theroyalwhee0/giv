/// Output trait for values that can be displayed in both plain and JSON formats.
mod output_trait;
/// Output handler for formatting and displaying values.
mod outputer;

pub use output_trait::Output;
pub use outputer::Outputer;
