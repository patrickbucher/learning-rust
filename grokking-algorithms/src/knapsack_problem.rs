use crate::utils::create_subsets;
use std::cmp::{Ord, Ordering};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Item {
    name: String,
    cost: usize,
    value: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection {
    items: HashSet<Item>,
}

impl Selection {
    pub fn empty() -> Self {
        Selection {
            items: HashSet::new(),
        }
    }

    pub fn cost(&self) -> usize {
        self.items.iter().fold(0, |acc, i| acc + i.cost)
    }

    pub fn value(&self) -> usize {
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

pub fn optimize(items: &[Item], space: usize) -> Selection {
    let mut grid: Vec<Vec<Selection>> = Vec::new();
    for _ in 0..items.len() {
        let mut row = Vec::new();
        for _ in 0..space {
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
            for row in grid.iter().take(i) {
                col.push(row[j].clone());
            }
            let col_max = col.into_iter().max().unwrap_or(Selection::empty());
            let col_best = if new_fit.value() > col_max.value() {
                new_fit
            } else {
                col_max
            };
            let cost_available = (j as isize + 1) - items[i].cost as isize;
            let new_best = if i > 0 && cost_available > 0 {
                let mut selection = grid[i - 1][cost_available as usize - 1].clone();
                selection.items.insert(items[i].clone());
                selection
            } else {
                Selection::empty()
            };
            grid[i][j] = if col_best.value() > new_best.value() {
                col_best
            } else {
                new_best
            };
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1].clone()
}

pub fn optimize_exhaustive(items: &[Item], space: usize) -> Selection {
    create_subsets(items)
        .iter()
        .map(|s| Selection {
            items: HashSet::from_iter(s.iter().cloned()),
        })
        .filter(|c| c.cost() <= space)
        .max()
        .unwrap_or(Selection::empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knapsack_problem() {
        let items = [
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
        let expected = Selection { items: expected };
        let actual = optimize(&items, 6);
        assert_eq!(actual, expected);
        let actual = optimize_exhaustive(&items, 6);
        assert_eq!(actual, expected);
    }
}
