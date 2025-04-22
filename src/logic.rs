pub struct Network {
    elements: Vec<Element>,
    stage_count: usize, // Only using a usize as it will prevent allignment issues in the
                        // architecture and potentially provide a preformance boost. Some testing
                        // would be interesting later.
    stage_buffer: Vec<bool>,
}
pub enum LogicType {
    AND,
    OR,
    NAND,
    NOR,
    XOR,
    NOT,
    INPUT,
}
fn and(inputs: &Vec<usize>, network: &Network) -> bool {
    let mut next_state: bool = false; 
    for input in inputs.iter() {
        match network.get_element_state(*input) { // This is because through the iteration of the
                                                  // vector the usize is borrowed.
            Some(state) => next_state = next_state && state,
            None => {},
        }
    }
    next_state
}
fn or(inputs: &Vec<usize>, network: &Network) -> bool {
    let mut next_state: bool = false; 
    for input in inputs.iter() {
        match network.get_element_state(*input) {
            Some(state) => {
                if state {
                    next_state = true;
                    break;
                }
            },
            None => {},
        }
    }
    next_state
}
fn nand(inputs: &Vec<usize>, network: &Network) -> bool {
    !and(inputs, network)
}
fn nor(inputs: &Vec<usize>, network: &Network) -> bool {
    !or(inputs, network)
}
fn xor(inputs: &Vec<usize>, network: &Network) -> bool {
    let mut next_state: bool = false; 
    for input in inputs.iter() {
        match network.get_element_state(*input) {
            Some(state) => next_state = next_state != state,
            None => {},
        }
    }
    next_state
}
struct Logic {
    state: bool,
    processor: fn(&Vec<usize>, &Network) -> bool,
    inputs: Vec<usize>,
}
impl Logic {
    fn process(&self, network: &Network) -> bool {
        (self.processor)(&self.inputs, network)
    }
    fn set_state(&mut self, state: bool) { // Should this be a reference to the bool?
        self.state = state;
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
    fn process(&self, network: &Network) -> bool {
        let mut next_state: bool = true;
        match network.get_element_state(self.input) {
            Some(state) => next_state = !state,
            None => {},
        }
        next_state
    }
    fn set_state(&mut self, state: bool) { // Should this be a reference to the bool?
        self.state = state;
    }
}
struct Input {
    state: bool,
}
impl Input {
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
    Input(Input),
}
impl Network {
    pub fn new() -> Self {
        Network {
            elements: Vec::new(),
            stage_count: 5, // This is the default stage count. Perhaps this should be modifiable in
                        // the future.
            stage_buffer: Vec::new(),
        }
    }
    pub fn add_empty_element(&mut self, element_type: LogicType) -> usize { // Takes
                                                                                    // ownership of
                                                                                    // the inputs
                                                                                    // Vec<usize>
        match element_type { // We still need to add the code for determining what section of
                                 // processing during the cycle per element.
            LogicType::AND => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: and,
                    inputs: Vec::new(), // This is going to throw an error for a copy or a reference
                                    // yeah...
                }));
            }
            LogicType::OR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: or,
                    inputs: Vec::new(),
                }));
            }
            LogicType::NAND => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: nand,
                    inputs: Vec::new(),
                }));
            }
            LogicType::NOR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: nor,
                    inputs: Vec::new(),
                }));
            }
            LogicType::XOR => {
                self.elements.push(Element::Logic(Logic {
                    state: false,
                    processor: xor,
                    inputs: Vec::new(),
                }));
            }
            LogicType::NOT => {
                self.elements.push(Element::LogicNot(LogicNot {
                    state: false,
                    input: std::usize::MAX,
                }));
            }
            LogicType::INPUT => {
                self.elements.push(Element::Input(Input {
                    state: false,
                }));
            }
        }
        let stage_size = ((self.elements.len()+self.stage_count)-1)/self.stage_count;
        self.stage_buffer = Vec::with_capacity(stage_size);
        for i in 0..stage_size {
            self.stage_buffer.push(false);
        }
        self.elements.len()-1
    }
    pub fn add_element(&mut self, element_type: LogicType, inputs: Vec<usize>) -> usize { // Takes
                                                                                    // ownership of
                                                                                    // the inputs
                                                                                    // Vec<usize>
        if (inputs.len() > 0) && (self.elements_exist(&inputs).len() != 0) {
            match element_type { // We still need to add the code for determining what section of
                                 // processing during the cycle per element.
                LogicType::AND => {
                    self.elements.push(Element::Logic(Logic {
                        state: false,
                        processor: and,
                        inputs: inputs, // This is going to throw an error for a copy or a reference
                                        // yeah...
                    }));
                }
                LogicType::OR => {
                    self.elements.push(Element::Logic(Logic {
                        state: false,
                        processor: or,
                        inputs: inputs,
                    }));
                }
                LogicType::NAND => {
                    self.elements.push(Element::Logic(Logic {
                        state: false,
                        processor: nand,
                        inputs: inputs,
                    }));
                }
                LogicType::NOR => {
                    self.elements.push(Element::Logic(Logic {
                        state: false,
                        processor: nor,
                        inputs: inputs,
                    }));
                }
                LogicType::XOR => {
                    self.elements.push(Element::Logic(Logic {
                        state: false,
                        processor: xor,
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
                LogicType::INPUT => {
                    self.elements.push(Element::Input(Input {
                        state: false,
                    }));
                }
            }
            let stage_size = ((self.elements.len()+self.stage_count)-1)/self.stage_count;
            self.stage_buffer = Vec::with_capacity(stage_size);
            for i in 0..stage_size {
                self.stage_buffer.push(false);
            }
            self.elements.len()-1
        } else {
            std::usize::MAX
        }
    }
    pub fn add_input(&mut self) -> usize {
        self.elements.push(Element::Input(Input {
            state: false,
        }));
        let stage_size = ((self.elements.len()+self.stage_count)-1)/self.stage_count;
        self.stage_buffer = Vec::with_capacity(stage_size);
        for i in 0..stage_size {
            self.stage_buffer.push(false);
        }
        self.elements.len()-1
    }
    pub fn remove_element(&mut self, index: usize) -> Vec<usize> { // Perhaps return effected elements or a truth value?
        let mut containing_elements: Vec<usize> = Vec::new();
        if self.elements.len() > index {
            self.elements.remove(index);
            for (e,element) in self.elements.iter_mut().enumerate() {
                match element {
                    Element::Input(input) => {},
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
        let stage_size = ((self.elements.len()+self.stage_count)-1)/self.stage_count;
        self.stage_buffer = Vec::with_capacity(stage_size);
        for i in 0..stage_size {
            self.stage_buffer.push(false);
        }
        containing_elements
    }
    pub fn get_element_type(&self, index: usize) -> Option<LogicType> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => {
                    if logic.processor == and as fn(&Vec<usize>, &Network) -> bool { // Here we are comparing
                                                                           // the function pointer
                                                                           // inside our element to
                                                                           // the actual function
                                                                           // to determin which
                                                                           // logic type it is
                                                                           // after initialization.
                        Some(LogicType::AND)
                    } else if logic.processor == or as fn(&Vec<usize>, &Network) -> bool {
                        Some(LogicType::OR)
                    } else if logic.processor == nand as fn(&Vec<usize>, &Network) -> bool {
                        Some(LogicType::NAND)
                    } else if logic.processor == nor as fn(&Vec<usize>, &Network) -> bool {
                        Some(LogicType::NOR)
                    } else if logic.processor == xor as fn(&Vec<usize>, &Network) -> bool {
                        Some(LogicType::XOR)
                    } else {
                        Some(LogicType::INPUT) // Seems the least risky option is to return a INPUT
                                          // value if somehow the pointer is not aimed at any
                                          // specific function!?
                    }
                }
                Element::LogicNot(logic) => Some(LogicType::NOT),
                Element::Input(input) => Some(LogicType::INPUT),
            }
        } else {
            None
        }
    }
    pub fn get_element_state(&self, index: usize) -> Option<bool> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => Some(logic.state),
                Element::LogicNot(logic) => Some(logic.state),
                Element::Input(input) => Some(input.state),
            }
        } else {
            None
        }
    }
    pub fn get_element_inputs(&self, index: usize) -> Option<Vec<usize>> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => {
                    Some(logic.inputs.clone()) // Return a duplicate representation.
                }
                Element::LogicNot(logic) => {
                    let mut inputs: Vec<usize> = vec![logic.input];
                    Some(inputs)
                }
                Element::Input(input) => {
                    None
                }
            }
        } else {
            None
        }
    }
    fn elements_exist(&self, inputs: &Vec<usize>) -> Vec<usize> {
        let mut non_existing_elements: Vec<usize> = Vec::new();
        for index in inputs {
            if self.elements.len() < *index {
                non_existing_elements.push(*index);
            }
        }
        non_existing_elements
    }
    pub fn set_element_inputs(&mut self, index: usize, inputs: &Vec<usize>) -> Option<Vec<usize>> {
        let mut non_existing_elements: Vec<usize> = self.elements_exist(inputs);
        if non_existing_elements.len() == 0 {
            if let Some(element) = self.elements.get_mut(index) {
                match element {
                    Element::Logic(logic) => {
                        logic.inputs = inputs.to_vec();
                        None
                    }
                    Element::LogicNot(logic) => {
                        logic.input = inputs[0];
                        None
                    }
                    Element::Input(input) => {
                        Some(non_existing_elements)
                    }
                }
            } else {
                Some(non_existing_elements)
            }
        } else {
            Some(non_existing_elements)
        }
    }
    pub fn add_element_input(&mut self, index: usize, input: usize) -> Option<usize> {
        let mut non_existing_elements: Vec<usize> = self.elements_exist(&vec![input]);
        if non_existing_elements.len() == 0 {
            if let Some(element) = self.elements.get_mut(index) {
                match element {
                    Element::Logic(logic) => {
                        logic.inputs.push(input);
                        None
                    }
                    Element::LogicNot(logic) => {
                        logic.input = input;
                        None
                    }
                    Element::Input(input) => {
                        Some(index)
                    }
                }
            } else {
                Some(index)
            }
        } else {
            Some(index)
        }       
    }
    fn process_element(&self, index: usize) -> Option<bool> {
        if let Some(element) = self.elements.get(index) {
            match element {
                Element::Logic(logic) => Some(logic.process(self)),
                Element::LogicNot(logic) => Some(logic.process(self)),
                Element::Input(input) => Some(input.process()),
            }
        } else {
            None
        }
    }
    pub fn set_input_state(&mut self, index: usize, state: bool) -> Option<LogicType> {
        if let Some(element) = self.elements.get_mut(index) {
            match element {
                Element::Input(input) => {
                    input.set_state(state);
                    Some(LogicType::INPUT)
                }
                _ => {
                    self.get_element_type(index)
                }
            }
        } else {
            None
        }
    }
    fn buffer_stage(&mut self, stage: usize) {
        for base_index in 0..self.stage_buffer.len() {
            match self.process_element((base_index*self.stage_count)+stage) {
                Some(next_state) => {
                    self.stage_buffer[base_index] = next_state;
                }
                None => {
                    self.stage_buffer[base_index] = false; // This should only be reached if
                                                               // the element does not exist in
                                                               // which case we dont want a value
                                                               // there for the next stage.
                },
            }
        }
    }
    fn write_stage(&mut self, stage: usize) {
        for base_index in 0..self.stage_buffer.len() {
            if let Some(element) = self.elements.get_mut((base_index*self.stage_count)+stage) {
                match element {
                    Element::Logic(logic) => {
                        logic.set_state(self.stage_buffer[base_index]);
                    }
                    Element::LogicNot(logic) => {
                        logic.set_state(self.stage_buffer[base_index]);
                    }
                    Element::Input(input) => {}
                }
            }
        }
    }
    pub fn cycle(&mut self) {
        for stage in 0..self.stage_count {
            // These two stages must happen in order but within each stage could be parallelized if
            // synced properly.
            self.buffer_stage(stage);
            self.write_stage(stage);
        }
    }
    pub fn get_size(&self) -> usize {
        self.elements.len()
    }
}
