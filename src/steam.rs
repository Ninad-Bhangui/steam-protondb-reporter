use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppListResponse {
    pub applist: AppList,
}
#[derive(Deserialize, Debug)]
pub struct AppList {
    pub apps: Vec<SteamApp>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SteamApp {
    pub appid: u32,
    pub name: String,
}
pub async fn get_steam_app_list() -> Result<AppListResponse, Box<dyn std::error::Error>> {
    let http_resp = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").await?;
    let resp: AppListResponse = http_resp.json().await?;
    Ok(resp)
}
#[derive(Deserialize, Debug)]
pub struct OwnedGamesResponse {
    pub response: OwnedGamesInternalResponse,
}
#[derive(Deserialize, Debug)]
pub struct OwnedGamesInternalResponse {
    pub game_count: u32,
    pub games: Vec<GameDetails>,
}
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct GameDetails {
    pub appid: u32,
    pub playtime_forever: u32,
    pub playtime_windows_forever: u32,
    pub playtime_mac_forever: u32,
    pub playtime_linux_forever: u32,
}
pub async fn get_steam_owned_games_list(
    steamid: &str,
    api_key: &str,
) -> Result<OwnedGamesResponse, Box<dyn std::error::Error>> {
    let format = "json";
    let client = reqwest::Client::new();
    let http_resp = client
        .get("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/")
        .query(&[("key", api_key), ("steamid", steamid), ("format", format)])
        .send()
        .await?;
    let resp: OwnedGamesResponse = http_resp.json().await?;
    Ok(resp)
}
