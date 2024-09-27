use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
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
pub struct Vertex<K: Eq + Clone + Hash + Debug, V: Clone> {
    id: K,
    value: V,
}

pub struct Graph<K: Eq + Clone + Hash + Debug, V: Clone> {
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
    NoSuchRoute,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Path<K: Eq + Clone + Hash + Debug> {
    from: K,
    to: K,
    path: Vec<K>,
    distance: isize,
}

impl<K, V> Graph<K, V>
where
    K: Eq + Clone + Hash + Debug,
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

    pub fn is_connected_breadth_first(&self, from: K, to: K) -> Result<bool, GraphError> {
        self.get_vertex(from.clone())
            .ok_or(GraphError::VertexInexistant)?;
        self.get_vertex(to.clone())
            .ok_or(GraphError::VertexInexistant)?;
        let mut visited = HashSet::new();
        let mut worklist = VecDeque::from([from.clone()]);
        while let Some(vertex) = worklist.pop_front() {
            let adjacents = self.get_edges(vertex.clone())?;
            for adjacent in adjacents.keys() {
                if *adjacent == to {
                    return Ok(true);
                } else if visited.contains(adjacent) {
                    continue;
                } else {
                    worklist.push_back(adjacent.clone());
                    visited.insert(adjacent.clone());
                }
            }
        }
        Ok(false)
    }

    pub fn is_connected_depth_first(&self, from: K, to: K) -> Result<bool, GraphError> {
        self.get_vertex(from.clone())
            .ok_or(GraphError::VertexInexistant)?;
        self.get_vertex(to.clone())
            .ok_or(GraphError::VertexInexistant)?;
        self.do_is_connected_depth_first(
            from.clone(),
            to.clone(),
            &mut HashSet::from([from.clone()]),
        )
    }

    pub fn find_shortest_paths(&self, from: K) -> Result<HashSet<Path<K>>, GraphError> {
        let mut shortest_paths: HashSet<Path<K>> = HashSet::new();
        for to in self.vertices.keys() {
            let (path, cost) = self.find_shortest_path(&from, to)?;
            shortest_paths.insert(Path {
                from: from.clone(),
                to: to.clone(),
                path,
                distance: cost,
            });
        }
        Ok(shortest_paths)
    }

    fn find_shortest_path(&self, from: &K, to: &K) -> Result<(Vec<K>, isize), GraphError> {
        let mut weights: HashMap<K, isize> = HashMap::from([(from.clone(), 0)]);
        let mut parents: HashMap<K, K> = HashMap::new();
        let mut visited: HashSet<K> = HashSet::new();
        let vertices: HashSet<K> = self.vertices.keys().cloned().collect();
        while visited.len() < vertices.len() {
            let mut candidates = weights.clone().into_iter().collect::<Vec<(K, isize)>>();
            candidates.sort_by_key(|(_, w)| *w);
            let (current, start_weight) =
                match candidates.into_iter().find(|(v, _)| !visited.contains(v)) {
                    Some((v, w)) => (v, w),
                    None => break,
                };
            let adjacents = self.get_edges(current.clone())?;
            for (adjacent, et) in adjacents {
                let edge_weight = match et {
                    EdgeType::Weighted(w) => w,
                    // TODO: for unweighted graphs, just use 1 as the weight!
                    _ => return Err(GraphError::EdgeTypeMismatch),
                };
                let new_weight = start_weight + edge_weight;
                let insert = if let Some(weight) = weights.get(&adjacent) {
                    new_weight < *weight
                } else {
                    true
                };
                if insert {
                    weights.insert(adjacent.clone(), new_weight);
                    parents.insert(adjacent.clone(), current.clone());
                }
            }
            visited.insert(current.clone());
        }
        let path = Self::backtrack(
            from,
            to,
            &parents
                .into_iter()
                .map(|(a, b)| (b, a))
                .collect::<Vec<(K, K)>>(),
        );
        let weight = weights.get(to).ok_or(GraphError::NoSuchRoute)?;
        Ok((path, *weight))
    }

