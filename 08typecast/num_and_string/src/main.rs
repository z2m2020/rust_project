fn main() {
    let x = 7 ;
    let y = x.to_string();
    println!("i32: {}, string: {} ", x, y);

    let x = 7.7;
    let y = x.to_string();
    println!("f64: {}, string: {}", x, y);

    let x = String::from("7");
    let y = x.parse::<i32>().unwrap();
    println!("string: {}, i32: {}", x, y);

    let x = String::from("7.7");
    let y = x.parse::<f64>().unwrap();
    println!("string: {}, f64: {}", x, y);

    let x = String::from("hello world");
    let y = x.as_str();

    let x = "hello";
    let y = x.to_string();
}