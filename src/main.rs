//use crate::logic::LogicGraph;

mod logic;

fn main() {
    let mut test_net: crate::logic::Network = crate::logic::Network::new();
    let mut elements: Vec<usize> = Vec::new();
    elements.push(test_net.add_sensor());
    elements.push(test_net.add_sensor());
    elements.push(test_net.add_empty_element(crate::logic::LogicType::NOR));
    elements.push(test_net.add_empty_element(crate::logic::LogicType::NOR));
    if let Some(index) = test_net.add_element_input(elements[2],100) {
        println!("ERROR SETTING ELEMENT {} TO CONTAIN INPUT {}",index,100);
    }
    if let None = test_net.add_element_input(elements[2],elements[0]) {
        println!("Proof we have added input {} to element {}!",elements[0],elements[2]);
    }
    test_net.add_element_input(elements[2],elements[3]);
    test_net.add_element_input(elements[3],elements[1]);
    test_net.add_element_input(elements[3],elements[2]);
    println!("test_net size: {}",test_net.get_size());
}
