use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct AppListResponse {
    pub applist: AppList,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct AppList {
    pub apps: Vec<SteamApp>,
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SteamApp {
    pub appid: u32,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct OwnedGamesResponse {
    pub response: OwnedGamesInternalResponse,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct OwnedGamesInternalResponse {
    pub game_count: u32,
    pub games: Vec<GameDetails>,
}
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct GameDetails {
    pub appid: u32,
    pub playtime_forever: u32,
    pub playtime_windows_forever: u32,
    pub playtime_mac_forever: u32,
    pub playtime_linux_forever: u32,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtonDbResponse {
    pub confidence: Option<String>,
    pub score: Option<f32>,
    pub tier: Option<String>,
    pub total: Option<f32>,
    pub trending_tier: Option<String>,
    pub best_reported_tier: Option<String>,
}
impl ProtonDbResponse {
    pub fn create_empty_response() -> ProtonDbResponse {
        ProtonDbResponse {
            confidence: None,
            score: None,
            tier: None,
            total: None,
            trending_tier: None,
            best_reported_tier: None,
        }
    }
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProtonDbDetails {
    pub appid: u32,
    pub proton_db_response: ProtonDbResponse,
}
#[derive(Serialize, Deserialize, Debug,PartialEq, Clone)]
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
