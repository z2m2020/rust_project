fn match_value(value: Option<i32>) {
    match value {
        Some(7) => println!("match_value : seven"),
        _ => (),
    }
}

fn if_let_value(value: Option<i32>) {
    if let Some(7) = value {
        println!("if_let : seven");
    }
}

fn match_vec() {
    let mut vec = vec![1, 2, 3, 4, 5];
    loop {
        match vec.pop() {
            Some(value) => println!("{}", value),
            None => break,
        }
    }
}

fn while_let_vec() {
    let mut vec = vec![1, 2, 3, 4, 5];
    while let Some(value) = vec.pop(){
        println!("{}", value);
    }
}

fn main() {
    if_let_value(Some(7));
    match_value(Some(7));
    match_vec();
    while_let_vec();
    println!("Hello, world!");
}
