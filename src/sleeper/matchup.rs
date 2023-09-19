use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Matchup {
    pub matchup_id: i16,
    pub players: Vec<String>,
    pub roster_id: i16,
    pub starters: Vec<String>,
}

pub struct MatchupsService {
    week: i16,
    league_id: String,
}

impl MatchupsService {
    pub fn new(league_id: String, week: i16) -> MatchupsService {
        MatchupsService { league_id, week }
    }

    pub async fn fetch(&self) -> Result<Vec<Matchup>, reqwest::Error> {
        let url = format!(
            "https://api.sleeper.app/v1/league/{}/matchups/{}",
            self.league_id, self.week
        );
        let result = reqwest::get(url).await?.json::<Vec<Matchup>>().await?;

        Ok(result)
    }
}
