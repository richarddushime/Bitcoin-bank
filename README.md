# Bitcoin Bank Project

This guide provides step-by-step instructions on how to run the Bitcoin Bank project.

For more information about the project, read the [Design Document](/docs/Bitcoin-bank_Design_Document.md).

## Prerequisites

Before running the project, make sure you have the following prerequisites installed on your system:

- Rust and Cargo: [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
- Git: [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- Node.js: [Install Node.js](https://nodejs.org/)

## Steps to Run the Project

### 1. Fork and Clone the Project:

Fork the project repository on GitHub by clicking the "Fork" button at the top right corner of the repository page. Then, clone the forked repository to your local machine using the following command:

```bash 
git clone https://github.com/<your-username>/Bitcoin-bank.git
```

### 2. Navigate to the Project Directory:

Once the project is cloned, navigate to the project directory using the `cd` command:

```bash
cd Bitcoin-bank
```

> Note:This project consists of 2 modules client(frontend) and backend,

### 3. Build the Project:
Navigate to backend folder
```
cd backend
```
Execute
```bash
cargo build
```

### Running the binaries
The service requires `bitcoind` to be run for both the hot wallet and cold wallet. 

Example configuration for the `bitcoin.conf` file for hot and cold wallets
```toml
# inside the /path/to/data/hot/wallet/directory/bitcoin.conf
regtest=1
server=1
txindex=1

# Options only for regtest
[regtest]
rpcallowip=127.0.0.1
rpcuser=foobar
rpcpassword=password
rpcport=18423
port=18424

```
Here, the `addnode` is added to connect hot-wallet bitcoind node to cold-wallet bitcoind node at port `18424` of the hot wallet
```toml
# inside the /path/to/data/cold/wallet/directory/bitcoin.conf
regtest=1
server=1
txindex=1

# Options only for regtest
[regtest]
rpcallowip=127.0.0.1
rpcuser=foobar
rpcpassword=password
rpcport=18433
port=18434
addnode=127.0.0.1:18424

```

Running the nodes try running with `fallbackfee` and `maxtxfee` to avoid any potential errors.
```sh
$ bitcoind -conf=bitcoin.conf -datadir=/path/to/hot-wallet/data/directory -fallbackfee=0.00001 -maxtxfee=0.0001

$ bitcoind -conf=bitcoin.conf -datadir=/path/to/hot-wallet/data/directory -fallbackfee=0.00001 -maxtxfee=0.0001
```

#### Running the cold wallet
Pass the config file path for the cold wallet to the `cold-wallet`  server
```sh
$ cargo run -p cold-wallet -- path/to/cold/wallet/config
```

#### Running the hot wallet
Pass the config file path for the hot wallet to the `hot-wallet`  server
```sh
$ cargo run -p hot-wallet -- path/to/hot/wallet/config
```

The cold wallet binary should mine some blocks when it has been started.

The config files to pass to the binaries have example in the `btc.conf.example` file

## Explore the Frontend:

Navigate to the frontend directory and follow the instructions provided in the [README](/client/README.md) file located there.

```bash
cd client
```

## Access the Application:

Once the project is running, you can access the application by opening a web browser and navigating to the appropriate URL.
