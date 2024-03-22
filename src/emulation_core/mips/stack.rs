use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stack {
    pub stack: Vec<StackFrame>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StackFrame {
    pub instruction: u32,
    pub address: u32,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
}

impl ToString for Stack {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for frame in &self.stack {
            output.push_str(&format!("{:08x}  {}\n", frame.address, frame.instruction));
        }

        output
    }
}

impl Stack {
    pub fn push(&mut self, frame: StackFrame) {
        self.stack.push(frame);
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&StackFrame> {
        self.stack.last()
    }
}

impl StackFrame {
    pub fn new(instruction: u32, address: u32) -> Self {
        Self {
            instruction,
            address,
        }
    }
}

pub struct StackIter<'a> {
    stack: &'a Stack,
    current_address: u32,
}

impl<'a> StackIter<'a> {
    pub fn new(
        stack: &'a Stack,
        current_address: u32,
    ) -> StackIter<'a> {
        StackIter {
            stack,
            current_address,
        }
    }
}

impl<'a> Iterator for StackIter<'a> {
    type Item = &'a StackFrame;
    fn next(&mut self) -> Option<Self::Item> {
        self.current_address = (self.current_address + 3) & !3;
        if self.current_address < self.stack.stack.len() as u32 {
            Some(&self.stack.stack[self.current_address as usize])
        } else {
            None
        }
    }
}
