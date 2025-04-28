// msg/mod.rs for campaign

extern crate cosmwasm_std;
extern crate serde;
use self::cosmwasm_std::Addr;
use self::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub expiration: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    RecordEngagement { user: Addr },
    Claim {},
    AddTasks { tasks: Vec<String> }, // 💥 new field
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetCampaign {},
    HasEngaged { user: Addr },
}

