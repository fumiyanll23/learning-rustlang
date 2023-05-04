fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1.0;
    println!("add one result v1: {}", add_one_v1(1));
    println!("add one result v2: {}", add_one_v2(1));
    // println!("add one result v3: {}", add_one_v3(1));
    println!("add one result v3: {}", add_one_v3(1.0));

    let x = vec![1, 2, 3];
    let equal_to_x = |z| z == x;

    let y = vec![1, 2, 3];
    println!("equal to x({:?})?: {}", x, equal_to_x(y));
}
