pub enum LogicType {
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    NOT,
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
        1
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
