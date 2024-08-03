use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone, Debug)]
pub struct Edge<T, W>
where
    T: PartialEq + Eq + Hash + Clone,
    W: Ord + Clone,
{
    source: T,
    destination: T,
    weight: W,
}

#[derive(Default, Clone, Debug)]
pub struct Graph<T, W>
where
    T: PartialEq + Eq + Hash + Clone,
    W: Ord + Clone,
{
    vertices: HashSet<T>,
    edges: Vec<Edge<T, W>>,
}
impl<T, W> Graph<T, W>
where
    T: PartialEq + Eq + Hash + Clone,
    W: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_vertex(&mut self, vertex: T) -> &mut Self {
        self.vertices.insert(vertex);
        self
    }
    pub fn add_edge(&mut self, source: T, destination: T, weight: W) -> &mut Self {
        self.edges.push(Edge {
            source,
            destination,
            weight,
        });
        self
    }
    pub fn kruskal(&mut self) -> Vec<Edge<T, W>> {
        self.edges.sort_by(|a, b| a.weight.cmp(&b.weight));
        let mut connected_vertices = HashSet::new();
        let mut isolated_groups = HashMap::new();
        let mut stanning_edges = Vec::new();
        for edge in self.edges.iter() {
            if !connected_vertices.contains(&edge.source)
                || !connected_vertices.contains(&edge.destination)
            {
                if !connected_vertices.contains(&edge.source)
                    && !connected_vertices.contains(&edge.destination)
                {
                    isolated_groups.insert(&edge.source, vec![&edge.source, &edge.destination]);
                    isolated_groups
                        .insert(&edge.destination, isolated_groups[&edge.source].clone());
                } else {
                    if !isolated_groups.contains_key(&edge.source) {
                        isolated_groups
                            .entry(&edge.destination)
                            .or_insert(Vec::new())
                            .push(&edge.source);
                        isolated_groups
                            .insert(&edge.source, isolated_groups[&edge.destination].clone());
                    } else {
                        isolated_groups
                            .entry(&edge.source)
                            .or_insert(Vec::new())
                            .push(&edge.destination);
                        isolated_groups
                            .insert(&edge.destination, isolated_groups[&edge.source].clone());
                    }
                    stanning_edges.push(edge.clone());
                    connected_vertices.insert(&edge.source);
                    connected_vertices.insert(&edge.destination);
                }
            }
        }
        for edge in self.edges.iter() {
            if let Some(group) = isolated_groups.clone().get(&edge.source) {
                if group.contains(&&edge.source) && !group.contains(&&edge.destination) {
                    stanning_edges.push(edge.clone());
                    let dest_group = isolated_groups[&edge.destination].clone();
                    isolated_groups
                        .entry(&edge.source)
                        .or_insert(Vec::new())
                        .extend(dest_group);
                    isolated_groups
                        .entry(&edge.destination)
                        .or_insert(Vec::new())
                        .extend(group);
                }
            }
        }
        stanning_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: [[i32; 3]; 9] = [
        [2, 3, 26],
        [3, 4, 3],
        [4, 6, 19],
        [6, 1, 22],
        [1, 5, 14],
        [5, 2, 22],
        [2, 1, 13],
        [1, 4, 17],
        [3, 1, 18],
    ];
    #[test]
    fn test_kruskal() {
        let mut graph = Graph::new();
        for i in 1..=6 {
            graph.add_vertex(i);
        }
        for data in TEST_DATA {
            graph.add_edge(data[0], data[1], data[2]);
        }
        let edges = graph.kruskal();
        assert_eq!(edges.len(), graph.vertices.len() - 1)
    }
}
