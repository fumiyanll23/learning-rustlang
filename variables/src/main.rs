fn main() {
    let mut x = 5;
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {CONSTANT}");
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");

    let some_strings = "aaa";
    println!("The value of some_strings is: {some_strings}");

    let some_strings = some_strings.len();
    println!("The value of some_strings is: {some_strings}");
}
