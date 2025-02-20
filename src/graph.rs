use core::f64;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, f64::INFINITY, hash::Hash};
use rstar::Point;
use serde::{Deserialize, Serialize};

use crate::{engine::EngineErrors, osm};

// depth-first search
// breadth-first search
// shortest-path algorithm: Dijkstra (given two node s = start/source, t = target, shortest: summation of all weights on edges along the path, find the smallest one)

// 25 which is 5 groups , each group 5 studuents
// group 1: a b c d e
// group 2: f g h i j
// group 3: k l m n o
// group 4: p q r s t
// group 5: u v w x y

// 1st step:
//   we have 5 run: to find the ordering of each group 
// 2nd step:
//   let 5 (a f k p u) to run: a is fastest, f is 2nd place (it maybe not the 2nd in 25 students)
// 3 step: find out 2nd in total/25 studuent:
//   let 5 (b f k p u) to run again: b is fastest , no other can be fastest.
// 4 step: find the 3rd in total:
//   c f k p u 

// total orderings of 25 students
// think the track (5 lanes) as a priority queue with limited capacity

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


// newtype pattern in rus
pub struct Graph {
    adj_edges: HashMap<NodeID, Vec<Edge>>,
    // nodes: HashMap<NodeID, Node>,
    nodes2: Vec<Node>,
    node_id_map: HashMap<osm::NodeID, NodeID> // mapping: external osm nodeid -> internal graph node id (which is just index)
}

impl Graph {
    // constructor function
    pub fn new(adj_edges: HashMap<NodeID, Vec<Edge>>, nodes: HashMap<NodeID, Node>) -> Self {

        let nodes2 = nodes.into_values().collect::<Vec<Node>>(); // turbo-fish
        let mut node_id_map = HashMap::new();
        let mut next_index = 0;
        for n in &nodes2 {
            node_id_map.insert(n.id, NodeID(next_index));
            next_index+=1;
        }

        Graph {
            adj_edges,
            // nodes,
            nodes2,
            node_id_map,
        }
    } 
}

impl Graph {
    pub fn for_each_node(&self) -> impl Iterator<Item = &NodeID>{
        self.adj_edges.keys()
    }
    pub fn adjacent_edges(&self, nodeId : NodeID) -> Option<&Vec<Edge>> /* Option<&[Edge]> */ { // zero length
        self.adj_edges.get(&nodeId)
    }
    // distinguish external osm id vs internal index-based id.
    pub fn get_latlon(&self, nodeId : NodeID) -> Option<LatLon> {
        Some(self.nodes2[nodeId.0].location) // or better change the return type to just LatLong intead of Option<LatLon>.

        // match self.nodes.get(&nodeId) {
        //     Some (x ) => {
        //         Some(x.location)
        //     }
        //     None => {None}
        // }
    }

    pub fn get_total_nodes(&self) -> usize {
        return self.nodes2.len()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Node {
    pub id : osm::NodeID,
    pub location : LatLon
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct NodeID(pub usize); 

#[derive(Clone)]
struct EdgeID(i64);

// only travel from <from_node> to <to_node>
pub struct Edge {
    // id: EdgeID,
    pub from_node: NodeID,
    pub to_node: NodeID,
    pub distance: f64,
}

#[derive(Debug, PartialEq)]
struct PQItem {
    id: NodeID,
    distance: f64,
}

impl Ord for PQItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // will return none if comparing f64::Nan with some other f64
        // self.distance.partial_cmp(&other.distance).unwrap() if self > other return Some(Greater)
        other.distance.partial_cmp(&self.distance).unwrap() // unwrap() panic on the None
    }
}

impl PartialOrd for PQItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Eq for PQItem {}


#[derive(Debug)]
pub struct NoRouteFound; // equivalent to ()

pub fn shortest_path(g:&Graph, s: NodeID, t: NodeID) -> Result<(f64, Vec<NodeID>), NoRouteFound> {
    // let mut dist:HashMap<NodeID, f64> = HashMap::new();
    // let mut prev:HashMap<NodeID, NodeID> = HashMap::new();
    let mut dist = vec![f64::INFINITY; g.get_total_nodes()]; // indexed by internal `NodeID`
    let mut prev = vec![Option::<NodeID>::None; g.get_total_nodes()];

    let mut pq:BinaryHeap<PQItem> = BinaryHeap::new();
    dist.insert(s.0, 0.0);
    pq.push(PQItem { id: s, distance: 0.0 });
    while let Some(item) = pq.pop() { //
        let u = item.id;
        if u == t {
            let mut path = Vec::new();
            let mut current = t;

            // prev: t -> v -> u -> v 
            while current != s { // loop until None (the last node (start node) has no prev node )
                path.push(current);
                match prev[current.0] {
                    Some(prev) => current = prev,
                    None => panic!("bug happens: node {} does not have a previous node", current.0),
                }
            }

            path.push(s);
            path.reverse();

            // println!("=== path build finish");

            // dist[t]
            return Ok(( item.distance, path )) 
        }

        let dist_to_u = item.distance; // distance from `s` to `u`
        
        // pop out node is: u
        if let Some(edges) = g.adjacent_edges(u) {
            for edge in edges {
                let v = edge.to_node;
                
                // u's adjacent edges.
                // for each adj edge, edge.to_node should be: v
                // at least there is a path to v through u: s -> ... -> u -> v.
                // for that path, the total cost/distance from start node `s` to `v` is: distance to u + edge (u -> v) distance 
                let dist_to_v_through_u = dist_to_u + edge.distance;

                // dist[&v]: distance to `v` throught some other ways/path already exist

                // Two cases:
                // 1. dist_to_v_through_u >. 
                

                if dist[v.0] < f64::INFINITY {
                    // indeed there is already some other path to `v` (not through `u`).
                   // compare distances of that other path (not through `u`) and current path to `v` through `u`.
                    if dist_to_v_through_u < dist[v.0] {
                        dist[v.0] = dist_to_v_through_u; 
                        prev[v.0] = Option::Some(u);
                        // dist.insert(v, dist_to_v_through_u);
                        // prev.insert(v, u);
                        pq.push(PQItem{ distance: dist_to_v_through_u, id: edge.to_node });
                    } else {
                        // do nothing here. no need to update with a worse path.
                    }
                } else {
                    // no existing path to `v` found yet.
                    dist[v.0] = dist_to_v_through_u; 
                    prev[v.0] = Option::Some(u);

                    pq.push(PQItem{ distance: dist_to_v_through_u, id: edge.to_node });
                }
            }
        }
    }

    



    // dist[start] = 0
    // prev[start] = None

    // initial:
    // we can assume/fill the infinite as the tentative shortest distance to all other nodes (except start node)
    // prev[x] = None


    


    Err(NoRouteFound)
    // in each step: find the shortest path to some node.
}

#[test]
fn test_f64() {
    println!("{:?}", f64::INFINITY.partial_cmp(&0.5));
    println!("{:?}", 0.5.partial_cmp(&f64::INFINITY));
}

#[test]
fn test_pq() {
    let mut pg : BinaryHeap<PQItem> = BinaryHeap::new();
    pg.push(PQItem{
        id: NodeID(1),
        distance: 1.0
    });

    pg.push(PQItem{
        id: NodeID(2),
        distance: 5.0
    });

    pg.push(PQItem{
        id: NodeID(3),
        distance: 2.0
    });

    println!("{:?}", pg.pop());
    println!("{:?}", pg.pop());
    println!("{:?}", pg.pop());
}