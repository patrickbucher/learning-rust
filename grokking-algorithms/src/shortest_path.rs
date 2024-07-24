use std::collections::HashMap;

pub struct Graph {
    connections: HashMap<(String, String), usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            connections: HashMap::new(),
        }
    }

    pub fn add(&mut self, from: &str, to: &str, weight: usize) {
        self.connections.insert((from.into(), to.into()), weight);
        self.connections.insert((to.into(), from.into()), weight);
    }

    pub fn get_weight(&self, from: &str, to: &str) -> Option<usize> {
        self.connections.get(&(from.into(), to.into())).copied()
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
        assert_eq!(graph.get_weight("b", "a"), Some(3));
        assert_eq!(graph.get_weight("b", "c"), Some(4));
        assert_eq!(graph.get_weight("c", "b"), Some(4));
    }
}
