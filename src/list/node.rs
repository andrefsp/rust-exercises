use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Debug, Default, PartialOrd)]
pub enum Node<T: Clone + Copy> {
    #[default]
    Nil,
    Content {
        value: T,
        next: RefCell<Rc<Node<T>>>,
    },
}

impl<T> Node<T>
where
    T: Clone + Copy,
{
    pub fn new(val: T) -> Rc<Node<T>> {
        Rc::new(Node::Content {
            value: val,
            next: RefCell::new(Node::nil()),
        })
    }

    pub fn nil() -> Rc<Node<T>> {
        Rc::new(Node::default())
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Node::Nil => true,
            _ => false,
        }
    }

    pub fn next(&self) -> Option<&RefCell<Rc<Node<T>>>> {
        match self {
            Node::Content { value: _, next } => {
                let node = next.borrow();
                match node.is_nil() {
                    false => Some(next),
                    true => None,
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

impl<T> PartialEq for Node<T>
where
    T: Clone + Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl<T> Display for Node<T>
where
    T: Clone + Copy + Display,
{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Node::Nil => fmt.write_str("nil"),
            Node::Content { value, next } => {
                fmt.write_fmt(format_args!("[ {} -> {}]", value, next.borrow()))
            }
        }
    }
}
