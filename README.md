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
The service requires `bitcoind` to be run for both the hot wallet and cold wallet. 
There is a default configuration in the `backend/bitcoin-bank-conf` directory.

To run the nodes for the hot and cold wallet nodes first run the hot wallet because the cold wallet node will try to connect to the hot wallet node. 
Remember to replace the part `/aboslute/path/to/project/directory` with the path to this github project in your local machine since it will look in the `<absolute_path>/backend/bitcoin-bank-conf` 
```sh
$ bitcoind -conf=bitcoin.conf -datadir=/aboslute/path/to/project/directory/backend/bitcoin-bank-conf/hot-wallet -fallbackfee=0.00001 -maxtxfee=0.0001
```
Then run the cold wallet replacing the path
```sh
$ bitcoind -conf=bitcoin.conf -datadir=/aboslute/path/to/project/directory/backend/bitcoin-bank-conf/cold-wallet -fallbackfee=0.00001 -maxtxfee=0.0001
```
We will use `cold` and `hot` commandline arguments to signify either running a cold wallet binary or the hot wallet + server binary.

Run the cold wallet to generate some blocks.
Inside the `backend` directory run
```sh
$ cargo run -p cold-wallet -- cold
```
Then run the hot wallet + server binary (still inside the backend directory)
```sh
$ cargo run -p hot-wallet -- hot
```

## Explore the Frontend:

Navigate to the frontend directory and follow the instructions provided in the [README](/client/README.md) file located there.

```bash
cd client
```

## Access the Application:

Once the project is running, you can access the application by opening a web browser and navigating to the appropriate URL.
