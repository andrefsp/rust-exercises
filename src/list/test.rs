use std::rc::Rc;

use super::list::Lifo;
use super::list::Methods;
use super::list::Node;

#[test]
fn test_node_get_value() {
    let v1 = Node::new(1);
    assert_eq!(v1.get_value().unwrap(), 1);

    assert!(v1.next().is_none());
}

#[test]
fn test_node_set_next() {
    let v1 = Node::new(1);
    let v2 = Node::new(2);
    v1.set_next(v2);

    assert_eq!(v1.next().unwrap().borrow().get_value().unwrap(), 2);
}
#[test]
fn test_node_get_next_and_set_next() {
    let v1 = Node::new(1);
    assert_eq!(v1.get_value().unwrap(), 1);

    let v2 = Node::new(2);
    v1.set_next(v2);

    let v2 = v1.next().unwrap().borrow();
    assert_eq!(v2.get_value().unwrap(), 2);

    let v3 = Node::new(3);
    v2.set_next(v3);

    let v3 = v2.next().unwrap().borrow();
    assert_eq!(v3.get_value().unwrap(), 3);
}

#[test]
fn test_node_get_next_and_set_next_with_list() {
    // 1 -> 2 -> 3
    let v1 = Node::new(1);
    let v2 = Node::new(2);
    let v3 = Node::new(3);

    v1.set_next(v2.clone());
    v2.set_next(v3);

    // insert in the beggining...
    let v0 = Node::new(0);
    v0.set_next(v1);

    assert_eq!(v0.get_value(), Some(0));
    let v1 = v0.next().unwrap().borrow();
    assert_eq!(v1.get_value(), Some(1));

    let v2 = v1.next().unwrap().borrow();
    assert_eq!(v2.get_value(), Some(2));

    let v3 = v2.next().unwrap().borrow();
    assert_eq!(v3.get_value(), Some(3));
}

#[test]
fn test_lifo_push_and_size() {
    let mut l = Lifo::new();

    l.push(1.0);
    l.push(3.0);
    l.push(2.0);

    assert_eq!(l.size(), 3);
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

/*
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
fn test_fifo_push_and_pop() {
    let mut l = Fifo::new();

    l.push(1);
    l.push(3);

    assert_eq!(l.pop().unwrap(), 1);
    assert_eq!(l.pop().unwrap(), 3);
    assert_eq!(l.pop(), None);
}
*/
