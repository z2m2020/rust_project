type MathOp = fn(i32, i32) -> i32;

fn math_op(op: &str) -> MathOp{
    match op {
        "add" => add,
        _ => subtract,
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let(x, y) = (8, 3);

    let  mut op = math_op("add");
    println!("operation result : {}", op(x, y));

    op = math_op("devide");
    println!("operation result : {}", op(x, y));
}