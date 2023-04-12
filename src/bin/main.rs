use rust_exercises::list::List;
use rust_exercises::list::Methods;

fn main() {
    let values = vec![1, 5, 2, 3];

    let lifo = List::lifo();
    let fifo = List::fifo();
    let ordered = List::ordered();

    let lists: Vec<Box<dyn Methods<_>>> = vec![lifo, fifo, ordered];

    for val in values {
        for list in &lists {
            list.push(val);
        }
    }

    for list in &lists {
        println!("{}", list);
    }
}
