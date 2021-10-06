pub fn run(){
    
    // and &&  or ||

    let age = 21;
    let check_id = false;

    if age >= 18 || check_id {
        println!("adult");

        
    }else{
        println!("nadult");

    }
    //short hand
    let k = if age > 18 { true } else { false };
    println!("{}", k)



}