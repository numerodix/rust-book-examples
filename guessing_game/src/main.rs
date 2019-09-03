use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn name(self: &User) -> String {
        self.username.clone()
    }

    fn neuw(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    fn nyah() -> Self {
        Self {
            email: String::from("nyah"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        }
    }
}

struct Empty {}

fn main() {
    let h = HashMap::new();

    let mut s = "bob".to_string();
    let mut ss = " bill".to_string();
    let s2 = s + &ss;
    println!("s2: {:?}", s2);

    let dev = "नमस्ते".to_string();
    println!("dev len: {:?}", dev.chars());



    let mut vec: Vec<String> = vec![];
    vec.push("five".to_string());
    vec.insert(0, "four".to_string());
    vec.insert(2, "last".to_string());

    for val in &mut vec {
        val.push('z');
    }
    println!("vec: {:#?}", vec);

    println!("Guess the number!");

    let some_u8_value = Some(7u8);

    if let Some(7) = some_u8_value {
        println!("oh goodie!");
    }

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let em = Empty {};

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: "bob".to_string(),
        ..user1.clone()
    };
    user2.username = "hank".to_string();

    let user3 = User::nyah();

    println!("user1: {:#?}", user1);
    println!("user2: {:?}", user2);
    println!("user3: {:?}", user3);

    loop {
        let pls = "Please input your guess.";
        println!("{}", pls);

        let mut guess_s = String::from("45");

        let sparkle_heart = vec![240, 159, 146, 150];
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

        // io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess_s.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        let word = first_word(pls);
        println!("First word: {}", pls.chars().nth(1).unwrap());

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        break;
    }
}