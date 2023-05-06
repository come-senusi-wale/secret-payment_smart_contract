import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm");

const secretjs = new SecretNetworkClient({
    chainId: "pulsar-2",
    url: "https://api.pulsar.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
});

//console.log(secretjs);

let upload_contract = async () => {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contract_wasm,
        source: "",
        builder: "",
      },
      {
        gasLimit: 4_000_000,
      }
    );

    //console.log(tx);
  
    const codeId = Number(
      tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
        .value
    );
  
    console.log("codeId: ", codeId);
  
    // const contractCodeHash = (
    //   await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    // ).code_hash;
    // console.log(`Contract hash: ${contractCodeHash}`);

    try {
      
      const contractCodeHash = (
        await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
      ).code_hash;
      console.log(`Contract hash: ${contractCodeHash}`);

    } catch (error) {
      console.log(error);
    }
    
};
  
//upload_contract();

let codeId = 21044;
let contractCodeHash = 'add0d4c751f7503a564031dab3b31007c696382d732c88fb95d4d07aee4c5fc8';

let instantiate_contract = async () => {
    // Create an instance of the Counter contract, providing a starting count
    const initMsg = { };
    let tx = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash: contractCodeHash,
        init_msg: initMsg,
        label: "My payment" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
      
    //console.log(tx);

    //Find the contract_address in the logs
    const contractAddress = tx.arrayLog.find(
      (log) => log.type === "message" && log.key === "contract_address"
    ).value;
  
    console.log(contractAddress);
};
  
//instantiate_contract();

let contract_address = 'secret17hxzcy4y8aznjsjlv6l33tk4fql5fcsl9zccjm';

// for query single invoice
let try_query_single_invoice = async (id) => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { single_invoice: {
          id: id,
          owner: wallet.address
        } },
    });

    console.log(my_query);
  };
  


// for query number of invoice
let try_query_all_invoice = async () => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { number_of_invoice: {
          owner: wallet.address
        } },
    });

    console.log(my_query);
  };

//for submiting invoice
let add_new_invoice = async () => {
    
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          sender: wallet.address,
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            submit_invoice: { purpose: "build contract", amount: "1", payer: wallet.address, days: 6, recurrent_time: 1, token: { native: "uscrt" }},
          },
          sentFunds: [], // optional
        },
        {
          gasLimit: 100_000,
        }
      );
      console.log(tx);
    } catch (error) {
      console.log(error);
    }
};




//for witgdraw payment
let withdraw_payment = async (id) => {
    
  try {
    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash, // optional but way faster
        msg: {
          withdraw_payment: { id: id},
        },
        sentFunds: [], // optional
      },
      {
        gasLimit: 100_000,
      }
    );
    console.log(tx);
  } catch (error) {
    console.log(error);
  }
};

let payer = "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n"

// for query single contract
let try_query_single_contract = async (id) => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { single_contract: {
          id: id,
          payer: wallet.address
        } },
    });
  
    console.log(my_query);
  };
  
  // for query number of contract
  let try_query_all_contract = async () => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { number_of_contract: {
          payer: wallet.address
        } },
    });
  
    console.log(my_query);
  };


  
//for accepting invoice
let accept_invoice = async (id) => {
    
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            accept_invoice: { id: id},
          },
      
          sender: wallet.address,
          sent_funds: [{ denom: "uscrt", amount: "10" }]
        },
        {
          gasLimit: 100_000,
        }
      );
      console.log(tx);
    } catch (error) {
      console.log(error);
    }
  };
  
  
  //for cancelling payment
  let cancel_payment = async (id) => {
      
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          sender: wallet.address,
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            cancel_payment: { id: id},
          },
          sentFunds: [], // optional
        },
        {
          gasLimit: 100_000,
        }
      );
      console.log(tx);
    } catch (error) {
      console.log(error);
    }
  };



//  try_query_single_invoice(1);
//    try_query_single_contract(1)
//   try_query_all_invoice();
//  try_query_all_contract()
 

 //add_new_invoice();
 //accept_invoice(1)
 //cancel_payment(1)
 withdraw_payment(1)
