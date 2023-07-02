mod calculator;
use crate::calculator::{calculate};

fn main() {
    println!("Hello, world! Use my calculator");
    let a = 2;
    let b = 6;
    
    let res = calculate(a,b,"-");

    match res {
        Ok(number) => println!("{}", number),
        Err(err) => println!("{}", err)
    }

}


