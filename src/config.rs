use std::env;
pub struct Config {
    pub api_key: String,
    pub export_path: String,
    pub steamid: String,
}

impl Config {
    pub fn get_config() -> Config {
        let args: Vec<String> = env::args().collect();
        Config {
            api_key: env::var("STEAM_API_KEY").unwrap(),
            export_path: env::var("EXPORT_PATH").unwrap(),
            steamid: args[1].clone(),
        }
    }
}
