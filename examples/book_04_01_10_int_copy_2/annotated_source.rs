fn main() {
    let <tspan data-hash="1">x</tspan> = 5;
    let <tspan data-hash="2">y</tspan> = <tspan data-hash="1">x</tspan>;

    println!("x = {}, y = {}", <tspan data-hash="1">x</tspan>, <tspan data-hash="2">y</tspan>);
}
