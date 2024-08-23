use regex::Regex;

pub struct TimeParser {
    colon: Regex,
    decimal: Regex,
}

impl TimeParser {
    pub fn new() -> Self {
        let colon = Regex::new(r"^([0-9]+):([0-5][0-9])$").unwrap();
        let decimal = Regex::new(r"^([0-9]+)\.([0-9]+)$").unwrap();
        TimeParser { colon, decimal }
    }

    pub fn parse(&self, time: &str) -> Option<(usize, usize)> {
        self.parse_colon(time).or(self.parse_decimal(time))
    }

    fn parse_colon(&self, time: &str) -> Option<(usize, usize)> {
        let matches = self.colon.captures(time)?;
        let hours = matches.get(1)?.as_str().parse().ok()?;
        let minutes = matches.get(2)?.as_str().parse().ok()?;
        Some((hours, minutes))
    }

    fn parse_decimal(&self, time: &str) -> Option<(usize, usize)> {
        let matches = self.decimal.captures(time)?;
        let hours = matches.get(1)?.as_str().parse().ok()?;
        let fraction: f64 = format!("0.{}", matches.get(2)?.as_str()).parse().ok()?;
        let minutes: usize = (fraction * 60.0).round() as usize;
        Some((hours, minutes))
    }
}

impl Default for TimeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_times() {
        let tests: Vec<(&str, Option<(usize, usize)>)> = vec![
            ("", None),
            ("0", None),
            ("0:0", None),
            ("0.0", Some((0, 0))),
            ("0:00", Some((0, 0))),
            ("0.00", Some((0, 0))),
            ("1:00", Some((1, 0))),
            ("1.00", Some((1, 0))),
            ("1:23", Some((1, 23))),
            ("1.25", Some((1, 15))),
            ("9:59", Some((9, 59))),
            ("9.5", Some((9, 30))),
            ("9:60", None),
            ("10:59", Some((10, 59))),
            ("10.90", Some((10, 54))),
            ("123:59", Some((123, 59))),
            ("123.95", Some((123, 57))),
        ];
        let parser = TimeParser::new();
        for (input, expected) in tests {
            let actual = parser.parse(input);
            assert_eq!(actual, expected);
        }
    }
}
