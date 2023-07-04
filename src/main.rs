use crate::queue::Queue;

mod queue;

fn main() {
    let mut a = Queue::new();

    a.enqueue(8);

    a.enqueue(16);

    a.enqueue(32);

    a.enqueue(64);

    a.enqueue(128);

    a.enqueue(256);

    let b = a.peek();

    println!("{a:?}\n{b:?}\n{}", a.len());

    a.dequeue();
    let b = a.peek();

    println!("{a:?}\n{b:?}\n{}", a.len());

    a.dequeue();

    println!("{a:?}\n{}", a.len());

    a.dequeue();

    println!("{a:?}\n{}", a.len());
}
