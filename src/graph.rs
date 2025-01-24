use std::collections::HashMap;
use rstar::Point;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Node {
    pub id : NodeID,
    pub location : LatLon
}



// // graph familiy 2
// struct Node2;
// struct Edge2;
// struct Graph2;

// // graph familiy 3
// struct Node3;
// struct Edge3;
// struct Graph3; // GraphGeneric<Node3, Edge3>  generic type instantiation

// // GraphGeneric<Node2, Edge3>

// // trait's associated type

// struct GraphGeneric<N, E>{nodes:Vec<N>, edges:Vec<E>}

// // impl GraphTrait for Graph2 {

// // }

// trait GraphTrait {
//     type Node;
//     type Edge;

//     fn all_nodes(&self) -> Vec<Self::Node>;
// }

// impl GraphTrait for Graph3 {
//     type Node = Node3;
//     type Edge = Edge3;

//     fn all_nodes(&self) -> Vec<Self::Node> {
//         todo!()
//     }
// }

// // 1-to-1 relation between Node2/Edge2/Graph2
// impl GraphTrait for Graph2 {
//     type Node = Node2;
//     type Edge = Edge2;

//     fn all_nodes(&self) -> Vec<Self::Node> {
//         todo!()
//     }
// }

// // generic + trait

// // trait bound
// fn generic_func<G: GraphTrait>(g: G) {
//     // what to do with `t`?
//     let nodes = g.all_nodes();
    
//     // Vec<<G as GraphTrait>::Node>
//     // compiler doesn't know the concrete Node type here.
// }

// fn generic_func2(g: GraphGeneric<N, E>) {
    
// }



// // #[test]
// // fn test_generic() {
// // //     let gx= GraphGeneric{nodes: Vec::<Node2>::new(), edges:Vec::<Edge3>::new()};
// // //    generic_func(gx);
// // }

#[derive(Clone)]
struct EdgeID(i64);

// only travel from <from_node> to <to_node>
pub struct Edge {
    // id: EdgeID,
    pub from_node: NodeID,
    pub to_node: NodeID,
}
