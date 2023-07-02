mod calculator;
use crate::calculator::{calculate};

fn main() {
    println!("Hello, world!");
    let number: i8 = 0;
    let a = 2;
    let b = 6;
    
    let res = calculate(a,b,"-");

    match res {
        Ok(number) => println!("{}", number),
        Err(err) => println!("{}", err)
    }

}


