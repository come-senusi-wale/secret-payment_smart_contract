use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response,  StdResult,
};


use crate::{
    execute,
    msg::{ InstantiateMsg, QueryMsg, ExecuteMsg,},
    state::{InvoiceStore, ContractStore,},
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    msg: ExecuteMsg) -> StdResult<Response> {

    match msg {
        ExecuteMsg::SubmitInvoice {
            purpose,
            amount,
            payer,
            days,
            recurrent_time,
            token,
        } => execute::new_invoice(
            deps,
            env,
            info,
            purpose,
            amount,
            payer,
            days,
            recurrent_time,
            token,
        ),
        ExecuteMsg::AcceptInvoice {
            id,
        } => execute::accept_invoice(
            deps,
            env,
            info,
            id,
        ),
        ExecuteMsg::CancelPayment {
            id,
        } => execute::stop_contract(
            deps,
            env,
            info,
            id,
        ),
        ExecuteMsg::WithdrawPayment {
            id,
        } => execute::withraw_payment(
            deps,
            env,
            info,
            id,
        ),
        
    }
}

#[entry_point]
pub fn query(
    deps: Deps, 
    _env: Env, 
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::SingleInvoice {
            id,
            owner
        } =>{
            
            to_binary(&InvoiceStore::load_invoice(deps.storage, &owner, id))
        },
        QueryMsg::NumberOfInvoice {
            owner
        } =>{
            
            to_binary(&InvoiceStore::num_invoice(deps.storage, &owner))
        },
        QueryMsg::PaginatedInvoice {
            owner,
            page,
            page_size
        } =>{
            
            to_binary(&InvoiceStore::paging_invoice_list(deps.storage, &owner, page, page_size)?)
        },
        QueryMsg::SingleContract {
            id,
            payer
        } =>{
            
            to_binary(&ContractStore::load_contract(deps.storage, &payer, id))
        },
        QueryMsg::NumberOfContract {
            payer
        } =>{
            
            to_binary(&ContractStore::num_contract(deps.storage, &payer))
        },
        QueryMsg::PaginatedContract {
            payer,
            page,
            page_size
        } =>{
            
            to_binary(&ContractStore::paging_contract_list(deps.storage, &payer, page, page_size)?)
        },
        
        
    }
}