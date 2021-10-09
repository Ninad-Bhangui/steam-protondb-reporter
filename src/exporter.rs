use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

use csv::Writer;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CsvRow {
    pub appid: u32,
    pub name: String,
    pub confidence: Option<String>,
    pub score: Option<f32>,
    pub tier: Option<String>,
    pub total: Option<f32>,
    pub trending_tier: Option<String>,
    pub best_reported_tier: Option<String>,
}

pub fn write_to_csv(rows: Vec<CsvRow>) -> Result<String, Box<dyn Error>> {
    let mut wtr = Writer::from_writer(vec![]);
    for row in rows {
        wtr.serialize(row)?;
    }
    return Ok(String::from_utf8(wtr.into_inner()?)?);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_normal() {
        let mut rows = Vec::new();
        rows.push(CsvRow{
            appid: 1,
            name: format!("test game"),
            confidence: Some(format!("good")),
            score: Some(0.5),
            tier: Some(format!("gold")),
            total: Some(20.0),
            trending_tier: Some(format!("gold")),
            best_reported_tier: Some(format!("platinum"))
        });
        let expected_output = "appid,name,confidence,score,tier,total,trendingTier,bestReportedTier\n1,test game,good,0.5,gold,20.0,gold,platinum\n";

        assert_eq!(expected_output, write_to_csv(rows).unwrap())
    }
    #[test]
    fn write_missing_fields() {
        let mut rows = Vec::new();
        rows.push(CsvRow{
            appid: 1,
            name: format!(""),
            confidence: Some(format!("good")),
            score: None,
            tier: Some(format!("gold")),
            total: Some(20.0),
            trending_tier: None,
            best_reported_tier: Some(format!("platinum"))
        });
        let expected_output = "appid,name,confidence,score,tier,total,trendingTier,bestReportedTier\n1,,good,,gold,20.0,,platinum\n";

        assert_eq!(expected_output, write_to_csv(rows).unwrap())
    }
}