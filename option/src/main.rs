// Implemented in Enum std::option::Option,
// enum Option<T> {
//     Some(T),
//     None,
// }
fn main() {
    let mut maybe_number = Some(5);
    println!("{:?}", maybe_number);
    maybe_number = None;
    println!("{:?}", maybe_number);
}
