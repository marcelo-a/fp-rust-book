fn main() {
    let <tspan data-hash="1">s1</tspan> = String::from("hello");

    let (<tspan data-hash="3">s2</tspan>, <tspan data-hash="2">len</tspan>) = calculate_length(<tspan data-hash="1">&amp;s1</tspan>);
    
    println!("The length of '{}' is {}.", <tspan data-hash="3">s2</tspan>, <tspan data-hash="2">len</tspan>);
}

fn calculate_length(<tspan data-hash="4">s</tspan>: String) -> (String, usize) {
    let <tspan data-hash="5">length</tspan> = <tspan data-hash="4">s</tspan>.len(); // len() returns the length of a String

    (<tspan data-hash="4">s</tspan>, <tspan data-hash="5">length</tspan>)
}