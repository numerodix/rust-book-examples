#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::*;

fn main() {
    let list = Cons(3,
        Box::new(Cons(4,
            Box::new(Nil))));

    println!("{:?}", list);
}