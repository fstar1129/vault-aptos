use crate::account::Account;
use crate::rest_client::RestClient;

pub struct ManagedCoinClient {
    rest_client: RestClient,
}

impl ManagedCoinClient {
    pub fn new( rest_client: RestClient) -> Self {
        Self { rest_client }
    }

    /// Initializes the new coin.
    pub fn initialize_coin(&self, account_from: &mut Account) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": "0x1::managed_coin::initialize",
            "type_arguments": [format!("0x{}::Vault::ManagedCoin", account_from.address())],
            "arguments": [
                hex::encode("Moon Coin".as_bytes()),
                hex::encode("MOON".as_bytes()),
                "6",
                false,
            ]
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }
    /// Receiver needs to register the coin before they can receive it.
    pub fn register_coin(&self, account_receiver: &mut Account, coin_type_address: &str) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": "0x1::coin::register",
            "type_arguments": [format!("0x{}::Vault::ManagedCoin", coin_type_address)],
            "arguments": []
        });
        self.rest_client
            .execution_transaction_with_payload(account_receiver, payload)
    }
    /// Receiver needs to register the coin before they can receive it.
    pub fn mint_coin(
        &self,
        account_owner: &mut Account,
        receiver_address: &str,
        amount: u64,
    ) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": "0x1::managed_coin::mint",
            "type_arguments": [format!("0x{}::Vault::ManagedCoin", account_owner.address())],
            "arguments": [
                receiver_address,
                amount.to_string(),
            ]
        });
        self.rest_client
            .execution_transaction_with_payload(account_owner, payload)
    }
    /// Receiver needs to register the coin before they can receive it.
    pub fn get_balance(&self, account_address: &str, coin_type_address: &str) -> u64 {
        let module_type = format!(
            "0x1::coin::CoinStore<0x{}::Vault::ManagedCoin>",
            coin_type_address,
        );
        self.rest_client
            .account_resource(account_address, &module_type)
            .map(|value| {
                value["data"]["coin"]["value"]
                    .as_str()
                    .unwrap()
                    .to_string()
                    .parse::<u64>()
                    .unwrap()
            })
            .unwrap()
    }
}

