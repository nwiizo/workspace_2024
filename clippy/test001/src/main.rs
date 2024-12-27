fn sum_squares(values: &Vec<i32>) -> i32 {
    values.iter().fold(0, |acc, value| acc + value * value)
}

fn main() {
    sum_squares(&vec![1, 2, 3]);
    println!("Hello, world!");
}
