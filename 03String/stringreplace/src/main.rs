fn main() {
    let s = String::from("aaabbbccaadd");
    let sbytes = s.bytes();
    for b in sbytes {
        print!("{} |", b);
    }
    let s1 = s.replacen("aa", "77", 1);
    let s2 = s.replace("bb", "55");
    let mut s3 = s;
    let s4 = s3.truncate(3);

    println!("s1 : {}", s1);
    println!("s2 : {}", s2.len());
    println!("s3 : {}", s3);

    let ss = String::from("lowe 老虎");
    let bytes = ss.bytes();
    for b in bytes {
        print!("{} | ",b);
    }
    println!();

    let  chars = ss.chars();
    for c in chars {
        print!("{} |", c);
    }
}
