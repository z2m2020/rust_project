fn main() {
    let mut count = 0;
    let counter = loop {
        count += 1;
        let counter1 = 0;
//        let counter = count * 2;
        println!("count: {}, counter: {}", count, counter1);
        let counter = count * 2;

        if count == 10 {
            break counter;
        }
    };

    println!("Counter after looop : {}", counter1);
}
