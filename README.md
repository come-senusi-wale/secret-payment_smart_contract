## Overview

This contract implements functionality related to invoice management and payment processing. It allows users to submit invoices, accept invoices, cancel payments, and withdraw payments.

## Entry Points

### Instantiate

The `instantiate` entry point is invoked upon contract deployment. It initializes the contract state by saving the admin wallet address.

### Execute

The `execute` entry point handles various execution messages related to invoice submission, invoice acceptance, payment cancellation, and payment withdrawal.

### Query

The `query` entry point handles queries to retrieve information about invoices and contracts.

## Test Cases

The contract includes test cases to ensure its functionality works as expected. These test cases cover scenarios such as invoice submission, invoice acceptance, and payment cancellation.

## Usage

To use the contract, follow these steps:

1. Deploy the contract to the testnet.
2. Interact with the contract using transactions to submit invoices, accept invoices, cancel payments, and withdraw payments.

## Test Environment

The contract includes test cases that can be run in a testing environment. These tests are designed to verify the correctness of the contract's functionality.

## Dependencies

The contract relies on the following dependencies:

- `cosmwasm_std`: Provides standard functionality for Cosmos contracts.
- `secret_toolkit`: Provides utility functions for interacting with the Secret Network.

## Functions

Instantiate
Description: Initializes the contract.

Input Parameters:

None

###
- `SubmitInvoice`
Description: Allows users to submit an invoice.

Input Parameters:

purpose: Description of the invoice purpose.
amount: Amount of the invoice.
admin_charge: Admin charge for processing the invoice.
customer_charge: Customer charge for the invoice.
payer: Wallet address of the payer.
days: Number of days for payment.
recurrent_time: Optional. Recurrent payment time.
token: Token used for payment.

###
`AcceptInvoice`
Description: Allows users to accept an invoice.

Input Parameters:

id: ID of the invoice to accept.

###
`CancelPayment`
Description: Allows users to cancel a payment.

Input Parameters:

id: ID of the payment to cancel.

###
`WithdrawPayment`
Description: Allows users to withdraw a payment.

Input Parameters:

id: ID of the payment to withdraw.

###
`AdminUpdateAmin`
Description: Allows admin to update the admin address.

Input Parameters:

newAdmin: New admin wallet address.

###
`SingleInvoice`
Description: Retrieves information about a single invoice.

Input Parameters:

id: ID of the invoice to retrieve.
owner: Wallet address of the invoice owner.

###
`NumberOfInvoice`
Description: Retrieves the number of invoices for a specific owner.

Input Parameters:

owner: Wallet address of the invoice owner.

###
`PaginatedInvoice`
Description: Retrieves a paginated list of invoices for a specific owner.

Input Parameters:

owner: Wallet address of the invoice owner.
page: Page number.
page_size: Size of each page.

###
`SingleContract`
Description: Retrieves information about a single contract.

Input Parameters:

id: ID of the contract to retrieve.
payer: Wallet address of the contract payer.

###
`NumberOfContract`
Description: Retrieves the number of contracts for a specific payer.

Input Parameters:

payer: Wallet address of the contract payer.

###
`PaginatedContract`
Description: Retrieves a paginated list of contracts for a specific payer.

Input Parameters:

payer: Wallet address of the contract payer.
page: Page number.
page_size: Size of each page.