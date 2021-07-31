use std::{
    collections::{HashSet, HashMap},
    hash::Hash,
};

type NodeIndex = usize;
type EdgeIndex = usize;

struct NodeData<T> {
    data: T,
    succ: Vec<NodeIndex>,
}

#[derive(Default)]
pub struct Graph<I, T> {
    nodes: Vec<NodeData<T>>,
    ids: HashMap<I, NodeIndex>,
    num_edges: usize,
}

impl<I, T> Graph<I, T>
    where I: Hash + Eq
{
    pub fn get(&self, index: NodeIndex) -> &T {
        &self.nodes.get(index).unwrap().data
    }

    pub fn add_node_unchecked(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData{data, succ: vec![]});
        index
    }

    // uses an optional hashable ID to make sure nothing is added twice
    pub fn add_node(&mut self, data: T, id: I) -> NodeIndex {
        match self.ids.get(&id) {
            Some(index) => {
                // already exists, just give them the index
                *index
            },
            None => {
                let index = self.nodes.len();
                self.nodes.push(NodeData{data, succ: vec![]});
                self.ids.insert(id, index);
                index
            }
        }
    }

    pub fn add_edge_unchecked(&mut self, from: NodeIndex, to: NodeIndex) {
        let node: &mut NodeData<T> = self.nodes.get_mut(from).unwrap();
        node.succ.push(to);
        self.num_edges += 1;
    }

    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    // pub fn add_bidirection(&mut self, a: NodeIndex, b: NodeIndex) {
    //     let first = self.add_edge(a, b);
    //     let second = self.add_edge(b, a);
    // }

    // this is not ordered by edge
    pub fn nodes(&self) -> impl Iterator<Item = &T> {
        self.nodes.iter().map(|n| &n.data)
    }

    pub fn successors(&self, node: NodeIndex) -> impl Iterator<Item = &NodeIndex> {
        self.nodes
            .get(node)
            .unwrap()
            .succ
            .iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        NodeIterator {
            graph: &self,
            visited: HashSet::new(),
            to_visit: vec![0]
        }
    }
}

struct NodeIterator<'a, I, T> {
    graph: &'a Graph<I, T>,
    visited: HashSet<usize>,
    to_visit: Vec<usize>,
}

impl<'a, I, T> Iterator for NodeIterator<'a, I, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.to_visit.pop() {
            Some(index) => {
                let data = self.graph.nodes.get(index)?;
                for succ in data.succ.iter() {
                    if self.visited.get(succ).is_none() {
                        self.to_visit.push(*succ);
                    }
                }
                self.visited.insert(index);
                Some(&data.data)
            },
            None => None
        }
    }
}