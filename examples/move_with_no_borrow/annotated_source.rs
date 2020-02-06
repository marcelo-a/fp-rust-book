fn fun() {
    let y = String::from("y");
    let z = String::from("z");
    let <tspan data-hash="1">x</tspan> = y;
    println!("{}", <var data-hash="1">x<\var>); // pretend nothing happens inside a macro
    // println!("{}", y);       // Uncomment this line incurs error[E0382]: borrow of moved value: `y`
    // println!("{}", z);       // Uncomment this line incurs error[E0382]: borrow of moved value: `z`
}

fn main() {
    fun();
}