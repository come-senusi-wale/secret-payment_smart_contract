use cosmwasm_std::{
    coins,  BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128, 
};

use secret_toolkit::{
    utils::types::{Token},
};

use crate::{
    state::{InvoiceStore, Invoice, Contract, ContractStore, get_next_invoice_id}, 
};

pub struct Empty {}

#[allow(clippy::too_many_arguments)]
pub fn new_invoice(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo,
    purpose: String,
    amount:  Uint128,
    payer: String,
    days: u64,
    recurrent_time: Option<u64>,
    token: Token) -> StdResult<Response> {
    
    //get the signer
    let receiver = info.sender;

    //validate payer address
    let payer_address = deps.api.addr_validate(payer.as_str())?;

    //get next invoice id
    let next_invoice_id = get_next_invoice_id(deps.storage)?;

    //check if recurrent time is specify
    let times_of_recurrent = match recurrent_time {
        Some(time) => time,
        None => 0,
    };

    let recurrent_status = match recurrent_time {
        Some(_time) => true,
        None => false,
    };

    let status = "not started".to_string();

    let invoice =  Invoice{
        invoice_id: next_invoice_id,
        receiver: receiver.to_string(),
        purpose: purpose,
        amount: amount,
        payer: payer_address.to_string(),
        days: days,
        recurrent: Some(recurrent_status),
        recurrent_times: times_of_recurrent,
        remaing_time_of_payment: 0,
        status: status,
        payment_time: 0,
        critical_time: 0,
        payment_condition: "no".to_string(),
        token: token
    };

    InvoiceStore::save(deps.storage, &receiver, next_invoice_id, &invoice)?;

    let contract = Contract{
        invoice_id : next_invoice_id,
        account_balance: 0,
        constract_process: "not started".to_string(), 
        invoice: invoice,
        contract_accepted: false,
    };

    ContractStore::save(deps.storage, &payer_address, next_invoice_id, &contract)?;

    deps.api.debug("invoice created successfully");
    Ok(Response::default())

}

pub fn accept_invoice(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    id: u64, ) -> StdResult<Response> {

    //get the signer which is the payer
    let payer = info.sender;

    //get the contract of specific id related to invoice
    let mut contract =  ContractStore::load_contract(deps.storage, &payer, id,);

    //get invoice of specific id in related to contract
    let mut invoice = &mut contract.invoice;

    //get the reciver adddress
    let reciever = deps.api.addr_validate(invoice.receiver.as_str())?;

    let payer_address = deps.api.addr_validate(invoice.payer.as_str())?;

    //check that the signer is one that invoce is ment for
    if payer_address != payer {
        return Err(StdError::generic_err(
            "the invoice is not ment for you",
        ));
    }

    if contract.contract_accepted == true {
        return Err(StdError::generic_err(
            "you have already accepted this invoice",
        ));
    }

    let mut amount = Uint128::zero();

    if invoice.recurrent == Some(true){
        let expected_amount = invoice.amount * Uint128::new(invoice.recurrent_times.into());

        for coin in &info.funds {
           amount += coin.amount 
        }

        if amount < expected_amount{
            return Err(StdError::generic_err(
                "provide correct amount please",
            ));
        }

        
    }else{

        for coin in &info.funds {
            amount += coin.amount 
        }
       
        if amount < invoice.amount{
            return Err(StdError::generic_err(
                "provide correct amount please",
            ));
        }
    }

    if amount.is_zero() {
        return Err(StdError::generic_err("No funds were sent to be deposited"));
    }


   
    let remaing_time_of_payment = match invoice.recurrent {
        Some(_remaing_time) => invoice.recurrent_times,
        None => 1,
    };

    // let account_balance = match payment {
    //     Some(pay) => pay.amount,
    //     None => Uint128::new(0),
    // };

    let account_balance = amount;

    let current_block_time = env.block.time.seconds();
    let day_in_timestmp = invoice.days * 86400;
    let paid_time = current_block_time + day_in_timestmp;
    let critical_time = paid_time / 2;

    // updating neccessary field
    invoice.payment_time = paid_time;
    invoice.critical_time = critical_time;
    invoice.payment_condition = "pay full".to_string();
    invoice.status = "accepted".to_string();
    invoice.remaing_time_of_payment = remaing_time_of_payment;

    contract.account_balance = account_balance.into();
    contract.contract_accepted = true;
    contract.constract_process = "started".to_string();

    // save the update
    InvoiceStore::save(deps.storage, &reciever, id, &invoice)?;
    ContractStore::save(deps.storage, &payer, id, &contract)?;

    deps.api.debug("invoice accepted successfully");
    Ok(Response::default())



}