    fn backtrack(start: &K, finish: &K, successors: &[(K, K)]) -> Vec<K> {
        let mut path: Vec<K> = Vec::new();
        let mut current = finish.clone();
        loop {
            path.push(current.clone());
            if let Some((next, _)) = successors.iter().find(|(_, c)| *c == current) {
                if next == start {
                    path.push(next.clone());
                    break;
                }
                current = next.clone();
            } else {
                break;
            }
        }
        path.reverse();
        path
    }

    fn do_is_connected_depth_first(
        &self,
        from: K,
        to: K,
        visited: &mut HashSet<K>,
    ) -> Result<bool, GraphError> {
        for adjacent in self.get_edges(from)?.keys() {
            if *adjacent == to {
                return Ok(true);
            } else if visited.contains(adjacent) {
                continue;
            } else {
                visited.insert(adjacent.clone());
                if self.do_is_connected_depth_first(adjacent.clone(), to.clone(), visited)? {
                    return Ok(true);
                }
            }
        }
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

        assert_eq!(graph.is_connected_breadth_first("b", "g"), Ok(true));
        assert_eq!(graph.is_connected_breadth_first("o", "n"), Ok(true));
        assert_eq!(graph.is_connected_breadth_first("z", "p"), Ok(true));
        assert_eq!(graph.is_connected_breadth_first("a", "i"), Ok(false));
        assert_eq!(graph.is_connected_breadth_first("l", "y"), Ok(false));
        assert_eq!(graph.is_connected_breadth_first("t", "e"), Ok(false));

        assert_eq!(graph.is_connected_depth_first("b", "g"), Ok(true));
        assert_eq!(graph.is_connected_depth_first("o", "n"), Ok(true));
        assert_eq!(graph.is_connected_depth_first("z", "p"), Ok(true));
        assert_eq!(graph.is_connected_depth_first("a", "i"), Ok(false));
        assert_eq!(graph.is_connected_depth_first("l", "y"), Ok(false));
        assert_eq!(graph.is_connected_depth_first("t", "e"), Ok(false));

        Ok(())
    }

    #[test]
    fn test_shortest_paths() -> Result<(), GraphError> {
        let mut graph = Graph::new_weighted(Kind::Directed);
        graph.add_vertex("a", "Atlanta")?;
        graph.add_vertex("b", "Boston")?;
        graph.add_vertex("c", "Chicago")?;
        graph.add_vertex("d", "Denver")?;
        graph.add_vertex("e", "El Paso")?;

        graph.add_edge_weighted("a", "b", 100)?;
        graph.add_edge_weighted("a", "d", 160)?;
        graph.add_edge_weighted("b", "c", 120)?;
        graph.add_edge_weighted("b", "d", 180)?;
        graph.add_edge_weighted("c", "e", 80)?;
        graph.add_edge_weighted("d", "c", 40)?;
        graph.add_edge_weighted("d", "e", 140)?;
        graph.add_edge_weighted("e", "b", 100)?;

        let expected = HashSet::from([
            Path {
                from: "a",
                to: "a",
                path: vec!["a"],
                distance: 0,
            },
            Path {
                from: "a",
                to: "b",
                path: vec!["a", "b"],
                distance: 100,
            },
            Path {
                from: "a",
                to: "c",
                path: vec!["a", "d", "c"],
                distance: 200,
            },
            Path {
                from: "a",
                to: "d",
                path: vec!["a", "d"],
                distance: 160,
            },
            Path {
                from: "a",
                to: "e",
                path: vec!["a", "d", "c", "e"],
                distance: 280,
            },
        ]);
        let actual = graph.find_shortest_paths("a")?;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_backtrack() {
        let successors = vec![
            ("d", "e"),
            ("f", "g"),
            ("b", "c"),
            ("c", "d"),
            ("h", "i"),
            ("e", "f"),
            ("b", "c"),
            ("a", "b"),
            ("g", "h"),
        ];

        let expected = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        let actual = Graph::<&str, usize>::backtrack(&"a", &"i", &successors);
        assert_eq!(actual, expected);

        let expected = vec!["c", "d", "e", "f", "g"];
        let actual = Graph::<&str, usize>::backtrack(&"c", &"g", &successors);
        assert_eq!(actual, expected);

        let expected = vec!["a"];
        let actual = Graph::<&str, usize>::backtrack(&"a", &"a", &successors);
        assert_eq!(actual, expected);
    }
}
