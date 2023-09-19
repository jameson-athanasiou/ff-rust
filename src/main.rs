mod sleeper;

use sleeper::{matchup, projection};

struct MatchupProjection {
    matchup_id: i16,
    roster_id: i16,
    projected_points: f64,
}

fn calculate_projections(
    projections: projection::Projections,
    matchups: Vec<matchup::Matchup>,
) -> (Vec<MatchupProjection>, Vec<String>) {
    let mut missing_projections: Vec<String> = Vec::new();

    let matchup_projections: Vec<MatchupProjection> = matchups
        .iter()
        .map(|matchup| {
            let starter_projections: Vec<f64> = matchup
                .starters
                .iter()
                .map(|starter| {
                    let mut projection_value: f64 = 0.0;
                    match projections.get(starter) {
                        Some(val) => projection_value = *val,
                        None => {
                            println!("No projection found for player id {}", starter);
                            missing_projections.push(starter.clone());
                        }
                    }

                    projection_value
                })
                .collect();

            let total_starter_projections = starter_projections.iter().fold(0.0, |acc, p| acc + p);

            println!("{:?}", starter_projections);
            println!("total = {}", total_starter_projections);

            MatchupProjection {
                matchup_id: matchup.matchup_id,
                roster_id: matchup.roster_id,
                projected_points: total_starter_projections,
            }
        })
        .collect();

    (matchup_projections, missing_projections)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let projections_service = projection::ProjectionsService::new(3);
    let result = projections_service.fetch().await;

    // match result {
    //     Ok(data) => println!("{:?}", data),
    //     Err(e) => println!("{:?}", e),
    // }

    let matchups_service = matchup::MatchupsService::new(String::from("992213104140025856"), 3);
    let matchups_result = matchups_service.fetch().await.unwrap();

    let projections_result = result.unwrap();

    let (projections, missing_projections) =
        calculate_projections(projections_result, matchups_result);

    // match matchups_result {
    //     Ok(data) => {
    //         for m in data {
    //             println!("{}", serde_json::json!(m));
    //         }
    //     }
    //     Err(e) => println!("{:?}", e),
    // }
}
