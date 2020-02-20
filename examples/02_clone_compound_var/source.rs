fn fun() {
    let y = String::from("y");
    let z = String::from("z");
    let a = y.clone();
    println!("{}", a); 
    println!("{}", y); 
    let b = z.clone();
    println!("{}", b); 
    println!("{}", z);

}

fn main() {
    fun();
}
