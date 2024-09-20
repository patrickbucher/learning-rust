use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub enum Kind {
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq)]
struct Vertex<K: Eq + Clone + Hash, V: Clone> {
    id: K,
    value: V,
}

pub struct Graph<K: Eq + Clone + Hash, V: Clone> {
    kind: Kind,
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, HashSet<K>>,
}

#[derive(PartialEq, Debug)]
pub enum GraphError {
    VertexAlreadyExists,
    VertexInexistant,
    EdgeAlreadyExists,
}

impl<K, V> Graph<K, V>
where
    K: Eq + Clone + Hash,
    V: Clone,
{
    pub fn new(kind: Kind) -> Self {
        Graph {
            kind,
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, id: K, value: V) -> Result<(), GraphError> {
        if self.vertices.contains_key(&id) {
            return Err(GraphError::VertexAlreadyExists);
        }
        let vertex = Vertex {
            id: id.clone(),
            value,
        };
        self.vertices.insert(id.clone(), vertex);
        self.edges.insert(id.clone(), HashSet::new());
        Ok(())
    }

    pub fn get_vertex(&self, id: K) -> Option<Vertex<K, V>> {
        match self.vertices.get(&id) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }

    pub fn add_edge(&mut self, from: K, to: K) -> Result<(), GraphError> {
        let _ = self
            .get_vertex(from.clone())
            .ok_or(GraphError::VertexInexistant)?;
        let _ = self
            .get_vertex(to.clone())
            .ok_or(GraphError::VertexInexistant)?;
        match self.kind {
            Kind::Directed => {
                self.edges
                    .entry(from.clone())
                    .and_modify(|e| {
                        e.insert(to.clone());
                    })
                    .or_insert(HashSet::from([to.clone()]));
            }
            Kind::Undirected => {
                self.edges
                    .entry(from.clone())
                    .and_modify(|e| {
                        e.insert(to.clone());
                    })
                    .or_insert(HashSet::from([to.clone()]));
                self.edges
                    .entry(to.clone())
                    .and_modify(|e| {
                        e.insert(from.clone());
                    })
                    .or_insert(HashSet::from([from.clone()]));
            }
        }
        Ok(())
    }

    pub fn get_edges_to(&self, from: K) -> Result<HashSet<K>, GraphError> {
        self.edges
            .get(&from)
            .ok_or(GraphError::VertexInexistant)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_vertex() {
        let mut graph = Graph::new(Kind::Directed);
        assert_eq!(graph.add_vertex("a", "Alice"), Ok(()));
        assert_eq!(graph.add_vertex("b", "Bob"), Ok(()));
        assert_eq!(
            graph.add_vertex("b", "Bernardo"),
            Err(GraphError::VertexAlreadyExists)
        );

        assert_eq!(
            graph.get_vertex("a"),
            Some(Vertex {
                id: "a",
                value: "Alice"
            })
        );
        assert_eq!(
            graph.get_vertex("b"),
            Some(Vertex {
                id: "b",
                value: "Bob"
            })
        );
        assert_eq!(graph.get_vertex("c"), None);
    }

    #[test]
    fn test_add_get_edge_directed() {
        let mut graph = Graph::new(Kind::Directed);
        graph.add_vertex("a", "Anderson");
        graph.add_vertex("b", "Beavis");
        graph.add_vertex("c", "Carla");
        graph.add_vertex("d", "Daria");
        graph.add_edge("a", "b");
        graph.add_edge("a", "d");
        graph.add_edge("b", "d");
        graph.add_edge("d", "a");

        assert_eq!(graph.get_edges_to("a"), Ok(HashSet::from(["b", "d"])));
        assert_eq!(graph.get_edges_to("b"), Ok(HashSet::from(["d"])));
        assert_eq!(graph.get_edges_to("c"), Ok(HashSet::new()));
        assert_eq!(graph.get_edges_to("d"), Ok(HashSet::from(["a"])));
        assert_eq!(graph.get_edges_to("e"), Err(GraphError::VertexInexistant));
    }
}
