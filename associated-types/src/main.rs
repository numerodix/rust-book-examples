struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.count;
        self.count += 1;
        Some(result)
    }
}


trait GenIterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl GenIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        let result = self.count;
        self.count += 1;
        Some(result)
    }
}

impl GenIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        let result = self.count;
        self.count += 1;
        Some(result.to_string())
    }
}


fn main() {
    let mut counter = Counter::new();
    for _ in 0..5 {
        println!("cnt: {:?}", Iterator::next(&mut counter));
    }

    for _ in 0..5 {
        println!("cnt: {:?}", <Counter as GenIterator<u32>>::next(&mut counter));
    }

    for _ in 0..5 {
        println!("cnt: {:?}", <Counter as GenIterator<String>>::next(&mut counter));
    }
}
