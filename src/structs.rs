pub fn run(){

//     let mut c = Color(
//         255,
//         0,
//         0,
    

// );
//     println!("{}, {}, {}", c.0, c.1, c.2);


    struct Person{
        fname : String,
        lname : String
    }

    impl Person{

        fn new(f : &str, l : &str) -> Person {
            Person{
                fname : f.to_string(),
                lname : l.to_string(),
            }

        }
        fn fullname(&self)-> String
        {
            format!("{} {}", self.fname, self.lname)


        }

    }

    let p = Person::new("farhan", "ali");
    println!("{}, {}", p.fname, p.lname);
    println!("{}", p.fullname());



}

// struct Color{
//     red: u8,
//     green: u8,
//     blue: u8,

// }
// tuple struct

// struct Color(u8, u8, u8);
// struct with function
