pub fn run(){
    //default i32
    let x = 1;
    //default f64
    let f = 2.5;

    //explicit

    let y : i64 = 123412341234; 
    println!("Max size i32 : {}", std::i32::MAX);

    //Boolean
    // let is_active = true;
    let is_greater : bool = 10 > 5;

    //char 
    // unicode
    let a1 = 'a';
    let face = '🤣';


    println!("{:?}", (is_greater, a1, face));

}


// statically typed