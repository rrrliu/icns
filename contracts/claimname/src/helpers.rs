use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Deps, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};

use crate::{
    msg::{ExecuteMsg, QueryMsg},
    state::{State, STATE},
    ContractError,
};

pub fn must_get_ownernft_address(state: State) -> Result<Addr, ContractError> {
    state
        .ownernfts_address
        .ok_or(ContractError::NFTContractNotInitialized {})
}

// /// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
// /// for working with this.
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct CwTemplateContract(pub Addr);

// impl CwTemplateContract {
//     pub fn addr(&self) -> Addr {
//         self.0.clone()
//     }

//     pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
//         let msg = to_binary(&msg.into())?;
//         Ok(WasmMsg::Execute {
//             contract_addr: self.addr().into(),
//             msg,
//             funds: vec![],
//         }
//         .into())
//     }

//     /// Get Count
//     pub fn count<Q, T, CQ>(&self, querier: &Q) -> StdResult<GetCountResponse>
//     where
//         Q: Querier,
//         T: Into<String>,
//         CQ: CustomQuery,
//     {
//         let msg = QueryMsg::GetCount {};
//         let query = WasmQuery::Smart {
//             contract_addr: self.addr().into(),
//             msg: to_binary(&msg)?,
//         }
//         .into();
//         let res: GetCountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
//         Ok(res)
//     }
// }
