use std::time::Duration;

struct Track<'a> {
    name: &'a str,
    race_laps: u8,
    base_lap_time: Duration,
    pit_stop_delta_time: Duration,
}

struct Tyre<'a> {
    description: &'a str,
    lap_time_factor: f32,
    lap_deg_percentage: f32,
}

struct Stint<'a> {
    laps: u8,
    tyre: &'a Tyre<'a>,
}

struct Strategy<'a> {
    stints: Vec<&'a Stint<'a>>,
}

fn main() {
    let canada = Track {
        name: "Circuit Gilles-Villeneuve",
        race_laps: 78,
        base_lap_time: Duration::new(76, 0),
        pit_stop_delta_time: Duration::new(28, 0),
    };
    let soft = Tyre {
        description: "Soft",
        lap_time_factor: 1.0,
        lap_deg_percentage: 1.5,
    };
    let medium = Tyre {
        description: "Medium",
        lap_time_factor: 1.01,
        lap_deg_percentage: 1.25,
    };
    let hard = Tyre {
        description: "Medium",
        lap_time_factor: 1.02,
        lap_deg_percentage: 1.0,
    };
    let mercedes_1 = Stint {
        laps: 19,
        tyre: &soft,
    };
    let mercedes_2 = Stint {
        laps: 40,
        tyre: &hard,
    };
    let mercedes_3 = Stint {
        laps: 19,
        tyre: &soft,
    };
    let mercedes = Strategy {
        stints: vec![&mercedes_1, &mercedes_2, &mercedes_3],
    };
    let ferrari_1 = Stint {
        laps: 25,
        tyre: &medium,
    };
    let ferrari_2 = Stint {
        laps: 25,
        tyre: &medium,
    };
    let ferrari_3 = Stint {
        laps: 28,
        tyre: &hard,
    };
    let ferrari = Strategy {
        stints: vec![&ferrari_1, &ferrari_2, &ferrari_3],
    };
    let red_bull1 = Stint {
        laps: 50,
        tyre: &hard,
    };
    let red_bull2 = Stint {
        laps: 28,
        tyre: &medium,
    };
    let red_bull = Strategy {
        stints: vec![&red_bull1, &red_bull2],
    };
    let mc_laren1 = Stint {
        laps: 15,
        tyre: &soft,
    };
    let mc_laren2 = Stint {
        laps: 25,
        tyre: &medium,
    };
    let mc_laren3 = Stint {
        laps: 38,
        tyre: &hard,
    };
    let mc_laren = Strategy {
        stints: vec![&mc_laren1, &mc_laren2, &mc_laren3],
    };
    println!("Mercedes: {:?}", calculate_race_time(&canada, &mercedes));
    println!("Ferrari: {:?}", calculate_race_time(&canada, &ferrari));
    println!("Red Bull: {:?}", calculate_race_time(&canada, &red_bull));
    println!("McLaren: {:?}", calculate_race_time(&canada, &mc_laren));
}

fn calculate_race_time(track: &Track, strategy: &Strategy) -> Option<Duration> {
    let planned_laps = strategy.stints.iter().fold(0, |acc, s| acc + s.laps);
    if planned_laps != track.race_laps {
        return None;
    }
    let mut race_time = Duration::new(0, 0);
    for stint in &strategy.stints {
        race_time = race_time.saturating_add(track.pit_stop_delta_time);
        let base_lap_time = track.base_lap_time.mul_f32(stint.tyre.lap_time_factor);
        for lap in 0..stint.laps {
            let lap_time =
                base_lap_time.mul_f32((100.0 + lap as f32 * stint.tyre.lap_deg_percentage) / 100.0);
            race_time = race_time.saturating_add(lap_time);
        }
    }
    Some(race_time)
}
