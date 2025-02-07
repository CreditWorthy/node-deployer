use core::f64;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, hash::Hash};
use rstar::Point;
use serde::{Deserialize, Serialize};

use crate::engine::EngineErrors;

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


// newtype pattern in rust
#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct NodeID(pub i64); 
pub struct Graph {
    adj_edges: HashMap<NodeID, Vec<Edge>>,
    nodes: HashMap<NodeID, Node>
}

impl Graph {
    // constructor function
    pub fn new(adj_edges: HashMap<NodeID, Vec<Edge>>, nodes: HashMap<NodeID, Node>) -> Self {
        Graph {
            adj_edges,
            nodes
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
    pub fn get_latlon(&self, nodeId : NodeID) -> Option<LatLon> {
        self.nodes.get(&nodeId).map(|n| n.location)

        // match self.nodes.get(&nodeId) {
        //     Some (x ) => {
        //         Some(x.location)
        //     }
        //     None => {None}
        // }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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


pub struct NoRouteFound; // equivalent to ()

// () just means "no route found". Of course, a better method is to define a specifc type for this.
pub fn shortest_path(g:&Graph, s: NodeID, t: NodeID) -> Result<(f64, Vec<NodeID>), NoRouteFound> {
    // find out all shortest path to all nodes
    
    // tentative 
    // dist: node id v -> tentative shortest distance to v from start node s.
    // prev: node id v -> previous node id w, through which we can reach to v, has the shortest distance. 
    //       used to recover the path detail (all nodes passed on shortest path)

    // PartialOrd trait: f64 indeed implemented
    // f64::Nan

    let mut dist:HashMap<NodeID, f64> = HashMap::new();
    let mut prev:HashMap<NodeID, NodeID> = HashMap::new();

    // priority queue: allows us to query the node with the "tentative" shortest distance (fast)
    // pq . find_smallest() -> related node/item which has the smallest value.
    let mut pq:BinaryHeap<PQItem> = BinaryHeap::new();

    // float number: Nan - not a number
    // f64::NAN > 1.0; // compare it with other f64

    // wikipedia version is to find shortest paths to all nodes.
    // 3      for each vertex v in Graph.Vertices:
    // 4          dist[v] ← INFINITY
    // 5          prev[v] ← UNDEFINED
    // 6          add v to Q
    // 7      dist[source] ← 0

    // 9      while Q is not empty:
    // 10          u ← vertex in Q with minimum dist[u]
    // 11          remove u from Q
    // 12         
    // 13          for each neighbor v of u still in Q:
    // 14              alt ← dist[u] + Graph.Edges(u, v)
    // 15              if alt < dist[v]:
    // 16                  dist[v] ← alt
    // 17                  prev[v] ← u
    // 18

    for &node in g.for_each_node() {
        // dist.insert(node, f64::INFINITY); // or just leave it alone. because if some node not exist in dist, it means infinite
        // prev.insert(node, None);
        pq.push(PQItem{id: node, distance: f64::INFINITY });
    }
    dist.insert(s, 0.0);
    pq.push(PQItem { id: s, distance: 0.0 });

    let mut count = 0;
    while let Some(item) = pq.pop() {
        count+=1;
        if count % 10000 ==0 {
            println!("visited {} nodes", count);
        }

        let u = item.id;
        if u == t {
            println!("=== target found: build path");
            let mut path = Vec::new();
            let mut current = t;

            // prev: t -> v -> u -> v 
            while current != s { // loop until None (the last node (start node) has no prev node )
                path.push(current);
                current = prev[&current];
            }

            path.push(s);
            path.reverse();

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

                if dist.contains_key(&v) {
                    // indeed there is already some other path to `v` (not through `u`).
                   // compare distances of that other path (not through `u`) and current path to `v` through `u`.
                    if dist_to_v_through_u < dist[&v] {
                        dist.insert(v, dist_to_v_through_u);
                        prev.insert(v, u);
                        pq.push(PQItem{ distance: dist_to_v_through_u, id: edge.to_node });
                    } else {
                        // do nothing here. no need to update with a worse path.
                    }
                } else {
                    // no existing path to `v` found yet.
                    dist.insert(v, dist_to_v_through_u);
                    prev.insert(v, u);
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