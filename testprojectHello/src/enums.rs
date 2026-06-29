pub fn main(){
    enum Animal{
        fish(String),
        dog(String),
        cow(String),
        cat(String),
        Skibidit(String)
    }
    let swimmy: Animal=Animal::fish(String::from("glofish"));
    let barkbark: Animal=Animal::dog(String::from("Golden Retriver"));


    // printing out the breed of the dog
    match barkbark{
        Animal::fish(breed)=>println!("This is a {} fish", breed),
        Animal::dog(breed)=>println!("This is a {} dog", breed),
        _=>println!("This is neither a fish or a dog")
    }

    // printing out the breed of the fish
    match swimmy{
        Animal::fish(breed)=>println!("This is a {} fish", breed),
        Animal::dog(breed)=>println!("This is a {} dog", breed),
        _=>println!("This is neither a fish or a dog")
    }
}