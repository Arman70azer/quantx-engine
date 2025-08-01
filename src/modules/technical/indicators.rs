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

impl OhlcData {
    /// Récupère la date formatée pour l'affichage
    pub fn formatted_date(&self) -> String {
        let dt = DateTime::<Utc>::from_timestamp(self.timestamp, 0)
            .unwrap_or_default();
        dt.format("%Y-%m-%d").to_string()
    }
}

#[derive(Debug)]
pub struct OhlcDataExt {
    pub data: Vec<OhlcData>,
    pub symbol: String,
}

#[allow(dead_code)]
impl OhlcDataExt {
    /// Constructeur principal - récupère les données OHLC
    pub async fn fetch(
        symbol: impl Into<String>,
        interval: &str,
        range: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let symbol = symbol.into();
        let provider = yahoo::YahooConnector::new()?;
        let response = provider.get_quote_range(&symbol, interval, range).await?;
        let quotes = response.quotes()?;
        
        let data = quotes
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

        Ok(Self { data, symbol })
    }

    /// Constructeur avec dates personnalisées
    
    pub async fn fetch_range(
        symbol: impl Into<String>,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Self, Box<dyn Error>> {
        let symbol = symbol.into();
        let provider = yahoo::YahooConnector::new()?;
        
        let start_offset = yahoo::time::OffsetDateTime::from_unix_timestamp(start.timestamp())?;
        let end_offset = yahoo::time::OffsetDateTime::from_unix_timestamp(end.timestamp())?;
        
        let response = provider.get_quote_history(&symbol, start_offset, end_offset).await?;
        let quotes = response.quotes()?;
        
        let data = quotes
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

        Ok(Self { data, symbol })
    }

    /// Vérifie si les données sont vides
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Nombre de points de données
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Récupère la première valeur
    pub fn first(&self) -> Option<&OhlcData> {
        self.data.first()
    }

    /// Récupère la dernière valeur
    pub fn last(&self) -> Option<&OhlcData> {
        self.data.last()
    }

    /// Affiche un résumé des données
    pub fn print_summary(&self) {
        if self.is_empty() {
            println!("❌ Aucune donnée OHLC disponible pour {}.", self.symbol);
            return;
        }

        let first = self.first().unwrap();
        let last = self.last().unwrap();
        
        println!("\n📈 Résumé OHLC pour {} ({} points):", self.symbol, self.len());
        println!("  🗓️  Début ({}): O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}", 
                 first.formatted_date(), first.open, first.high, first.low, first.close, first.volume);
        println!("  🗓️  Fin   ({}): O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}", 
                 last.formatted_date(), last.open, last.high, last.low, last.close, last.volume);
    }

    /// Filtre les N derniers éléments
    pub fn last_n(&self, count: usize) -> Vec<&OhlcData> {
        if count >= self.len() {
            return self.data.iter().collect();
        }
        
        self.data.iter().rev().take(count).rev().collect()
    }

    /// Calcule et affiche les statistiques de base
    pub fn analyze(&self) -> Option<BasicStats> {
        if self.is_empty() {
            println!("❌ Impossible de calculer les statistiques: aucune donnée disponible.");
            return None;
        }

        let prices: Vec<f64> = self.data.iter().map(|d| d.close).collect();
        let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
        
        let total_volume: u64 = self.data.iter().map(|d| d.volume).sum();
        let avg_volume = total_volume / self.len() as u64;

        let stats = BasicStats {
            symbol: self.symbol.clone(),
            min_price,
            max_price,
            avg_price,
            total_volume,
            avg_volume,
            data_points: self.len(),
        };

        stats.print();
        Some(stats)
    }

    /// Calcule le rendement total sur la période
    pub fn total_return(&self) -> Option<f64> {
        if self.len() < 2 {
            return None;
        }
        
        let first_price = self.first()?.close;
        let last_price = self.last()?.close;
        
        Some((last_price - first_price) / first_price * 100.0)
    }

    /// Affiche un rapport complet
    pub fn report(&self) {
        self.print_summary();
        
        if let Some(_stats) = self.analyze() {
            if let Some(return_pct) = self.total_return() {
                let trend = if return_pct > 0.0 { "📈" } else { "📉" };
                println!("  💰 Rendement total: {}{:.2}%", trend, return_pct);
            }
        }
    }
}

/// Implémentation Iterator pour OhlcDataExt
impl IntoIterator for OhlcDataExt {
    type Item = OhlcData;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Recherche un ticker par nom de société
#[allow(dead_code)]
pub async fn search_ticker(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.search_ticker(query).await?;
    
    Ok(response.quotes.into_iter().map(|quote| quote.symbol).collect())
}

#[derive(Debug, Clone)]
pub struct BasicStats {
    pub symbol: String,
    pub min_price: f64,
    pub max_price: f64,
    pub avg_price: f64,
    pub total_volume: u64,
    pub avg_volume: u64,
    pub data_points: usize,
}

impl BasicStats {
    pub fn print(&self) {
        println!("\n📊 Statistiques pour {}:", self.symbol);
        println!("  💵 Prix: min={:.2} | max={:.2} | moyenne={:.2}", 
                 self.min_price, self.max_price, self.avg_price);
        println!("  📦 Volume: total={} | moyenne={}", 
                 self.total_volume, self.avg_volume);
        println!("  📈 Points de données: {}", self.data_points);
    }
}