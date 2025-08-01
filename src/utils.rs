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
