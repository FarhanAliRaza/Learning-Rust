
//hold primitive data or reference to data
//immutable by default
// Rust is block scoped

pub fn run(){
    let name = "Brad";
    // let age = 37;
    // let mut agemut = 37;
    let agemut = 38;
    const ID: i32 = 001;

    // is not mutable
    // age = 38; do not allow 
    println!("name {} age {}", name, agemut);
    println!("ID: {}", ID);
    //assign multiple vars
    // let (name2, mage) = ("ali", 1);

}