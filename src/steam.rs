use super::schemas;
static API_BASE_URL: &str = "https://api.steampowered.com";
pub struct SteamClient {
    client: reqwest::Client,
}
impl SteamClient {
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self { client })
    }
    pub async fn get_steam_app_list(
        &self,
    ) -> Result<schemas::AppListResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{api_base_url}/ISteamApps/GetAppList/v2/",
            api_base_url = API_BASE_URL
        );
        Ok(self.client.get(url).send().await?.json().await?)
    }
    pub async fn get_steam_owned_games_list(
        &self,
        steamid: &str,
        api_key: &str,
    ) -> Result<schemas::OwnedGamesResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{api_base_url}/IPlayerService/GetOwnedGames/v0001/",
            api_base_url = API_BASE_URL
        );
        Ok(self
            .client
            .get(url)
            .query(&[("key", api_key), ("steamid", steamid), ("format", "json")])
            .send()
            .await?
            .json()
            .await?)
    }
}
