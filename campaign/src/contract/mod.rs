// contract/mod.rs for campaign

extern crate cosmwasm_std;

use self::cosmwasm_std::{
    entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult,
};
use crate::state::{Campaign, CAMPAIGN, ENGAGED_USERS};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let funds = info.funds.first().ok_or_else(|| StdError::generic_err("No reward funds sent"))?;

    let campaign = Campaign {
        creator: info.sender.clone(),
        name: msg.name,
        tasks: vec![], // No tasks at creation
        reward_pool: funds.clone(),
        expiration: msg.expiration,
    };

    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate_campaign")
        .add_attribute("creator", info.sender)
        .add_attribute("reward_pool", funds.amount))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::RecordEngagement { user } => try_record_engagement(deps, info, user),
        ExecuteMsg::Claim {} => try_claim(deps, info),
        ExecuteMsg::AddTasks { tasks } => try_add_tasks(deps, info, tasks), // 💥 new function
    }
}

fn try_record_engagement(deps: DepsMut, info: MessageInfo, user: Addr) -> Result<Response, StdError> {
    let campaign = CAMPAIGN.load(deps.storage)?;
    if info.sender != campaign.creator {
        return Err(StdError::generic_err("Unauthorized"));
    }

    ENGAGED_USERS.save(deps.storage, user.clone(), &true)?;
    Ok(Response::new()
        .add_attribute("action", "record_engagement")
        .add_attribute("user", user))
}

fn try_claim(deps: DepsMut, info: MessageInfo) -> Result<Response, StdError> {
    let campaign = CAMPAIGN.load(deps.storage)?;

    let has_engaged = ENGAGED_USERS.may_load(deps.storage, info.sender.clone())?.unwrap_or(false);
    if !has_engaged {
        return Err(StdError::generic_err("User not eligible"));
    }

    let engagees: Vec<Addr> = ENGAGED_USERS
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    let share_amount = campaign.reward_pool.amount.u128() / engagees.len() as u128;
    let payout = Coin {
        denom: campaign.reward_pool.denom.clone(),
        amount: share_amount.into(),
    };

    // Remove user from map to prevent double claims
    ENGAGED_USERS.remove(deps.storage, info.sender.clone());

    // If it's the last claim, clear state (optional)
    if engagees.len() == 1 {
        CAMPAIGN.remove(deps.storage);
    }

    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![payout.clone()],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "claim")
        .add_attribute("amount", payout.amount))
}

fn try_add_tasks(deps: DepsMut, info: MessageInfo, tasks: Vec<String>) -> Result<Response, StdError> {
    CAMPAIGN.update(deps.storage, |mut campaign| {
        if info.sender != campaign.creator {
            return Err(StdError::generic_err("Unauthorized"));
        }

        campaign.tasks.extend(tasks);
        Ok(campaign)
    })?;

    Ok(Response::new()
        .add_attribute("action", "add_tasks"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCampaign {} => {
            let campaign = CAMPAIGN.load(deps.storage)?;
            to_json_binary(&campaign)
        }
        QueryMsg::HasEngaged { user } => {
            let engaged = ENGAGED_USERS
                .may_load(deps.storage, user)?
                .unwrap_or(false);
            to_json_binary(&engaged)
        }
    }
}
