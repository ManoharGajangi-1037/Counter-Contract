use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IncrementMsg{
    pub num_add: u64,
}


// Enum to represent different message types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
   Increment(IncrementMsg)
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CountResponse)] GetCount {
    },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct CountResponse {
    pub count: u64,
}