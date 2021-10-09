use std::env;
use std::fs;
mod exporter;
mod protondb;
mod schemas;
mod steam;
use std::time::Instant;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let api_key = env::var("STEAM_API_KEY").unwrap();
    let args: Vec<String> = env::args().collect();
    let steam_client = steam::SteamClient::new().unwrap();
    let steamid = args[1].clone();
    let (steamapps, ownedgames) = tokio::join!(
        steam_client.get_steam_app_list(),
        steam_client.get_steam_owned_games_list(&steamid, &api_key)
    );
    let csv_rows = merge_details(&ownedgames?.response.games, &steamapps?.applist.apps).await?;
    let csv_data = exporter::write_to_csv(csv_rows);
    fs::write("export.csv", csv_data.unwrap()).expect("Unable to write to file");
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    Ok(())
}

async fn merge_details(
    ownedgames: &[schemas::GameDetails],
    apps: &[schemas::SteamApp],
) -> Result<Vec<schemas::CsvRow>, Box<dyn std::error::Error>> {
    let mut csv_rows: Vec<schemas::CsvRow> = Vec::new();
    let protondb_client = protondb::ProtonDbClient::new("https://www.protondb.com/api/v1").unwrap();
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
        let proton_details = protondb_client.get_protondb_score(game.appid).await?;
        let csv_row = schemas::CsvRow {
            appid: game.appid,
            name,
            confidence: proton_details.confidence,
            score: proton_details.score,
            tier: proton_details.tier,
            total: proton_details.total,
            trending_tier: proton_details.trending_tier,
            best_reported_tier: proton_details.best_reported_tier,
        };
        csv_rows.push(csv_row);
    }
    Ok(csv_rows)
}
