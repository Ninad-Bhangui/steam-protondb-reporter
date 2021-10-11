use std::{fs, path::PathBuf};
pub mod config;
mod exporter;
mod protondb;
mod schemas;
mod steam;
use config::Config;
use std::time::Instant;

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let steam_client = steam::SteamClient::new("https://api.steampowered.com").unwrap();
    let (steamapps, ownedgames) = tokio::join!(
        steam_client.get_steam_app_list(),
        steam_client.get_steam_owned_games_list(&config.steamid, &config.api_key)
    );
    let steam_games = steamapps?.applist.apps;
    let owned_games = ownedgames?.response.games;
    let protondb_client = protondb::ProtonDbClient::new("https://www.protondb.com/api/v1").unwrap();
    let owned_app_ids: Vec<u32> = owned_games.iter().map(|x| x.appid).collect();
    let protondb_details = protondb_client
        .bulk_get_protondb_score(&owned_app_ids[..])
        .await?;
    let csv_rows =
        merge_details(&owned_games[..], &steam_games[..], &protondb_details[..]).unwrap();
    let csv_data = exporter::write_to_csv(csv_rows);
    let mut export_path = PathBuf::new();
    export_path.push(config.export_path);
    export_path.push("export.csv");
    fs::write(export_path, csv_data.unwrap()).expect("Unable to write to file");
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}

fn merge_details(
    ownedgames: &[schemas::GameDetails],
    apps: &[schemas::SteamApp],
    protondb_details: &[schemas::ProtonDbDetails],
) -> Result<Vec<schemas::CsvRow>, Box<dyn std::error::Error>> {
    let mut csv_rows: Vec<schemas::CsvRow> = Vec::new();
    for game in ownedgames {
        let name = apps
            .iter()
            .find(|x| x.appid == game.appid)
            .unwrap_or(&schemas::SteamApp {
                appid: 1,
                name: format!(""),
            })
            .name
            .clone();
        let new_proton_details = protondb_details.to_vec().clone();
        let protondb_detail = new_proton_details
            .into_iter()
            .find(|x| x.appid == game.appid)
            .unwrap();
        let csv_row = schemas::CsvRow {
            appid: game.appid,
            name,
            confidence: protondb_detail.proton_db_response.confidence,
            score: protondb_detail.proton_db_response.score,
            tier: protondb_detail.proton_db_response.tier,
            total: protondb_detail.proton_db_response.total,
            trending_tier: protondb_detail.proton_db_response.trending_tier,
            best_reported_tier: protondb_detail.proton_db_response.best_reported_tier,
        };
        csv_rows.push(csv_row);
    }
    Ok(csv_rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemas::{CsvRow, GameDetails, ProtonDbDetails, ProtonDbResponse, SteamApp};
    #[test]
    fn test_merge_details() {
        let test_owned_games = vec![
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
        ];
        let test_steam_apps = vec![
            SteamApp {
                appid: 998,
                name: String::from("Test Game 1"),
            },
            SteamApp {
                appid: 999,
                name: String::from("Test Game 2"),
            },
        ];
        let test_protondb_details = vec![
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
        let expected_result = vec![
            CsvRow {
                appid: 998,
                name: String::from("Test Game 1"),
                confidence: Some(String::from("good")),
                score: Some(0.53),
                tier: Some(String::from("gold")),
                total: Some(20.0),
                trending_tier: Some(format!("gold")),
                best_reported_tier: Some(String::from("platinum")),
            },
            CsvRow {
                appid: 999,
                name: String::from("Test Game 2"),
                confidence: Some(String::from("good")),
                score: Some(0.53),
                tier: Some(String::from("gold")),
                total: Some(20.0),
                trending_tier: Some(format!("gold")),
                best_reported_tier: Some(String::from("platinum")),
            },
        ];

        let result = merge_details(
            &test_owned_games[..],
            &test_steam_apps[..],
            &test_protondb_details[..],
        )
        .unwrap();
        assert_eq!(expected_result, result);
    }
}
