#[derive(Default, Debug)]
struct Mystruct {
    foo: i32,
    boo: i32,
}

fn main() {
    let options1: Mystruct = Default::default();
    let options2 = Mystruct { foo: 7, ..Default::default() };

    println!("option1: {:?}", options1);
    println!("option2: {:?}", options2);
}