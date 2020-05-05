fn main() {
    let <tspan data-hash="1">s1</tspan> = String::from("hello");

    let <tspan data-hash="2">len</tspan> = calculate_length(<tspan data-hash="1">&amp;s1</tspan>);

    println!("The length of '{}' is {}.", <tspan data-hash="1">s1</tspan>, <tspan data-hash="2">len</tspan>);
}

fn calculate_length(<tspan data-hash="3">s</tspan>: &amp;String) -> usize {
    <tspan data-hash="3">s</tspan>.len()
}