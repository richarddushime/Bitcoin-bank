# Bitcoin-bank Design Document

### Outline
Bitcoin “bank” (hot, warm, cold wallet backend) A Bitcoin bank provides security guarantees for its users by not storing all bitcoins online in a hot wallet. Instead, a scheme should be devised whereby an attacker cannot steal all the bitcoins even with total remote access to the bank's computers. You may want to incorporate threshold multi-signature schemes, timeouts, and other scripting capabilities. 

You should also decide how partially signed transactions can be communicated between different stages in the bank "vault", creating any required facilities to import/export, verify, and sign for them. The bank will need some nominal amount of its total reserve in a hot wallet to perform everyday operations. Decide on an amount and consider how hot and cold wallets will be balanced. Finally, consideration should be given to your users' privacy and your bank's privacy when considering implementing any internal UTXO consolidation and transaction batching for user withdrawals.

## Format
Software to manage the bank wallet. Optional frontend website for users to log in, see their balance, and make deposits and withdrawals. If the frontend website is not provided, backend software should expose API to request transactions and user balances.

## Repository
https://github.com/richarddushime/Bitcoin-bank

## Tracking Issues
https://github.com/richarddushime/Bitcoin-bank/issues


## Goals

### In Scope:

1. Implementing a multi-wallet backend with hot, warm, and cold wallets.
2. Providing a basic front-end website for user interactions.

#### Out of Scope:

Integration with third-party exchanges.

## Architecture

### Components:
1. Backend Server
2. Database
3. Frontend Website

### Backend Server 
The backend server hosts the service that holds the wallets for our users. This holds both hot and cold wallets for our users. This backend application allows the user to perform services like creating their accounts, viewing their balance, depositing btc into their account and withdrawing from their accounts. This service will be accessed by our users via API endpoints exposed by the service. Our service will also be connected to a bitcoin core node to broadcast transactions that have occurred, perform consolidation of UTXOs and to update other chain details for our various users.

### Database 
The database will be used by our backend service to retrieve and store information for users. Information like usernames, passwords, keys, transaction history, and UTXOs available. Some key factors for our database base is that it must be secure as keys of users are being held in our custody and easy to update as values of UTXOs are constantly changing.

### Frontend Website
This is a web interface for the users of the bank to access bank services. It helps users to manage their wallets, send or receive btc, and view the status of their account. The front end will access the back end services via API calls provided by the backend. 

![Bitcoin bank architectural diagram](/Bitcoin-Bank.png)
**Figure 1: Bitcoin bank architectural diagram**
 
## Interface

1. API Endpoints
2. Frontend Views
   - Login page
   - Dashboard displaying account balance and transaction history
   - Deposit and withdrawal forms

## Dependencies:

Backend
  - Rust
Frontend
  - Html, CSS, Javascript, Nextjs

## MVP Milestone:

The project can be considered to have achieved MVP state when:
- Users can register, log in, and view their account balance.
- Users can deposit Bitcoins into their accounts.
- Users can withdraw Bitcoins from their account.
- The project has two Bitcoin wallets, a hot wallet and a cold wallet. The cold wallet is simulated.

> Using Github we create issues which need to be tackled in order to reach MVP. Contributors sel-assign a task they are comfortable with and work on it.

## Backend Structure
Structure of backend folder
- The `cold-wallet` crate will simulate a cold wallet
- The `hot-wallet` crate will be the backend that can communicate with both the cold wallet and the frontend
- The `common-operations` crate will include features that are present in both the cold-wallet and hot wallet

We also add the `common-operations` crate to both the `hot-wallet` and the `cold-wallet`.

### Dependencies
The dependencies in the common library are used to build and sign transactions
since creating those libraries is beyond the scope of the project:
- secp256k1 - used to sign transactions using ecdsa and schnorr signatures
- hex-conservative - used to convert bytes to hex
- bitcoin_hashes - create SHA256 and RIPEMD hashes of bytes

## Conclusion
This project simulates a simple MVP that combines aspects of transactions, scripting, and cryptography.
