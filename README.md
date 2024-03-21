# Bitcoin-bank
**Outline**: Bitcoin “bank” (hot, warm, cold wallet backend) A bitcoin bank provides security guarantees for its users by not storing all bitcoins online in a hot wallet. Instead a scheme should be devised whereby an attacker, even with total remote access to the bank's computers, cannot steal all the bitcoins. You may want to incorporate threshold multisignature schemes, timeouts, and other scripting capabilities. You should also decide how partially signed transactions can be communicated between different stages in the bank "vault", creating any required facilities to import/export, verify and sign for them. The bank will need some nominal amount of its total reserve in a hot wallet to perform everyday operations. Decide on an amount and also consider how hot and cold wallets will be balanced. Finally, consideration should be given to the privacy of your users and the privacy of your own bank when thinking about implementing any internal UTXO consolidation and transaction batching for user withdrawals.

### Format
Software to manage the bank wallet. Optional frontend website for users to log in, see their balance and make deposits and withdrawals. If frontend website not provided, backend software should expose API to request transactions and user balances.


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
Pass the config file path for the cold wallet to the `cold-wallet`  server
```sh
$ cargo run -p hot-wallet -- path/to/cold/wallet/config
```

The cold wallet binary should mine some blocks when it has been started.

The config files to pass to the binaries have example in the `btc.conf.example` file
