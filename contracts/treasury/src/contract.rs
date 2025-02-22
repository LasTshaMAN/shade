use cosmwasm_std::{
    debug_print, to_binary, Api, Binary,
    Env, Extern, HandleResponse, InitResponse, 
    Querier, StdResult, Storage, 
};

use shade_protocol::{
    treasury::{
        InitMsg, TreasuryConfig,
        HandleMsg,
        QueryMsg,
    },
};

use crate::{
    state::{
        viewing_key_w,
        config_w,
        self_address_w,
    },
    handle, query,
};


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let state = TreasuryConfig {
        owner: match msg.admin {
            None => { env.message.sender.clone() }
            Some(admin) => { admin }
        },
    };

    config_w(&mut deps.storage).save(&state)?;
    viewing_key_w(&mut deps.storage).save(&msg.viewing_key)?;
    self_address_w(&mut deps.storage).save(&env.contract.address)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse {
        messages: vec![],
        log: vec![]
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Receive {
            sender,
            from,
            amount,
            msg,
            ..
        } => handle::receive(deps, env, sender, from, amount, msg),
        HandleMsg::UpdateConfig {
            owner,
        } => handle::try_update_config(deps, env, owner),
        HandleMsg::RegisterAsset {
            contract,
        } => handle::try_register_asset(deps, &env, &contract),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query::config(deps)?),
        QueryMsg::GetBalance { contract } => to_binary(&query::balance(deps, contract)?),
    }
}
