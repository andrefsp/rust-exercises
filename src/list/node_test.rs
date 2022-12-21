use super::node::Node;

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
fn test_node_cmp() {
    let nil = Node::default();

    let v1 = Node::new(1);
    let v2 = Node::new(2);

    let v2_eq = Node::new(2);

    assert!(v2 > v1);
    assert_ne!(v1, v2);
    assert_eq!(v2, v2_eq);

    assert!(v2 > nil.into());
}

#[test]
fn test_node_reshuffle() {
    // 1 -> 5 -> 10
    let v1 = Node::new(1);
    let v5 = Node::new(5);
    let v10 = Node::new(10);

    v1.set_next(v5.clone());
    v5.set_next(v10.clone());

    // 1 -> 5 -> 7 -> 10
    let v7 = Node::new(7);

    v7.set_next(v10.clone());
    v5.set_next(v7);

    let v5 = v1.next().unwrap().borrow();
    assert_eq!(v5.get_value(), Some(5));

    let v7 = v5.next().unwrap().borrow();
    assert_eq!(v7.get_value(), Some(7));

    let v10 = v7.next().unwrap().borrow();
    assert_eq!(v10.get_value(), Some(10));
}
