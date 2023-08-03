use crate::state::{Role, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstatiateMsg {
    pub hr: User,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]

pub enum ExecuteMsg {
    AddEmployee {
        name: String,
        age: u64,
        address: String,
        role: Role,
    },
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]

pub enum QueryMsg {
    GetEmployess {},
    GetEmployee { uid: u64 },
    GetSuperAdmin {},
}
