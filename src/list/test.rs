use super::list::Node;
use super::list::{Fifo, Lifo, Methods};
use std::rc::Rc;

#[test]
fn test_list_push_and_iterate_float32() {
    let mut l = Lifo::new();

    l.push(1.0);
    l.push(3.0);
    l.push(2.0);

    assert_eq!(l.size(), 3);

    let mut iter = l.into_iter();
    assert_eq!(iter.next(), Some(2.0));
    assert_eq!(iter.next(), Some(3.0));
    assert_eq!(iter.next(), Some(1.0));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_list_push_and_iterate_u8() {
    let mut l = Lifo::new();

    l.push(1);
    l.push(3);
    l.push(2);

    assert_eq!(l.size(), 3);

    let mut iter = l.into_iter();
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_lifo_push_and_pop() {
    let mut l = Lifo::new();

    l.push(1);
    l.push(3);

    assert_eq!(l.pop().unwrap(), 3);
    assert_eq!(l.pop().unwrap(), 1);
    assert_eq!(l.pop(), None);
}

#[test]
fn test_fifo_push_and_pop() {
    let mut l = Fifo::new();

    l.push(1);
    l.push(3);

    assert_eq!(l.pop().unwrap(), 1);
    assert_eq!(l.pop().unwrap(), 3);
    assert_eq!(l.pop(), None);
}

#[test]
fn test_node_get_value() {
    let v1 = Node::new(1);
    assert_eq!(v1.get_value().unwrap(), 1);
}

#[test]
fn test_node_set_next() {
    let mut v1 = Node::new(1);

    v1.set_next(Rc::new(Node::new(2)));

    assert_eq!(v1.next().unwrap().get_value().unwrap(), 2);
}
