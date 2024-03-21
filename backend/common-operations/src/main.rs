mod wallet;
pub use wallet::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let wallet = Wallet::new(args[1].as_str());

    dbg!(&wallet.does_wallet_exist());
    //dbg!(&wallet.load_wallet());
    /*dbg!(&wallet.get_balance());
    dbg!(&wallet.generate_blocks(100));
    dbg!(&wallet.get_balance());
    dbg!(&wallet.generate_blocks(10));*/
    dbg!(&wallet.get_balance());

    let address = "bcrt1p5aafnyuujnsq26ljcufdlnrws7ddh22067sjn23mwgmmh5ksmgqqwekq3z";
    let tx = wallet.send_amount(address, 400_000).unwrap();
    dbg!(&tx);
    //dbg!(&wallet.get_p2wsh_address());
}
