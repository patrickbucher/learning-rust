use std::collections::HashMap;

#[derive(Debug)]
pub struct Athlete {
    pub name: String,
    pub height: f32,
    pub weight: usize,
    pub sport: String,
}

pub fn knn_predict_sport(known: &[Athlete], unknown: &Athlete, k: usize) -> Option<String> {
    let mut athlete_distances: Vec<(&Athlete, usize)> = known
        .iter()
        .map(|a| (a, (distance(a, unknown) * 100.0) as usize))
        .collect();
    athlete_distances.sort_by_key(|(_, d)| *d);
    let nearest_neighbors: Vec<String> = athlete_distances
        .iter()
        .take(k)
        .map(|(a, _)| a.sport.clone())
        .collect();
    let mut sports_count: HashMap<String, usize> = HashMap::new();
    for neighbour in &nearest_neighbors {
        sports_count
            .entry(neighbour.clone())
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }
    let mut sports_count: Vec<(String, usize)> = sports_count.into_iter().collect();
    sports_count.sort_by_key(|(_, v)| *v);
    println!("{sports_count:?}");
    sports_count.last().map(|(k, _)| k).cloned()
}

fn distance(some: &Athlete, other: &Athlete) -> f32 {
    f32::sqrt(
        (some.height - other.height).powf(2.0)
            + (some.weight as f32 - other.weight as f32).powf(2.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predict_sport() {
        let athletes = vec![
            Athlete {
                name: "Jan Ulrich".into(),
                height: 1.83,
                weight: 73,
                sport: "Cycling".into(),
            },
            Athlete {
                name: "Lance Armstrong".into(),
                height: 1.77,
                weight: 75,
                sport: "Cycling".into(),
            },
            Athlete {
                name: "Chris Froome".into(),
                height: 1.86,
                weight: 69,
                sport: "Cycling".into(),
            },
            Athlete {
                name: "Egan Bernal".into(),
                height: 1.75,
                weight: 60,
                sport: "Cycling".into(),
            },
            Athlete {
                name: "Geraint Thomas".into(),
                height: 1.83,
                weight: 71,
                sport: "Cycling".into(),
            },
            Athlete {
                name: "Christian Stucki".into(),
                height: 1.98,
                weight: 150,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Arnold Forrer".into(),
                height: 1.94,
                weight: 118,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Joel Wicki".into(),
                height: 1.82,
                weight: 107,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Philipp Laimbacher".into(),
                height: 1.85,
                weight: 102,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Matthias Glarner".into(),
                height: 1.84,
                weight: 116,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Kilian Wenger".into(),
                height: 1.90,
                weight: 112,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Roger Brügger".into(),
                height: 1.98,
                weight: 135,
                sport: "Swiss Wrestling".into(),
            },
            Athlete {
                name: "Daniel Yule".into(),
                height: 1.87,
                weight: 87,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Marco Odermatt".into(),
                height: 1.83,
                weight: 79,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Beat Feuz".into(),
                height: 1.73,
                weight: 85,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Didier Cuche".into(),
                height: 1.74,
                weight: 91,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Didier Defago".into(),
                height: 1.83,
                weight: 88,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Aksel Lund Svindal".into(),
                height: 1.89,
                weight: 97,
                sport: "Alpine Skiing".into(),
            },
            Athlete {
                name: "Kisenosato Yutaka".into(),
                height: 1.87,
                weight: 175,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Kakuryū Rikisaburō".into(),
                height: 1.86,
                weight: 161,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Musashimaru Kōyō".into(),
                height: 1.92,
                weight: 235,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Kisenosato Yutaka".into(),
                height: 1.88,
                weight: 177,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Akebono Tarō".into(),
                height: 2.03,
                weight: 233,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Ōnokuni Yasushi".into(),
                height: 1.89,
                weight: 203,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Hakuhō Shō".into(),
                height: 1.92,
                weight: 151,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Harumafuji Kōhei".into(),
                height: 1.86,
                weight: 137,
                sport: "Sumo Wrestling".into(),
            },
            Athlete {
                name: "Eliud Kipchoge".into(),
                height: 1.67,
                weight: 52,
                sport: "Long-Distance Running".into(),
            },
            Athlete {
                name: "Haile Gebrselassie".into(),
                height: 1.64,
                weight: 56,
                sport: "Long-Distance Running".into(),
            },
            Athlete {
                name: "Samuel Wanjiru".into(),
                height: 1.63,
                weight: 52,
                sport: "Long-Distance Running".into(),
            },
        ];

        let candidate = Athlete {
            name: "Tadej Pogačar".into(),
            height: 1.76,
            weight: 66,
            sport: "Cycling".into(),
        };
        let expected: Option<String> = Some(candidate.sport.clone());
        let actual = knn_predict_sport(&athletes, &candidate, 5);
        assert_eq!(actual, expected);

        let candidate = Athlete {
            name: "Aleksander Aamodt Kilde".into(),
            height: 1.81,
            weight: 90,
            sport: "Alpine Skiing".into(),
        };
        let expected: Option<String> = Some(candidate.sport.clone());
        let actual = knn_predict_sport(&athletes, &candidate, 5);
        assert_eq!(actual, expected);

        let candidate = Athlete {
            name: "Patrick Bucher".into(),
            height: 1.88,
            weight: 78,
            sport: "Cycling".into(),
        };
        let expected: Option<String> = Some(candidate.sport.clone());
        let actual = knn_predict_sport(&athletes, &candidate, 5);
        assert_eq!(actual, expected);
    }
}
