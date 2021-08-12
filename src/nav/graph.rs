use std::{
    collections::{HashMap},
};

type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(Default)]
pub struct Graph<T> {
    pub nodes: Vec<T>,
    pub edges: HashMap<NodeIndex, Vec<NodeIndex>>,
    num_edges: usize,
}

impl<T> Graph<T> {
    pub fn get(&self, index: NodeIndex) -> Option<&T> {
        self.nodes.get(index)
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(data);
        index
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.edges
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
        self.num_edges += 1;
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    pub fn nodes_iter(&self) -> impl Iterator<Item = &T> {
        self.nodes.iter()
    }

    pub fn succ(&self, node: NodeIndex) -> Option<&Vec<NodeIndex>> {
        self.edges.get(&node)
    }

    pub fn edges(&self) -> Vec<(&T, &T)> {
        self.edges
            .iter()
            .fold(vec![], |mut acc, (k, v)| {
                let from = self.nodes.get(*k).unwrap();
                if let Some(succ) = self.edges.get(k) {
                    for to_inx in succ {
                        let to = self.nodes.get(*to_inx).unwrap();
                        acc.push((from, to));
                    }
                }
                acc
            })
    }
}