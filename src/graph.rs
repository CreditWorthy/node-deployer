use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

impl LatLon {
    fn to_string(&self) -> String {
        todo!()
    }

    pub fn parse(input: &str) -> Result<LatLon, String> {
        match input.split(',').collect::<Vec<_>>().as_slice() {
            [lat, long] => {
                // correct case
                return Ok(LatLon {
                    lat: lat.parse().map_err(|e| format!("{}", e))?,
                    lon: long.parse().map_err(|e| format!("{}", e))?,
                });
            }
            _ => return Err("incorrect format".to_string()), // ()
        };
    }
}

//    |
//  - . -

struct X(i64, u8, String);


// newtype pattern in rust
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct NodeID(pub i64); // struct tuple, with only one element

// &NodeID is 8 bytes (underlying is pointer: usize which 64bit )

// directed graph (undirected)
pub struct Graph {
    // for every node, its adjacent edges
    // implementation detail
    adj_edges: HashMap<NodeID, Vec<Edge>> // node id -> list of edge id
}

impl Graph {
    // constructor function
    pub fn new(adj_edges: HashMap<NodeID, Vec<Edge>>) -> Self {
        Graph {
            adj_edges 
        }
    } 
}

// if a type is not `Copy`-able, it's moved when assign or passed as parameter to function

// &Vec<T> vs. &[T] // slice of T 
// &[T] vs. [T] // [T] a contingous region of memeory: Dynamic sized type, compiler don't know their size at compile-time
// &[T] // fixed sized type, which is just 16bytes: 8bytes for length of slice, 8bytes for pointer to memory region.
impl Graph {
    pub fn for_each_node(&self) -> impl Iterator<Item = &NodeID>{
        self.adj_edges.keys()
    }
    pub fn adjacent_edges(&self, nodeId : NodeID) -> Option<&Vec<Edge>> /* Option<&[Edge]> */ { // zero length
        self.adj_edges.get(&nodeId)
    }
}

pub struct Node {
    pub id : NodeID,
    pub location : LatLon
}

#[derive(Clone)]
struct EdgeID(i64);

// only travel from <from_node> to <to_node>
pub struct Edge {
    // id: EdgeID,
    pub from_node: NodeID,
    pub to_node: NodeID,
}
