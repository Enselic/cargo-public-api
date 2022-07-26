#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The API diff is not allowed as per --deny")]
    DiffDenied,

    #[error("The API diff is not allowed as per --deny=additions")]
    DiffAddedDenied,

    #[error("The API diff is not allowed as per --deny=changes")]
    DiffChangedDenied,

    #[error("The API diff is not allowed as per --deny=deletions")]
    DiffDeletionsDenied,
}
