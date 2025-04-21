pub enum LogicType {
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    NOT,
}

fn And(inputs: &Vec<usize>) -> bool {
    // AND all input bools.
    inputs.iter().all(|&b| b) // Check to make sure this works bro. Alternative method using fold 
                              // would be: inputs.iter().fold(true, |acc, &b| acc && b). For OR the
                              // fold method would be: index.iter().fold(false, |acc, &b| acc || b);
}
fn Or(inputs: &Vec<usize>) -> bool {
    inputs.iter().any(|&b| b)
}
fn Nand(inputs: &Vec<usize>) -> bool {
    !inputs.iter().all(|&b| b)
}
fn Nor(inputs: &Vec<usize>) -> bool {
    !inputs.iter().any(|&b| b)
}
fn Xor(inputs: &Vec<usize>) -> bool {
    inputs.iter().fold(false, |acc, &b| acc ^ b)
}
/* fn Not(inputs: &Vec<usize>) -> bool { // Do we want a vector of inputs? I mean its only practical
                                      // with one input so if we keep the vector we must decide how
                                      // we want to process that like an OR of all of them and then
                                      // an invert of that which is just a NOR...???? or we do a
                                      // seperate one with just one value...?
    !inputs.iter().any(|&b| b)
} */

struct Logic {
    state: bool,
    processor: fn(&Vec<usize>) -> bool,
    inputs: Vec<usize>,
}

impl Logic {
    fn process(&self) -> bool {
        (self.processor)(&self.inputs)
    }
}

struct Sensor {
    state: bool,
}

impl Sensor {
    fn process(&self) -> bool {
        self.state // This perhaps could be defined as a function like the other logic in which
                   // case it could be seen as something like a reader, perhaps a bit more dynamic.
    }
    fn set_state(&mut self, state: bool) { // Should this be a reference to the bool?
        self.state = state;
    }
}

enum Element {
    Logic(Logic),
    Sensor(Sensor),
}

pub struct Network {
    elements: Vec<Element>,
}

impl Network {
    fn add_logic(&mut self, element_type: LogicType, inputs: &Vec<usize>) -> usize {
        match element_type {
            LogicType::AND => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: And(inputs),
                    inputs: inputs, // This is going to throw an error for a copy or a reference
                                    // yeah...
                }));
            }
            LogicType::OR => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: Or(inputs),
                    inputs: inputs,
                }));
            }
            LogicType::NAND => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: Nand(inputs),
                    inputs: inputs,
                }));
            }
            LogicType::NOR => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: Nor(inputs),
                    inputs: inputs,
                }));
            }
            LogicType::XOR => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: Xor(inputs),
                    inputs: inputs,
                }));
            }
            LogicType::NOT => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
                    processor: Nor(inputs), // Should this structure be different because only one
                                            // input is practical for a not? Then perhaps it would
                                            // not allow people to do crasy stuff?
                    inputs: inputs,
                }));
            }
        }
        self.elements.len()
    }
    fn add_sensor(&mut self) -> usize {
        self.elements.push(Element::Sensor(Sensor {
            state: false,
        }));
        self.elements.len()
    }
    fn remove_element(&mut self, index: usize) { // Perhaps return effected elements or a truth value?

    }
    fn get_element_state(&self, index: usize) -> Option<bool> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => Some(logic.state),
                Element::Sensor(sensor) => Some(sensor.state),
            }
        } else {
            None
        }
    }
}
