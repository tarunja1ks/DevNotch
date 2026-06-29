pub fn main(){
    let mut dog: animal=animal{
        species: String::from("dog"),
        age: 5,
        is_mammal:true
    };


    println!("This is the age old {}", dog.age);
    let mut dog: animal=increase_age(dog); // NEXT TIME USE PASSING OF MUTATIONS
    println!("This is the age new {}", dog.age);


    //tuple struct
    struct Ages(i32,i32,i32,i32);
    let dogAges: Ages=Ages(10, 3, 4, 5);
    println!("{}, {}, {}", dogAges.0,dogAges.1,dogAges.2);

    // unit struct
    struct is_a_dog;
    let animal:is_a_dog=is_a_dog;

}

pub fn increase_age(mut Animal: animal) -> animal{
    Animal.age+=1;
    Animal
}


struct animal{
    species: String,
    age:i32,
    is_mammal: bool
}