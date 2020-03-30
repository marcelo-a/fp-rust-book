fn main() {
    let <tspan data-hash="1">s1</tspan> = <tspan data-hash="5">String::from</tspan>("hello");

    let <tspan data-hash="2">len</tspan> = <tspan data-hash="4">calculate_length</tspan>(<tspan data-hash="1">&amp;s1</tspan>);

    println!("The length of '{}' is {}.", <tspan data-hash="1">s1</tspan>, <tspan data-hash="2">len</tspan>);
}

fn <tspan data-hash="4">calculate_length</tspan>(<tspan data-hash="3">s</tspan>: &amp;String) -> usize {
    <tspan data-hash="3">s</tspan>.len()
}