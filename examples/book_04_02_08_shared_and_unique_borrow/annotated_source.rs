fn fun(){
    let mut <tspan data-hash="1">s</tspan> = String::from("hello");

    let <tspan data-hash="2">x</tspan> = <tspan data-hash="1">&amp;s</tspan>; // no problem
    let <tspan data-hash="3">y</tspan> = <tspan data-hash="1">&amp;s</tspan>; // no problem
    println!("{} and {}", <tspan data-hash="2">x</tspan>, <tspan data-hash="3">y</tspan>);
    // x and y are no longer used after this point

    let <tspan data-hash="4">z</tspan> = <tspan data-hash="1">&amp;mut s</tspan>; // no problem
    println!("{}", <tspan data-hash="4">z</tspan>);

}

fn main(){
    fun();
}
