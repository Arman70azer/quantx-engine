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

pub trait OhlcVecExt {
    fn print_summary(&self);
    fn filter_last_days(&self, count: usize) -> Vec<OhlcData>;
}
impl OhlcVecExt for Vec<OhlcData> {
    fn print_summary(&self) {
        if self.is_empty() {
            println!("No OHLC data available.");
            return;
        }
        let first = &self[0];
        let last = &self[self.len() - 1];
        println!("OHLC Summary:");
        println!("  Start: {} - Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {}",
                 first.timestamp, first.open, first.high, first.low, first.close, first.volume);
        println!("  End: {} - Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {}",
                 last.timestamp, last.open, last.high, last.low, last.close, last.volume);
    }

    /// Filtre les données OHLC pour ne garder que les N derniers éléments
    fn filter_last_days(&self, count: usize) -> Vec<OhlcData> {
        if self.len() <= count {
            return self.clone();
        }
        
        self.clone().into_iter()
        .rev()
        .take(count)
        .collect::<Vec<_>>()  // Here the iterator yields &OhlcData but you want OhlcData
        .into_iter()
        .rev()
        .collect()

    }

}
/// Récupère les données OHLC d'une action pour une période donnée
pub async fn get_ohlc_data(
    symbol: &str,
    interval: &str,  // "1d", "1h", "1wk", etc.
    range: &str,     // "1d", "1mo", "3mo", "1y", "ytd", "max", etc.
) -> Result<Vec<OhlcData>, Box<dyn Error>> {
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

    Ok(ohlc_data)
}

/// Récupère les données OHLC avec des dates personnalisées
pub async fn get_ohlc_data_range(
    symbol: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<OhlcData>, Box<dyn Error>> {
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

    Ok(ohlc_data)
}

/// Récupère la dernière cotation disponible
pub async fn get_latest_quote(symbol: &str) -> Result<OhlcData, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes(symbol, "1d").await?;
    let quote = response.last_quote()?;
    
    Ok(OhlcData {
        timestamp: quote.timestamp,
        open: quote.open,
        high: quote.high,
        low: quote.low,
        close: quote.close,
        volume: quote.volume,
    })
}

/// Recherche un ticker par nom de société
pub async fn search_ticker(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.search_ticker(query).await?;
    
    let symbols: Vec<String> = response.quotes
        .into_iter()
        .map(|quote| quote.symbol)
        .collect();
    
    Ok(symbols)
}

/// Calcule les statistiques de base pour les données OHLC
pub fn calculate_basic_stats(data: &[OhlcData]) -> Option<BasicStats> {
    if data.is_empty() {
        return None;
    }

    let prices: Vec<f64> = data.iter().map(|d| d.close).collect();
    let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
    
    let total_volume: u64 = data.iter().map(|d| d.volume).sum();
    let avg_volume = total_volume / data.len() as u64;

    Some(BasicStats {
        min_price,
        max_price,
        avg_price,
        total_volume,
        avg_volume,
        data_points: data.len(),
    })
}

#[derive(Debug)]
pub struct BasicStats {
    pub min_price: f64,
    pub max_price: f64,
    pub avg_price: f64,
    pub total_volume: u64,
    pub avg_volume: u64,
    pub data_points: usize,
}

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

/// Lance l'analyse technique pour un symbole donné
pub async fn run_technical_analysis(symbol: &str, interval: &str, range: &str) -> Result<(), Box<dyn Error>> {
    println!("🔍 Analyse technique pour: {}", symbol);
    println!("{}", "=".repeat(50));
    
    // Récupérer les données du dernier mois avec intervalle journalier
    let data = get_ohlc_data(symbol, interval, range).await?;
    
    if data.is_empty() {
        println!("❌ Aucune donnée trouvée pour {}", symbol);
        return Ok(());
    }

    // Afficher les 30 derniers jours
    let last_30_days = data.filter_last_days(30);
    println!("\n📈 Données OHLC pour {} (derniers {} éléments):", symbol, last_30_days.len());
    let _ =&last_30_days.print_summary();
    
    // Récupérer la dernière cotation
    match get_latest_quote(symbol).await {
        Ok(latest) => {
            println!("\n💰 Dernière cotation en temps réel:");
            println!("  Prix: {:.2}", latest.close);
            println!("  Ouverture: {:.2}", latest.open);
            println!("  Plus haut: {:.2}", latest.high);
            println!("  Plus bas: {:.2}", latest.low);
            println!("  Volume: {}", latest.volume);
        }
        Err(e) => {
            println!("\n⚠️ Impossible de récupérer la dernière cotation: {}", e);
        }
    }
    
    // Calculs de variation
    if last_30_days.len() >= 2 {
        let latest_close = last_30_days.last().unwrap().close;
        let previous_close = last_30_days[last_30_days.len() - 2].close;
        let change = latest_close - previous_close;
        let change_percent = (change / previous_close) * 100.0;
        
        let trend_emoji = if change > 0.0 { "📈" } else if change < 0.0 { "📉" } else { "➡️" };
        println!("{} Variation journalière: {:.2} ({:.2}%)", trend_emoji, change, change_percent);
    }

    // Statistiques de base
    if let Some(stats) = calculate_basic_stats(&last_30_days) {
        stats.print_stats();
    }

    // Analyse d'une période spécifique (juillet 2024)
    let start_date = DateTime::parse_from_rfc3339("2024-07-01T00:00:00Z")?.with_timezone(&Utc);
    let end_date = DateTime::parse_from_rfc3339("2024-07-31T23:59:59Z")?.with_timezone(&Utc);
    
    match get_ohlc_data_range(symbol, start_date, end_date).await {
        Ok(july_data) => {
            if !july_data.is_empty() {
                println!("\n\n📅 Données OHLC pour {} (Juillet 2024):", symbol);
                let _ =&july_data.print_summary();
                
                if let Some(july_stats) = calculate_basic_stats(&july_data) {
                    july_stats.print_stats();
                }
            } else {
                println!("\n\n⚠️  Aucune donnée trouvée pour {} en juillet 2024", symbol);
            }
        }
        Err(e) => {
            println!("\n⚠️  Impossible de récupérer les données de juillet 2024: {}", e);
        }
    }

    // Recherche de tickers similaires
    match search_ticker(symbol).await {
        Ok(tickers) => {
            if !tickers.is_empty() {
                println!("\n🔍 Tickers trouvés pour '{}':", symbol);
                for (i, ticker) in tickers.iter().take(5).enumerate() {
                    println!("  {}. {}", i + 1, ticker);
                }
                if tickers.len() > 5 {
                    println!("  ... et {} autres", tickers.len() - 5);
                }
            }
        }
        Err(e) => {
            println!("\n⚠️  Impossible de rechercher des tickers similaires: {}", e);
        }
    }

    println!("\n✅ Analyse terminée pour {}", symbol);
    Ok(())
}