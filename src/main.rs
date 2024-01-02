fn main() -> () {
    let mut age: u8 = 20;

    println!("{}", age); // 印出 20

    if true {
        let a = 20;
    }

    // println!("{}", a); // cannot find value `a` in this scope

    const my_age: u8 = 10;
    println!("{}", my_age);
}
