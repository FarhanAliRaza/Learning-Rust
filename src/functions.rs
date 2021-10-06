pub fn run(){
    println!("{}",add(5, 5));


    //closure
    let n3 = 12;
    //can access ouside variables
    let add_nums = |n1 : i32, n2 : i32| n1 + n2 + n3;
    println!("{}", add_nums(3, 3));
}


fn add(n1 : i32, n2 : i32) -> i32{
    n1 + n2

}