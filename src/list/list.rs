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
    first: RefCell<Rc<Node<T>>>,
}

impl<T> List<T>
where
    T: Clone + Copy,
{
    fn new() -> List<T> {
        List {
            first: RefCell::new(Node::nil()),
        }
    }

    pub fn lifo() -> Lifo<T> {
        Lifo { l: List::new() }
    }

    pub fn fifo() -> Fifo<T> {
        Fifo { l: List::new() }
    }

    pub fn ordered() -> Ordered<T> {
        Ordered { l: List::new() }
    }

    pub fn size(&self) -> u8 {
        let mut count = 0;
        let mut current = self.head();
        loop {
            if let Node::Nil = *current {
                return count;
            };
            count += 1;

            current = match current.next() {
                Some(next) => next.borrow().clone(),
                None => Node::nil(),
            };
        }
    }

    pub fn head(&self) -> Rc<Node<T>> {
        self.first.borrow().clone()
    }

    pub fn pop(&self) -> Option<T> {
        // pop from the beggining
        self.first
            .replace(match self.head().next() {
                Some(next) => next.borrow().clone(),
                None => Node::nil(),
            })
            .get_value()
    }
}

// last in - first out list
pub struct Lifo<T> {
    l: List<T>,
}

impl<T> Methods<T> for Lifo<T>
where
    T: Display + Clone + Copy,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&mut self) -> Option<T> {
        // pop from the beggining
        self.l.pop()
    }

    fn push(&mut self, val: T) {
        // append on the beggining
        let head = self.head();
        self.l.first.replace(match *head {
            Node::Nil => Node::new(val),
            _ => {
                let elem = Node::new(val);
                elem.set_next(head);
                elem
            }
        });
    }
}

// first in - first out list
pub struct Fifo<T> {
    l: List<T>,
}

impl<T> Methods<T> for Fifo<T>
where
    T: Display + Clone + Copy,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&mut self) -> Option<T> {
        // pop from the beggining
        self.l.pop()
    }

    fn push(&mut self, val: T) {
        // append in the end
        if let Node::Nil = *self.head() {
            self.l.first.replace(Node::new(val));
            return;
        };

        let mut n = self.l.first.borrow_mut().clone();
        loop {
            if n.next().is_none() {
                n.set_next(Node::new(val));
                break;
            };
            let nn = n.next().unwrap().borrow().clone();
            n = nn;
        }
    }
}

pub struct Ordered<T> {
    l: List<T>,
}

impl<T> Methods<T> for Ordered<T>
where
    T: Display + Clone + Copy + PartialOrd,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&mut self) -> Option<T> {
        self.l.pop()
    }

    fn push(&mut self, val: T) {
        let new = Node::new(val);
        let mut current = self.head();

        if let Node::Nil = *current {
            self.l.first.replace(new);
            return;
        } else if new <= current {
            new.set_next(self.head());
            self.l.first.replace(new);
            return;
        };

        loop {
            match current.next() {
                Some(next_ref) => {
                    let next = next_ref.borrow().clone();
                    if new > current && new <= next {
                        new.set_next(next);
                        current.set_next(new.clone());
                        return;
                    };
                    current = next;
                }
                None => {
                    current.set_next(new.clone());
                    return;
                }
            }
        }
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
        ListIterator::new(self.l.first.borrow().clone())
    }
}
