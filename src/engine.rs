use std::error::Error;

use crate::{graph::{shortest_path, Graph, LatLon, NodeID}, parser::{parse_map, NodeLocation, SpaitialIndex}};


pub struct Engine {
    spaitial_index: SpaitialIndex,
    graph: Graph,
}

// Error is a trait, not a concrete type.
// But Result<_, E> E should be type
// The problem: we don't know the type yet (or we just want it to be flexible, any type implemented Error is ok)
// use "dyn Error": any type implemented Error trait.
// So if it's any type, then we don't know its size.

// trait object (term): dynamic dispatch

// static dispatch (generic + trait)
// 1 function for 1 conrecte type T
// adv: perf better
// downside: binary size
fn test_static<T>(t:T) where T:Trait1 {
    t.method1();
}

// dynamic dipatch
// only 1 function for all type T: Trait1
// perf worse: virtual table 

// &dyn Trait1: special pointer/referece (fat pointer) (contains metadata about the Type: virtual table: find the concrete function impl)
// diff from: &String/ &Custom: simple pointer / unsize
fn test_dyn(t: &dyn Trait1)  {
    t.method1();
}

trait Trait1 {
    fn method1(&self);
}

impl Trait1 for String {
    fn method1(&self) {
        ///////////
        todo!()
    }
}

impl Trait1 for u8 {
    fn method1(&self) {
        /// xxxxx
        todo!()
    }
}

// monomorphization: how to implement generic.
// diff languages choose diff strategy:
// java: type erasure. Object
// rust: monomorphization, generate a specific function instance for each different concrete type.
fn test_x() {
    test_static(String::from("helo")); // test_static_String
    test_static(1u8); // test_static_u8
}

pub struct RouteResult {
    pub total_distance: f64,
    pub route_path: Vec<LatLon>,
    pub nodes: Vec<NodeID>,
}

// naming is hard: it's an abstraction of things
#[derive(Debug)]
pub enum EngineErrors {
    CantFindNearestNode,
    CantFindRoute,
    CantFindLatLon,
}

impl std::fmt::Display for EngineErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("engine error: {:?}", self))
    }
}

impl std::error::Error for EngineErrors {
    
}

impl Engine {
    // dyn Error: unknown size
    // function call at run-time will create a call stack: a() -> b() 
    // Box: smart pointer, an owned pointer (it owns the content it's pointing to), fixed size
    // &str: a reference / pointer, doen't own the string content.

    // Box<dyn Error> == &dyn Error : pointer
    // Box<dyn Error> (fixed size like String) != dyn Error (unkown size/unsized type)
    pub fn build(osmfile: &str) -> Result<Engine, Box<dyn Error>> {
        // ? report error, two methods to fix:
        // 1. implement Error for ParseError
        // 2. .map_err() to convert error type (make ParseError compatible with Error trait)
        // 3. implement From<ParseError> for Error (Best one)
        let (graph, tree) = parse_map(osmfile)?;

        // when s is destroyed (out of scope), compiler will reclaim the memory of underlying string.
        // let s = Box::new(String::from("hello"));
        Ok(Engine{
            spaitial_index: tree,
            graph
        })
    } 

    pub fn routing(&self, origin : LatLon, destination : LatLon) -> Result<RouteResult, Box<dyn Error>> {
        // let a: Option<&NodeLocation> = self.spaitial_index.nearest_neighbor(&[origin.lon, origin.lat]);
        // let b: Result<_, &str> = a.ok_or("error ...");
        // let c: &NodeLocation = b?;

        


        let start_node = self.spaitial_index.nearest_neighbor(&[origin.lon, origin.lat]).ok_or(EngineErrors::CantFindNearestNode)?;
        let target_node = self.spaitial_index.nearest_neighbor(&[destination.lon, destination.lat]).ok_or(EngineErrors::CantFindNearestNode)?;

        // println!("== start: {}, target: {}",start_node.data.0, target_node.data.0);


        let (dist, path) = shortest_path(&self.graph, start_node.data, target_node.data).map_err(|_| EngineErrors::CantFindRoute)?;

        // that closure function will be executed for each iteration.
        // we want some short-circuit effect like before (for loop)
        // let mut navpath = vec![];
        // for n in path {
        //     let loc = self.graph.get_latlon(n).ok_or(EngineErrors::CantFindLatLon)?; // return early
        //     navpath.push(loc);
        // }

        // on longer early-return
        // rust provide magic
        // Vec<Result<LatLon, EngineErrors>> no early-return -> Result<Vec<LatLon>, EngineErrors> return early
        let navpath = path.iter().map(|nodeId| { // new function context
            // return or ? only exit that closure instead of the outer function.
            self.graph.get_latlon(*nodeId).ok_or(EngineErrors::CantFindLatLon) // no longer use ? to return (outer function) early
        }).collect::< Result<Vec<LatLon>, EngineErrors> >()?; // turbofish ::<>

        // for nodeId in path {
        //     let latlon = self.graph.get_latlon(nodeId).ok_or(EngineErrors::CantFindLatLon)?;
        //     navpath.push(latlon);
        // }

        Ok(RouteResult{
            total_distance: dist,
            route_path: navpath,
            nodes: path,
        })
    }
}