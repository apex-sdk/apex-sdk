use crate::{Result, ReviveAdapter};
use apex_sdk_types::Address;
use subxt::tx::Signer;
use tracing::info;

/// High-level API for Solidity contract lifecycle on pallet-revive
pub struct ContractManager<'a, S: Signer<subxt::PolkadotConfig>> {
    adapter: &'a ReviveAdapter,
    signer: S,
}

impl<'a, S: Signer<subxt::PolkadotConfig>> ContractManager<'a, S> {
    pub fn new(adapter: &'a ReviveAdapter, signer: S) -> Self {
        Self { adapter, signer }
    }

    /// Deploy a Solidity contract (PolkaVM bytecode)
    pub async fn deploy(
        &self,
        code: Vec<u8>,
        constructor_data: Vec<u8>,
        salt: [u8; 32],
    ) -> Result<Address> {
        info!("Deploying contract to pallet-revive...");

        // Prepare extrinsic call
        let client = self.adapter.client();
        let tx = subxt::dynamic::tx(
            "PalletRevive",
            "instantiate",
            vec![
                subxt::dynamic::Value::from(code),
                subxt::dynamic::Value::from(constructor_data),
                subxt::dynamic::Value::from(salt.to_vec()),
            ],
        );

        let finalized = client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        // Extract contract address from events using Subxt dynamic API
        let address = finalized
            .iter()
            .filter_map(|ev| {
                let event_details = ev.as_ref().ok()?;
                // Use dynamic inspection of event details
                if event_details.pallet_name() == "PalletRevive"
                    && event_details.variant_name() == "Instantiated"
                {
                    let fields = event_details.field_bytes();
                    if fields.len() >= 32 {
                        let addr_bytes = &fields[fields.len() - 32..];
                        let hex_addr = format!("0x{}", hex::encode(addr_bytes));
                        return Some(Address::evm(hex_addr));
                    }
                }
                None
            })
            .next()
            .unwrap_or_else(|| Address::evm("0x0000000000000000000000000000000000000000"));
        Ok(address)
    }
}

/// Represents a deployed contract on pallet-revive
pub struct Contract<T> {
    address: Address,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Contract<T> {
    pub fn new(address: Address) -> Self {
        Self {
            address,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}
