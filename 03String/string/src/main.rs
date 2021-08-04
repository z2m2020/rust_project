fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(",");
    let s3 =String::from("Rust ");
    let s4 = String::from("World !");
    let s5 = "world";

    let mut s = s1 + &s2 + s3.as_str() + s5;

    println!("s : {}", s);
}
