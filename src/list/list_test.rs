use super::list::List;
use super::list::Methods;

#[test]
fn test_lifo_push_and_size() {
    let mut l = List::lifo();

    l.push(1.0);
    l.push(3.0);
    l.push(2.0);

    assert_eq!(l.size(), 3);

    l.pop();
    assert_eq!(l.size(), 2);
    l.pop();
    l.pop();
    assert_eq!(l.size(), 0);

    // pop once more to check for possible errors
    l.pop();
    assert_eq!(l.size(), 0);
}

#[test]
fn test_lifo_push_and_pop() {
    let mut l = List::lifo();

    l.push(1);
    l.push(3);

    assert_eq!(l.pop().unwrap(), 3);
    assert_eq!(l.pop().unwrap(), 1);
    assert_eq!(l.pop(), None);
}

#[test]
fn test_list_push_and_iterate_float32() {
    let mut l = List::lifo();

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
    let mut l = List::lifo();

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
    let mut l = List::fifo();

    l.push(1);
    l.push(3);
    l.push(2);

    assert_eq!(l.pop(), Some(1));
    assert_eq!(l.pop(), Some(3));
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), None);
}

#[test]
fn test_ordered_push_and_pop() {
    let mut l = List::ordered();

    l.push(3);
    l.push(1);
    l.push(4);
    l.push(2);

    assert_eq!(l.pop(), Some(1));
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), Some(3));
    assert_eq!(l.pop(), Some(4));
    assert_eq!(l.pop(), None);
}
