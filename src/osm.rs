#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct NodeID(pub i64); 



// a -> b -> c -> d
//   d1   d2   d3
pub struct Way {
    pub nodes: Vec<NodeID>,
    pub distances: Vec<f64>,
}

