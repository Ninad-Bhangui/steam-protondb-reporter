# steam-protondb-reporter
(WIP) Currently exports a csv of protondb scores of all games owned by a steam account.

With valve launching the steam deck, Linux gaming might soon be a reality. However, it's not easy for someone who owns a lot of games on steam to get an idea about how many games owned would be reasonably compatible with proton. Steam does not show a protondb score as of now.
This tool generates a csv report with steam game id, name, and protondb metrics taken from https://www.protondb.com/.

This could have been much easier in python/node. But my intention was to practice rust on something I needed. Also wanted to try async rust via tokio. This is my first attempt at coding in rust (beyond following tutorials and easy exercism exercises)
## Steps to generate Report

1. Get steam api Key
2. Find your steam id
3. export api Key as environment variable
4. steam id is a command line argument (You can generate reports for your id and any other public id as well)
### Create steam api Key
1. Go to: https://steamcommunity.com/dev/apikey
2. Get your steam api Key.

### Find your steam id
1. Click on any profile in steam.
2. The URL would be something like: https://steamcommunity.com/profiles/{account_id}/

### Compile and run in rust
Set the following environment variables:
1. STEAM_API_KEY
2. EXPORT_PATH: the path where the csv will be exported to

steamid should be passed as a command line argument. (I have chosen to keep it so because it is possible to generate exports of any public steam account using the same api key)
```bash
cargo run {steamid}
```

#### TODO
1. Dockerfile setup
3. Refactor and test cases
5. create cli tool
