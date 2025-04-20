//use crate::logic::LogicGraph;

mod logic;

fn main() {
    let mut test: crate::logic::LogicGraph = crate::logic::LogicGraph::new();
    println!("Initial Size: {}",test.size());
    test.add_element();
    test.add_element();
    test.add_element();
    test.remove_element();
}
