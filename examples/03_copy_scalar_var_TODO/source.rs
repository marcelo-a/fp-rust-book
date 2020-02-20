fn fun(){
    let mut s = String::from("hello");

    let x = &s; // immut reference
    let y = &s; // no problem
    println!("{} and {}", x, y);
    // r1 and r2 are no longer used after this point

    let z = &mut s; // no problem
    println!("{}", z);
    
}

fn main(){
    fun();
}
