use std::cell::RefCell;
use std::rc::Rc;

pub enum Node<T> {
    Content {
        value: T,
        next: RefCell<Rc<Node<T>>>,
    },
    Nil,
}

impl<T> Node<T>
where
    T: Clone + Copy,
{
    pub fn new(val: T) -> Rc<Node<T>> {
        Rc::new(Node::Content {
            value: val,
            next: RefCell::new(Rc::new(Node::Nil)),
        })
    }

    pub fn next(&self) -> Option<&RefCell<Rc<Node<T>>>> {
        match self {
            Node::Content { value: _, next } => {
                let node = next.borrow();
                match node.clone().get_value() {
                    Some(_) => Some(next),
                    None => None,
                }
            }
            Node::Nil => None,
        }
    }

    // XXX(andrefsp): &self because we are using
    // internal mutability via RefCell.
    pub fn set_next(&self, n: Rc<Node<T>>) {
        if let Node::Content { value: _, next } = self {
            next.replace(n);
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
    top: RefCell<Rc<Node<T>>>,
    size: u8,
}

impl<T> List<T>
where
    T: Clone + Copy,
{
    pub fn new() -> List<T> {
        List {
            top: RefCell::new(Rc::new(Node::Nil)),
            size: 0,
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

pub trait Methods<T> {
    fn new() -> Self;
    fn size(&self) -> u8;
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
}

// last in - first out list
pub struct Lifo<T> {
    l: List<T>,
}

impl<T> Methods<T> for Lifo<T>
where
    T: Clone + Copy,
{
    fn new() -> Self {
        Self { l: List::new() }
    }

    fn size(&self) -> u8 {
        self.l.size()
    }

    fn push(&mut self, val: T) {
        // append on the beggining
        match self.l.size {
            0 => {
                self.l.top.replace(Node::new(val));
            }
            _ => {
                let elem = Node::new(val);
                elem.set_next(self.l.top.borrow().clone());
                self.l.top.replace(elem);
            }
        };
        self.l.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // pop from the beggining
        let ret = self.l.top.borrow().get_value();

        let mut top = self.l.top.borrow_mut();

        *top = match top.next() {
            Some(next) => next.borrow().clone(),
            None => Rc::new(Node::Nil),
        };

        ret
    }
}

// first in - first out list
pub struct Fifo<T> {
    l: List<T>,
}

impl<T> Methods<T> for Fifo<T>
where
    T: Clone + Copy,
{
    fn new() -> Self {
        Self { l: List::new() }
    }

    fn size(&self) -> u8 {
        self.l.size()
    }

    fn push(&mut self, val: T) {
        // append in the end
        self.l.size += 1;

        let top_val = self.l.top.borrow().get_value();
        if let None = top_val {
            self.l.top.replace(Node::new(val));
            return;
        };

        let mut n = self.l.top.borrow_mut().clone();
        loop {
            if n.next().is_none() {
                n.set_next(Node::new(val));
                break;
            };
            let nn = n.next().unwrap().borrow().clone();
            n = nn;
        }
    }

    fn pop(&mut self) -> Option<T> {
        // pop from the beggining

        let ret = self.l.top.borrow().get_value();

        let mut top = self.l.top.borrow_mut();

        *top = match top.next() {
            Some(next) => next.borrow().clone(),
            None => Rc::new(Node::Nil),
        };

        ret
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

        self.current = match self.current.next() {
            Some(next) => next.borrow().clone(),
            None => Rc::new(Node::Nil),
        };

        val
    }
}

impl<T> IntoIterator for Lifo<T>
where
    T: std::cmp::PartialEq + Clone + Copy,
{
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.l.top.borrow().clone())
    }
}
