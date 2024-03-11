use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Execute(ExecMsg),
}

#[cw_serde]
pub struct ExecMsg {}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(QueryResponse)]
    Query(QMsg),
}

#[cw_serde]
pub struct QMsg {}

#[cw_serde]
pub struct QueryResponse {}

#[cw_serde]
pub struct MigrateMsg {}
