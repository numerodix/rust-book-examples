use add_one;
use rand;
use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("id {}", add_two::id(3));
}
