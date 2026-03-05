# Solana Theory
_______________________________________________________________________________

### Differences between Solana and Ethereum

- Solana smart contract development is usually done in Rust or Python.
- Solana smart contracts are compiled to `Solana Bytecode Format (SBF)`.
- One of the most popular frameworks is the Anchor framework.

- Unlike Ethereum Smart Contracts, where the `code` to transfer tokens between
the users, and the `state` (the balance of the tokens of the users),
are both stored in contracts... In Solana the `code` and `state` are separate.

- The code is stored in `programs`, and the state is stored in `accounts`.

- Unlike Ethereum where all transactions are executed sequentially,
in Solana transactions can be executed in parallel.

- When you send a transaction in Solana, 
you must include all of the accounts that the transaction will modify.

- When deploying a Solana smart contract, you have to lock SOL in proportion
to the data size. When you close an account or delete the program,
the Sol that you locked will be repaid.

- Unlike Ethereum where contracts are immutable by default (unless you
create a proxy contract), Solana smart contracts are upgradable.

_______________________________________________________________________________

### What are `accounts` in Solana?

- All data on Solana are stored in accounts.

- `lamports` = The accounts balance. Every account must have minimum balance
called rent.

1 SOL = 1 billion lamports (1,000,000,000)

_______________________________________________________________________________

### What is a `program` in Solana?

- It is an executable stateless account

_______________________________________________________________________________

### What is an `instruction` in Solana?

This is a function call to the program.

_______________________________________________________________________________

### What is a `transaction` in Solana?

It is a bundle of instructions. So a transaction can call multiple programs,
or call the same program multiple times.

Examples of transactions:

E.g. A transaction to initialize a counter state to 0

`tx` = transaction

`ix` = instruction

_______________________________________________________________________________

### What is `PDA`?

Program derived address.

A typical account has a private key and a public key

A PDA has a public key but no private key. It is created using the program ID

_______________________________________________________________________________

### What is the program ID?

It is a randomly generated private key.

_______________________________________________________________________________
