use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};
use std::error::Error;

#[allow(dead_code)]
pub struct TemporalScope;

#[allow(dead_code)]
impl TemporalScope {
    pub const ONE_HOUR: &'static str = "1h";
    pub const FOUR_HOURS: &'static str = "4h";
    pub const SIX_HOURS: &'static str = "6h";
    pub const TWELVE_HOURS: &'static str = "12h";
    pub const ONE_DAY: &'static str = "1d";
    pub const ONE_WEEK: &'static str = "1wk";
    pub const ONE_MONTH: &'static str = "1mo";
    pub const THREE_MONTHS: &'static str = "3mo";
    pub const SIX_MONTHS: &'static str = "6mo";
    pub const ONE_YEAR: &'static str = "1y";
    pub const YEAR_TO_DATE: &'static str = "ytd";
    pub const TEN_YEAR: &'static str = "10y";
}


#[derive(Debug, Clone)]
pub struct OhlcData {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[allow(dead_code)]
pub struct  OhlcDataExt {
    pub ohlc_data: Vec<OhlcData>,
}

#[allow(dead_code)]
impl OhlcDataExt  {
    /// Récupère les données OHLC d'une action pour une période donnée
    pub async fn get_ohlc_data(
        symbol: &str,
        interval: &str,  // "1d", "1h", "1wk", etc.
        range: &str,     // "1d", "1mo", "3mo", "1y", "ytd", "max", etc.
    ) -> Result<OhlcDataExt, Box<dyn Error>> {
        let provider = yahoo::YahooConnector::new()?;
        let response = provider.get_quote_range(symbol, interval, range).await?;
        let quotes = response.quotes()?;
        
        let ohlc_data: Vec<OhlcData> = quotes
            .into_iter()
            .map(|quote| OhlcData {
                timestamp: quote.timestamp,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            })
            .collect();

        Ok(OhlcDataExt { ohlc_data: ohlc_data })
    }

    pub fn print_summary(&self) {
        if self.ohlc_data.is_empty() {
            println!("No OHLC data available.");
            return;
        }
        let first = &self.ohlc_data[0];
        let last = &self.ohlc_data[self.ohlc_data.len() - 1];
        println!("OHLC Summary:");
        println!("  Start: {} - Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {}",
                 first.timestamp, first.open, first.high, first.low, first.close, first.volume);
        println!("  End: {} - Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {}",
                 last.timestamp, last.open, last.high, last.low, last.close, last.volume);
    }

    /// Filtre les données OHLC pour ne garder que les N derniers éléments
    pub fn filter_last_days(&self, count: usize) -> Vec<OhlcData> {
        if self.ohlc_data.len() <= count {
            return self.ohlc_data.clone();
        }
        
        self.ohlc_data.clone().into_iter()
        .rev()
        .take(count)
        .collect::<Vec<_>>()  // Here the iterator yields &OhlcData but you want OhlcData
        .into_iter()
        .rev()
        .collect()

    }

    /// Calcule les statistiques de base pour les données OHLC
    pub fn calculate_basic_stats(&self) -> Option<BasicStats> {
        if self.ohlc_data.is_empty() {
            return None;
        }

        let prices: Vec<f64> = self.ohlc_data.iter().map(|d| d.close).collect();
        let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
        
        let total_volume: u64 = self.ohlc_data.iter().map(|d| d.volume).sum();
        let avg_volume = total_volume / self.ohlc_data.len() as u64;

        Some(BasicStats {
            min_price,
            max_price,
            avg_price,
            total_volume,
            avg_volume,
            data_points: self.ohlc_data.len(),
        })
    }

    /// Récupère les données OHLC avec des dates personnalisées
    pub async fn get_ohlc_data_range(
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<OhlcDataExt, Box<dyn Error>> {
        let provider = yahoo::YahooConnector::new()?;
        
        // Convertir DateTime<Utc> en OffsetDateTime pour l'API
        let start_offset = yahoo::time::OffsetDateTime::from_unix_timestamp(start.timestamp())?;
        let end_offset = yahoo::time::OffsetDateTime::from_unix_timestamp(end.timestamp())?;
        
        let response = provider.get_quote_history(symbol, start_offset, end_offset).await?;
        let quotes = response.quotes()?;
        
        let ohlc_data: Vec<OhlcData> = quotes
            .into_iter()
            .map(|quote| OhlcData {
                timestamp: quote.timestamp,
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            })
            .collect();

        Ok(OhlcDataExt { ohlc_data })
    }

    /// Récupère la dernière cotation disponible
    #[allow(dead_code)]
    pub async fn get_actual_ohlc(&self) -> Result<OhlcData, Box<dyn Error>> {
        if self.ohlc_data.is_empty() {
            return Err("No OHLC data available".into());
        }
        
        // Retourne le dernier élément de la liste
        Ok(self.ohlc_data.last().cloned().unwrap())
    }


}


/// Recherche un ticker par nom de société
#[allow(dead_code)]
pub async fn search_ticker(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.search_ticker(query).await?;
    
    let symbols: Vec<String> = response.quotes
        .into_iter()
        .map(|quote| quote.symbol)
        .collect();
    
    Ok(symbols)
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BasicStats {
    pub min_price: f64,
    pub max_price: f64,
    pub avg_price: f64,
    pub total_volume: u64,
    pub avg_volume: u64,
    pub data_points: usize,
}

#[allow(dead_code)]
impl BasicStats {
    pub fn print_stats(&self) {
        println!("\n📊 Statistiques de base:");
        println!("  Prix minimum: {:.2}", self.min_price);
        println!("  Prix maximum: {:.2}", self.max_price);
        println!("  Prix moyen: {:.2}", self.avg_price);
        println!("  Volume total: {}", self.total_volume);
        println!("  Volume moyen: {}", self.avg_volume);
        println!("  Nombre de points de données: {}", self.data_points);
    }
}
