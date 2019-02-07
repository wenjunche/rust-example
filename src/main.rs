use std::env;

struct FullName {
    first: String,
    middle: Option<String>,
    last: String,
}

impl FullName {
    pub fn print(&self) {
//        match self.middle {
//            Some(ref middle) =>println!("middle {:?}", middle),
//            None => ()
//        }
        println!("first {:?}", self.first);
        println!("middle {:?}", self.middle.as_ref().unwrap());
        println!("last {:?}", self.last);
    }
}

fn main() {
    println!("Hello, world!");

    let _v1: Vec<i32> = Some(1).into_iter()
        .chain(None)
        .collect();

    println!("size  {:?}", _v1);

    env::var_os("SCCACHE_DIR").and_then(|dir| {
        println!("dir  {:?}", dir);
        Some(dir)
    });
    
    let jon = FullName {
        first: String::from("Jon"),
        middle: Some(String::from("Steve")),
        last: String::from("Snow")
    };
    jon.print();

}
