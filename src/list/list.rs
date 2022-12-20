use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::Node;

pub trait Methods<T>
where
    T: Display + Clone + Copy,
{
    fn size(&self) -> u8;
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn head(&self) -> Rc<Node<T>>;
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
            top: RefCell::new(Node::nil()),
            size: 0,
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

// last in - first out list
pub struct Lifo<T> {
    l: List<T>,
}

impl<T> Lifo<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self { l: List::new() }
    }
}

impl<T> Methods<T> for Lifo<T>
where
    T: Display + Clone + Copy,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.top.borrow().clone()
    }

    fn push(&mut self, val: T) {
        // append on the beggining
        self.l.size += 1;

        match *self.head() {
            Node::Nil => {
                self.l.top.replace(Node::new(val));
            }
            _ => {
                let elem = Node::new(val);
                elem.set_next(self.l.top.borrow().clone());
                self.l.top.replace(elem);
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        // pop from the beggining
        let ret = self.l.top.borrow().get_value();

        let mut top = self.l.top.borrow_mut();

        *top = match top.next() {
            Some(next) => next.borrow().clone(),
            None => Node::nil(),
        };

        ret
    }
}

// first in - first out list
pub struct Fifo<T> {
    l: List<T>,
}

impl<T> Fifo<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self { l: List::new() }
    }
}

impl<T> Methods<T> for Fifo<T>
where
    T: Display + Clone + Copy,
{
    fn size(&self) -> u8 {
        self.l.size()
    }
    fn head(&self) -> Rc<Node<T>> {
        self.l.top.borrow().clone()
    }

    fn push(&mut self, val: T) {
        // append in the end
        self.l.size += 1;

        if let Node::Nil = *self.head() {
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
            None => Node::nil(),
        };

        ret
    }
}

pub struct Ordered<T> {
    l: List<T>,
}

impl<T> Ordered<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self { l: List::new() }
    }
}

impl<T> Methods<T> for Ordered<T>
where
    T: Display + Clone + Copy + PartialOrd,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.top.borrow().clone()
    }

    fn push(&mut self, val: T) {
        if let Node::Nil = *self.head() {
            self.l.top.replace(Node::new(val));
            return;
        }

        let cmp = Node::new(val);

        let mut cr = &self.l.top;
        let mut current = self.l.top.borrow_mut().clone();

        loop {
            if current.next().is_none() {
                current.set_next(cmp);
                break;
            };
            // TODO(andrefsp):: proceed here
            let k = current.next().unwrap().borrow().clone();
            current = k;
        }
    }

    fn pop(&mut self) -> Option<T> {
        None
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

        self.current = match self.current.next() {
            Some(next) => next.borrow().clone(),
            None => Node::nil(),
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
