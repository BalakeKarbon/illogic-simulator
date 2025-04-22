mod logic;

fn test_cycle(network: &mut crate::logic::Network, elements: &Vec<usize>) {
    network.cycle();
    for element in elements.iter() {
        if let Some(state) = network.get_element_state(*element) {
            if state {
                print!("|---T---");
            } else {
                print!("|   F   ");
            }
        }
    }
    println!("|");
} 

fn test_callback(state: bool) {
    print!("|=Latch=");
    if state {
        println!("on!=====================|");
    } else {
        println!("off!====================|");
    }
}

fn main() {
    let mut test_net: crate::logic::Network = crate::logic::Network::new();
    let mut elements: Vec<usize> = Vec::new();
    elements.push(test_net.add_input());
    elements.push(test_net.add_input());
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
    test_net.set_element_sensor(elements[2],test_callback);
    println!("test_net size: {}",test_net.get_size());

    println!("| Inp 1 | Inp 2 | NOR 1 | NOR 2 |");
    for element in elements.iter() {
        if let Some(state) = test_net.get_element_state(*element) {
            if state {
                print!("|---T---");
            } else {
                print!("|   F   ");
            }
        }
    }
    println!("|");
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[0],true);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[0],false);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[1],true);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[1],false);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[0],true);
    test_net.set_input_state(elements[1],true);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_net.set_input_state(elements[0],false);
    test_net.set_input_state(elements[1],false);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    println!("Removing element 2...");
    test_net.remove_element(elements[2]);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
    test_cycle(&mut test_net, &elements);
}
