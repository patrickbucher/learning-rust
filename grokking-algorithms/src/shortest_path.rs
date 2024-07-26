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
        if !self.connections.contains_key(from.into()) {
            return Err(format!("no such node '{from}' in graph"));
        }
        Ok(Vec::new())
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
        graph.add("A", "B", 3);
        graph.add("A", "Finish", 1);
        graph.add("B", "Finish", 5);
        let expected: Vec<String> = vec!["Start".into(), "B".into(), "A".into(), "Finish".into()];
        let actual = graph.get_shortest_path("Start", "Finish");
        assert_eq!(expected, actual.unwrap());
    }
}
