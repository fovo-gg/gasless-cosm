// error/mod.rs for campaign

extern crate cosmwasm_std;
extern crate thiserror;
use self::cosmwasm_std::StdError;
use self::thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

