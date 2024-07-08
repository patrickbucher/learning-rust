use std::collections::{HashMap, HashSet, VecDeque};

pub struct DirectedGraph {
    graph: HashMap<String, Vec<String>>,
}

impl DirectedGraph {
    pub fn new() -> Self {
        DirectedGraph {
            graph: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, from: &str, to: &str) {
        self.graph
            .entry(from.to_string())
            .and_modify(|v| v.push(to.to_string()))
            .or_insert(vec![to.to_string()]);
    }

    pub fn get_relations(&self, from: &str) -> Option<Vec<String>> {
        self.graph.get(from).cloned()
    }

    pub fn has_path(&self, from: &str, to: &str) -> bool {
        let initial_neighbours = match self.graph.get(from) {
            None => return false,
            Some(vec) => vec,
        };
        let mut search_queue: VecDeque<String> = VecDeque::from(initial_neighbours.clone());
        let mut searched: HashSet<String> = HashSet::new();
        while !search_queue.is_empty() {
            match search_queue.pop_back() {
                None => return false,
                Some(node) => {
                    if !searched.contains(&node) {
                        if node == to {
                            return true;
                        } else if let Some(neighbours) = self.graph.get(&node) {
                            search_queue.extend(neighbours.clone());
                            searched.insert(node);
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_relation() {
        let mut graph = DirectedGraph::new();
        graph.add_relation("Alice", "Bob");
        graph.add_relation("Alice", "Charlene");
        graph.add_relation("Bob", "Elvira");
        graph.add_relation("Charlene", "Dan");
        graph.add_relation("Dan", "Freddy");
        graph.add_relation("Dan", "Elvira");
        graph.add_relation("Elvira", "Waldo");
        graph.add_relation("Freddy", "Gina");
        graph.add_relation("Gina", "Waldo");

        let expected = Some(vec!["Freddy".to_string(), "Elvira".to_string()]);
        let actual = graph.get_relations("Dan");
        assert_eq!(expected, actual);

        let expected = None;
        let actual = graph.get_relations("Waldo");
        assert_eq!(expected, actual);
    }

    #[test]
    fn search_from_to() {
        let mut graph = DirectedGraph::new();
        graph.add_relation("Alice", "Bob");
        graph.add_relation("Alice", "Charlene");
        graph.add_relation("Bob", "Elvira");
        graph.add_relation("Charlene", "Dan");
        graph.add_relation("Dan", "Freddy");
        graph.add_relation("Dan", "Elvira");
        graph.add_relation("Elvira", "Waldo");
        graph.add_relation("Freddy", "Gina");
        graph.add_relation("Gina", "Waldo");

        assert!(graph.has_path("Alice", "Bob")); // Alice -> Bob
        assert!(graph.has_path("Alice", "Elvira")); // Alice -> Bob -> Elvira
        assert!(graph.has_path("Alice", "Waldo")); // Alice -> Bob -> Elvira -> Waldo
        assert!(!graph.has_path("Bob", "Alice"));
        assert!(!graph.has_path("Bob", "Dan"));
        assert!(!graph.has_path("Waldo", "Alice"));
    }
}
