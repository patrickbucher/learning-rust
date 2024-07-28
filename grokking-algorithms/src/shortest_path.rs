use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Graph {
    connections: HashMap<String, HashMap<String, usize>>,
}

impl Graph {
    pub fn add(&mut self, from: &str, to: &str, weight: usize) {
        self.connections
            .entry(from.into())
            .or_default()
            .insert(to.into(), weight);
    }

    pub fn get_weight(&self, from: &str, to: &str) -> Option<usize> {
        match self.connections.get(from) {
            Some(outgoing) => outgoing.get(to).copied(),
            None => None,
        }
    }

    pub fn get_shortest_path(&self, from: &str, to: &str) -> Result<(Vec<String>, usize), String> {
        let mut current: String = from.into();
        let mut costs: HashMap<String, usize> = HashMap::from([(current.clone(), 0)]);
        let mut parents: HashMap<String, String> = HashMap::new();
        let mut processed: HashSet<String> = HashSet::new();

        loop {
            // determine the cheapest node to continue with
            let mut cheapest: Option<(String, usize)> = None;
            for (node, weight) in &costs {
                if processed.contains(node) {
                    continue;
                }
                cheapest = match cheapest {
                    Some((_, w)) => {
                        if *weight < w {
                            Some((node.into(), *weight))
                        } else {
                            cheapest
                        }
                    }
                    None => Some((node.into(), *weight)),
                }
            }
            if cheapest.is_none() {
                break;
            }
            current = cheapest.unwrap().0;

            // process the cheapest node's outnodes
            let outnodes = match self.connections.get(&current) {
                Some(node) => node,
                None => &HashMap::new(),
            };
            let start_weight = *costs.get(&current).unwrap();
            for (outnode, weight) in outnodes {
                let new = start_weight + weight;
                let old = costs.get(outnode);
                if old.is_none() || old.is_some() && new < *old.unwrap() {
                    costs.insert(outnode.clone(), new);
                    parents.insert(outnode.clone(), current.clone());
                }
            }

            // mark the node as processed
            processed.insert(current.clone());
        }

        // back-track parent path
        let path = backtrack(&parents, from, to);
        let length: usize = *costs.get(to).unwrap();
        Ok((path, length))
    }
}

fn backtrack(parents: &HashMap<String, String>, from: &str, to: &str) -> Vec<String> {
    let mut path: Vec<String> = vec![to.into()];
    let mut node: String = to.into();
    while node != *from {
        node = match parents.get(&node) {
            Some(child) => {
                path.push(child.clone());
                child.into()
            }
            None => break,
        };
    }
    let path: Vec<String> = path.into_iter().rev().collect();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_available_weights() {
        let mut graph: Graph = Default::default();
        graph.add("a", "b", 3);
        graph.add("b", "c", 4);
        assert_eq!(graph.get_weight("a", "b"), Some(3));
        assert_eq!(graph.get_weight("b", "c"), Some(4));
    }

    #[test]
    fn get_shortest_path_small() {
        let mut graph: Graph = Default::default();
        graph.add("Start", "A", 6);
        graph.add("Start", "B", 2);
        graph.add("B", "A", 3);
        graph.add("A", "Finish", 1);
        graph.add("B", "Finish", 5);

        let path: Vec<String> = vec!["Start".into(), "B".into(), "A".into(), "Finish".into()];
        let weight: usize = 6;
        let expected = Ok((path, weight));
        let actual = graph.get_shortest_path("Start", "Finish");
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_shortest_path_big() {
        let mut graph: Graph = Default::default();
        graph.add("Start", "A", 5);
        graph.add("Start", "B", 2);
        graph.add("B", "A", 8);
        graph.add("A", "C", 4);
        graph.add("A", "D", 2);
        graph.add("B", "D", 7);
        graph.add("C", "D", 6);
        graph.add("C", "Finish", 3);
        graph.add("D", "Finish", 1);

        let path: Vec<String> = vec!["Start".into(), "A".into(), "D".into(), "Finish".into()];
        let weight: usize = 8;
        let expected = Ok((path, weight));
        let actual = graph.get_shortest_path("Start", "Finish");
        assert_eq!(expected, actual);
    }

    #[test]
    fn backtrack_parents() {
        let parents: HashMap<String, String> = HashMap::from([
            ("A".into(), "Start".into()),
            ("B".into(), "Start".into()),
            ("C".into(), "B".into()),
            ("D".into(), "C".into()),
            ("Finish".into(), "D".into()),
        ]);
        let expected: Vec<String> = vec![
            "Start".into(),
            "B".into(),
            "C".into(),
            "D".into(),
            "Finish".into(),
        ];
        let actual = backtrack(&parents, "Start", "Finish");
        assert_eq!(actual, expected);
    }
}
