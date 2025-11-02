use std::collections::HashMap;
use riven::RiotApi;

pub struct Account {
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub mastery: HashMap<riven::consts::Champion, HashMap<String, i64>>,
    pub matches: HashMap<String, riven::models::match_v5::Match>
}

impl Account {
    async fn fill_mastery(&mut self, api: riven::RiotApi, platform: riven::consts::PlatformRoute) {
        let masteries = api
            .champion_mastery_v4()
            .get_all_champion_masteries_by_puuid(platform, &self.puuid).await.expect("Get masteries failed");
        for mastery in masteries.iter() {
            let mut mastery_info: HashMap<String, i64> = HashMap::new();
            mastery_info.insert(String::from("Last Played"), mastery.last_play_time);
            mastery_info.insert(String::from("Level"), i64::from(mastery.champion_level));
            mastery_info.insert(String::from("Points"), i64::from(mastery.champion_points));
            self.mastery.insert(mastery.champion_id, mastery_info);
        }
    }

    async fn fill_matches(&mut self, api: riven::RiotApi, region: riven::consts::RegionalRoute) {
        let matches = api.match_v5().get_match_ids_by_puuid(region, &self.puuid, None, None, None, None, None, None).await;
        for m in matches.iter() {
            for id in m {
                self.matches.insert(id.to_string(), api.match_v5().get_match(region, id).await.expect("Failed to find match").unwrap());
            }
            println!("");
        }
    }

    // Prints mastery from account, TODO add sort
    pub fn print_mastery(&mut self) {
        println!("{}#{}'s masteries: ", self.game_name, self.tag_line);
        for (id, map) in self.mastery.iter() {
            println!("{}: {} ({})", id.name().unwrap(), map["Points"], map["Level"]);
        }
    }

    pub fn print_matches(&mut self) {
        for (_id, m) in self.matches.iter() {
            for p in m.info.participants.clone() {
                if p.puuid == self.puuid {
                    println!("{}'s game: {}", self.game_name, p.champion_name);
                }
            }
        }
    }
}
pub async fn init_account(api: &str, platform: riven::consts::PlatformRoute, region: riven::consts::RegionalRoute, game_name: String, tag_line: String, puuid: String) -> Account {
    let mut acc = Account{game_name: game_name, tag_line: tag_line, puuid: puuid, mastery: HashMap::new(), matches: HashMap::new()};
    let riot_api = RiotApi::new(&api);
    acc.fill_mastery(riot_api, platform).await;
    let riot_api = RiotApi::new(&api);
    acc.fill_matches(riot_api, region).await;
    return acc;
}
