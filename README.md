# Withdraw Transaction System

This project is a simple withdrawal transaction system implemented in Rust. It handles withdrawal transactions between accounts, including the calculation and deduction of a transaction fee that goes to an admin account.

## Table of Contents

- [Description](#description)
- [Project Structure](#project-structure)
- [Constants](#constants)
- [Structs](#structs)
  - [Withdraw](#withdraw)
- [Usage](#usage)
  - [Execute Function](#execute-function)
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)
- [Contact Information](#contact-information)

## Description

A Rust-based system for handling withdrawal transactions between accounts with a 1% transaction fee that is credited to an admin account.

## Project Structure

The project is structured into the following modules:

1. **state**: Handles the various states of a transaction.
2. **types**: Defines different types of users and transactions.
3. **account**: Manages account details and operations.
4. **transaction**: Defines the modes and types of transactions.

## Constants

- `FEE_PERCENT`: The fee percentage for each transaction (1%).
- `ZERO`: A constant representing zero, used for validation purposes.

## Structs

### Withdraw

The `Withdraw` struct represents a withdrawal transaction. It includes:

- `amount`: The amount to be withdrawn.
- `state`: The current state of the transaction.
- `from`: The account from which the amount is to be withdrawn.
- `to`: The account to which the amount is to be transferred.

#### Methods

- `build(from: &mut Account, to: &mut Account, amount: f64) -> Self`: Initializes a new `Withdraw` instance.
- `consume_fee(&self) -> f64`: Calculates the fee for the transaction.
- `submit(&mut self) -> Result<(), &'static str>`: Submits the withdrawal request.
- `do_transaction(&mut self, admin: &mut Account)`: Executes the transaction by transferring the amount from the source account to the destination account and deducting the fee to the admin account.

## Usage

### Execute Function

The `execute` function demonstrates the usage of the withdrawal transaction system. It performs the following steps:

1. Creates two user accounts, `ram` and `shyam`.
2. Builds a withdrawal transaction from `ram` to `shyam`.
3. Calculates the transaction fee.
4. Submits the withdrawal request.
5. Approves the transaction using an admin account.
6. Prints the balances and transaction histories of `ram`, `shyam`, and the admin account.
7. Creates another user account, `goldy`, and performs another withdrawal transaction from `ram` to `goldy`.

### Example

To run the example, call the `execute` function. It will demonstrate the creation of accounts, submission and approval of withdrawal transactions, and the resulting balances and transaction histories of the involved accounts.

```rust
fn main() {
    execute();
}
