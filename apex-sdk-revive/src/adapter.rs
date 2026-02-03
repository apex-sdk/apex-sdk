use crate::{Error, Result};
use apex_sdk_types::Address;
use subxt::dynamic::Value;
use subxt::{OnlineClient, PolkadotConfig};

/// Adapter for interacting with pallet-revive on System Chains
pub struct ReviveAdapter {
    client: OnlineClient<PolkadotConfig>,
}

impl ReviveAdapter {
    /// Connect to a node with pallet-revive
    pub async fn connect(url: &str) -> Result<Self> {
        let client = OnlineClient::from_url(url)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(Self { client })
    }

    /// Get the underlying subxt client
    pub fn client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    /// Get the balance of an address (native currency)
    pub async fn get_balance(&self, address: &Address) -> Result<u128> {
        let _addr_str = match address {
            apex_sdk_types::Address::Substrate(addr) => addr.clone(),
            apex_sdk_types::Address::Evm(addr) => addr.clone(),
        };

        // Query the system account storage for balances
        let storage_address = subxt::dynamic::storage(
            "System",
            "Account",
            vec![Value::from_bytes(address.as_str().as_bytes())],
        );

        let account_info = self
            .client
            .storage()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?
            .fetch(&storage_address)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let account_info_value = account_info
            .expect("Missing account info")
            .to_value()
            .expect("Failed to decode value");

        // Extract the balance from AccountInfo
        let balance = account_info_value
            .as_u128()
            .ok_or(Error::Other("Invalid balance type".into()))?;
        Ok(balance)
    }
}
