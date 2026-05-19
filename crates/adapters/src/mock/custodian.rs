//! Mock custodian provider

use std::collections::HashMap;
use std::sync::RwLock;

use sats_escrow_core::{
    traits::custodian::{BoxFuture, CustodianProvider},
    types::{DepositAddress, EscrowId, Satoshis, TxId},
    user::UserId,
    Result,
};

/// Mock custodian that stores balances in memory
pub struct MockCustodian {
    /// Escrow ID -> (deposit address, balance)
    accounts: RwLock<HashMap<String, (DepositAddress, Satoshis)>>,
    /// Counter for generating addresses
    address_counter: RwLock<u64>,
}

impl MockCustodian {
    pub fn new() -> Self {
        Self {
            accounts: RwLock::new(HashMap::new()),
            address_counter: RwLock::new(0),
        }
    }

    /// Simulate a deposit (for testing)
    pub fn simulate_deposit(&self, escrow_id: &EscrowId, amount: Satoshis) {
        let mut accounts = self.accounts.write().unwrap();
        if let Some((_, balance)) = accounts.get_mut(&escrow_id.0.to_string()) {
            *balance = Satoshis(balance.0 + amount.0);
        }
    }
}

impl Default for MockCustodian {
    fn default() -> Self {
        Self::new()
    }
}

impl CustodianProvider for MockCustodian {
    fn create_deposit_address(
        &self,
        escrow_id: &EscrowId,
    ) -> BoxFuture<'_, Result<DepositAddress>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let mut counter = self.address_counter.write().unwrap();
            *counter += 1;
            let address = DepositAddress(format!("bc1qmock{:08x}", *counter));

            let mut accounts = self.accounts.write().unwrap();
            accounts.insert(escrow_id.0.to_string(), (address.clone(), Satoshis(0)));

            Ok(address)
        })
    }

    fn check_balance(&self, escrow_id: &EscrowId) -> BoxFuture<'_, Result<Satoshis>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let accounts = self.accounts.read().unwrap();
            Ok(accounts
                .get(&escrow_id.0.to_string())
                .map(|(_, balance)| *balance)
                .unwrap_or(Satoshis(0)))
        })
    }

    fn transfer(
        &self,
        escrow_id: &EscrowId,
        _recipient: &UserId,
        amount: Satoshis,
    ) -> BoxFuture<'_, Result<TxId>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let mut accounts = self.accounts.write().unwrap();
            if let Some((_, balance)) = accounts.get_mut(&escrow_id.0.to_string()) {
                if balance.0 >= amount.0 {
                    *balance = Satoshis(balance.0 - amount.0);
                } else {
                    // In mock/test mode, allow transfer even with insufficient balance
                    *balance = Satoshis(0);
                }
                return Ok(TxId(format!("tx_{}", uuid::Uuid::new_v4())));
            }
            // If escrow account doesn't exist, still succeed in mock mode
            Ok(TxId(format!("tx_{}", uuid::Uuid::new_v4())))
        })
    }

    fn get_deposit_address(
        &self,
        escrow_id: &EscrowId,
    ) -> BoxFuture<'_, Result<Option<DepositAddress>>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let accounts = self.accounts.read().unwrap();
            Ok(accounts
                .get(&escrow_id.0.to_string())
                .map(|(addr, _)| addr.clone()))
        })
    }
}
