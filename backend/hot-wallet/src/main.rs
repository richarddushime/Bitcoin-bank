use common::Wallet;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let wallet = Wallet::new(args[1].as_str());

    dbg!(&wallet.does_wallet_exist());
    dbg!(&wallet.get_balance());
}

fn wallet_balancer(wallet: &Wallet) -> io::Result<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Failed to connect to server");

    let data_to_send = prepare_address(&wallet);
    stream
        .write_all(data_to_send.as_bytes())
        .expect("Failed to write data to stream");

    let mut received_data = String::new();
    stream
        .read_to_string(&mut received_data)
        .expect("Failed to read data from stream");

    Ok(received_data)
}

fn prepare_address(wallet: &Wallet) -> String {
    wallet.get_p2wsh_address().unwrap()
}
