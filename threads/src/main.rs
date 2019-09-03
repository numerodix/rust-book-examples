use std::thread;
use std::time::Duration;

fn main() {
    let vec = vec![1, 2];

    let handle = thread::spawn(move || {
        println!("vec {:?} from the spawned thread!", vec);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}