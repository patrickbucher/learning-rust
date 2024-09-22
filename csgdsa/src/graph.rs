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
    pub fn new_weighted(kind: Kind) -> Self {
        Graph {
            kind,
            edge_type: EdgeType::Weighted(0),
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn new_unweighted(kind: Kind) -> Self {
        Graph {
            kind,
            edge_type: EdgeType::Unweighted,
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

    pub fn get_edges(&self, from: K) -> Result<HashMap<K, EdgeType>, GraphError> {
        self.edges
            .get(&from)
            .ok_or(GraphError::VertexInexistant)
            .cloned()
    }

    pub fn is_connected(&self, from: K, to: K) -> Result<bool, GraphError> {
        self.get_vertex(from).ok_or(GraphError::VertexInexistant)?;
        self.get_vertex(to).ok_or(GraphError::VertexInexistant)?;
        Ok(false)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_vertex() {
        let mut graph = Graph::new_unweighted(Kind::Directed);
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
    fn test_add_get_edge_directed_weighted() -> Result<(), GraphError> {
        let mut graph = Graph::new_weighted(Kind::Directed);
        graph.add_vertex("h", "Homer")?;
        graph.add_vertex("m", "Marge")?;
        graph.add_vertex("b", "Bart")?;
        graph.add_vertex("l", "Lisa")?;
        graph.add_edge_weighted("h", "m", 1)?;
        graph.add_edge_weighted("b", "l", 3)?;
        graph.add_edge_weighted("m", "l", 2)?;
        graph.add_edge_weighted("h", "b", 4)?;

        assert_eq!(
            graph.get_edges("h"),
            Ok(HashMap::from([
                ("b", EdgeType::Weighted(4)),
                ("m", EdgeType::Weighted(1)),
            ]))
        );
        assert_eq!(
            graph.get_edges("m"),
            Ok(HashMap::from([("l", EdgeType::Weighted(2))]))
        );
        assert_eq!(graph.get_edges("l"), Ok(HashMap::new()));
        assert_eq!(
            graph.get_edges("b"),
            Ok(HashMap::from([("l", EdgeType::Weighted(3))]))
        );
        assert_eq!(graph.get_edges("z"), Err(GraphError::VertexInexistant));
        Ok(())
    }

    #[test]
    fn test_add_get_edge_undirected_weighted() -> Result<(), GraphError> {
        let mut graph = Graph::new_weighted(Kind::Undirected);
        graph.add_vertex("h", "Homer")?;
        graph.add_vertex("m", "Marge")?;
        graph.add_vertex("b", "Bart")?;
        graph.add_vertex("l", "Lisa")?;
        graph.add_edge_weighted("h", "m", 13)?;
        graph.add_edge_weighted("b", "l", 25)?;
        graph.add_edge_weighted("m", "l", 18)?;
        graph.add_edge_weighted("h", "b", 78)?;

        assert_eq!(
            graph.get_edges("h"),
            Ok(HashMap::from([
                ("b", EdgeType::Weighted(78)),
                ("m", EdgeType::Weighted(13)),
            ]))
        );
        assert_eq!(
            graph.get_edges("m"),
            Ok(HashMap::from([
                ("l", EdgeType::Weighted(18)),
                ("h", EdgeType::Weighted(13))
            ]))
        );
        assert_eq!(
            graph.get_edges("l"),
            Ok(HashMap::from([
                ("b", EdgeType::Weighted(25)),
                ("m", EdgeType::Weighted(18)),
            ]))
        );
        assert_eq!(
            graph.get_edges("b"),
            Ok(HashMap::from([
                ("l", EdgeType::Weighted(25)),
                ("h", EdgeType::Weighted(78)),
            ]))
        );
        assert_eq!(graph.get_edges("z"), Err(GraphError::VertexInexistant));
        Ok(())
    }

    #[test]
    fn test_add_get_edge_directed_unweighted() -> Result<(), GraphError> {
        let mut graph = Graph::new_unweighted(Kind::Directed);
        graph.add_vertex("a", "Anderson")?;
        graph.add_vertex("b", "Beavis")?;
        graph.add_vertex("c", "Carla")?;
        graph.add_vertex("d", "Daria")?;
        graph.add_edge_unweighted("a", "b")?;
        graph.add_edge_unweighted("a", "d")?;
        graph.add_edge_unweighted("b", "d")?;
        graph.add_edge_unweighted("d", "a")?;

        assert_eq!(
            graph.get_edges("a"),
            Ok(HashMap::from([
                ("b", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(
            graph.get_edges("b"),
            Ok(HashMap::from([("d", EdgeType::Unweighted)]))
        );
        assert_eq!(graph.get_edges("c"), Ok(HashMap::new()));
        assert_eq!(
            graph.get_edges("d"),
            Ok(HashMap::from([("a", EdgeType::Unweighted)]))
        );
        assert_eq!(graph.get_edges("e"), Err(GraphError::VertexInexistant));
        Ok(())
    }

    #[test]
    fn test_add_get_edge_undirected_unweighted() -> Result<(), GraphError> {
        let mut graph = Graph::new_unweighted(Kind::Undirected);
        graph.add_vertex("a", "Anderson")?;
        graph.add_vertex("b", "Beavis")?;
        graph.add_vertex("c", "Carla")?;
        graph.add_vertex("d", "Daria")?;
        graph.add_edge_unweighted("a", "b")?;
        graph.add_edge_unweighted("a", "d")?;
        graph.add_edge_unweighted("b", "d")?;
        graph.add_edge_unweighted("d", "a")?;

        assert_eq!(
            graph.get_edges("a"),
            Ok(HashMap::from([
                ("b", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(
            graph.get_edges("b"),
            Ok(HashMap::from([
                ("a", EdgeType::Unweighted),
                ("d", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(graph.get_edges("c"), Ok(HashMap::new()));
        assert_eq!(
            graph.get_edges("d"),
            Ok(HashMap::from([
                ("a", EdgeType::Unweighted),
                ("b", EdgeType::Unweighted)
            ]))
        );
        assert_eq!(graph.get_edges("e"), Err(GraphError::VertexInexistant));
        Ok(())
    }

    #[test]
    fn test_are_vertices_connected() -> Result<(), GraphError> {
        let mut graph = Graph::new_unweighted(Kind::Undirected);
        graph.add_vertex("a", "Alice")?;
        graph.add_vertex("b", "Bob")?;
        graph.add_vertex("c", "Charlene")?;
        graph.add_vertex("d", "Dan")?;
        graph.add_vertex("e", "Elvira")?;
        graph.add_vertex("f", "Frank")?;
        graph.add_vertex("g", "Gina")?;
        graph.add_vertex("h", "Hank")?;
        graph.add_vertex("i", "Isabella")?;
        graph.add_vertex("j", "Jules")?;
        graph.add_vertex("k", "Kira")?;
        graph.add_vertex("l", "Larry")?;
        graph.add_vertex("m", "Mary")?;
        graph.add_vertex("n", "Nate")?;
        graph.add_vertex("o", "Olivia")?;
        graph.add_vertex("p", "Paul")?;
        graph.add_vertex("q", "Quinn")?;
        graph.add_vertex("r", "Ron")?;
        graph.add_vertex("s", "Sally")?;
        graph.add_vertex("t", "Tom")?;
        graph.add_vertex("u", "Uma")?;
        graph.add_vertex("v", "Vince")?;
        graph.add_vertex("w", "Winona")?;
        graph.add_vertex("x", "Xaver")?;
        graph.add_vertex("y", "Yumi")?;
        graph.add_vertex("z", "Zed")?;

        graph.add_edge_unweighted("a", "b")?;
        graph.add_edge_unweighted("a", "c")?;
        graph.add_edge_unweighted("c", "d")?;
        graph.add_edge_unweighted("c", "e")?;
        graph.add_edge_unweighted("e", "f")?;
        graph.add_edge_unweighted("f", "g")?;
        graph.add_edge_unweighted("h", "i")?;
        graph.add_edge_unweighted("i", "j")?;
        graph.add_edge_unweighted("i", "k")?;
        graph.add_edge_unweighted("j", "n")?;
        graph.add_edge_unweighted("k", "l")?;
        graph.add_edge_unweighted("k", "o")?;
        graph.add_edge_unweighted("l", "m")?;
        graph.add_edge_unweighted("p", "q")?;
        graph.add_edge_unweighted("q", "s")?;
        graph.add_edge_unweighted("q", "t")?;
        graph.add_edge_unweighted("t", "r")?;
        graph.add_edge_unweighted("t", "w")?;
        graph.add_edge_unweighted("u", "v")?;
        graph.add_edge_unweighted("w", "x")?;
        graph.add_edge_unweighted("x", "y")?;
        graph.add_edge_unweighted("y", "z")?;

        Ok(())
    }
}
