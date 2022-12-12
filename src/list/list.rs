pub fn one() -> u8 {
    1
}

enum Value {
    Element { content: u8, next: Box<Value> },
    Nil,
}

impl Value {
    fn new(val: u8) -> Value {
        Value::Element {
            content: val,
            next: Box::new(Value::Nil),
        }
    }
}

pub struct List {
    top: Value,
}

impl List {
    pub fn new() -> List {
        List { top: Value::Nil }
    }

    pub fn add(&mut self, val: u8) {
        if let Value::Nil = self.top {
            let elem = Value::new(val);
            self.top = elem;
        }
    }
}
