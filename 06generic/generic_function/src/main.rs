fn foo<T>(x: T) -> T {
    return x;
}

fn main() {
    println!("{}", foo(5));
    println!("{}", foo("hello"));

}