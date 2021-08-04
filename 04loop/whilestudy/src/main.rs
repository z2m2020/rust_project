fn main() {
    let mut counter = 0;
    let mut count = 0;
    while count != 10 {
        count += 1;
        counter = count * 2;
        println!("count: {},counter {}", count, counter);
    }
}
