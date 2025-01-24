
// unit test

// strong type: type-level programming

use std::{collections::HashMap, fmt::Debug, ptr::read};

use osmpbf::{Element, ElementReader};
use rstar::{primitives::GeomWithData, RTree};
use crate::graph::{self, Edge, Graph, LatLon, Node, NodeID};

#[derive(Debug)]
enum ParseError {
    OSMPBFError(osmpbf::Error) // keep the original error inside
}

// cargo expand: you can see the code after macro expansion

// impl Debug for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::OSMPBFError(arg0) => f.debug_tuple("OSMPBFError").field(arg0).finish(),
//         }
//     }
// }

type NodeLocation = GeomWithData<[f64; 2], NodeID>;
// the user/caller of this function can tell what errors happens
fn parse_map(map_file:&str) -> Result<(Graph, RTree<NodeLocation>), ParseError> {
    let reader = ElementReader::from_path(map_file).unwrap();

    let mut nodes = HashMap::new();

    // key: from node id
    // values: edges has the key as from node
    let mut adj_edges: HashMap<NodeID, Vec<Edge>> = HashMap::new();

    // let adj_nodes: HashMap<NodeID, Vec<NodeID>>= HashMap::new();
    // count neighbor node number of every node, if the number is 2, then it's shape node, not useful for routing.

    let ways: HashMap<i64, Vec<i64>> = HashMap::new(); // convert way to edges of Graph.
    
    // let mut tree = RTree::new();

    let mut node_locations = Vec::new();



    let mut node_count = 0;
    reader.for_each(|element| {
        match element {
            Element::DenseNode ( node  ) => {
                let node_object = Node {
                    id: NodeID(node.id()),
                    location: LatLon {
                        lat: node.lat(),
                        lon: node.lon()
                    }
                };

                node_count+=1;
                if node_count % 100000==0 {
                    println!("== sample node: {} {}", node.lon(), node.lat());
                }

                // lat -> y, lon -> x,
                // spehre coordinate (lat/lon), 
                // 2d coordinate (y/x)
                // computation intensive

                node_locations.push(NodeLocation::new( [ node.lon(), node.lat() ], NodeID(node.id()) ));

                nodes.insert(node_object.id, node_object);
            }

            Element::Way( way ) => {
                // iterate all nodes in way
                // and store node's neighbor into adj_nodes

                // way: a list of node is, [a, b, c, d]
                // edges: a->b, a<-b, b->c, b<-c, c->d, c<-d 

                // a osm way: oneway (one direction) or twoway (bi directional) // assume all ways are two way
                
                let all_way_nodes:Vec<_> = way.refs().map(|node_id| NodeID(node_id)).collect();
                for curr in 0..all_way_nodes.len() - 1 {
                    let next = curr+1;
                    let curr_node_id = all_way_nodes[curr];
                    let next_node_id = all_way_nodes[next];

                    let forward_edge = Edge {
                        from_node: curr_node_id,
                        to_node: next_node_id,
                    };

                    let reverse_edge = Edge {
                        from_node: next_node_id,
                        to_node: curr_node_id,
                    };

                    adj_edges
                        .entry(curr_node_id)
                        .or_insert_with(Vec::new)
                        .push(forward_edge);

                    adj_edges
                        .entry(next_node_id)
                        .or_insert_with(Vec::new)
                        .push(reverse_edge);
                }

            }

            _ => {

            }
        }
    }).map_err(|e| ParseError::OSMPBFError(e))?;

    let tree =  RTree::bulk_load(node_locations);


    let graph = Graph::new(adj_edges);
    Ok((graph, tree))
    // build graph
}

// only need compilation when cargo test
#[cfg(test)] // conditional compilation
mod tests {
    use osmpbf::{Element, ElementReader};

    use super::parse_map;

    #[test]
    fn test_parse() {
        let reader = ElementReader::from_path("./data/delaware-latest.osm.pbf").unwrap();
        let mut ways = 0_u64;

        reader.for_each(|element| {
            if let Element::Way(_) = element {
                ways += 1;
            }
        }).unwrap();

        println!("Number of ways: {ways}");
    }

    #[test]
    fn test_parsed_map() {
        // destructuring syntax
        // pattern matching: everywhere
        let (graph, tree) = parse_map("./data/delaware-latest.osm.pbf").unwrap();

        let mut total_nodes = 0;
        let mut total_edges = 0;

        let my_location = [-75.384600, 38.691489];

        let nearest_node = tree.nearest_neighbor(&my_location).unwrap();
        println!("Node ID: {:?}", nearest_node.data);

        // {} : type implements std::fmt::Display trait
        // {:?}: type .. std::fmt::Debug / Debug

        for node_id in graph.for_each_node() {
            total_nodes += 1;
            if let Some(edges) = graph.adjacent_edges(*node_id) {
                total_edges += edges.len();
            }
        }

        println!("Total nodes: {}", total_nodes);
        println!("Total edges: {}", total_edges);
    }
}
