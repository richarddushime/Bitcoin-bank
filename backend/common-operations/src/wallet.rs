use bitcoincore_rpc::{
    bitcoin::{Address, Amount, Network},
    bitcoincore_rpc_json::{
        AddressType, EstimateMode, GetNetworkInfoResult, ListUnspentResultEntry, LoadWalletResult,
    },
    Auth, Client, Result as BtcResult, RpcApi,
};
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct Wallet {
    client: Client,
    pub wallet_name: String,
    pub data_dir: PathBuf,
    pub rpc_port: String,
}

impl Wallet {
    pub fn new(type_info: &str) -> Self {
        let config = Config::load(type_info).expect("Encountered error  while opening file");
        let rpc_port = format!("http://localhost:{}", &config.rpc_port);
        let client = Client::new(
            &rpc_port,
            Auth::UserPass(config.rpc_username, config.rpc_password),
        )
        .expect("Encountered error while creating a connection to Bitcoin core");

        Self {
            client,
            wallet_name: config.wallet_name,
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

    pub fn does_wallet_exist(&self) {
        match self.load_wallet() {
            Ok(_) => println!("WALLET {} was loaded", self.wallet_name),
            Err(error) => {
                let error_str = error.to_string();

                if error_str.contains("is already loaded") {
                    println!("WALLET ALREADY LOADED");
                } else {
                    println!("WALLET NOT FOUND, CREATING NEW ONE");

                    let wallet_outcome = self.create_wallet(self.wallet_name.as_str()).unwrap();

                    println!(
                        "Wallet {} created with Outcome: {:?}",
                        self.wallet_name, wallet_outcome
                    );
                }
            }
        }
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

    pub fn generate_blocks(&self, num: u64) {
        let rpc_port = format!("-rpcport={}", self.rpc_port);
        let data_dir = format!("-datadir={}", self.data_dir.display());
        std::process::Command::new("bitcoin-cli")
            .args([
                rpc_port.as_str(),
                data_dir.as_str(),
                "-generate",
                num.to_string().as_str(),
            ])
            .output()
            .expect("failed to execute process");

        println!("GENERATED {} BLOCKS ", num);
    }

    pub fn send_amount(&self, address: &str, amount: u64) -> BtcResult<String> {
        let address = self.parse_address(address).unwrap();
        let amount = Amount::from_sat(amount);

        let tx = self.client.send_to_address(
            &address,
            amount,
            None,
            None,
            None,
            None,
            None,
            Some(EstimateMode::Conservative),
        )?;

        Ok(tx.to_string())
    }

    pub fn load_wallet(&self) -> BtcResult<LoadWalletResult> {
        self.client.load_wallet(&self.wallet_name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    rpc_username: String,
    rpc_password: String,
    rpc_port: String,
    wallet_name: String,
    data_dir: PathBuf,
}

impl Config {
    fn load(type_info: &str) -> io::Result<Self> {
        let mut contents = String::new();
        let data_dir;
        let mut file = if type_info == "cold" {
            data_dir = Config::cold_wallet_dir();

            File::open(Config::cold_wallet_conf())?
        } else if type_info == "hot" {
            data_dir = Config::hot_wallet_dir();
            File::open(Config::hot_wallet_conf())?
        } else {
            panic!("Incorrect parameter, expected `hot` or `cold`");
        };

        file.read_to_string(&mut contents)?;

        let mut config = toml::from_str::<Self>(&contents).unwrap();
        config.data_dir = data_dir;

        Ok(config)
    }

    pub fn manifest_dir_path() -> PathBuf {
        let env_path = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(env_path).parent().unwrap();

        path.to_path_buf()
    }

    pub fn config_dir() -> PathBuf {
        let mut manifest = Config::manifest_dir_path();

        manifest.push("bitcoin-bank-conf");

        manifest
    }

    fn cold_wallet_conf() -> PathBuf {
        let mut conf = Config::config_dir();
        conf.push("btc_cold.conf");

        conf
    }

    fn hot_wallet_conf() -> PathBuf {
        let mut conf = Config::config_dir();
        conf.push("btc_hot.conf");

        conf
    }

    fn cold_wallet_dir() -> PathBuf {
        let mut conf = Config::config_dir();
        conf.push("cold-wallet");

        conf
    }

    fn hot_wallet_dir() -> PathBuf {
        let mut conf = Config::config_dir();
        conf.push("hot-wallet");

        conf
    }
}
