use std::collections::HashMap;
use tracing::debug;

use serde::Deserialize;
use serde_json::json;

const API_URL: &str = "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1";

/// Holds the information for a certain currency and its exchange rates.
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyRate {
    pub name: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

impl CurrencyRate {
    /// Get the exchange rate from this currency to the target currency.
    pub fn get_rate_to(&self, target_currency: &str) -> Option<f64> {
        self.rates.get(target_currency).cloned()
    }
}

/// Holds the information about all currencies and their exchange rates.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Currencies {
    pub rates: HashMap<String, CurrencyRate>,
    pub official_name: HashMap<String, String>,
}

impl Currencies {
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the supported currencies from the API.
    pub async fn load(&mut self) -> Result<(), reqwest::Error> {
        debug!(
            "Fetching currencies: {}",
            format!("{API_URL}/currencies.json")
        );

        // get the all the supported currencies and populate the .rates first with None.
        let supported_currencies: HashMap<String, String> =
            reqwest::get(format!("{API_URL}/currencies.json"))
                .await?
                .json()
                .await?;

        self.official_name = supported_currencies;

        // preload common currencies used in our servers.
        for currency in ["myr", "idr", "sgd", "jpy", "eur"] {
            let _ = self.fetch(currency).await;
        }

        Ok(())
    }

    /// Get the official name of a currency, or return the currency code if not found.
    pub fn get_official_name(&self, currency: &str) -> String {
        self.official_name
            .get(currency)
            .cloned()
            .unwrap_or_else(|| currency.to_string())
    }

    /// Fetches the exchange rates for a specific currency if not already loaded.
    pub async fn fetch(&mut self, currency: &str) -> Result<Option<CurrencyRate>, reqwest::Error> {
        if !self.rates.contains_key(currency) {
            debug!(
                "Fetching currency rate for {}: {}",
                currency,
                format!("{API_URL}/currencies/{currency}.json")
            );

            let resp = reqwest::get(format!("{API_URL}/currencies/{currency}.json")).await?;
            if !resp.status().is_success() {
                return Ok(None);
            }
            let data: serde_json::Value = resp.json().await?;

            let rate = CurrencyRate {
                name: currency.to_string(),
                date: data["date"].as_str().unwrap_or_default().to_string(),
                rates: serde_json::from_value(json!(data[currency])).unwrap_or_default(),
            };

            self.rates.insert(currency.to_string(), rate);
        }

        Ok(Some(self.rates.get(currency).cloned().unwrap()))
    }

    /// Clears the cached exchange rates.
    pub fn clear_cache(&mut self) {
        self.rates.clear();
    }

    /// Refreshes the exchange rates for all cached currencies.
    pub async fn refresh(&mut self) {
        let currencies: Vec<String> = self.rates.keys().cloned().collect();
        self.rates.clear();

        for currency in currencies {
            let _ = self.fetch(&currency).await;
        }
    }
}
