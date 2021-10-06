pub fn run(){
// fixed array
//same datatpe
    //can not have value more or less than size
    let mut numbers : [i32;5] = [1, 2,3, 4,5]; 

    // reassign
    numbers[2] = 20;
    println!("{}", numbers.len());
//size in bytes
    println!("{} bytes", std::mem::size_of_val(&numbers));

    let slice : &[i32] = &numbers[0..2];
    println!("{:?}", &slice);

}