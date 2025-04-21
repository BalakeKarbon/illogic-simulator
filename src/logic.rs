pub enum LogicType {
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    NOT,
    SENSOR,
}
fn And(inputs: &Vec<usize>) -> bool {
    // AND all input bools.
    //inputs.iter().all(|&b| b) // Check to make sure this works bro. Alternative method using fold 
                              // would be: inputs.iter().fold(true, |acc, &b| acc && b). For OR the
                              // fold method would be: index.iter().fold(false, |acc, &b| acc || b);
    true
}
fn Or(inputs: &Vec<usize>) -> bool {
    //inputs.iter().any(|&b| b)
    true
}
fn Nand(inputs: &Vec<usize>) -> bool {
    // !inputs.iter().all(|&b| b)
    true
}
fn Nor(inputs: &Vec<usize>) -> bool {
    // !inputs.iter().any(|&b| b)
    true
}
fn Xor(inputs: &Vec<usize>) -> bool {
    //inputs.iter().fold(false, |acc, &b| acc ^ b)
    true
}
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
struct LogicNot { // It might be benifical to just make this a NOR, then we dont have to check for
                  // std::usize::MAX for a no input situation. Although the processing may be
                  // faster without the use of a vector the speedup might not be worth the
                  // complexity? Think about this...
    state: bool,
    input: usize,
}
impl LogicNot {
    fn process(&self) -> bool {
        // !self.input
        true
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
    LogicNot(LogicNot),
    Sensor(Sensor),
}
pub struct Network {
    elements: Vec<Element>,
}
impl Network {
    fn add_logic(&mut self, element_type: LogicType, inputs: Vec<usize>) -> usize { // Takes
                                                                                    // ownership of
                                                                                    // the inputs
                                                                                    // Vec<usize>
        match element_type { // We still need to add the code for determining what section of
                             // processing during the cycle per element.
            LogicType::AND => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: And,
                    inputs: inputs, // This is going to throw an error for a copy or a reference
                                    // yeah...
                }));
            }
            LogicType::OR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: Or,
                    inputs: inputs,
                }));
            }
            LogicType::NAND => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: Nand,
                    inputs: inputs,
                }));
            }
            LogicType::NOR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: Nor,
                    inputs: inputs,
                }));
            }
            LogicType::XOR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: Xor,
                    inputs: inputs, // What is the significance of XOR with more than two inputs?
                }));
            }
            LogicType::NOT => {
                match inputs.get(0) {
                    Some(&input) => {
                        self.elements.push(Element::LogicNot(LogicNot {
                            state: false,
                            input: input, //Returns Some????? Need more logic here folks.
                        }));
                    }
                    None => {
                        return std::usize::MAX;
                    }
                }
            }
            LogicType::SENSOR => {
                self.elements.push(Element::Sensor(Sensor {
                    state: false,
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
    fn remove_element(&mut self, index: usize) -> Vec<usize> { // Perhaps return effected elements or a truth value?
        let mut containing_elements: Vec<usize> = Vec::new();
        if self.elements.len() >= index {
            self.elements.remove(index);
            for (e,element) in self.elements.iter_mut().enumerate() {
                match element {
                    Element::Sensor(sensor) => {}
                    Element::LogicNot(logic) => {
                        if logic.input == index {
                            logic.input = std::usize::MAX; // This is under the assumption that
                                                             // the max usize value is basically
                                                             // being used as no input. We must
                                                             // decide if we really do want this to
                                                             // be the case or if we should return
                                                             // NOTS to be like NORS...? NO
                                                             // ACTUALLY THIS CAUSES ERROR RIGHT
                                                             // BECAUSE IN THE PROCESSING IT WILL
                                                             // LOOK FOR AN ELEMENT AT THAT VALUE
                                                             // UNLESS IT IS CAUGHT!!?! 
                            containing_elements.push(e);
                        } else if logic.input > index {
                            logic.input -= 1;
                            containing_elements.push(e);
                        }
                    }
                    Element::Logic(logic) => {
                        let mut found: bool = false;
                        for input in logic.inputs.iter_mut() {
                            if *input > index { // How is this safe rust? Take a minute to
                                                // understand this at some point pls.
                                *input -= 1;
                                found = true;
                            } else if *input == index {
                                found = true;
                            }
                        }
                        logic.inputs.retain(|&i| i != index); //Make sure this is removing the
                                                              //element at that index.
                        if found {
                            containing_elements.push(e);
                        }
                    }
                }
            }
        }
        containing_elements
    }
    fn get_element_type(&self, index: usize) -> Option<LogicType> { // holdup gotta add sensor
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => {
                    if logic.processor == And as fn(&Vec<usize>) -> bool { // Here we are comparing
                                                                           // the function pointer
                                                                           // inside our element to
                                                                           // the actual function
                                                                           // to determin which
                                                                           // logic type it is
                                                                           // after initialization.
                        Some(LogicType::AND)
                    } else if logic.processor == Or as fn(&Vec<usize>) -> bool {
                        Some(LogicType::OR)
                    } else if logic.processor == Nand as fn(&Vec<usize>) -> bool {
                        Some(LogicType::NAND)
                    } else if logic.processor == Nor as fn(&Vec<usize>) -> bool {
                        Some(LogicType::NOR)
                    } else if logic.processor == Xor as fn(&Vec<usize>) -> bool {
                        Some(LogicType::XOR)
                    } else {
                        Some(LogicType::SENSOR) // Seems the least risky option is to return a SENSOR
                                          // value if somehow the pointer is not aimed at any
                                          // specific function!?
                    }
                }
                Element::LogicNot(logic) => Some(LogicType::NOT),
                Element::Sensor(sensor) => Some(LogicType::SENSOR),
            }
        } else {
            None
        }
    }
    fn get_element_state(&self, index: usize) -> Option<bool> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => Some(logic.state),
                Element::LogicNot(logic) => Some(logic.state),
                Element::Sensor(sensor) => Some(sensor.state),
            }
        } else {
            None
        }
    }
    fn set_sensor_state(&mut self, index: usize, state: bool) -> Option<LogicType> {
        if let Some(element) = self.elements.get_mut(index) {
            match element {
                Element::Sensor(sensor) => {
                    sensor.set_state(state);
                    Some(LogicType::SENSOR)
                }
                _ => {
                    self.get_element_type(index)
                }
            }
        } else {
            None
        }
    }
}
