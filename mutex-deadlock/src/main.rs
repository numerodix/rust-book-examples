use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Let's pretend User and Group are mutually linked and that adding a
// user to a group requires having a reference to both.
struct User {}
struct Group {}


fn main() {
    let user = Arc::new(Mutex::new(User {}));
    let group = Arc::new(Mutex::new(Group {}));

    let user_t1 = Arc::clone(&user);
    let group_t1 = Arc::clone(&group);
    let user_t2 = Arc::clone(&user);
    let group_t2 = Arc::clone(&group);

    // Take a lock on the user first, later on the group
    let t1 = thread::spawn(move || {
        println!("t1: Taking the user lock");
        if let Ok(_) = user_t1.lock() {
            println!("t1: Took the user lock, sleeping 5s");
            thread::sleep_ms(5000);

            loop {
                match group_t1.try_lock() {
                    Ok(_) => {
                        println!("t1: Took the group lock");
                        break;
                    },
                    Err(_) => { 
                        println!("t1: Failed to take the group lock, trying again");
                        thread::sleep_ms(1500);
                    },
                }
            }
        };
    });

    // Take a lock on the group first, later on the user
    let t2 = thread::spawn(move || {
        println!("t2: Taking the group lock");
        if let Ok(_) = group_t2.lock() {
            println!("t2: Took the group lock, sleeping 5s");
            thread::sleep_ms(5000);

            loop {
                match user_t2.try_lock() {
                    Ok(_) => {
                        println!("t2: Took the user lock");
                        break;
                    },
                    Err(_) => { 
                        println!("t2: Failed to take the user lock, trying again");
                        thread::sleep_ms(2000);
                    },
                }
            }
        };
    });

    t1.join().unwrap();
    t2.join().unwrap();
}