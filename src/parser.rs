use std::{
    collections::{HashMap, HashSet},
    error::{self, Error},
    fmt::Debug,
    ptr::read,
};

use crate::graph::{self, Edge, Graph, LatLon, Node, NodeID};
use geo::{Distance, Haversine, Point};
use osmpbf::{Element, ElementReader};
use rstar::{primitives::GeomWithData, RTree};

pub struct HighlevelError;

#[derive(Debug)]
pub enum ParseError {
    Simple,
    OSMPBFError(osmpbf::Error),
}

// correct usage for From
// when you do this, the compiler will convert ParseError to HighlevelError automatically in ? context.
impl From<ParseError> for HighlevelError {
    fn from(value: ParseError) -> Self {
        HighlevelError
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Simple => f.write_str("some simple error"),
            ParseError::OSMPBFError(e) => f.write_str(&format!("underlying osmbpf error: {}", e)),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<osmpbf::Error> for ParseError {
    fn from(value: osmpbf::Error) -> Self {
        ParseError::OSMPBFError(value)
    }
}

pub type SpaitialIndex = RTree<NodeLocation>;

// external type
pub type NodeLocation = GeomWithData<[f64; 2], NodeID>;

// our defined type
// newtype pattern
pub struct NodeLocation2(GeomWithData<[f64; 2], NodeID>);

pub fn parse_map(map_file: &str) -> Result<(Graph, RTree<NodeLocation>), ParseError> {
    let mut all_nodes: HashMap<NodeID, Node> = HashMap::new();
    let mut adj_edges: HashMap<NodeID, Vec<Edge>> = HashMap::new();
    let ways: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut node_count = 0;

    // 2 pass of pbf parse
    // 1st pass is to collect all locations of nodes
    // 2nd pass : generate edges from ways

    let reader = ElementReader::from_path(map_file).unwrap();
    reader.for_each(|element| match element {
        Element::DenseNode(node) => {
            let node_object = Node {
                id: NodeID(node.id()),
                location: LatLon {
                    lat: node.lat(),
                    lon: node.lon(),
                },
            };

            all_nodes.insert(node_object.id, node_object);
        }

        _ => {}
    })?;

    let mut all_way_count = 0;
    let mut highway_count = 0;
    let mut used_nodes = HashSet::new(); // only care about node id

    let reader2 = ElementReader::from_path(map_file).unwrap();
    reader2
        .for_each(|element| {
            match element {
                Element::Way(way) => {
                    all_way_count += 1;
                    // quality of osm map data is not very high. sometime, because road properties (wrong tags) will cause the map divied into muliple parts.

                    // filter out all ways without "highway" tag
                    if way.tags().find(|(k, _)| *k == "highway").is_none() {
                        return; // skip this way
                    }

                    highway_count += 1;

                    let all_way_nodes: Vec<_> = way.refs().map(|node_id| NodeID(node_id)).collect();
                    for curr in 0..all_way_nodes.len() - 1 {
                        let next = curr + 1;
                        let curr_node_id = all_way_nodes[curr];
                        let next_node_id = all_way_nodes[next];

                        let (from_location, to_location) =
                            match (all_nodes.get(&curr_node_id), all_nodes.get(&next_node_id)) {
                                (Some(from), Some(to)) => (from, to),
                                _ => return,
                            };

                        // let from_location = all_nodes.get(&curr_node_id).unwrap(); // risk!
                        // let to_location = all_nodes.get(&next_node_id).unwrap(); //

                        let from_point =
                            Point::new(from_location.location.lon, from_location.location.lat);
                        let to_point =
                            Point::new(to_location.location.lon, from_location.location.lat);
                        let distance = Haversine::distance(from_point, to_point);

                        // how to get location (latlon) for curr_node and next_node

                        // locations.get_location_by_node_id(curr_node)

                        let forward_edge = Edge {
                            from_node: curr_node_id,
                            to_node: next_node_id,
                            distance,
                        };

                        let reverse_edge = Edge {
                            from_node: next_node_id,
                            to_node: curr_node_id,
                            distance,
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

                    for n in way.refs() {
                        used_nodes.insert(NodeID(n));
                    }
                }

                _ => {}
            }
        })
        .map_err(|e| ParseError::OSMPBFError(e))?;

    println!("all ways: {}, highway: {}", all_way_count, highway_count);

    let node_locations = used_nodes
        .into_iter()
        .map(|nodeId| {
            let node = all_nodes.get(&nodeId).unwrap(); // safe!
            NodeLocation::new([node.location.lon, node.location.lat], nodeId)
        })
        .collect();

    let tree = RTree::bulk_load(node_locations);
    let graph = Graph::new(adj_edges, all_nodes);
    Ok((graph, tree))
}

#[cfg(test)] // conditional compilation
mod tests {
    use osmpbf::{Element, ElementReader};

    use crate::graph::shortest_path;

    use super::parse_map;

    #[test]
    fn test_parse() {
        let reader = ElementReader::from_path("./data/delaware-latest.osm.pbf").unwrap();
        let mut ways = 0_u64;

        reader
            .for_each(|element| {
                if let Element::Way(_) = element {
                    ways += 1;
                }
            })
            .unwrap();

        println!("Number of ways: {ways}");
    }

    #[test]
    fn test_parsed_map() {
        let (graph, tree) = parse_map("./data/delaware-latest.osm.pbf").unwrap();

        // let mut total_nodes = 0;
        // let mut total_edges = 0;

        let start_location = [-75.382757, 38.692588];
        let target_location = [-75.384656, 38.696308];

        let start_node = tree.nearest_neighbor(&start_location).unwrap();
        let target_node = tree.nearest_neighbor(&target_location).unwrap();
        println!("Start ID: {:?}", start_node.data);
        println!("Target ID: {:?}", target_node.data);

        let start_node_edges = graph.adjacent_edges(start_node.data).unwrap();
        for edge in start_node_edges {
            println!("To Node {}", edge.to_node.0)
        }

        let (dist, path) = shortest_path(&graph, start_node.data, target_node.data).unwrap();
        println!("Distance: {}", dist);
        println!("Path: {:?}", path);

        // for node_id in graph.for_each_node() {
        //     total_nodes += 1;
        //     if let Some(edges) = graph.adjacent_edges(*node_id) {
        //         total_edges += edges.len();
        //     }
        // }

        // println!("Total nodes: {}", total_nodes);
        // println!("Total edges: {}", total_edges);
    }
}
