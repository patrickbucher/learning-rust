use std::collections::HashMap;

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
        let outnodes = match self.connections.get(from.into()) {
            Some(node) => node,
            None => return Err(format!("no such node '{from}' in graph")),
        };
        let mut distances: HashMap<String, Option<usize>> = HashMap::new();
        for (from, outnodes) in self.connections.clone() {
            distances.insert(from.into(), None);
            for outnode in outnodes.keys() {
                distances.insert(outnode.into(), None);
            }
        }
        // TODO: use get_cheapest
        // println!("{:?}", distances);
        Ok(Vec::new())
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
