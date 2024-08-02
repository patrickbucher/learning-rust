use std::cmp::{Ord, Ordering};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Item {
    name: String,
    cost: usize,
    value: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Selection {
    items: HashSet<Item>,
}

impl Selection {
    fn empty() -> Self {
        Selection {
            items: HashSet::new(),
        }
    }

    fn cost(&self) -> usize {
        self.items.iter().fold(0, |acc, i| acc + i.cost)
    }

    fn value(&self) -> usize {
        self.items.iter().fold(0, |acc, i| acc + i.value)
    }
}

impl Ord for Selection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Selection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn optimize(items: &Vec<Item>, space: usize) -> Selection {
    let mut grid: Vec<Vec<Selection>> = Vec::new();
    for i in 0..items.len() {
        let mut row = Vec::new();
        for j in 0..space {
            row.push(Selection::empty());
        }
        grid.push(row);
    }
    for i in 0..items.len() {
        for j in 0..space {
            let new_fit: Selection = if items[i].cost <= j + 1 {
                Selection {
                    items: HashSet::from([items[i].clone()]),
                }
            } else {
                Selection::empty()
            };
            let mut col: Vec<Selection> = Vec::new();
            for k in 0..i {
                col.push(grid[k][j].clone());
            }
            let col_max = col.into_iter().max().unwrap_or(Selection::empty());
            let col_best = if new_fit.value() > col_max.value() {
                new_fit
            } else {
                col_max
            };
            let cost_available = items[i].cost as isize - (j as isize + 1);
            let new_best = if i > 0 && cost_available > 0 {
                let mut selection = grid[i - 1][cost_available as usize - 1].clone();
                selection.items.insert(items[i].clone());
                selection
            } else {
                Selection::empty()
            };
            grid[i][j] = if col_best.value() > new_best.value() {
                println!("grid[{i}][{j}] = {col_best:?}");
                col_best
            } else {
                println!("grid[{i}][{j}] = {new_best:?}");
                new_best
            };
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack_problem() {
        let items = vec![
            Item {
                name: "Water".into(),
                cost: 3,
                value: 10,
            },
            Item {
                name: "Book".into(),
                cost: 1,
                value: 3,
            },
            Item {
                name: "Food".into(),
                cost: 2,
                value: 9,
            },
            Item {
                name: "Jacket".into(),
                cost: 2,
                value: 5,
            },
            Item {
                name: "Camera".into(),
                cost: 1,
                value: 6,
            },
        ];
        let expected = HashSet::from([
            Item {
                name: "Water".into(),
                cost: 3,
                value: 10,
            },
            Item {
                name: "Camera".into(),
                cost: 1,
                value: 6,
            },
            Item {
                name: "Food".into(),
                cost: 2,
                value: 9,
            },
        ]);
        let actual = optimize(&items, 6);
        assert_eq!(actual, Selection { items: expected });
    }
}
