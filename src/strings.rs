
//str Immutable fixed length strings
// String Growable heap allocated data structure use when 
// need to modify data 


pub fn run(){
    // default type str
    let _hello = "hello";

    let hello2 = String::from("Hello World");
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    //length
    // println!("{}", hello2.len());
// push char
    // hello2.push('✅');
    // only for string
    // hello2.push_str("Hello World");
    // println!("{}", hello2);

    //Capicity in bytes

    // println!("{}", hello2.capacity());
    // println!("{}", hello2.is_empty());

    //contains

    // println!("{}", hello2.contains("hello"));

    // loop through strings
    for word in hello2.split_whitespace(){
        println!("{}", word);
    }










}