use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;
use tracing::debug;

const API_URL: &str = "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@{date}/v1";
const DEFAULT_DATE: &str = "latest";

fn get_api_url(date: Option<&str>) -> String {
    API_URL.replace("{date}", date.unwrap_or(DEFAULT_DATE))
}

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
            format!("{}/currencies.json", get_api_url(None))
        );

        // get the all the supported currencies and populate the .rates first with None.
        let supported_currencies: HashMap<String, String> =
            reqwest::get(format!("{}/currencies.json", get_api_url(None)))
                .await?
                .json()
                .await?;

        self.official_name = supported_currencies;

        // preload common currencies used in our servers.
        for currency in ["myr", "idr", "sgd", "jpy", "eur"] {
            let _ = self.fetch(currency, None).await;
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
    pub async fn fetch(
        &mut self,
        currency: &str,
        date_opt: Option<&str>,
    ) -> Result<Option<CurrencyRate>, reqwest::Error> {
        let date = date_opt.unwrap_or(DEFAULT_DATE);
        let key: String = format!("{}-{}", currency, date);

        if let std::collections::hash_map::Entry::Vacant(e) = self.rates.entry(key.clone()) {
            debug!(
                "Fetching currency rate for {}: {}",
                currency,
                format!("{}/currencies/{currency}.json", get_api_url(date_opt))
            );

            let resp = reqwest::get(format!(
                "{}/currencies/{currency}.json",
                get_api_url(date_opt)
            ))
            .await?;
            if !resp.status().is_success() {
                return Ok(None);
            }
            let data: serde_json::Value = resp.json().await?;

            let rate = CurrencyRate {
                name: currency.to_string(),
                date: data["date"].as_str().unwrap_or_default().to_string(),
                rates: serde_json::from_value(json!(data[currency])).unwrap_or_default(),
            };

            e.insert(rate);
        }

        Ok(Some(self.rates.get(&key).cloned().unwrap()))
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
            let _ = self.fetch(&currency, None).await;
        }
    }
}
