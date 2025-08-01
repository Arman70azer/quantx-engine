mod modules;

use crate::modules::technical::indicators::{TemporalScope, OhlcDataExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Analyser une action
    let symbol = "AAPL";
    let interval = TemporalScope::ONE_DAY;
    let range = TemporalScope::SIX_MONTHS;
    let ohlc_data = OhlcDataExt::get_ohlc_data(symbol, interval, range).await?;
    ohlc_data.print_summary();
    ohlc_data.calculate_basic_stats()
        .map(|stats| stats.print_stats())
        .unwrap_or_else(|| println!("Aucune donnée OHLC disponible pour l'action {}", symbol));
    
    
    Ok(())
}