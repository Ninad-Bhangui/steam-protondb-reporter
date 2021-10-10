use super::schemas;
pub struct SteamClient {
    base_url: String,
    client: reqwest::Client,
}
impl SteamClient {
    pub fn new(base_url: &str) -> reqwest::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self {
            base_url: String::from(base_url),
            client,
        })
    }
    pub async fn get_steam_app_list(
        &self,
    ) -> Result<schemas::AppListResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{api_base_url}/ISteamApps/GetAppList/v2/",
            api_base_url = self.base_url
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
            api_base_url = self.base_url
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

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::schemas::{
        AppList, AppListResponse, GameDetails, OwnedGamesInternalResponse, OwnedGamesResponse,
        SteamApp,
    };
    use mockito::mock;

    use super::*;
    #[tokio::test]
    async fn test_get_steam_app_list_available() {
        let steam_client = SteamClient::new(&mockito::server_url()).unwrap();
        let _mock = mock("GET", "/ISteamApps/GetAppList/v2/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"applist":{"apps":[{"appid":999,"name":"Test Game"}]}}"#)
            .create();

        let response = steam_client.get_steam_app_list().await;
        let expected_result = AppListResponse {
            applist: AppList {
                apps: vec![SteamApp {
                    appid: 999,
                    name: String::from("Test Game"),
                }],
            },
        };
        assert_eq!(expected_result, response.unwrap());
    }
    #[tokio::test]
    async fn test_get_get_steam_owned_games_list_available() {
        let steam_client = SteamClient::new(&mockito::server_url()).unwrap();
        let test_api_key = "TEST_KEY";
        let test_steamid = "999";
        let test_url_path = format!(
            "/IPlayerService/GetOwnedGames/v0001/?key={api_key}&steamid={steamid}&format=json",
            api_key = test_api_key,
            steamid = test_steamid
        );
        let _mock = mock("GET", test_url_path.as_str()).with_status(200).with_header("content-type", "application/json").with_body(r#"{"response":{"game_count":2,"games":[{"appid":998,"playtime_forever":400,"playtime_windows_forever":300,"playtime_mac_forever":0,"playtime_linux_forever":100},{"appid":999,"playtime_forever":50,"playtime_windows_forever":50,"playtime_mac_forever":0,"playtime_linux_forever":0}]}}"#).create();

        let response = steam_client
            .get_steam_owned_games_list(test_steamid, test_api_key)
            .await;
        let expected_result = OwnedGamesResponse {
            response: OwnedGamesInternalResponse {
                game_count: 2,
                games: vec![
                    GameDetails {
                        appid: 998,
                        playtime_forever: 400,
                        playtime_windows_forever: 300,
                        playtime_linux_forever: 100,
                        playtime_mac_forever: 0,
                    },
                    GameDetails {
                        appid: 999,
                        playtime_forever: 50,
                        playtime_windows_forever: 50,
                        playtime_linux_forever: 0,
                        playtime_mac_forever: 0,
                    },
                ],
            },
        };
        assert_eq!(expected_result, response.unwrap());
    }
}
