use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("error is ==============")]
    Std(#[from] StdError),
    #[error("Didn't instantiate ")]
    InstateError {},
}
