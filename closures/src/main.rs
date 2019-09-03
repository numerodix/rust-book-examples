use std::collections::HashMap;
use std::hash::Hash;


struct Cacher<C, K, V>
    where 
        C: Fn(K) -> V,
        K: Eq + Hash + Clone,
        V: Clone,
{
    calculation: C,
    value: HashMap<K, V>,
}

impl<C, K, V> Cacher<C, K, V>
    where 
        C: Fn(K) -> V,
        K: Eq + Hash + Clone,
        V: Clone,
{
    fn new(calculation: C) -> Cacher<C, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &K) -> V {
        match self.value.contains_key(arg) {
            true => self.value.get(arg).unwrap().clone(),
            false => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg.clone(), v.clone());
                v
            },
        }
    }
}


fn main() {
    let mut c = Cacher::new(|a| a);
    c.value(&1);

    let mut c2 = Cacher::new(|a| a);
    let v3 = c2.value(&"abc");
    println!("{}", v3);
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(&1);
    let v2 = c.value(&2);

    assert_eq!(v2, 2);
}