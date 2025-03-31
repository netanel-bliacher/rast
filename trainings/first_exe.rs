use std::io;

fn main(){
    let num: i32 = 123;
    let test: bool;
    test = if num > 0 { true } else { false };
    if test {
        println!("Positive");
    } else {
        println!("Negative");
    }
}