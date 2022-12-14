use std::rc::Rc;

pub enum Node {
    Content { value: u8, next: Rc<Node> },
    Nil,
}

impl Node {
    pub fn new(val: u8) -> Node {
        Node::Content {
            value: val,
            next: Rc::new(Node::Nil),
        }
    }

    pub fn next(&self) -> Option<Rc<Node>> {
        match self {
            Node::Content { value: _, next } => Some(next.clone()),
            Node::Nil => None,
        }
    }

    pub fn set_next(&mut self, n: Rc<Node>) {
        if let Node::Content { value: _, next } = self {
            *next = n
        };
    }

    pub fn get_value(&self) -> Option<u8> {
        match self {
            Node::Content { value, next: _ } => Some(*value),
            Node::Nil => None,
        }
    }
}

pub struct List {
    top: Rc<Node>,
    size: u8,
}

impl List {
    pub fn new() -> List {
        List {
            top: Rc::new(Node::Nil),
            size: 0,
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn push(&mut self, val: u8) {
        self.size += 1;

        if let Node::Nil = *self.top {
            self.top = Rc::new(Node::new(val));
            return;
        }

        let mut elem = Node::new(val);
        elem.set_next(self.top.clone());
        self.top = Rc::new(elem);
    }
}

pub struct ListIterator {
    current: Rc<Node>,
}

impl ListIterator {
    pub fn new(current: Rc<Node>) -> Self {
        ListIterator { current }
    }
}

impl Iterator for ListIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current.get_value();
        if None == val {
            return None;
        };

        self.current = self.current.next().unwrap();

        val
    }
}

impl IntoIterator for List {
    type Item = u8;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.top)
    }
}
