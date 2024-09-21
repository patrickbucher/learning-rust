// TODO
// - delete vertex (with all its edges)
// - delete edge
// - ???
// weighted/unweighted edges

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub enum Kind {
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeType {
    Weighted(isize),
    Unweighted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vertex<K: Eq + Clone + Hash, V: Clone> {
    id: K,
    value: V,
}

pub struct Graph<K: Eq + Clone + Hash, V: Clone> {
    kind: Kind,
    edge_type: EdgeType,
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, HashMap<K, EdgeType>>,
}

#[derive(PartialEq, Debug)]
pub enum GraphError {
    VertexAlreadyExists,
    VertexInexistant,
    EdgeAlreadyExists,
    EdgeTypeMismatch,
}

impl<K, V> Graph<K, V>
where
    K: Eq + Clone + Hash,
    V: Clone,
{
    pub fn new(kind: Kind, edge_type: EdgeType) -> Self {
        Graph {
            kind,
            edge_type,
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
        self.edges.insert(id.clone(), HashMap::new());
        Ok(())
    }

    pub fn get_vertex(&self, id: K) -> Option<Vertex<K, V>> {
        self.vertices.get(&id).cloned()
    }

    pub fn add_edge_weighted(&mut self, from: K, to: K, weight: isize) -> Result<(), GraphError> {
        if let EdgeType::Unweighted = self.edge_type {
            return Err(GraphError::EdgeTypeMismatch);
        }
        self.add_edge(from, to, EdgeType::Weighted(weight))
    }

    pub fn add_edge_unweighted(&mut self, from: K, to: K) -> Result<(), GraphError> {
        if let EdgeType::Weighted(_) = self.edge_type {
            return Err(GraphError::EdgeTypeMismatch);
        }
        self.add_edge(from, to, EdgeType::Unweighted)
    }

    fn add_edge(&mut self, from: K, to: K, edge_type: EdgeType) -> Result<(), GraphError> {
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
                        e.insert(to.clone(), edge_type.clone());
                    })
                    .or_insert(HashMap::from([(to.clone(), edge_type.clone())]));
            }
            Kind::Undirected => {
                self.edges
                    .entry(from.clone())
                    .and_modify(|e| {
                        e.insert(to.clone(), edge_type.clone());
                    })
                    .or_insert(HashMap::from([(to.clone(), edge_type.clone())]));
                self.edges
                    .entry(to.clone())
                    .and_modify(|e| {
                        e.insert(from.clone(), edge_type.clone());
                    })
                    .or_insert(HashMap::from([(from.clone(), edge_type.clone())]));
            }
        }
        Ok(())
    }

    pub fn get_edges_to(&self, from: K) -> Result<HashMap<K, EdgeType>, GraphError> {
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
        let mut graph = Graph::new(Kind::Directed, EdgeType::Unweighted);
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
    fn test_add_get_edge_directed_unweighted() -> Result<(), GraphError> {
        let mut graph = Graph::new(Kind::Directed, EdgeType::Unweighted);
        graph.add_vertex("a", "Anderson")?;
        graph.add_vertex("b", "Beavis")?;
        graph.add_vertex("c", "Carla")?;
        graph.add_vertex("d", "Daria")?;
        graph.add_edge_unweighted("a", "b")?;
        graph.add_edge_unweighted("a", "d")?;
        graph.add_edge_unweighted("b", "d")?;
        graph.add_edge_unweighted("d", "a")?;

        assert_eq!(
            graph.get_edges_to("a"),
            Ok(HashMap::from([
                ("b", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(
            graph.get_edges_to("b"),
            Ok(HashMap::from([("d", EdgeType::Unweighted)]))
        );
        assert_eq!(graph.get_edges_to("c"), Ok(HashMap::new()));
        assert_eq!(
            graph.get_edges_to("d"),
            Ok(HashMap::from([("a", EdgeType::Unweighted)]))
        );
        assert_eq!(graph.get_edges_to("e"), Err(GraphError::VertexInexistant));
        Ok(())
    }

    #[test]
    fn test_add_get_edge_undirected_unweighted() -> Result<(), GraphError> {
        let mut graph = Graph::new(Kind::Undirected, EdgeType::Unweighted);
        graph.add_vertex("a", "Anderson")?;
        graph.add_vertex("b", "Beavis")?;
        graph.add_vertex("c", "Carla")?;
        graph.add_vertex("d", "Daria")?;
        graph.add_edge_unweighted("a", "b")?;
        graph.add_edge_unweighted("a", "d")?;
        graph.add_edge_unweighted("b", "d")?;
        graph.add_edge_unweighted("d", "a")?;

        assert_eq!(
            graph.get_edges_to("a"),
            Ok(HashMap::from([
                ("b", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(
            graph.get_edges_to("b"),
            Ok(HashMap::from([
                ("a", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(graph.get_edges_to("c"), Ok(HashMap::new()));
        assert_eq!(
            graph.get_edges_to("d"),
            Ok(HashMap::from([
                ("a", EdgeType::Unweighted),
                ("b", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(graph.get_edges_to("e"), Err(GraphError::VertexInexistant));
        Ok(())
    }
}
