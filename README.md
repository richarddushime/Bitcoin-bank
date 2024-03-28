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

### 3. Running the binaries
The service requires `bitcoind` and `bitcoin-cli` to installed.
To run the binary, pass the config file which configures the bitcoin daemons for the mining node (node that consistently generates blocks), hot walled node and cold wallet node. The example config file can be found in the `backend/btc_conf.toml` file.

The binary checks for the home directory of your specific platform and uses that to create the data directory for the bitcoind for the nodes as follows:
```sh
UserHomeDir
  |-- bitcoin-bank-data #Application directory 
          |-- mining-node # Mining node data directory
          |-- hot-node # Hot wallet node data directory
          |-- cold-node # Cold wallet node data directory
```

To run the binary with the default config file
```sh
$ cd ./backend

$ cargo run -- ./btc_conf.toml
```


## Explore the Frontend:

Navigate to the frontend directory and follow the instructions provided in the [README](/client/README.md) file located there.

```bash
cd client
```

## Access the Application:

Once the project is running, you can access the application by opening a web browser and navigating to the appropriate URL.
