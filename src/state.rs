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
    // pub leaves: Vec<leavetype>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]

pub struct LeaveReq {
    pub id: u64,
    pub leavetypeid:u64,
    pub from: String,
    pub to: String,
    pub status: String,
    pub reason: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]

pub struct UserDetails1 {
    pub uid: u64,
    pub username: String,
    pub age: u64,
    pub address: String,
    pub role: Role,
    pub leaves:Vec<leavetype1>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]
pub struct leavetype{
    pub types:String,
    pub count:u64,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, JsonSchema)]

pub struct leavetype1{
    pub id :u64,
    pub types:String,
    pub count:u64,
}
pub const HR: Item<UserDetails> = Item::new("hr");
pub const USERS: Map<u64, UserDetails> = Map::new("user");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LEAVE_SEQ: Item<u128> = Item::new("leaveseq");
pub const LEAVE_LIST: Map<u128, LeaveReq> = Map::new("leaves");
pub const USERS_LEAVES :Map<u64,UserDetails1>=Map::new("all");
pub const LEAVE_TYPES:Map<u64,leavetype>=Map::new("leavetype");
pub const LEAVETYPE_SEQ: Item<u64> = Item::new("leavetypeseq");

