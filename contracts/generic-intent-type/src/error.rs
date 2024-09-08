use ckb_std::error::SysError;

/// Error
#[repr(i8)]
#[derive(Debug, PartialEq)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,

    MoleculeVerification,
    IntentDataMissing,
    InvalidTypeIDCellNum, // There can only be at most one input and at most one output type ID cell
    TypeIDNotMatch,       // Type id does not match args
    IntentDataUnmatched,
    CheckFailed,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<molecule::error::VerificationError> for Error {
    fn from(_err: molecule::error::VerificationError) -> Self {
        Self::MoleculeVerification
    }
}
