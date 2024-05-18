mod parsing;
mod table_row;

use parsing::{list_relevant_files, read_lines, MatchResult};
use std::collections::{hash_map::Entry, HashMap};
use std::path::Path;
use table_row::TableRow;

pub struct Table {
    pub rows: Vec<TableRow>,
}

pub fn compute_table(dir: &Path, day: Option<usize>) -> Result<Table, ()> {
    let mut lines: Vec<String> = Vec::new();
    if let Ok(files) = list_relevant_files(dir, day) {
        for p in files {
            lines.append(&mut read_lines(&p))
        }
    }
    let mut single_rows: Vec<TableRow> = Vec::new();
    for line in lines {
        let r: Result<MatchResult, _> = line.try_into();
        match r {
            Ok(m) => {
                let (home, away) = TableRow::from(m);
                single_rows.push(home);
                single_rows.push(away);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    let grouped = group_by_team(single_rows);
    let rows: Vec<TableRow> = grouped
        .iter()
        .map(|(k, v)| {
            v.iter()
                .fold(TableRow::new(&k), |acc, r| acc.combine(r.clone()).unwrap())
        })
        .collect();
    Ok(Table { rows })
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
}
