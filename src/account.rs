use std::collections::HashMap;

pub struct Account {
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub mastery: HashMap<riven::consts::Champion, HashMap<String, i64>>,
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

    // Prints mastery from account, TODO add sort
    pub fn print_mastery(&mut self) {
        println!("{}#{}'s masteries: ", self.game_name, self.tag_line);
        for (id, map) in self.mastery.iter() {
            println!("{}: {} ({})", id.name().unwrap(), map["Points"], map["Level"]);
        }
    }
}
pub async fn init_account(api: riven::RiotApi, platform: riven::consts::PlatformRoute, game_name: String, tag_line: String, puuid: String) -> Account {
    let mut acc = Account{game_name: game_name, tag_line: tag_line, puuid: puuid, mastery: HashMap::new()};
    acc.fill_mastery(api, platform).await;
    return acc;
}