pub fn stop_contract(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    id: u64, ) -> StdResult<Response> {
    
    //get the signer which is the payer
    let payer = info.sender;

    //get the contract of specific id related to invoice
    let mut contract =  ContractStore::load_contract(deps.storage, &payer, id,);

    //get invoice of specific id in related to contract
    let mut invoice = &mut contract.invoice;

    //get the reciver adddress
    let reciever = deps.api.addr_validate(invoice.receiver.as_str())?;

    let payer_address = deps.api.addr_validate(invoice.payer.as_str())?;

    //check that the signer is one that invoce is ment for
    if payer_address != payer {
        return Err(StdError::generic_err(
            "the invoice is not ment for you",
        ));
    }

    //check if the payer has accepted the contract
    if contract.contract_accepted != true {
        return Err(StdError::generic_err(
            "you have not accepted this invoice",
        ));
    }

    //check if th contract has been carry out
    if contract.constract_process == "done".to_string() {
        return Err(StdError::generic_err(
            "this contract has been carry out already",
        ));
    }

    
    //check if th contract has been stop already
    if contract.constract_process == "stop".to_string() {
        return Err(StdError::generic_err(
            "this contract has been stoped already",
        ));
    }

    let current_block_time = env.block.time.seconds();

    let denom = "uscrt".to_string();

    if invoice.critical_time > current_block_time {
        //set the amount to half of current payment
        let amount_to_pay = invoice.amount / Uint128::new(2);

        //get the remaining balance
        let remaining_balance:u128 = contract.account_balance - <Uint128 as Into<u128>>::into(amount_to_pay);

        // payer should recieve their remaining balance
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: payer.to_string(),
            amount: coins(remaining_balance, denom)
        });

        invoice.payment_condition = "half".to_string();
        invoice.amount = amount_to_pay;
        invoice.status = "stop".to_string();
        invoice.remaing_time_of_payment = 1;

        contract.constract_process = "stop".to_string();
        contract.account_balance = amount_to_pay.into();
        
        // save the update
        InvoiceStore::save(deps.storage, &reciever, id, &invoice)?;
        ContractStore::save(deps.storage, &payer, id, &contract)?;

    }else{

        // payer should recieve all their money back
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: payer.to_string(),
            amount: coins(contract.account_balance, denom)
        });

        invoice.payment_condition = "no".to_string();
        invoice.amount = Uint128::new(0);
        invoice.status = "stop".to_string();
        invoice.remaing_time_of_payment = 0;

        contract.constract_process = "stop".to_string();
        contract.account_balance= 0;

        // save the update
        InvoiceStore::save(deps.storage, &reciever, id, &invoice)?;
        ContractStore::save(deps.storage, &payer, id, &contract)?;

    }

    deps.api.debug("invoice canceled successfully");
    Ok(Response::default())
}

pub fn withraw_payment(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    id: u64, ) -> StdResult<Response> {
    
    //get the signer which is the reciever of payment
    let receiver = info.sender;

    //get invoice of specific id in related to contract
    let mut invoice = InvoiceStore::load_invoice(deps.storage, &receiver, id,);
    
    //reciver address in the invoice
    let reciever_address = deps.api.addr_validate(invoice.receiver.as_str())?;

    //payer address in the invoice
    let payer = deps.api.addr_validate(invoice.payer.as_str())?;

    //check that the signer is one that submited the invoice
    if reciever_address != receiver {
        return Err(StdError::generic_err(
            "this is not your invoice",
        ));
    }

    //get the contract of specific id related to invoice
    let contract =  ContractStore::load_contract(deps.storage, &payer, id,);

    //check if the contract has been accepted
    if contract.contract_accepted != true {
        return Err(StdError::generic_err(
            "this invoice has not been accepted",
        ));
    }

    let current_block_time = env.block.time.seconds();

    if current_block_time < invoice.payment_time {
        return Err(StdError::generic_err(
            "is not time for payment",
        ));
    }

    if invoice.remaing_time_of_payment == 0 {
        return Err(StdError::generic_err(
            "no payment for you again",
        ));
    }

    if invoice.payment_condition == "no".to_string() {
        return Err(StdError::generic_err(
            "is like your invoice was canceled",
        ));
    }

    //check if the payer has money in is account
    if contract.account_balance < 1 {
        return Err(StdError::generic_err(
            "the payer has no money in his account",
        ));
    }

    let denom = "uscrt".to_string();

    if invoice.payment_condition == "half".to_string() {

        invoice.status = "done".to_string();
        invoice.remaing_time_of_payment = 0;

        let amount_to_pay = invoice.amount / Uint128::new(2);

        // resave invoice changes
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;

        let contract_store = Contract{
            invoice_id: id,
            account_balance: 0,
            constract_process: contract.constract_process,
            invoice : invoice,
            contract_accepted: contract.contract_accepted,
        };


        // employee recieve their payment
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: coins(amount_to_pay.into(), denom)
        });

        // resave contract changes
        ContractStore::save(deps.storage, &payer, id, &contract_store)?;
        
    }else{

        //get the remaing time
        let remaing_time_of_payment = match invoice.recurrent {
            Some(_remaing_time) => invoice.remaing_time_of_payment - 1,
            None => 0,
        };

        let account_balance = contract.account_balance - <Uint128 as Into<u128>>::into(invoice.amount);

        invoice.remaing_time_of_payment = remaing_time_of_payment;

        let amount_to_pay = invoice.amount.into();

        // resave invoice changes
        InvoiceStore::save(deps.storage, &receiver, id, &invoice)?;

        let contract_store = Contract{
            invoice_id: id,
            account_balance: account_balance,
            constract_process: contract.constract_process,
            invoice : invoice,
            contract_accepted: contract.contract_accepted,
        };


        // employee recieve their payment
        CosmosMsg::<Empty>::Bank(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: coins(amount_to_pay, denom)
        });

        // resave contract changes
        ContractStore::save(deps.storage, &payer, id, &contract_store)?;

    }

    


    deps.api.debug("invoice accepted successfully");
    Ok(Response::default())
}




