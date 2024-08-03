struct Athlete {
    name: String,
    height: f32,
    weight: usize,
    sport: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_athletes() {
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
    }
}
