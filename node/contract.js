
let contractCodeHash = '1f067ea778f0fa43c58d8f41775d8443dba76b583d0a0b9409e5e5c2a1e98b6b';
let contract_address = 'secret19zv44wcau7mthd6tr4agkelpxd6y4e847g4mly';
let wallet = "secret1py4ryg3atyz5cru2m64p0mtga5y09q5a26pa7n"

// for query single contract
let try_query_single_contract = async (id) => {
    const my_query = await secretjs.query.compute.queryContract({
        contract_address: contract_address,
        code_hash: contractCodeHash,
        query: { single_contract: {
          id: id,
          payer: wallet
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
          payer: wallet
        } },
    });
  
    console.log(my_query);
  };


  
//for accepting invoice
let accept_invoice = async (id) => {
    
    try {
      let tx = await secretjs.tx.compute.executeContract(
        {
          sender: wallet.address,
          contract_address: contract_address,
          code_hash: contractCodeHash, // optional but way faster
          msg: {
            accept_invoice: { id: id},
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

//   try_query_single_contract(1)
//  try_query_all_contract()
 

  
 accept_invoice(1)
// cancel_payment(1)
