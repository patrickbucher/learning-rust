use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone)]
pub enum Kind {
    Directed,
    Undirected,
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vertex() {
        let mut graph = Graph::new(Kind::Directed);
        assert_eq!(graph.add_vertex("a", "Alice"), Ok(()));
        assert_eq!(graph.add_vertex("b", "Bob"), Ok(()));
        assert_eq!(graph.add_vertex("c", "Charlene"), Ok(()));
        assert_eq!(graph.add_vertex("d", "Dan"), Ok(()));
        assert_eq!(graph.add_vertex("b", "Bernardo"), Err(GraphError::VertexAlreadyExists));
    }
}
