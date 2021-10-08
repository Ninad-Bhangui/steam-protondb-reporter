use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

use csv::Writer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CsvRow {
    pub appid: u32,
    pub name: String,
    pub confidence: Option<String>,
    pub score: Option<f32>,
    pub tier: Option<String>,
    pub total: Option<f32>,
    pub trendingTier: Option<String>,
    pub bestReportedTier: Option<String>,
}

pub fn write_to_csv(rows: Vec<CsvRow>) -> Result<String, Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);
    for row in rows {
        wtr.serialize(row)?;
    }
    return Ok(String::from_utf8(wtr.into_inner()?)?);
}
