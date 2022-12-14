use std::rc::Rc;

pub enum Node<T> {
    Content { value: T, next: Rc<Node<T>> },
    Nil,
}

impl<T> Node<T>
where
    T: Clone + Copy,
{
    pub fn new(val: T) -> Node<T> {
        Node::Content {
            value: val,
            next: Rc::new(Node::Nil),
        }
    }

    pub fn next(&self) -> Option<Rc<Node<T>>> {
        match self {
            Node::Content { value: _, next } => Some(next.clone()),
            Node::Nil => None,
        }
    }

    pub fn set_next(&mut self, n: Rc<Node<T>>) {
        if let Node::Content { value: _, next } = self {
            *next = n
        };
    }

    pub fn get_value(&self) -> Option<T> {
        match self {
            Node::Content { value, next: _ } => Some(*value),
            Node::Nil => None,
        }
    }
}

pub struct List<T> {
    top: Rc<Node<T>>,
    size: u8,
}

impl<T> List<T>
where
    T: Clone + Copy,
{
    pub fn new() -> List<T> {
        List {
            top: Rc::new(Node::Nil),
            size: 0,
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn push(&mut self, val: T) {
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

pub struct ListIterator<T> {
    current: Rc<Node<T>>,
}

impl<T> ListIterator<T> {
    pub fn new(current: Rc<Node<T>>) -> Self {
        ListIterator { current }
    }
}

impl<T> Iterator for ListIterator<T>
where
    T: std::cmp::PartialEq + Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current.get_value();
        if None == val {
            return None;
        };

        self.current = self.current.next().unwrap();

        val
    }
}

impl<T> IntoIterator for List<T>
where
    T: std::cmp::PartialEq + Clone + Copy,
{
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.top)
    }
}
