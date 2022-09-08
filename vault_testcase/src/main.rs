pub mod account;
pub mod rest_client;
pub mod faucet_client;
pub mod managed_coin_client;
pub mod vault_client;

use crate::account::Account;
use crate::vault_client::VaultClient;
use crate::faucet_client::FaucetClient;
use crate::managed_coin_client::ManagedCoinClient;

use std::env;

pub const TESTNET_URL: &str = "https://fullnode.devnet.aptoslabs.com";
pub const FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";

fn main() -> () {    
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        2,
        "Expecting an argument that points to the MoonCoin module"
    );

    let client = VaultClient::new(TESTNET_URL.to_string());
    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), client.rest_client.clone());
    let managed_coin_client = ManagedCoinClient::new(client.rest_client.clone());

    // Create three accounts, Admin, Alice and Bob
    let mut admin = Account::new(None);
    let mut alice = Account::new(None);
    

    println!("\n=== Addresses ===");
    println!("Admin: 0x{}", admin.address());
    println!("Alice: 0x{}", alice.address());
    

    faucet_client.fund_account(&admin.auth_key(), 10_000_000);
    faucet_client.fund_account(&alice.auth_key(), 10_000_000);    

    println!("\n=== Initial Balances ===");
    println!("Admin: {:?}", client.rest_client.account_balance(&admin.address()));
    println!("Alice: {:?}", client.rest_client.account_balance(&alice.address()));
    

    println!("\nUpdate the module with Admin's address, build, copy to the provided path, and press enter.");
    match std::io::stdin().read_line(&mut String::new()) {
        Ok(_n) => {}
        Err(error) => println!("error: {}", error),
    }

    let module_path = args.get(1).unwrap();
    let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    println!("Publishing Vault module...");    
    let mut tx_hash = client.publish_module(&mut admin, &module_hex);
    client.rest_client.wait_for_transaction(&tx_hash);


    
    println!("admin will initialize the new managed_coin");
    tx_hash = managed_coin_client.initialize_coin(&mut admin);
    client.rest_client.wait_for_transaction(&tx_hash);

    println!("Alice registers the newly created coin so he can receive it from Admin");
    tx_hash = managed_coin_client.register_coin(&mut alice, &admin.address());
    client.rest_client.wait_for_transaction(&tx_hash);

    println!("mint to alice some of the new coin");
    tx_hash = managed_coin_client.mint_coin(&mut admin, &alice.address(), 100);
    client.rest_client.wait_for_transaction(&tx_hash);

    println!(
        "alice's manage_coin balance: {}",
        managed_coin_client.get_balance(&alice.address(), &admin.address())
    );

 
        
    println!("Initialize Vault");
    tx_hash = client.initialize_vault(&admin.address(), &mut admin);
    client.rest_client.wait_for_transaction(&tx_hash);

    println!("Pause Vault");
    tx_hash = client.pause_vault(&admin.address(), &mut admin);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!(
        "Pause status: {:?}",
        client.get_pause_status(&admin.address(), &admin.address()).unwrap()
    );
    
    println!("Unpause Vault");
    tx_hash = client.unpause_vault(&admin.address(), &mut admin);
    client.rest_client.wait_for_transaction(&tx_hash);

    println!(
        "Pause status: {:?}",
        client.get_pause_status(&admin.address(), &admin.address()).unwrap()
    );



    println!("Deposit with 10 managed_coin");
    tx_hash = client.deposit(&admin.address(), &mut alice, 10);
    client.rest_client.wait_for_transaction(&tx_hash);
    
    println!(
        "alice's managed_coin balance: {}",
        managed_coin_client.get_balance(&alice.address(), &admin.address())
    );

    println!("Withdraw for 5 managed_coin");
    tx_hash = client.withdraw(&admin.address(), &mut alice, 5);
    client.rest_client.wait_for_transaction(&tx_hash);
    
    println!(
        "alice's manage_coin balance: {}",
        managed_coin_client.get_balance(&alice.address(), &admin.address())
    );

    

    

    // println!("Withdraw");



}
