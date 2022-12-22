use rust_exercises::list::List;
use rust_exercises::list::Methods;

fn main() {
    let vec = vec![1, 5, 2, 3];

    let lifo = List::lifo();
    let fifo = List::fifo();
    let ordered = List::ordered();

    for val in vec {
        lifo.push(val);
        fifo.push(val);
        ordered.push(val);
    }

    println!("lifo:    \t{} ", lifo.head());
    println!("fifo:    \t{} ", fifo.head());
    println!("ordered: \t{} ", ordered.head());
}
