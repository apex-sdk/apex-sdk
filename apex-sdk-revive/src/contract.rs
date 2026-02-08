use crate::{Error, Result, ReviveAdapter};
use apex_sdk_types::Address;
use subxt::dynamic::{At, Value};
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
        value: u128,
        gas_limit: Option<u64>,
    ) -> Result<Address> {
        info!("Deploying contract to pallet-revive...");

        let gas_limit_val = if let Some(g) = gas_limit {
            Value::from(g)
        } else {
            Value::unnamed_variant("ReadOnly", vec![])
        };

        // Prepare extrinsic call
        let client = self.adapter.client();
        let tx = subxt::dynamic::tx(
            "Revive",
            "instantiate",
            vec![
                Value::from(value),
                gas_limit_val,
                Value::from(code),
                Value::from(constructor_data),
                Value::from(salt.to_vec()),
            ],
        );

        let finalized = client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        // Extract contract address from events
        let address = finalized
            .iter()
            .filter_map(|ev| {
                let ev = ev.ok()?;
                if ev.pallet_name() == "Revive" && ev.variant_name() == "Instantiated" {
                    let fields = ev.field_values().ok()?;
                    if let Some(contract_field) = fields.at("contract") {
                        let s = format!("{:?}", contract_field);
                        if s.contains("0x") {
                            let parts: Vec<&str> = s.split('\"').collect();
                            for p in parts {
                                if p.starts_with("0x") && p.len() == 42 {
                                    return Some(Address::evm(p));
                                }
                            }
                        }
                        if s.contains('[') && s.contains(']') {
                            let content = s.trim_matches(|c| c == '[' || c == ']' || c == ' ');
                            let bytes: Vec<u8> = content
                                .split(',')
                                .filter_map(|b| b.trim().parse::<u8>().ok())
                                .collect();
                            if bytes.len() == 20 {
                                return Some(Address::evm(format!("0x{}", hex::encode(bytes))));
                            }
                        }
                    }
                }
                None
            })
            .next()
            .ok_or_else(|| {
                Error::Contract("Failed to extract contract address from events".into())
            })?;

        Ok(address)
    }

    /// Call a method on a deployed contract (Transaction)
    pub async fn call(
        &self,
        address: &Address,
        data: Vec<u8>,
        value: u128,
        gas_limit: Option<u64>,
    ) -> Result<Vec<u8>> {
        info!("Calling contract at {}...", address);

        let dest_bytes = match address {
            Address::Evm(e) => hex::decode(e.trim_start_matches("0x"))
                .map_err(|_| Error::Contract("Invalid EVM address".into()))?,
            Address::Substrate(_) => {
                return Err(Error::Contract(
                    "Revive calls require EVM-style addresses".into(),
                ))
            }
        };

        let gas_limit_val = if let Some(g) = gas_limit {
            Value::from(g)
        } else {
            Value::unnamed_variant("ReadOnly", vec![])
        };

        let client = self.adapter.client();
        let tx = subxt::dynamic::tx(
            "Revive",
            "call",
            vec![
                Value::from(dest_bytes),
                Value::from(value),
                gas_limit_val,
                Value::from(data),
            ],
        );

        let finalized = client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        // Extract return data from events if present
        let return_data = finalized
            .iter()
            .filter_map(|ev| {
                let ev = ev.ok()?;
                if ev.pallet_name() == "Revive" && ev.variant_name() == "Called" {
                    let fields = ev.field_values().ok()?;
                    if let Some(data_field) = fields.at("return_data") {
                        let s = format!("{:?}", data_field);
                        if s.contains('[') && s.contains(']') {
                            let content = s.trim_matches(|c| c == '[' || c == ']' || c == ' ');
                            let bytes: Vec<u8> = content
                                .split(',')
                                .filter_map(|b| b.trim().parse::<u8>().ok())
                                .collect();
                            return Some(bytes);
                        }
                    }
                }
                None
            })
            .next()
            .unwrap_or_default();

        Ok(return_data)
    }

    /// Query contract state (Dry-run/Static call)
    pub async fn read(&self, address: &Address, data: Vec<u8>, value: u128) -> Result<Vec<u8>> {
        info!("Reading contract state at {}...", address);

        let dest_bytes = match address {
            Address::Evm(e) => hex::decode(e.trim_start_matches("0x"))
                .map_err(|_| Error::Contract("Invalid EVM address".into()))?,
            Address::Substrate(_) => {
                return Err(Error::Contract(
                    "Revive reads require EVM-style addresses".into(),
                ))
            }
        };

        let client = self.adapter.client();
        let tx = subxt::dynamic::tx(
            "Revive",
            "read",
            vec![
                Value::from(dest_bytes),
                Value::from(value),
                Value::unnamed_variant("ReadOnly", vec![]),
                Value::from(data),
            ],
        );

        let finalized = client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &self.signer)
            .await?
            .wait_for_finalized_success()
            .await?;

        let return_data = finalized
            .iter()
            .filter_map(|ev| {
                let ev = ev.ok()?;
                if ev.pallet_name() == "Revive" && ev.variant_name() == "Read" {
                    let fields = ev.field_values().ok()?;
                    if let Some(data_field) = fields.at("return_data") {
                        let s = format!("{:?}", data_field);
                        if s.contains('[') && s.contains(']') {
                            let content = s.trim_matches(|c| c == '[' || c == ']' || c == ' ');
                            let bytes: Vec<u8> = content
                                .split(',')
                                .filter_map(|b| b.trim().parse::<u8>().ok())
                                .collect();
                            return Some(bytes);
                        }
                    }
                }
                None
            })
            .next()
            .unwrap_or_default();

        Ok(return_data)
    }

    /// Estimate gas for a deployment
    pub async fn estimate_deploy_gas(
        &self,
        _code: Vec<u8>,
        _constructor_data: Vec<u8>,
        _value: u128,
    ) -> Result<u64> {
        Ok(500_000)
    }

    /// Estimate gas for a call
    pub async fn estimate_call_gas(
        &self,
        _address: &Address,
        _data: Vec<u8>,
        _value: u128,
    ) -> Result<u64> {
        Ok(200_000)
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
