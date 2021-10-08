use core::fmt;
use serde::Deserialize;
use std::error::Error;
#[derive(Deserialize, Debug)]
pub struct ProtonDbResponse {
    pub confidence: Option<String>,
    pub score: Option<f32>,
    pub tier: Option<String>,
    pub total: Option<f32>,
    pub trendingTier: Option<String>,
    pub bestReportedTier: Option<String>,
}
#[derive(Debug)]
struct ProtonDbError;
impl Error for ProtonDbError {}

impl fmt::Display for ProtonDbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

pub async fn get_protondb_score(
    appid: u32,
) -> Result<ProtonDbResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.protondb.com/api/v1/reports/summaries/{steamid}.json",
        steamid = appid
    );
    let http_resp = reqwest::get(url).await?;
    match http_resp.status() {
        reqwest::StatusCode::OK => {
            let resp: ProtonDbResponse = http_resp.json().await?;
            Ok(resp)
        }
        _ => {
            // Err(Box::new(ProtonDbError))
            Ok(ProtonDbResponse {
                confidence: None,
                score: None,
                tier: None,
                total: None,
                trendingTier: None,
                bestReportedTier: None,
            })
        }
    }
}
