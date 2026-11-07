use std::collections::HashMap;

use chrono::{
    Duration,
    NaiveDate,
};
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
struct CurrenciesToken {
    wise: String,
    twelve: String,
}

#[derive(Debug, Clone, Default)]
pub struct Currencies {
    cache: HashMap<String, HashMap<String, f64>>,
    token: CurrenciesToken,
}

impl Currencies {
    pub fn new(wise_token: String, twelve_token: String) -> Self {
        let mut curr = Self::default();
        curr.token.wise = wise_token;
        curr.token.twelve = twelve_token;
        curr
    }

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
        // direct pair, no USD triangulation
        if let Ok(mut wise_rate) =
            self.fetch_wise(base, target, start_date, end_date).await
            && !wise_rate.is_empty()
        {
            Self::fill_to_end_date(&mut wise_rate, end_date);
            return Ok((wise_rate, "Very High".to_string()));
        }

        // very accurate, but relies on FX market data
        if let Ok(mut twelve_rate) =
            self.fetch_twelve_data(base, target, start_date, end_date).await
        {
            Self::fill_to_end_date(&mut twelve_rate, end_date);
            return Ok((twelve_rate, "High".to_string()));
        }

        // works everytime, not accurate.
        if let Ok(mut frank_rate) =
            self.fetch_frankfurter(base, target, start_date, end_date).await
        {
            Self::fill_to_end_date(&mut frank_rate, end_date);
            return Ok((frank_rate, "Average".to_string()));
        }

        Ok((HashMap::new(), "Unknown".to_string()))
    }
}

// wise
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
struct WiseRate {
    rate: f64,
    time: String,
}

impl Currencies {
    async fn fetch_wise(
        &mut self,
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

        let key = format!(
            "wise-{}",
            Self::cache_key(&base, &target, start_date, end_date)
        );

        if let Some(rates) = self.cache.get(&key) {
            return Ok(rates.clone());
        }

        let url = format!(
            "https://api.wise.com/v1/rates?source={}&target={}&from={}&to={}&group=day",
            base, target, start_date, end_date
        );

        let resp: Vec<WiseRate> = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.token.wise)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let mut rates = HashMap::new();
        for r in resp {
            // keep only the YYYY-MM-DD part so keys match the other providers
            if let Some(date) = r.time.get(..10) {
                rates.insert(date.to_string(), r.rate);
            }
        }

        self.cache.insert(key, rates.clone());

        Ok(rates)
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

        // USD/X = how many X per 1 USD
        // USD base   -> rate = USD/target
        // USD target -> rate = 1 / USD/base
        // otherwise  -> rate = (1 / USD/base) * USD/target
        let usd_base = if base == "USD" {
            None
        } else {
            Some(
                self.fetch_twelve_pair(
                    &format!("USD/{}", base),
                    start_date,
                    end_date,
                )
                .await?,
            )
        };

        let usd_target = if target == "USD" {
            None
        } else {
            Some(
                self.fetch_twelve_pair(
                    &format!("USD/{}", target),
                    start_date,
                    end_date,
                )
                .await?,
            )
        };

        let mut rates = HashMap::new();

        match (usd_base, usd_target) {
            (None, Some(t)) => {
                rates = t;
            },
            (Some(b), None) => {
                for (date, base_rate) in b {
                    rates.insert(date, 1.0 / base_rate);
                }
            },
            (Some(b), Some(t)) => {
                for (date, base_rate) in b {
                    if let Some(target_rate) = t.get(&date) {
                        rates.insert(date, (1.0 / base_rate) * target_rate);
                    }
                }
            },
            (None, None) => unreachable!(), // base == target handled above
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
            symbol, start_date, end_date, self.token.twelve
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

impl Currencies {
    // HACK: this is to fill the missing dates until the end_date
    fn fill_to_end_date(rates: &mut HashMap<String, f64>, end_date: &str) {
        const FMT: &str = "%Y-%m-%d";

        let Ok(end) = NaiveDate::parse_from_str(end_date, FMT) else {
            return;
        };

        // find the latest date we actually have data for
        let Some((last_date, last_rate)) = rates
            .iter()
            .filter_map(|(d, r)| {
                NaiveDate::parse_from_str(d, FMT).ok().map(|nd| (nd, *r))
            })
            .max_by_key(|(nd, _)| *nd)
        else {
            return;
        };

        let mut day = last_date + Duration::days(1);
        while day <= end {
            rates.insert(day.format(FMT).to_string(), last_rate);
            day += Duration::days(1);
        }
    }
}
