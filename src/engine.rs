use std::error::Error;

use crate::{graph::Graph, parser::{parse_map, SpaitialIndex}};


struct Engine {
    spaitial_index: SpaitialIndex,
    graph: Graph,
}

// Error is a trait, not a concrete type.
// But Result<_, E> E should be type
// The problem: we don't know the type yet (or we just want it to be flexible, any type implemented Error is ok)
// use "dyn Error": any type implemented Error trait.
// So if it's any type, then we don't know its size.

#[derive(Debug)]
struct E1; // zero-sized type
impl std::fmt::Display for E1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
struct E2(u16); // 2 bytes type
impl std::fmt::Display for E2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// implemented Error for both E1 and E2
impl Error for E1 {

}

impl Error for E2 {

}

impl Engine {
    // dyn Error: unknown size
    // function call at run-time will create a call stack: a() -> b() 
    // Box: smart pointer, an owned pointer (it owns the content it's pointing to), fixed size
    // &str: a reference / pointer, doen't own the string content.
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
}