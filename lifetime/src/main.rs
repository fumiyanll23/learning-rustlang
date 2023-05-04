#[derive(Debug)]
struct SomeStruct<'a> {
    part: &'a str,
}

fn main() {
    let s;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        s = SomeStruct { part: &novel };
        println!("{:?}", &s);
    }
    // println!("{:?}", &s);
}
