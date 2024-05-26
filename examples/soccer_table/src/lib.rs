mod parsing;
mod table_row;

use parsing::{list_relevant_files, read_lines, MatchResult, ParseError};
use std::collections::{hash_map::Entry, HashMap};
use std::fmt::{Display, Error, Formatter};
use std::path::Path;
use table_row::TableRow;

pub struct Table {
    rows: Vec<TableRow>,
}

impl Table {
    pub fn ranked(&self) -> Vec<TableRow> {
        let mut rows = self.rows.clone();
        rows.sort();
        rows.iter_mut()
            .enumerate()
            .map(|(i, r)| {
                r.rank = (i as u8) + 1;
                r.clone()
            })
            .collect()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let title = format!(
            "{:>3} {:30} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3}",
            "#", "Team", "P", "W", "T", "L", "+", "-", "="
        );
        let separator = "-".repeat(title.chars().count());
        f.write_str(&format!("{}\n", &title))?;
        f.write_fmt(format_args!("{}\n", separator))?;
        let rows = self.ranked();
        for r in rows {
            f.write_fmt(format_args!(
                "{:>3} {:30} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3} {:>3}\n",
                r.rank,
                r.team,
                r.points,
                r.wins,
                r.defeats,
                r.ties,
                r.goals_scored,
                r.goals_conceded,
                r.goals_diff
            ))?;
        }
        Ok(())
    }
}

pub enum Failure {
    Parsing(String),
    Other,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Failure::Parsing(s) => write!(f, "{s}"),
            Failure::Other => write!(f, "other failure"),
        }
    }
}

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Table, Failure> {
    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = list_relevant_files(dir, day) {
        for p in files {
            lines.append(&mut read_lines(&p))
        }
    }

    let result: Vec<Result<MatchResult, ParseError>> =
        MatchResult::parse_all(lines).map_err(|e| Failure::Parsing(format!("{e}")))?;
    let (results, errs): (Vec<_>, Vec<_>) = result.iter().partition(|e| e.is_ok());
    if !errs.is_empty() {
        let msg = errs
            .into_iter()
            .map(|e| e.as_ref().unwrap_err())
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(Failure::Parsing(msg));
    }
    let single_rows: Vec<TableRow> = results
        .into_iter()
        .map(|r| r.as_ref().unwrap())
        .map(|r| TableRow::from(r.clone()))
        .flat_map(|(a, b)| vec![a, b])
        .collect();

    let grouped = group_by_team(single_rows);
    let res: Vec<Result<TableRow, String>> = grouped
        .iter()
        .map(|(k, v)| {
            v.iter()
                .try_fold(TableRow::new(k), |acc, r| acc.combine(r.clone()))
        })
        .collect();

    let mut rows: Vec<TableRow> = Vec::new();
    let mut errs: Vec<String> = Vec::new();
    for r in res {
        match r {
            Ok(row) => rows.push(row),
            Err(err) => errs.push(err),
        }
    }
    if errs.is_empty() {
        Ok(Table { rows })
    } else {
        Err(Failure::Other)
    }
}

fn group_by_team(rows: Vec<TableRow>) -> HashMap<String, Vec<TableRow>> {
    let mut rows_by_team: HashMap<String, Vec<TableRow>> = HashMap::new();
    for row in rows {
        match rows_by_team.entry(row.team.clone()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(row.clone());
            }
            Entry::Vacant(e) => {
                e.insert(vec![row.clone()]);
            }
        }
    }
    rows_by_team
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_team() {
        let rows = vec![
            TableRow::new("A"),
            TableRow::new("B"),
            TableRow::new("C"),
            TableRow::new("A"),
            TableRow::new("B"),
            TableRow::new("A"),
        ];
        let grouped = group_by_team(rows);
        assert_eq!(grouped.get("A").unwrap().len(), 3);
        assert_eq!(grouped.get("B").unwrap().len(), 2);
        assert_eq!(grouped.get("C").unwrap().len(), 1);
    }

    #[test]
    fn test_ranking() {
        let names = vec!["A", "B", "C", "D", "E"];
        let table = Table {
            rows: names.iter().map(|n| TableRow::new(n)).collect(),
        };
        let ranked = table.ranked();
        for i in 0..ranked.len() {
            assert_eq!(ranked.get(i).unwrap().rank as usize, i + 1);
        }
    }
}
