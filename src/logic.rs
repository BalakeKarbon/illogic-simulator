enum LogicType {
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    NOT,
}

struct LogicElement {
    value: bool,
    element: LogicType,
    inputs: Vec<u32>, 
}

impl LogicElement {

}

pub struct LogicGraph {
    elements: Vec<LogicElement>,
}

impl LogicGraph {
    pub fn new() -> Self {
        LogicGraph {
            elements: Vec::new(),
        }
    }
    pub fn add_element(&mut self) {
        self.elements.push(LogicElement {
            value: false,
            element: LogicType::NAND,
            inputs: Vec::new(),
        });
        println!("Add {}!",self.elements.len());
    }
    pub fn remove_element(&self) {
        println!("Remove!");
    }
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

