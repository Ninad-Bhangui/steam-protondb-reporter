use super::schemas;

pub async fn get_protondb_score(
    appid: u32,
) -> Result<schemas::ProtonDbResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.protondb.com/api/v1/reports/summaries/{steamid}.json",
        steamid = appid
    );
    let http_resp = reqwest::get(url).await?;
    match http_resp.status() {
        reqwest::StatusCode::OK => {
            let resp: schemas::ProtonDbResponse = http_resp.json().await?;
            Ok(resp)
        }
        _ => {
            Ok(schemas::ProtonDbResponse {
                confidence: None,
                score: None,
                tier: None,
                total: None,
                trending_tier: None,
                best_reported_tier: None,
            })
        }
    }
}
