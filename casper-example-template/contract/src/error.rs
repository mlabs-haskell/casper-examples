use casper_types::ApiError;

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.

#[repr(u16)]
pub enum Error {
    AlredayDeployed = 0,
    AlreadyInitialized = 1,
    UserAlreadyRegistered = 2,
    UnregisteredTriedToAdd = 3,
    ValueKeyNotFound = 4,
    RegistrationMapNotFound = 5,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}
