mod modules;
use modules::technical::indicators::run_technical_analysis;

use crate::modules::technical::indicators::TemporalScope;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Analyser une action
    run_technical_analysis("AAPL", TemporalScope::ONE_DAY, TemporalScope::ONE_MONTH).await?;
    
    
    Ok(())
}