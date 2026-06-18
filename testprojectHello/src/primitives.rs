pub fn main() {
    println!("Hello, rust from Cargo primitives!");

    // datatypes: int, float, bool, char
    let x:i32=-30; // can take both negative and positive
    let y:u32=30; //can only take positive values
    println!("{}",x);
    println!("{}",y);

    // float: f32 and f64
    let pi:f32=0.64;
    println!("The magic number is {}", pi);

    // booolean: bool
    let isTrue:bool=true;
    println!("Is it true?{}", isTrue);

    // characters: char
    let theChar:char='A';
    println!("The first letter of the alphabet is {}", theChar);

}


