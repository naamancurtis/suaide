use thiserror::Error;

#[derive(Error, Debug)]
pub enum SuaideError {
    #[error("Sub-command not found")]
    SubCommandNotFound,

    #[error("Incorrect arguments passed")]
    IncorrectArgs,

    #[error("Task not found")]
    NotFound,

    #[error(transparent)]
    ConnectionError(#[from] diesel::result::ConnectionError),

    #[error(transparent)]
    StorageError(#[from] diesel::result::Error),

    #[error(transparent)]
    ArgumentFormatError(#[from] chrono::ParseError),

    #[error(transparent)]
    InputError(#[from] std::io::Error),

    #[error(transparent)]
    MigrationError(#[from] diesel_migrations::RunMigrationsError),
}
