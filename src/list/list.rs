use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use super::Node;

pub trait Methods<T>
where
    T: Clone + Copy + Default,
{
    fn size(&self) -> u8;
    fn push(&self, val: T);
    fn pop(&self) -> Option<T>;
    fn head(&self) -> Rc<Node<T>>;
    fn impl_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[derive(Default)]
pub struct List<T> {
    first: RefCell<Rc<Node<T>>>,
}

impl<T> List<T>
where
    T: Clone + Copy + Default,
{
    pub fn lifo() -> Box<Lifo<T>> {
        Box::new(Lifo { l: List::default() })
    }

    pub fn fifo() -> Box<Fifo<T>> {
        Box::new(Fifo { l: List::default() })
    }

    pub fn ordered() -> Box<Ordered<T>> {
        Box::new(Ordered { l: List::default() })
    }

    pub fn size(&self) -> u8 {
        let mut count = 0;
        let mut node = self.head();
        loop {
            if node.is_nil() {
                return count;
            };
            count += 1;
            node = match node.next() {
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
    T: Clone + Copy + Default,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&self) -> Option<T> {
        // pop from the beggining
        self.l.pop()
    }

    fn push(&self, val: T) {
        // append on the beggining
        let new = Node::new(val);
        let head = self.head();
        if !head.is_nil() {
            new.set_next(head);
        };
        self.l.first.replace(new);
    }
}

// first in - first out list
pub struct Fifo<T> {
    l: List<T>,
}

impl<T> Methods<T> for Fifo<T>
where
    T: Clone + Copy + Default,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&self) -> Option<T> {
        // pop from the beggining
        self.l.pop()
    }

    fn push(&self, val: T) {
        // append in the end
        if self.head().is_nil() {
            self.l.first.replace(Node::new(val));
            return;
        };

        let mut n = self.l.first.borrow_mut().clone();
        loop {
            let next = n.next();
            if next.is_none() {
                n.set_next(Node::new(val));
                break;
            };
            let nn = next.unwrap().borrow().clone();
            n = nn;
        }
    }
}

impl<T> Display for dyn Methods<T>
where
    T: Clone + Copy + Default + Display + 'static,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{} {}", self.impl_name(), self.head()))
    }
}

pub struct Ordered<T> {
    l: List<T>,
}

impl<T> Methods<T> for Ordered<T>
where
    T: Clone + Copy + Default + PartialOrd,
{
    fn size(&self) -> u8 {
        self.l.size()
    }

    fn head(&self) -> Rc<Node<T>> {
        self.l.head()
    }

    fn pop(&self) -> Option<T> {
        self.l.pop()
    }

    fn push(&self, val: T) {
        let new = Node::new(val);
        let mut node = self.head();

        if node.is_nil() {
            self.l.first.replace(new);
            return;
        } else if new <= node {
            new.set_next(self.head());
            self.l.first.replace(new);
            return;
        };

        loop {
            match node.next() {
                Some(next_ref) => {
                    let next = next_ref.borrow().clone();
                    if node < new && new <= next {
                        new.set_next(next);
                        node.set_next(new);
                        return;
                    };
                    node = next;
                }
                None => {
                    node.set_next(new);
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
    T: Clone + Copy + Default + std::cmp::PartialEq,
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
    T: Clone + Copy + Default + std::cmp::PartialEq,
{
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.l.first.borrow().clone())
    }
}
