type MathOp = fn(i32, i32) -> i32;
fn math(op: MathOp, x: i32, y: i32) -> i32 {
    println!("{:p}", op);
    op(x, y)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let (x, y) = (8 , 3);
    println!("add operation result : {}", math(add, x, y));
    println!(" subtract operation result : {} ", math( subtract, x, y));
}