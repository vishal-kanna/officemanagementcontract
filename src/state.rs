use cw_storage_plus::Item;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
pub struct User {
    pub username: String,
    pub age: u64,
    pub address: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
pub enum Role {
    HR,
    Manager,
    Lead,
    #[default]
    Employee,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
pub struct UserDetails {
    pub uid: u64,
    pub username: String,
    pub age: u64,
    pub address: String,
    pub role: Role,
}
pub const HR: Item<UserDetails> = Item::new("hr");
pub const USERS: Map<u64, UserDetails> = Map::new("user");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
