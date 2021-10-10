use super::schemas;
use futures::future::join_all;
#[cfg(test)]
use mockito;

pub struct ProtonDbClient {
    base_url: String,
    client: reqwest::Client,
}
impl ProtonDbClient {
    pub fn new(base_url: &str) -> reqwest::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self {
            base_url: String::from(base_url),
            client,
        })
    }

    pub async fn get_protondb_score(
        &self,
        appid: u32,
    ) -> Result<schemas::ProtonDbDetails, Box<dyn std::error::Error>> {
        let url = format!(
            "{api_base_url}/reports/summaries/{steamid}.json",
            api_base_url = self.base_url,
            steamid = appid
        );
        let http_resp = self.client.get(url).send().await?;
        match http_resp.status() {
            reqwest::StatusCode::OK => {
                let resp: schemas::ProtonDbResponse = http_resp.json().await?;
                Ok(schemas::ProtonDbDetails {
                    appid,
                    proton_db_response: resp,
                })
            }
            _ => Ok(schemas::ProtonDbDetails {
                appid,
                proton_db_response: schemas::ProtonDbResponse::create_empty_response(),
            }),
        }
    }
    pub async fn bulk_get_protondb_score(
        &self,
        appid_list: &[u32],
    ) -> Result<Vec<schemas::ProtonDbDetails>, Box<dyn std::error::Error>> {
        let futures = appid_list.iter().map(|x| self.get_protondb_score(*x));
        let results = join_all(futures).await;
        Ok(results.into_iter().map(|x| x.unwrap()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::schemas::{ProtonDbDetails, ProtonDbResponse};
    use mockito::mock;

    use super::*;
    #[tokio::test]
    async fn test_get_protondb_score_available() {
        let protondb_client = ProtonDbClient::new(&mockito::server_url()).unwrap();
        let _mock = mock("GET", "/reports/summaries/999.json").with_status(200).with_header("content-type", "application/json").with_body(r#"{"confidence":"good","score":0.53,"tier":"gold","total":20,"trendingTier":"gold","bestReportedTier":"platinum"}"#).create();

        let response = protondb_client.get_protondb_score(999).await;
        let expected_result = ProtonDbDetails {
            appid: 999,
            proton_db_response: ProtonDbResponse {
                confidence: Some(format!("good")),
                score: Some(0.53),
                tier: Some(format!("gold")),
                total: Some(20.0),
                trending_tier: Some(format!("gold")),
                best_reported_tier: Some(format!("platinum")),
            },
        };
        assert_eq!(expected_result, response.unwrap());
    }
    #[tokio::test]
    async fn test_bulk_get_protondb_scores_available() {
        let protondb_client = ProtonDbClient::new(&mockito::server_url()).unwrap();
        let _mock = mock("GET", "/reports/summaries/998.json").with_status(200).with_header("content-type", "application/json").with_body(r#"{"confidence":"good","score":0.53,"tier":"gold","total":20,"trendingTier":"gold","bestReportedTier":"platinum"}"#).create();
        let _mock_2 = mock("GET", "/reports/summaries/999.json").with_status(200).with_header("content-type", "application/json").with_body(r#"{"confidence":"good","score":0.53,"tier":"gold","total":20,"trendingTier":"gold","bestReportedTier":"platinum"}"#).create();

        let response = protondb_client.bulk_get_protondb_score(&[998, 999]).await;
        let expected_result = vec![
            ProtonDbDetails {
                appid: 998,
                proton_db_response: ProtonDbResponse {
                    confidence: Some(format!("good")),
                    score: Some(0.53),
                    tier: Some(format!("gold")),
                    total: Some(20.0),
                    trending_tier: Some(format!("gold")),
                    best_reported_tier: Some(format!("platinum")),
                },
            },
            ProtonDbDetails {
                appid: 999,
                proton_db_response: ProtonDbResponse {
                    confidence: Some(format!("good")),
                    score: Some(0.53),
                    tier: Some(format!("gold")),
                    total: Some(20.0),
                    trending_tier: Some(format!("gold")),
                    best_reported_tier: Some(format!("platinum")),
                },
            },
        ];
        assert_eq!(expected_result, response.unwrap());
    }
}
