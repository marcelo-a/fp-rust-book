fn main() {
    let <tspan data-hash="1">x</tspan> = <tspan data-hash="2">plus_one</tspan>(5);

    println!("The value of x is: {}", <tspan data-hash="1">x</tspan>);
}

fn <tspan data-hash="2">plus_one</tspan>(<tspan data-hash="3">x</tspan>: i32) -> i32 {
    <tspan data-hash="3">x</tspan> + 1;
}