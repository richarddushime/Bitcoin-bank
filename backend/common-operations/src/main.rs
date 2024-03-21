mod wallet;
pub use wallet::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let wallet = Wallet::new(args[1].as_str());

    dbg!(&wallet.does_wallet_exist());
    //dbg!(&wallet.load_wallet());
    dbg!(&wallet.get_balance());
    dbg!(&wallet.generate(100));
    dbg!(&wallet.get_balance());
    dbg!(&wallet.generate(10));
    dbg!(&wallet.get_balance());

    dbg!(&wallet.list_unspent());

    dbg!(&wallet.get_p2wsh_address());
}
