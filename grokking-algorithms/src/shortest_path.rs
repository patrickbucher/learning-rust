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

    pub fn get_shortest_path(&self, from: &str, to: &str) -> Result<(Vec<String>, usize), String> {
        let mut costs: HashMap<String, usize> = HashMap::new();
        let mut parents: HashMap<String, String> = HashMap::new();
        let mut processed: HashSet<String> = HashSet::new();

        let mut current: String = from.into();
        costs.insert(current.into(), 0);

        loop {
            // add outnodes to the costs table

            // determine the cheapest node to continue with
            let mut cheapest: Option<(String, usize)> = None;
            for (node, weight) in &costs {
                cheapest = match cheapest {
                    Some((_, w)) => {
                        if *weight < w && !processed.contains(node.into()) {
                            Some((node.into(), *weight))
                        } else {
                            cheapest
                        }
                    }
                    None => cheapest,
                }
            }
            if cheapest.is_none() {
                break;
            }
            let current = cheapest.unwrap().0;
            println!("continue with node {}", current);
            
            // process the cheapest node's outnodes
            let mut outnodes = match self.connections.get(&current) {
                Some(node) => node,
                None => return Err(format!("no such node {current}")),
            };
            let start_weight = costs.get(&current).unwrap().clone();
            for (outnode, weight) in outnodes {
                let new_weight = start_weight + weight;
                if costs.contains_key(outnode) {
                    let old_weight = costs.get(outnode).unwrap();
                    if new_weight < *old_weight {
                        costs.insert(outnode.clone(), new_weight);
                        parents.insert(current.clone(), outnode.clone());
                    }
                } else {
                    costs.insert(outnode.clone(), new_weight);
                    parents.insert(current.clone(), outnode.clone());
                }
            }

            // mark the node as processed
            processed.insert(current.into());
        }
        println!("{:?}", costs);
        Ok((Vec::new(), usize::MAX))
    }
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
    fn get_shortest_path() {
        let mut graph = Graph::new();
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
}
