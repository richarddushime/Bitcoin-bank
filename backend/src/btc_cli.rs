use std::{io, process::Output};

use crate::CommonNodeDetails;

#[derive(Default)]
pub struct BtcNative {
    program: String,
    args: Vec<String>,
}

impl BtcNative {
    pub fn bitcoind_start(config: &CommonNodeDetails, addnode: Option<&str>) -> io::Result<Output> {
        let mut outcome = Self {
            program: "bitcoind".to_owned(),
            args: vec![
                "-daemon".to_owned(),
                "-regtest=1".to_owned(),
                "-server=1".to_owned(),
                "-txindex=1".to_owned(),
                "-rpcallowip=127.0.0.1".to_owned(),
                "-fallbackfee=0.00001".to_owned(),
                "-maxtxfee=0.0001".to_owned(),
                format!("-port={}", config.port),
                BtcNative::datadir(config),
                BtcNative::rpcport(config),
                BtcNative::rpcuser(config),
                BtcNative::rpcpassword(config),
            ],
        };

        if let Some(addnode_exists) = addnode {
            outcome.args.push(format!("-addnode={}", addnode_exists))
        }

        outcome.execute()
    }

    pub fn bitcoin_cli(config: &CommonNodeDetails) -> Self {
        Self {
            program: "bitcoin-cli".to_owned(),
            args: vec![
                BtcNative::rpcport(config),
                BtcNative::rpcuser(config),
                BtcNative::rpcpassword(config),
            ],
        }
    }

    pub fn add_arg(&mut self, arg: String) -> &mut Self {
        self.args.push(arg);

        self
    }

    pub fn add_args(&mut self, args: &[&str]) -> &mut Self {
        args.into_iter().for_each(|arg| {
            self.args.push(arg.to_string());
        });

        self
    }

    pub fn generate_blocks(&mut self, blocks: u64) -> io::Result<Output> {
        self.args.push(String::from("-generate"));
        self.args.push(format!("{}", blocks.to_string().trim()));

        self.execute()
    }

    pub fn bitcoind_stop(config: &CommonNodeDetails) -> io::Result<Output> {
        BtcNative::bitcoin_cli(config)
            .add_arg(String::from("stop"))
            .execute()
    }

    pub fn rpcport(config: &CommonNodeDetails) -> String {
        format!("-rpcport={}", config.rpc_port)
    }

    pub fn datadir(config: &CommonNodeDetails) -> String {
        format!("--datadir={}", config.data_dir)
    }

    pub fn rpcuser(config: &CommonNodeDetails) -> String {
        format!("-rpcuser={}", config.rpc_username)
    }

    pub fn rpcpassword(config: &CommonNodeDetails) -> String {
        format!("-rpcpassword={}", config.rpc_password)
    }

    pub fn create_wallet(config: &CommonNodeDetails) -> io::Result<Output> {
        BtcNative::bitcoin_cli(config)
            .add_args(&[
                "-named",
                "createwallet",
                format!("wallet_name={}", config.wallet_name,).as_str(),
                "load_on_startup=true",
            ])
            .execute()
    }

    pub fn execute(&self) -> io::Result<Output> {
        std::process::Command::new(&self.program)
            .args(&self.args)
            .output()
    }
}
