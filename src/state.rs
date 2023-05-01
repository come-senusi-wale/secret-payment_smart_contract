use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use secret_toolkit::serialization::Json;
use secret_toolkit::storage::{Item, Keymap};
use secret_toolkit::{
    utils::types::{Token},
};


pub const PREFIX_INVOICE: &[u8] = b"invoice";
pub const PREFIX_CONTRACT: &[u8] = b"contract";

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Invoice{
    pub invoice_id: u64,
    pub receiver: String,
    pub purpose: String,
    pub amount: Uint128,
    pub payer: String,
    pub days: u64,
    pub recurrent: Option<bool>,
    pub recurrent_times: u64,
    pub remaing_time_of_payment: u64,
    pub status: String,
    pub payment_time: u64,
    pub critical_time: u64,
    pub payment_condition: String,
    pub token: Token
    
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Eq,  JsonSchema)]
pub struct Contract{
    pub invoice_id: u64,
    pub account_balance: u128,
    pub constract_process: String,
    pub invoice : Invoice,
    pub contract_accepted: bool,
}

const INVOICE_ID: Item<u64> = Item::new(b"invoice_id");

pub fn get_next_invoice_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let new_id = match INVOICE_ID.may_load(storage)? {
        Some(id) => id + 1,
        None => 1,
    };
    INVOICE_ID.save(storage, &new_id)?;

    Ok(new_id)
}

pub static INVOICE: Keymap<u64, Invoice, Json> = Keymap::new(PREFIX_INVOICE);

pub struct InvoiceStore {}

impl InvoiceStore {

    pub fn load_invoice(store: &dyn Storage, owner: &Addr, id: u64,) -> Invoice {
        INVOICE
            .add_suffix(owner.as_bytes())
            .get(store, &id.clone())
            //.unwrap_or_default()
            .unwrap()
    }

    pub fn save(
        store: &mut dyn Storage,
        owner: &Addr,
        id: u64,
        invoice: &Invoice,
    ) -> StdResult<()> {
        INVOICE
            .add_suffix(owner.as_bytes())
            .insert(store, &id, invoice)     
    }

    pub fn paging_invoice_list(
        store: &dyn Storage,
        owner: &Addr,
        page: u32,
        page_size: u32,
    ) -> StdResult<Vec<(u64, Invoice)>> {
        INVOICE
            .add_suffix(owner.as_bytes())
            .paging(store, page, page_size)
            
    }

    pub fn num_invoice(store: &dyn Storage, owner: &Addr,) -> u32 {
        INVOICE
            .add_suffix(owner.as_bytes())
            .get_len(store)
            .unwrap_or(0)
    }


}


pub static  CONTRACT: Keymap<u64, Contract, Json> = Keymap::new(PREFIX_CONTRACT);

pub struct ContractStore {}

impl ContractStore {
    pub fn save(
        store: &mut dyn Storage,
        payer: &Addr,
        id: u64,
        contract: &Contract,
    ) -> StdResult<()> {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .insert(store, &id, contract)     
    }

    pub fn load_contract(store: &dyn Storage, payer: &Addr, id: u64,) -> Contract {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .get(store, &id.clone())
            //.unwrap_or_default()
            .unwrap()
    }

    pub fn paging_contract_list(
        store: &dyn Storage,
        payer: &Addr,
        page: u32,
        page_size: u32,
    ) -> StdResult<Vec<(u64, Contract)>> {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .paging(store, page, page_size)
            
    }

    pub fn num_contract(store: &dyn Storage, payer: &Addr,) -> u32 {
        CONTRACT
            .add_suffix(payer.as_bytes())
            .get_len(store)
            .unwrap_or(0)
    }
}