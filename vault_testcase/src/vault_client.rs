use crate::rest_client::{RestClient};
use crate::account::{Account};

pub struct VaultClient {
    pub rest_client: RestClient,
}

impl VaultClient {
    /// Represents an account as well as the private, public key-pair for the Aptos blockchain.
    pub fn new(url: String) -> Self {
        Self {
            rest_client: RestClient::new(url),
        }
    }
    pub fn publish_module(&self, account_from: &mut Account, module_hex: &str) -> String {
        let payload = serde_json::json!({
            "type": "module_bundle_payload",
            "modules": [{"bytecode": format!("0x{}", module_hex)}],
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }    
    
    /// Initializes the vault
    pub fn initialize_vault(&self, contract_address: &str, account_from: &mut Account) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Vault::init_vault", contract_address),
            "type_arguments": [],
            "arguments": []
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }
    // Pause the vault
    pub fn pause_vault(&self, contract_address: &str, account_from: &mut Account) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Vault::pause_vault", contract_address),
            "type_arguments": [],
            "arguments": []
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }

    // Unpause the vault (resume)
    pub fn unpause_vault(&self, contract_address: &str, account_from: &mut Account) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Vault::unpause_vault", contract_address),
            "type_arguments": [],
            "arguments": []
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }
    pub fn get_pause_status(&self, contract_address: &str, account_address: &str) -> Option<String> {
        let module_type = format!("0x{}::Vault::VaultHolder", contract_address);
        self.rest_client
            .account_resource(account_address, &module_type)
            .map(|value| format!("{:?}", value["data"]["paused"]))
    }
    
    // Deposit the vault
    pub fn deposit(&self, contract_address: &str, account_from: &mut Account, amount: u64) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Vault::deposit", contract_address),
            "type_arguments": [],
            "arguments": [
                format!("{}",amount),
                contract_address
            ]
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }

    // Deposit the vault
    pub fn withdraw(&self, contract_address: &str, account_from: &mut Account, amount: u64) -> String {
        let payload = serde_json::json!({
            "type": "script_function_payload",
            "function": format!("0x{}::Vault::withdraw", contract_address),
            "type_arguments": [],
            "arguments": [
                format!("{}",amount),
                contract_address
            ]
        });
        self.rest_client
            .execution_transaction_with_payload(account_from, payload)
    }

}