fn main() -> () {
    let list = [1450, 9527, 5566];

    for item in list.iter() {
        println!("{}", item);
    }

    println!("{:?}", list);

    let pet = ('🐈', false, 18);
    println!("{} {} {}", pet.0, pet.1, pet.2)
}
