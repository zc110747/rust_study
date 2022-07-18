
fn main() {

    //move and clone
    let s0:String = String::from("start");
    let s1 = s0;
    let s2 = s1.clone();

    //println!("{s0}"); //error, s0 have been moved
    println!("s1:{s1}, s2:{s2}");

    ownership_change(s1);
    //println!("{s1}, {s2}"); //error, s2 have been moved

    let s3 = get_ownership(s2);
    println!("s3:{s3}");
    let s4 = format!("format, {s3}");
    println!("s4:{s4}");

    //copy trait
    //整型，布尔型，浮点型，
    //字符型，元组，数组
    let x:char  = 'a';
    let y = x;
    println!("{x}, {y}");

    //{:?} shows compositon type
    //{:#?} shows compositon type with wrap
    let a:[i16;2] = [1, 2];
    let a1 = a;
    println!("{:#?}, {:#?}", a, a1);

    let t:(u8, u16, [i32;2]) = (1, 5, [2, 4]);
    let t1 = t;
    println!("{:?}, {:?}", t, t1);


}

fn ownership_change(s:String){
    println!("s:{s}")
}

fn get_ownership(s:String) -> String{

    let s2 = String::from(" Get");
    s + &s2
}