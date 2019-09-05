use std::ffi::{OsStr, OsString};
use std::path::Path;


fn main() {
    // let mut s = OsString::from(String::from("hi"));
    let mut s = OsString::from("hi");
    s.push(" jimmy");
    // let bs = s.as_os_str();
    let bs: &OsStr = &s;

    let p = Path::new(&bs);

    println!("s {:?}", s);
    println!("bs {:?}", bs);
}
