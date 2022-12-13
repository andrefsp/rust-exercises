use super::list::List;
use super::list::Node;
use std::rc::Rc;

#[test]
fn test_value_next() {
    let mut l = List::new();

    l.push(1);
    l.push(3);
    l.push(2);

    let mut iter = l.into_iter();
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
}

#[test]
fn test_get_value() {
    let v1 = Node::new(1);
    assert_eq!(v1.get_value().unwrap(), 1);
}

#[test]
fn test_set_next() {
    let mut v1 = Node::new(1);
    let v2 = Node::new(2);

    v1.set_next(Rc::new(v2));

    assert_eq!(v1.next().unwrap().get_value().unwrap(), 2);
}
