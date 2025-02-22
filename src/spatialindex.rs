use std::collections::HashMap;

use rstar::{primitives::GeomWithData, RTree};

use crate::{graph::Node, osm};


// make it a real type
pub struct SpatialIndex(RTree<NodeLocation>);


pub type NodeLocation = GeomWithData<[f64; 2], osm::NodeID>;


impl SpatialIndex {
    pub fn build(used_nodes: &HashMap<osm::NodeID, Node>) -> Self {
        let node_locations = used_nodes
        .iter()
        .map(|(nodeId, node)| {
            NodeLocation::new([node.location.lon, node.location.lat], *nodeId)
        })
        .collect();
        let tree = RTree::bulk_load(node_locations);

        SpatialIndex(tree)
    }

    // todo
    pub fn nearest_neighbor () {

    }
}
