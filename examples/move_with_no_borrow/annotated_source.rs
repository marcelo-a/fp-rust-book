fn fun() {
    let <tspan data-hash="2">y</tspan> = String::from("y");
    let <tspan data-hash="3">z</tspan> = String::from("z");
    let <tspan data-hash="1">x</tspan> = <tspan data-hash="2">y</tspan>;
    println!("{}", <tspan data-hash="1">x</tspan>); // pretend nothing happens inside a macro
    // println!("{}", y);       // Uncomment this line incurs error[E0382]: borrow of moved value: `<tspan data-hash="2">y</tspan>`
    // println!("{}", z);       // Uncomment this line incurs error[E0382]: borrow of moved value: `<tspan data-hash="2">y</tspan>`

}

fn main() {
    fun();
}