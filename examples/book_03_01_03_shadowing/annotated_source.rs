fn main() {
    let <tspan data-hash="1">x</tspan> = 5;

    let <tspan data-hash="2">x</tspan> = <tspan data-hash="1">x</tspan> + 1;

    let <tspan data-hash="3">x</tspan> = <tspan data-hash="2">x</tspan> * 2;

    println!("The value of x is: {}", <tspan data-hash="3">x</tspan>);
}
