#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult };
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::COUNT_SEQ;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sample-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg
) -> Result<Response, ContractError> {
    COUNT_SEQ.save(_deps.storage, &0u64)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Increment(_increment_msg) => {
            execute_increment(_deps, _env, _info, _increment_msg)
        }
    }
}

pub fn execute_increment(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: IncrementMsg
) -> Result<Response, ContractError> {
    // Load the current count from storage
    let mut count = COUNT_SEQ.may_load(_deps.storage)?.unwrap_or(0);

    // Add the new count to be added
    count += _msg.num_add;

    // Save the updated count back into storage
    COUNT_SEQ.save(_deps.storage, &count)?;

    Ok(Response::new().add_attribute("method", "increment"))
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg{
        QueryMsg::GetCount {}=>{
            to_binary(&query_count(_deps,_env)?)
        }
    }
}pub fn query_count(deps:Deps,env: Env)->StdResult<CountResponse>{
    let count=COUNT_SEQ.load(deps.storage)?;
    Ok((CountResponse{count}))
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract with default params
        let msg = InstantiateMsg {};
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        // Ensure the response contains the correct event attribute
        assert_eq!(res.attributes[0].value, "instantiate");

        // Query the count and ensure it's set to 0
        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response: CountResponse = from_binary(&query_res).unwrap();
        assert_eq!(count_response.count, 0);
    }

    #[test]
    fn increment_count() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract
        let init_msg = InstantiateMsg {};
        let _ = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

        // Increment the count by 5
        let increment_msg = IncrementMsg { num_add: 5 };
        let execute_msg = ExecuteMsg::Increment(increment_msg);
        let res = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

        // Ensure the response contains the correct event attribute
        assert_eq!(res.attributes[0].value, "increment");

        // Query the count and ensure it's updated to 5
        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response: CountResponse = from_binary(&query_res).unwrap();
        assert_eq!(count_response.count, 5);
    }

    #[test]
    fn multiple_increments() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract
        let init_msg = InstantiateMsg {};
        let _ = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

        // First increment by 3
        let increment_msg = IncrementMsg { num_add: 3 };
        let execute_msg = ExecuteMsg::Increment(increment_msg);
        let _ = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

        // Second increment by 7
        let increment_msg = IncrementMsg { num_add: 7 };
        let execute_msg = ExecuteMsg::Increment(increment_msg);
        let _ = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

        // Query the count and ensure it's updated to 10
        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response: CountResponse = from_binary(&query_res).unwrap();
        assert_eq!(count_response.count, 10);
    }

    #[test]
    fn increment_with_initial_value() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialize the contract
        let init_msg = InstantiateMsg {};
        let _ = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();

        // Set initial value manually in storage
        COUNT_SEQ.save(deps.as_mut().storage, &10).unwrap();

        // Increment the count by 4
        let increment_msg = IncrementMsg { num_add: 4 };
        let execute_msg = ExecuteMsg::Increment(increment_msg);
        let _ = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

        // Query the count and ensure it's updated to 14
        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_response: CountResponse = from_binary(&query_res).unwrap();
        assert_eq!(count_response.count, 14);
    }
}
