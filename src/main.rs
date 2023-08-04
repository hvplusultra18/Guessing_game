use std::{io, cmp::Ordering};

use rand::Rng;
fn main() {
    println!("guess the number");
    loop{
    println!("enter your guess:");
let mut guess= String::new();
let secret_number=rand::thread_rng().gen_range(0..=100);
    io::stdin()
    .read_line(&mut guess)
    .expect("failure to read line");
    println!("your guess is {guess}");
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    
    match guess.cmp(&secret_number) {
        Ordering::Equal=>{println!("correct");break},
        Ordering::Greater=>println!("you are bigger"),
        Ordering::Less=>{ println!("you are smaller");
    }
    }
}
    
    
}


