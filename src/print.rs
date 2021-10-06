pub fn run(){
    println!("Hello ");
    //can not print number directly
    println!("{}", 1);
    //basic frmating
    println!("My name is {} and i am from {}", "farhan", "Pakistan");
    //positional argument
    println!("My name is {0} and {0} is from {1}", "faryhan", "Pakistan");

    //named arguments
    println!("{name} likes to {activity}", name = "farhan", activity = "code" );
    //placeholder traits
    println!("Binary: {:b} Hex : {:x} Octal : {:o}", 10, 10, 10);
    //placeholder fro ebug traits
    println!("{:?}", (true, "hello", 1));

    //basic math
    println!("10 + 10 = {}", 10 + 10);

}