use core::num;

use rand::{Rng, RngExt};
pub fn main(){
    match i_love_errors(){
        Some(val) => println!("{}",val),
        None => println!("You got an odd number")
    }
    match i_love_errors_resultversion(){
        Ok(val) => println!("{}",val),
        Err(e) => println!("{}",e)
    }
}

fn i_love_errors()->Option<String>{
    let mut random_gen=rand::rng();
    let number_using=random_gen.random_range(1..100);
    // println!("{}",number_using);
    if(number_using%2==0){
        Some(String::from("Good job you got an even"))
    }
    else{
     None
    }
}

fn i_love_errors_resultversion()->Result<String, String>{
    let mut random_gen=rand::rng();
    let number_using=random_gen.random_range(1..100);
    // println!("{}",number_using);
    if(number_using%2==0){
        Ok(String::from("Good job you got an even"))
    }
    else{
        Err(String::from("Now you have an odd womp womp"))
    }
}