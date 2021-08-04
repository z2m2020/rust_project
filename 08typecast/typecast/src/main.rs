fn main() {
    let x: u16 = 7;
    let y = x as u32;
    println!("u16: {}, u32: {}", x, y);

    let x = std::u32::MAX;
    let y = x as u16;
    println!("u32: {}, u16: {}", x, y);

    let x = 65u8;
    let y = x as char;
    println!("u8: {},char: {}", x, y);

    let x = 'A';
    let y = x as u8;
    println!("char: {}, u8:{}", x, y);

    let x = 7 ;
    let y = x as f64;
    println!(" i32: {}, f64: {}", x, y);

    let x = 7.7;
    let y = x as i32;
    println!("f64: {}, i32: {}", x, y);
}