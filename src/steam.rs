use super::schemas;
pub async fn get_steam_app_list() -> Result<schemas::AppListResponse, Box<dyn std::error::Error>> {
    let http_resp = reqwest::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/").await?;
    let resp: schemas::AppListResponse = http_resp.json().await?;
    Ok(resp)
}

pub async fn get_steam_owned_games_list(
    steamid: &str,
    api_key: &str,
) -> Result<schemas::OwnedGamesResponse, Box<dyn std::error::Error>> {
    let format = "json";
    let client = reqwest::Client::new();
    let http_resp = client
        .get("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/")
        .query(&[("key", api_key), ("steamid", steamid), ("format", format)])
        .send()
        .await?;
    let resp: schemas::OwnedGamesResponse = http_resp.json().await?;
    Ok(resp)
}
