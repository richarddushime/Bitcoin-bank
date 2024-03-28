use crate::CommonNodeDetails;
use bitcoincore_rpc::{
    bitcoin::{Address, Amount, Network},
    bitcoincore_rpc_json::{
        AddressType, EstimateMode, GetNetworkInfoResult, ListUnspentResultEntry, LoadWalletResult,
    },
    Auth, Client, Result as BtcResult, RpcApi,
};
use std::str::FromStr;

pub struct RpcOps {
    client: Client,
}

impl RpcOps {
    pub fn new(config: &CommonNodeDetails) -> Self {
        let rpc_port = format!("http://localhost:{}", &config.rpc_port);
        let client = Client::new(
            &rpc_port,
            Auth::UserPass(
                config.rpc_username.to_owned(),
                config.rpc_password.to_owned(),
            ),
        )
        .expect("Encountered error while creating a connection to Bitcoin core");

        Self { client }
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

    pub fn create_wallet(&self, name: &str) -> BtcResult<LoadWalletResult> {
        self.client
            .create_wallet(name, Some(false), Some(false), None, None)
    }

    pub fn get_balance(&self) -> BtcResult<Amount> {
        self.client.get_balance(None, None)
    }

    pub fn get_bech32m_address(&self) -> BtcResult<String> {
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
}
