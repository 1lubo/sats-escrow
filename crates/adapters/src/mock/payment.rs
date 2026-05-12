//! Mock payment processor

use chrono::Utc;

use sats_escrow_core::{
    escrow::Escrow,
    traits::payment::{BoxFuture, PaymentProcessor},
    types::{CurrencyPair, ExchangeRate, Invoice},
    Result,
};

/// Mock payment processor with configurable exchange rate
pub struct MockPaymentProcessor {
    /// Fixed BTC/USD rate for mocking
    btc_usd_rate: f64,
}

impl MockPaymentProcessor {
    pub fn new(btc_usd_rate: f64) -> Self {
        Self { btc_usd_rate }
    }
}

impl Default for MockPaymentProcessor {
    fn default() -> Self {
        Self::new(60_000.0) // Default $60k per BTC
    }
}

impl PaymentProcessor for MockPaymentProcessor {
    fn create_invoice(&self, escrow: &Escrow) -> BoxFuture<'_, Result<Invoice>> {
        let escrow = escrow.clone();
        Box::pin(async move {
            let deposit_address = escrow
                .deposit_address
                .clone()
                .unwrap_or_else(|| sats_escrow_core::types::DepositAddress("pending".to_string()));

            Ok(Invoice {
                id: format!("inv_{}", uuid::Uuid::new_v4()),
                escrow_id: escrow.id.clone(),
                amount: escrow.amount,
                deposit_address,
                expires_at: Utc::now() + chrono::Duration::hours(24),
            })
        })
    }

    fn get_exchange_rate(&self, pair: &CurrencyPair) -> BoxFuture<'_, Result<ExchangeRate>> {
        let pair = pair.clone();
        Box::pin(async move {
            if pair.base == "BTC" && pair.quote == "USD" {
                Ok(ExchangeRate(self.btc_usd_rate))
            } else if pair.base == "USD" && pair.quote == "BTC" {
                Ok(ExchangeRate(1.0 / self.btc_usd_rate))
            } else {
                // Return 1.0 for unknown pairs
                Ok(ExchangeRate(1.0))
            }
        })
    }

    fn cancel_invoice(&self, _invoice_id: &str) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            // Mock always succeeds
            Ok(())
        })
    }
}
