/// Date kinds.
#[derive(clap::ValueEnum, Clone, Default)]
pub enum DateKind {
    /// Now defaulting to a Rfc3339 format.
    #[default]
    Now,
    /// Now defaulting to a Timestamp format.
    Timestamp,
    /// Now defaulting to a `Rfc3339Date` format.
    Today,
    /// Yesterday defaulting to a `Rfc3339Date` format.
    Yesterday,
    /// Tomorrow defaulting to a `Rfc3339Date` format.
    Tomorrow,
}
