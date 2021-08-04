fn main() {
    let v = [1, 2, 3];
    let mut iter = v.iter();

    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());
    println!("{:?}",iter.next());

    // sum()
    let total: i32 = v.iter().sum();
    println!("total: {}", total);

    // any()
    let v2 = [1, 3, 4, 5];
    let result1 = v2.iter().any(|&x| x == 4);
    let result2 = v2.iter().any(|x| *x == 4);

    println!("result1 : {}", result1);
    println!("result2 : {}", result2);

    // collect()

    let v3 : Vec<i32> = v2.iter().map(|x| x + 1).collect();
    println!("v3: {:?}", v3);

    //take()

    let result3 = v2.iter().take(2);

    for i in result3 {
        println!("element : {}", i);
    }

    // filter()
    let v4 = [1, 2, 3];
    let result: Vec<i32> = v4.iter()
        .map(|x| x + 3)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("result : {:?}", result);

    // rev()
    let result5 = v4.iter().rev();
    println!("reved result : {:?}", result5);
    for i in result5 {
        println!("{}", i)
    }

    // zip()
    let v5 = [1, 2, 3];
    let v6 = [2, 3, 6];

    let result6: Vec<i32> = v5.iter().zip(v6.iter())
        .map(|(a, b)| a + b)
        .filter(|x| x % 3 == 0)
        .collect();

    println!("result6 : {:?} ", result6);
}