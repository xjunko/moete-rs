use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Currencies {
    cache: HashMap<String, HashMap<String, f64>>,
}

impl Currencies {
    pub fn new() -> Self { Self::default() }

    fn cache_key(
        base: &str,
        target: &str,
        start_date: &str,
        end_date: &str,
    ) -> String {
        format!("{}-{}-{}-{}", base, target, start_date, end_date)
    }

    fn custom_provider(base: &str, target: &str) -> Option<String> {
        // some providers gave more detail floats, use those whenever possible...

        match (base, target) {
            ("myr", _) => Some("bnm".to_string()),
            (_, "myr") => Some("bnm".to_string()),
            _ => None,
        }
    }

    /// Fetches the exchange rates for a specific currency pair over a date range.
    pub async fn fetch_range(
        &mut self,
        base: &str,
        target: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<(HashMap<String, f64>, String), reqwest::Error> {
        // very accurate, but relies on FX market data
        if let Ok(twelve_rate) =
            self.fetch_twelve_data(base, target, start_date, end_date).await
        {
            return Ok((twelve_rate, "High".to_string()));
        }

        // works everytime, not accurate.
        if let Ok(frank_rate) =
            self.fetch_frankfurter(base, target, start_date, end_date).await
        {
            return Ok((frank_rate, "Average".to_string()));
        }

        Ok((HashMap::new(), "Unknown".to_string()))
    }
}

// frankfurter
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct FrankfurterRate {
    pub date: String,
    pub base: String,
    pub quote: String,
    pub rate: f64,
}

impl Currencies {
    async fn fetch_frankfurter(
        &mut self,
        base: &str,
        target: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<HashMap<String, f64>, reqwest::Error> {
        let key = Self::cache_key(base, target, start_date, end_date);

        if let Some(rates) = self.cache.get(&key) {
            return Ok(rates.clone());
        }

        let mut url = format!(
            "https://api.frankfurter.dev/v2/rates?from={}&to={}&base={}&quotes={}",
            start_date, end_date, base, target
        );

        if let Some(provider) = Self::custom_provider(base, target) {
            url.push_str(&format!("&providers={}", provider));
        }

        let resp: Vec<FrankfurterRate> =
            reqwest::get(&url).await?.json().await?;

        let mut rates = HashMap::new();
        for currency_rate in resp {
            rates.insert(currency_rate.date, currency_rate.rate);
        }

        self.cache.insert(key, rates.clone());

        Ok(rates)
    }
}

// twelve data
#[derive(Debug, Deserialize)]
struct TwelveDataResponse {
    values: Vec<TwelveDataValue>,
}

#[derive(Debug, Deserialize)]
struct TwelveDataValue {
    datetime: String,
    close: String,
}

impl Currencies {
    async fn fetch_twelve_data(
        &self,
        base: &str,
        target: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<HashMap<String, f64>, reqwest::Error> {
        let base = base.to_uppercase();
        let target = target.to_uppercase();

        if base == target {
            let mut rates = HashMap::new();
            rates.insert(start_date.to_string(), 1.0);
            return Ok(rates);
        }

        // we fetch the USD first because all currency _should_ have a USD pair.
        let usd_base = self
            .fetch_twelve_pair(&format!("USD/{}", base), start_date, end_date)
            .await?;

        let usd_target = self
            .fetch_twelve_pair(&format!("USD/{}", target), start_date, end_date)
            .await?;

        let mut rates = HashMap::new();

        for (date, base_rate) in usd_base {
            if let Some(target_rate) = usd_target.get(&date) {
                // base => usd => target
                let rate = (1.0 / base_rate) * target_rate;

                rates.insert(date, rate);
            }
        }

        Ok(rates)
    }

    async fn fetch_twelve_pair(
        &self,
        symbol: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<HashMap<String, f64>, reqwest::Error> {
        let url = format!(
            "https://api.twelvedata.com/time_series\
            ?symbol={}\
            &interval=1day\
            &start_date={}\
            &end_date={}\
            &apikey={}",
            symbol, start_date, end_date, "1dd95d513a1d4cf4907a7c209d8ee61b"
        );

        let response: TwelveDataResponse =
            reqwest::get(&url).await?.json().await?;

        let mut rates = HashMap::new();

        for item in response.values {
            if let Ok(rate) = item.close.parse::<f64>() {
                rates.insert(item.datetime, rate);
            }
        }

        Ok(rates)
    }
}
