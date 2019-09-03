fn main() {
    let mut vec = vec![1, 2, 3];

    for item in vec.iter_mut().skip(1) {    
        *item += 7;
        println!("{}", *item);
    }

    let k = 2;
    let vec2: Vec<_> = vec.iter().map(|n| n + k).collect();
    println!("{:?}", vec2);

    for item in vec.into_iter() {
        println!("{}", item);
    }

    // println!("{:?}", vec);
}
