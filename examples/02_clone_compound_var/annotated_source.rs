fn fun() {
    let <tspan data-hash="1">y</tspan> = String::from("y");
    let <tspan data-hash="2">z</tspan> = String::from("z");
    let <tspan data-hash="3">a</tspan> = <tspan data-hash="1">y</tspan>.clone();
    println!("{}", <tspan data-hash="3">a</tspan>); 
    println!("{}", <tspan data-hash="1">y</tspan>); 
    let <tspan data-hash="4">b</tspan> = <tspan data-hash="2">z</tspan>.clone();
    println!("{}", <tspan data-hash="4">b</tspan>); 
    println!("{}", <tspan data-hash="2">z</tspan>); 
}

fn main() {
    fun();
}
