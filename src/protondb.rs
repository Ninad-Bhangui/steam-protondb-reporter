use super::schemas;

static API_BASE_URL: &str = "https://www.protondb.com/api/v1";
pub struct ProtonDbClient {
    client: reqwest::Client,
}
impl ProtonDbClient {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self { client })
    }

    pub async fn get_protondb_score(&self, appid: u32) -> Result<schemas::ProtonDbResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{api_base_url}/reports/summaries/{steamid}.json",
            api_base_url=API_BASE_URL,
            steamid = appid
        );
        let http_resp = self.client.get(url).send().await?;
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
}
