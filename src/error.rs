use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("error is ")]
    Std(#[from] StdError),
    #[error("Didn't instantiate ")]
    InstateError {},
    #[error("UserDidn't Found")]
    UserNotFound {},
    #[error("Sender needs to be the one who applied leave")]
    SenderNotMatched {},
    #[error("Sender is the the super admin")]
    NotSuperAdmin{},
}
