

pub fn main(){
    let mut dog_ages:Vec<i32>=Vec::new(); // defining an empty vector

    let mut dog_ages:Vec<i32>=Vec::with_capacity(6);// defining a vector with capacity

    let mut dog_ages=vec![3,6,9,7,7,8]; // defining a vector with numbers in it

    println!("Here is the original vector: {:?}", dog_ages);

    dog_ages.pop();
    dog_ages.push(4);
    dog_ages.insert(0,1);

    println!("Here is the final vector: {:?}", dog_ages);


}