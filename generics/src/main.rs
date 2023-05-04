use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let numbers_list = vec![34, 50, 25, 100];

    println!("The largest number is {}", largest(&numbers_list));

    let numbers_list = vec![100, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&numbers_list));

    let numbers_list = vec![100.2, 34.5, 6000.9, 89.1, 54.989, 2.2, 413.2, 8.0];

    println!("The largest number is {}", largest(&numbers_list));
}
