pub fn run(){
    //growable arrays
    // fixed array
    //same datatpe
        //can not have value more or less than size
        let mut numbers : Vec<i32> = vec![1, 2,3, 4,5]; 
    
        // reassign
        numbers[2] = 20;
        println!("{}", numbers.len());
        //size in bytes
        println!("{} bytes", std::mem::size_of_val(&numbers));
        //addtovector
        numbers.push(5);
        //pop last value
        println!("{:?}", numbers);

        numbers.pop();
        // let slice : &[i32] = &numbers[0..2];
        println!("{:?}", numbers);

        for x in numbers.iter(){
            println!("{}", x)
        }

        //also mut values
        for x in numbers.iter_mut(){
            *x *= 2;
        }
        println!("{:?}", numbers);

    }