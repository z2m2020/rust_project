fn hello() {
    println!("hello man!");
}

fn main() {
    let fn_ptr: fn() = hello;
    println!("{:p}", fn_ptr);

    let other_fn = hello; // other_fn 是函数 hello 本身, fn() {hello}
//    println!("{:p}", other_fn);
    fn_ptr();
    other_ptr();
}