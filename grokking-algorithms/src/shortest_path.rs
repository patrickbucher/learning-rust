use std::collections::{HashMap, HashSet};

pub struct Graph {
    connections: HashMap<String, HashMap<String, usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            connections: HashMap::new(),
        }
    }

    pub fn add(&mut self, from: &str, to: &str, weight: usize) {
        self.connections
            .entry(from.into())
            .or_insert(HashMap::new())
            .insert(to.into(), weight);
    }

    pub fn get_weight(&self, from: &str, to: &str) -> Option<usize> {
        match self.connections.get(from.into()) {
            Some(outgoing) => outgoing.get(to.into()).copied(),
            None => None,
        }
    }

    pub fn get_shortest_path(&self, from: &str, to: &str) -> Result<Vec<String>, String> {
        println!("{:?}", self.build_costs(from));
        println!("{:?}", self.build_parents(from));
        Ok(Vec::new())
    }

    fn build_costs(&self, from: &str) -> HashMap<String, Option<usize>> {
        let mut costs: HashMap<String, Option<usize>> = HashMap::new();
        for (node, outnodes) in self.connections.clone() {
            if node != from {
                costs.insert(node.into(), None);
            }
            for outnode in outnodes.keys() {
                costs.insert(outnode.into(), None);
            }
        }
        match self.connections.get(from.into()) {
            Some(outnodes) => {
                for (node, dist) in outnodes {
                    costs
                        .entry(node.into())
                        .and_modify(|mut v| *v = Some(*dist))
                        .or_insert(Some(*dist));
                }
                costs
            }
            None => return HashMap::new(),
        }
    }

    fn build_parents(&self, from: &str) -> HashMap<String, Option<String>> {
        let mut parents: HashMap<String, Option<String>> = HashMap::new();
        for (node, outnodes) in &self.connections {
            parents.insert(node.into(), None);
            for outnode in outnodes.keys() {
                parents.insert(outnode.into(), None);
            }
        }
        match &self.connections.get(from.into()) {
            Some(outnodes) => {
                for outnode in outnodes.keys() {
                    parents
                        .entry(outnode.into())
                        .and_modify(|mut v| *v = Some(from.into()))
                        .or_insert(Some(from.into()));
                }
            }
            None => return HashMap::new(),
        }
        parents
    }
}

fn get_cheapest(nodes: &HashMap<String, usize>) -> Option<(String, usize)> {
    let mut cheapest: Option<(String, usize)> = None;
    for (outnode, distance) in nodes {
        cheapest = match cheapest {
            Some((node, dist)) => {
                if *distance < dist {
                    Some((outnode.into(), *distance))
                } else {
                    Some((node.into(), dist))
                }
            }
            None => Some((outnode.into(), *distance)),
        }
    }
    cheapest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_available_weights() {
        let mut graph = Graph::new();
        graph.add("a", "b", 3);
        graph.add("b", "c", 4);
        assert_eq!(graph.get_weight("a", "b"), Some(3));
        assert_eq!(graph.get_weight("b", "c"), Some(4));
    }

    #[test]
    fn get_cheapest_node() {
        let mut distances: HashMap<String, usize> = HashMap::new();
        distances.insert("a".into(), 8);
        distances.insert("b".into(), 3);
        distances.insert("c".into(), 1);
        distances.insert("d".into(), 5);
        distances.insert("e".into(), 2);
        let expected = Some(("c".into(), 1));
        let actual = get_cheapest(&distances);
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_shortest_path() {
        let mut graph = Graph::new();
        graph.add("Start", "A", 6);
        graph.add("Start", "B", 2);
        graph.add("A", "B", 3);
        graph.add("A", "Finish", 1);
        graph.add("B", "Finish", 5);
        let expected: Vec<String> = vec!["Start".into(), "B".into(), "A".into(), "Finish".into()];
        let actual = graph.get_shortest_path("Start", "Finish");
        assert_eq!(expected, actual.unwrap());
    }
}
