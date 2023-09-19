use reqwest;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

struct ProjectionsService {
    base_url: String,
    week: i16,
}

struct Matchup {
    custom_points: f64,
    matchup_id: i16,
    players: Vec<String>,
}

type Projections = HashMap<String, f64>;

impl ProjectionsService {
    fn new(week: i16) -> ProjectionsService {
        ProjectionsService {
            base_url: String::from("https://api.sleeper.app/v1/projections/nfl/regular/2023"),
            week,
        }
    }

    async fn fetch(&self) -> Result<Projections, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, self.week);
        let result = reqwest::get(url).await?.json::<serde_json::Value>().await?;

        let mut filtered_results: Projections = HashMap::new();

        for (key, value) in result.as_object().unwrap().into_iter() {
            let projected_points = value["pts_half_ppr"].as_f64();

            match projected_points {
                Some(val) => {
                    filtered_results.insert(key.to_owned(), val);
                }
                None => (),
            }
        }

        Ok(filtered_results)
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let projections_service = ProjectionsService::new(3);
    let result = projections_service.fetch().await;

    match result {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{:?}", e),
    }
}
