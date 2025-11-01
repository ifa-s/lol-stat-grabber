use riven::consts::PlatformRoute;
use riven::RiotApi;
use std::env;
mod account;
use account::{init_account};
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api = env::var("API").expect("API must be set in .env file");
    println!("API={}", api);
    let riot_api = RiotApi::new(api);

    let platform = PlatformRoute::NA1;

    let acc = riot_api
        .account_v1()
        .get_by_riot_id(platform.to_regional(), "CARTPUSHER12", "KBD")
        .await
        .expect("Get summoner failed.")
        .expect("There is no summoner with that name.");

    println!("{}", acc.game_name.unwrap_or_default());
    let puid = &acc.puuid;

    let mut na = init_account("CARTPUSHER12".to_string(), "KBD".to_string(), puid.to_string());
    na.fill_mastery(riot_api, platform).await;
    na.print_mastery();

}
