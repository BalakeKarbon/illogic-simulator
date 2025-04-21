//use crate::logic::LogicGraph;

mod logic;

fn main() {
    let mut test_net: crate::logic::Network = crate::logic::Network::new();
    let mut elements: Vec<usize> = Vec::new();
    elements.push(test_net.add_sensor());
    elements.push(test_net.add_sensor());
    elements.push(test_net.add_empty_element(crate::logic::LogicType::NOR));
    elements.push(test_net.add_empty_element(crate::logic::LogicType::NOR));
    test_net.add_element_input(elements[2],elements[0]);
    test_net.add_element_input(elements[2],elements[3]);
    test_net.add_element_input(elements[3],elements[1]);
    test_net.add_element_input(elements[3],elements[2]);
    println!("test_net size: {}",test_net.get_size());
}
