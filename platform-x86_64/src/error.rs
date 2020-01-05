
#[derive(Debug)]
pub enum X8664Error {
    UEFIError(uefi::Status)
}

impl <T: core::fmt::Debug> From<uefi::Error<T>> for X8664Error {
    fn from(error: uefi::Error<T>) -> X8664Error {
        X8664Error::UEFIError(error.status())
    }
}

impl From<uefi::Status> for X8664Error {
    fn from(status: uefi::Status) -> X8664Error {
        X8664Error::UEFIError(status)
    }
}
