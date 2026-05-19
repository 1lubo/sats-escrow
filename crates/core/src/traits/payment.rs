//! Payment processor trait for invoicing and exchange rates

use std::future::Future;
use std::pin::Pin;

use crate::{
    error::Result,
    escrow::Escrow,
    types::{CurrencyPair, ExchangeRate, Invoice},
};

/// Boxed future type for dyn compatibility
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for payment processors
///
/// Payment processors handle:
/// - Creating invoices for escrows
/// - Getting exchange rates
/// - (Future) Fiat on/off ramps
pub trait PaymentProcessor: Send + Sync {
    /// Create an invoice for an escrow
    fn create_invoice(&self, escrow: &Escrow) -> BoxFuture<'_, Result<Invoice>>;

    /// Get current exchange rate for a currency pair
    fn get_exchange_rate(&self, pair: &CurrencyPair) -> BoxFuture<'_, Result<ExchangeRate>>;

    /// Cancel an existing invoice
    fn cancel_invoice(&self, invoice_id: &str) -> BoxFuture<'_, Result<()>>;
}
