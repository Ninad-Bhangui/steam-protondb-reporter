use std::env;
pub struct Config {
    pub api_key: String,
    pub export_path: String,
    pub steamid: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Config {
            api_key: env::var("STEAM_API_KEY").or_else(|_| Err("STEAM_API_KEY not found in environment"))?,
            export_path: env::var("EXPORT_PATH").or_else(|_| Err("EXPORT_PATH not found in environment"))?,
            steamid: args[1].clone(),
        })
    }
}
