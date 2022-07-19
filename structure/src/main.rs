struct User{
    name:String,
    age:i16,
    height:i16,
}

fn main() {
    let a = User{
        name: String::from("john"),
        age:  18,
        height: 150,
    };
    println!("a:{}, {}, {}", a.name, a.age, a.height);
}
