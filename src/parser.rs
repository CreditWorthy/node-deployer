
// unit test

// strong type: type-level programming

use std::{collections::HashMap, ptr::read};

use osmpbf::{Element, ElementReader};
use crate::graph::{Graph, LatLon, Node, NodeID};

enum ParseError {
    FileCorrupt,
    DataIntegrityError,
}

// the user/caller of this function can tell what errors happens
fn parse_map(map_file:&str) -> Result<Graph, ParseError> {
    let reader = ElementReader::from_path("./data/delaware-latest.osm.pbf").unwrap();

    let nodes = HashMap::new();

    let adj_nodes: HashMap<NodeID, Vec<NodeID>>= HashMap::new();
    // count neighbor node number of every node, if the number is 2, then it's shape node, not useful for routing.

    let ways: HashMap<i64, Vec<i64>> = HashMap::new(); // convert way to edges of Graph.
    reader.for_each(|element| {
        match element {
            Element::Node ( node  ) => {
                let node_object = Node {
                    id: NodeID(node.id()),
                    location: LatLon {
                        lat: node.lat(),
                        lon: node.lon()
                    }
                };

                nodes.insert(node_object.id, node_object);
            }

            Element::Way( way ) => {
                // iterate all nodes in way
                // and store node's neighbor into adj_nodes

                // way: a list of node is, [a, b, c, d, ...]
                // store a->b, b->c, c->d in the adj_nodes data.

                // 
                // multiple ways:
                // X: [a, b, c, d]
                // Y: [a, e, f]
                // Z: [a, g]
                let all_way_nodes:Vec<_> = way.refs().map(|node_id| NodeID(node_id)).collect();
                for curr in 0..all_way_nodes.len() {
                    let next = curr+1;
                    let curr_node_id = all_way_nodes[curr];
                    let next_node_id = all_way_nodes[next];

                    // a -> [b] for X
                    // a -> [b e] after Y
                    // a -> [b e g] after Z
                    adj_nodes.insert(curr, next_node_id);
                }

            }

            _ => {

            }
        }
    });
}

// only need compilation when cargo test
#[cfg(test)] // conditional compilation
mod tests {
    use osmpbf::{Element, ElementReader};

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
}
