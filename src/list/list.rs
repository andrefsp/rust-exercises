pub fn one() -> u8 {
    1
}

pub enum Node {
    Element { content: u8, next: Box<Node> },
    Nil,
}

impl Node {
    pub fn new(val: u8) -> Node {
        Node::Element {
            content: val,
            next: Box::new(Node::Nil),
        }
    }

    pub fn next_node(&self) -> Option<Node> {
        /*
         *  XXX(andrefsp): we must turn Node into a reference Rc.
         *
        match self {
            Node::Element { content: _, next } => Some(*next),
            Node::Nil => None,
        }
        */
        None
    }

    pub fn set_next(&mut self, n: Node) {
        if let Node::Element { content: _, next } = self {
            *next = Box::new(n)
        };
    }

    pub fn get_value(&self) -> Option<u8> {
        match self {
            Node::Element { content, next: _ } => Some(*content),
            Node::Nil => None,
        }
    }
}

pub struct List {
    top: Box<Node>,
}

impl List {
    pub fn new() -> List {
        List {
            top: Box::new(Node::Nil),
        }
    }

    pub fn push(&mut self, val: u8) {
        if let Node::Nil = *self.top {
            self.top = Box::new(Node::new(val));
            return;
        }

        let mut elem = Node::new(val);

        //elem.set_next(*self.top)

        //* self.top = elem;
    }
}
