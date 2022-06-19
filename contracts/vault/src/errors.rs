use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum VaultError {
    TestError = 0,
}

impl From<VaultError> for ApiError {
    fn from(e: VaultError) -> Self {
        ApiError::User(e as u16)
    }
}
