fn main() -> () {
    let age = 20;

    let message = if age < 8 {
        "小朋友"
    } else if age >= 8 && age < 18 {
        "年輕人"
    } else {
        "成年人"
    };

    println!("{}", message);
}
