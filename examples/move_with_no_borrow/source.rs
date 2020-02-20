fn fun() {
    let y = String::from("y");
    let z = String::from("z");
    let x = y;
    let t = & x;
    println!("{}", x); // pretend nothing happens inside a macro
    // println!("{}", y);       // Uncomment this line incurs error[E0382]: borrow of moved value: `y`
    // println!("{}", z);       // Uncomment this line incurs error[E0382]: borrow of moved value: `z`

}

fn main() {
    fun();
}