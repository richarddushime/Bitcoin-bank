use bitcoincore_rpc::{
    bitcoin::{
        address::{self, ParseError},
        Address, Amount, BlockHash, Network,
    },
    bitcoincore_rpc_json::{
        AddressType, GetNetworkInfoResult, ListUnspentResultEntry, LoadWalletResult,
    },
    Auth, Client, Result as BtcResult, RpcApi,
};
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub struct Wallet {
    client: Client,
    wallet_name: String,
    cold: bool,
    data_dir: String,
    pub rpc_port: String,
}

impl Wallet {
    pub fn new(config_file_path: impl AsRef<Path>) -> Self {
        let config = Config::load(config_file_path).expect("Encountered error  while opening file");
        let rpc_port = format!("http://localhost:{}", &config.rpc_port);
        let client = Client::new(
            &rpc_port,
            Auth::UserPass(config.rpc_username, config.rpc_password),
        )
        .expect("Encountered error while creating a connection to Bitcoin core");

        Self {
            client,
            wallet_name: config.wallet_name,
            cold: config.cold,
            data_dir: config.data_dir,
            rpc_port: config.rpc_port,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn network(&self) -> BtcResult<GetNetworkInfoResult> {
        self.client.get_network_info()
    }

    pub fn best_blockhash(&self) -> BtcResult<String> {
        let hash = self.client.get_best_block_hash()?;

        Ok(hash.to_string())
    }

    pub fn does_wallet_exist(&self) -> BtcResult<()> {
        let wallets = self.client.list_wallets()?;
        let mut wallet_names = Vec::<String>::new();

        wallets.iter().for_each(|wallet| {
            let path = Path::new(wallet);
            wallet_names.push(path.file_name().unwrap().to_string_lossy().to_string());
        });

        if !wallet_names.is_empty() {
            println!("WALLETS FOUND: {:?}", &wallet_names);
        }

        if !wallet_names.contains(&self.wallet_name) {
            println!("WALLET NOT FOUND, CREATING NEW ONE");

            let wallet_outcome = self.create_wallet(self.wallet_name.as_str()).unwrap();

            println!(
                "Wallet {} created with Outcome: {:?}",
                self.wallet_name, wallet_outcome
            );
        }

        Ok(())
    }

    pub fn create_wallet(&self, name: &str) -> BtcResult<LoadWalletResult> {
        self.client
            .create_wallet(name, Some(false), Some(false), None, None)
    }

    pub fn get_balance(&self) -> BtcResult<Amount> {
        self.client.get_balance(None, None)
    }

    pub fn get_p2wsh_address(&self) -> BtcResult<String> {
        let address = self
            .client
            .get_new_address(None, Some(AddressType::Bech32m))?
            .require_network(Network::Regtest)
            .unwrap()
            .to_string();

        Ok(address)
    }

    pub fn parse_address(&self, address: &str) -> Result<Address, String> {
        use core::str::FromStr;

        match Address::from_str(address) {
            Ok(parsed) => match parsed.require_network(Network::Regtest) {
                Ok(value) => Ok(value),
                Err(error) => Err(error.to_string()),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn list_unspent(&self) -> Vec<ListUnspentResultEntry> {
        self.client
            .list_unspent(Option::None, Option::None, None, None, None)
            .unwrap()
    }

    pub fn load_wallet(&self) {
        let load_wallet = self.client.load_wallet(self.wallet_name.as_str()).unwrap();
        dbg!(load_wallet);
    }

    pub fn generate(&self, num: u64) {
        let rpc_port = format!("-rpcport={}", self.rpc_port);
        let data_dir = format!("-datadir={}", self.data_dir.as_str());
        let outcome = std::process::Command::new("bitcoin-cli")
            .args([
                rpc_port.as_str(),
                data_dir.as_str(),
                "-generate",
                num.to_string().as_str(),
            ])
            .output()
            .expect("failed to execute process");

        println!("GENERATED BLOCKS OUTCOME: \n {:#?}", outcome);
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    rpc_username: String,
    rpc_password: String,
    rpc_port: String,
    wallet_name: String,
    cold: bool,
    data_dir: String,
}

impl Config {
    fn load(config_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let mut file = File::open(config_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config = toml::from_str::<Self>(&contents).unwrap();

        Ok(config)
    }
}
