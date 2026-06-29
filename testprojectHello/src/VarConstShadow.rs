pub fn main(){

    // constants
    const x: i32=3;
    const y: i32=3;
    println!("This is the sum: {}",addition(x,y));

    // variables

    let nameofme: &str="Tarun";
    let mut lastnameofme= String::new();
    lastnameofme.push_str("Jai");
    lastnameofme.push_str("kumar");

    println!("This is my name {}{}", nameofme, lastnameofme);


    // shadowing: redefining a variable

    let a: i32=4;
    let b: i32=x+1;
    let b: i32=b+2;
    let a: i32=a+b;

    println!("Here are 2 variables {}, {}", a, b);

}



fn addition(x: i32,  y: i32)->i32{
    x+y
}