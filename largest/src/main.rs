fn largest_unitype(list: &[i32]) -> i32 {
    let mut largest: i32 = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clones<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest: T = list[0].clone();

    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn largest_copyable<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list.iter() {
        if *item > *largest {
            largest = &item;
        }
    }

    largest
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let str_list = vec!["hello".to_string(), "world".to_string()];


    let result = largest_unitype(&number_list);
    println!("The largest number is {}", result);


    let result = largest_clones(&number_list);
    println!("The largest number is {}", result);
    let result = largest_clones(&char_list);
    println!("The largest char is {}", result);
    let result = largest_clones(&str_list);
    println!("The largest string is {}", result);


    let result = largest_copyable(&number_list);
    println!("The largest number is {}", result);
    let result = largest_copyable(&char_list);
    println!("The largest char is {}", result);
    // let result = largest_copyable(&str_list);
    // println!("The largest string is {}", result);


    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);
    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);
    let result = largest_ref(&str_list);
    println!("The largest string is {}", result);
}
