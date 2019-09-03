#[derive(Debug)]
struct Wrapper<T>(Vec<T>);

impl<T> std::ops::Deref for Wrapper<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut w = Wrapper(vec![String::from("hello my son"), String::from("world")]);
    w.push("last".to_string());
    let l = w.iter().map(|s| s.len()).collect::<Vec<_>>();

    let w2 = w.iter().map(|s| s.to_string() + "!").collect::<Vec<_>>();

    println!("w = {:?}", w);
    println!("l = {:?}", l);
    println!("w2 = {:?}", w2);
}