pub struct MyStack {
    stack: Vec<(u8, u8)>,
}

impl MyStack {
    pub fn new() -> MyStack {
        MyStack { stack: Vec::new() }
    }

    pub fn push(&mut self, value: (u8, u8)) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<(u8, u8)> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&(u8, u8)> {
        self.stack.last() 
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty() 
    }
}