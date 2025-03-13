use std::io;

fn main(){
    let mut num_1=String::new();
    let mut num_2=String::new();
    println!("enter first number: ");
    io::stdin().read_line(&mut num_1).expect("fail information");
    println!("enter second number: ");
    io::stdin().read_line(&mut num_2).expect("fail information");

    let data1: i16 = num_1.trim().parse().expect("please enter a valid data");
    let data2: u16 = num_2.trim().parse().expect("please enter a valid data");
    println!("result: {}, result two: {}",data1,data2);
    let mut sum:i16 =data1+data2 as i16;
    println!("sum of two numbers {} + {} = {}",data1,data2,sum)
}