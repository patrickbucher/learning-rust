use regex::Regex;

pub struct TimeParser {
    re: Regex,
}

impl TimeParser {
    pub fn new() -> Self {
        let re = Regex::new("^([0-9]+):([0-5][0-9])$").unwrap();
        TimeParser { re }
    }

    pub fn parse(&self, time: &str) -> Option<(usize, usize)> {
        let matches = self.re.captures(time)?;
        let hours = matches.get(1)?.as_str().parse().ok()?;
        let minutes = matches.get(2)?.as_str().parse().ok()?;
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
            ("0:00", Some((0, 0))),
            ("1:00", Some((1, 0))),
            ("1:23", Some((1, 23))),
            ("9:59", Some((9, 59))),
            ("9:60", None),
            ("10:59", Some((10, 59))),
            ("123:59", Some((123, 59))),
        ];
        let parser = TimeParser::new();
        for (input, expected) in tests {
            let actual = parser.parse(input);
            assert_eq!(actual, expected);
        }
    }
}
