use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StackFrame {
    pub call_instruction: u32,
    pub call_address: u32,
    pub return_address: u64,
    pub frame_pointer: u64,
    pub stack_pointer: u64,
    pub jump_address: u64,
}

impl StackFrame {
    pub fn new(
        call_instruction: u32,
        call_address: u64,
        return_address: u64,
        frame_pointer: u64,
        stack_pointer: u64,
        jump_address: u64,
    ) -> Self {
        Self {
            call_instruction,
            call_address: call_address as u32,
            return_address,
            frame_pointer,
            stack_pointer,
            jump_address,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Stack {
    pub stack: Vec<StackFrame>,
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

pub struct StackIter<'a> {
    stack: &'a Stack,
    current_address: u32,
}

impl<'a> StackIter<'a> {
    pub fn new(stack: &'a Stack, current_address: u32) -> StackIter<'a> {
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
