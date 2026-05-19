//! Custodian provider trait for fund management

use std::future::Future;
use std::pin::Pin;

use crate::{
    error::Result,
    types::{DepositAddress, EscrowId, Satoshis, TxId},
    user::UserId,
};

/// Boxed future type for dyn compatibility
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for custodian providers (e.g., Fireblocks, BitGo, or mock)
///
/// Custodians are responsible for:
/// - Creating deposit addresses for escrows
/// - Checking balances
/// - Executing transfers when escrows are released
pub trait CustodianProvider: Send + Sync {
    /// Create a new deposit address for an escrow
    fn create_deposit_address(&self, escrow_id: &EscrowId)
        -> BoxFuture<'_, Result<DepositAddress>>;

    /// Check the current balance for an escrow
    fn check_balance(&self, escrow_id: &EscrowId) -> BoxFuture<'_, Result<Satoshis>>;

    /// Transfer funds from escrow to a recipient
    fn transfer(
        &self,
        escrow_id: &EscrowId,
        recipient: &UserId,
        amount: Satoshis,
    ) -> BoxFuture<'_, Result<TxId>>;

    /// Get the deposit address for an escrow if it exists
    fn get_deposit_address(
        &self,
        escrow_id: &EscrowId,
    ) -> BoxFuture<'_, Result<Option<DepositAddress>>>;
}
