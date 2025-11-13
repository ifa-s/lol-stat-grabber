use riven::consts::{PlatformRoute, RegionalRoute};
use riven::RiotApi;
use std::env;
mod account;
use account::{init_account};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let api = env::var("API").expect("API must be set in .env file");
    let riot_api = RiotApi::new(&api);
    let platform = PlatformRoute::NA1;
    let region = RegionalRoute::AMERICAS;
    let acc = &riot_api
        .account_v1()
        .get_by_riot_id(platform.to_regional(), "stars alighted", "KBD")
        .await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    let puid = &acc.puuid;

    let mut na: account::Account = init_account(&api, platform, region, "CARTPUSHER12".to_string(), "KBD".to_string(), puid.to_string()).await;
    na.print_matches();

    //let config = bincode::config::standard();
    //account::get_largest_crit(na);
    let tree = sled::open("./testdb")?;
    //let str_acc = serde_json::to_string(&na).unwrap();
    //let ov = tree.insert(na.puuid, str_acc.as_bytes())?;
    let str_acc = tree.get(na.puuid)?.unwrap();
    let json_str = std::str::from_utf8(&str_acc).unwrap();
    let tempacc: account::Account = serde_json::from_str(&json_str).unwrap();
    account::get_largest_crit(tempacc);
    Ok(())
    //riven::endpoints::MatchV5::try_get_match_ids_by_puuid(platform, puid, )

    // TODO Create match struct (or use riven provided one), create a vector?

}
