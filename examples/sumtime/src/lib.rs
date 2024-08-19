use regex::Regex;

pub struct TimeParser {
    re: Regex,
}

impl TimeParser {
    pub fn new() -> Self {
        let re = Regex::new("^([0-9]+):([0-5][0-9])$").unwrap();
        TimeParser { re }
    }

    pub fn parse(&self, time: &str) -> (usize, usize) {
        match self.re.captures(time) {
            Some(matches) => {
                if matches.len() == 3 {
                    let (hours, minutes) = match (matches.get(1), matches.get(2)) {
                        (Some(h), Some(m)) => (h.as_str(), m.as_str()),
                        _ => ("0", "0"),
                    };
                    (hours.parse().unwrap_or(0), minutes.parse().unwrap_or(0))
                } else {
                    (0, 0)
                }
            }
            None => (0, 0),
        }
    }
}
