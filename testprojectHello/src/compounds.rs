pub fn main(){
    //arrays
    let statusbit: [i32;8]=[0,1,0,0,1,0,0,0];
    println!("These are the zucky zuck statusbits {:?}", statusbit);
    

    //tuples(both with and without the definitions work)
    let human:(&str,i32,bool)=("Bob",10,false);
    println!("Here is the human {:?}",human);
    let human=("Bob",10,false);
    println!("Here is the human {:?}",human);

    //strings
    let fruits:[&str;3]=["Apple","Bananana","Orange"]; //&str is a reference to the String datatype
    println!("Here are some fruits {:?}", fruits);

    let fruit=String::from("Orange");
    println!("Here is a vitamin C ahh fruit: {}", fruit);

    let mut changingfruit=String::from("Orange"); // you need mut in order to change also u can only modify String muts
    changingfruit.push_str(" and a Apple");
    println!("Here is a vitamin C and Vitamin Apple ahh fruits: {}", changingfruit);
    println!("Here is Vitamin Apple ahh fruit: {}", &changingfruit[13..changingfruit.len()]);//segment of the overall string is printed
    



    // slices
    let tester: &[&str]=&["thing","thing2","thing3"];
    let tester2: &[i32]=&[1,2,3,4,5];
    println!("Here are the 2 arrays {:?} {:?}", tester, tester2);

    
    

}