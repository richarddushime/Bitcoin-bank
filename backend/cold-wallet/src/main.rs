use common::Wallet;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let wallet = Wallet::new(args[1].as_str());

    wallet.generate_blocks(100);
    wallet.generate_blocks(50);

    dbg!(&wallet.does_wallet_exist());
    dbg!(&wallet.get_balance());

    let socket = "127.0.0.1:8000";

    let listener = TcpListener::bind(socket).unwrap();

    println!("LISTENING AT {}", socket);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &wallet).unwrap();
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, wallet: &Wallet) -> io::Result<()> {
    let mut buffer = [0u8; 2usize.pow(16) - 1]; //Address should not be bigger than this
    loop {
        let nbytes = stream.read(&mut buffer).unwrap();
        if nbytes == 0 {
            return Ok(());
        }
        let address = String::from_utf8_lossy(&buffer[..nbytes]);
        println!("Received: {}", &address);

        let outcome = process_tx(&wallet, address.to_string().as_str());
        stream.write_all(outcome.as_bytes())?;

        stream.flush()?;

        return Ok(());
    }
}

fn process_tx(wallet: &Wallet, address: &str) -> String {
    let current_amount = wallet.get_balance().unwrap().to_sat();

    if current_amount < 100_000_000 {
        println!("BALANCE LOW... GENERATING BLOCKS");
        wallet.generate_blocks(100);
        println!("GENERATED 100 BLOCKS");
        println!("MINING THE BLOCKS");
        wallet.generate_blocks(10);
        println!("GENERATED 100 BLOCKS");
    }
    let thirty_percent = current_amount as f64 * 0.45;

    let tx = wallet.send_amount(address, thirty_percent as u64).unwrap();
    wallet.generate_blocks(5);

    tx
}
