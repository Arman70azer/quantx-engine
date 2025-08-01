mod modules;
mod utils;

use crate::modules::technical::indicators::OhlcDataExt;
use crate::utils::TemporalScope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Version simplifiée et plus lisible
    let ohlc = OhlcDataExt::fetch("AAPL", TemporalScope::ONE_DAY, TemporalScope::SIX_MONTHS).await?;
    
    // Rapport complet en une seule méthode
    ohlc.report();
    
    // Exemple d'utilisation des nouvelles méthodes
    println!("\n🔍 Analyse des 5 derniers jours:");
    for data in ohlc.last_n(5) {
        println!("  {} - Close: {:.2}", data.formatted_date(), data.close);
    }
    
    Ok(())
}