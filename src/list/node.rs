use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub enum Node<T> {
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
