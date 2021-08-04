fn main() {
    let add_one = |x: u32| -> u32 {x + 1};
    println!("{}", add_one(1));

    let add_another_one = |x| x + 1.1;
    println!("add_another_one : {}", add_another_one(2.0));
}