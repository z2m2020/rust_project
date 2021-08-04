use std::collections::HashMap;
fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();

    let zhangsan1 = map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 10);

    println!("{:?}", zhangsan1);
    println!("{:?}", map);

    let zhangsan2 = map.insert("zhangsan", 79);
    println!("{:?}", zhangsan2);
    println!("{:?}", map);

    map.insert("zhangsan", 20);
    println!("{:?}",map);

    let result = map.remove("zhangsan");
    println!("result: {:?}", result);
    println!("map : {:?}", map);

    println!("wangwu : {:?}", map.get("wangwu"));
    println!("lisi : {:?}", map.get("lisi"));
    println!("zhangsan  : {:?}", map.get("zhangsan"))


}
