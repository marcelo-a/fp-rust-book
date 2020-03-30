fn main() {
    let <tspan data-hash="1">s1</tspan> = <tspan data-hash="7">String::from</tspan>("hello");

    let (<tspan data-hash="3">s2</tspan>, <tspan data-hash="2">len</tspan>) = <tspan data-hash="6">calculate_length</tspan>(<tspan data-hash="1">&amp;s1</tspan>);
    
    <tspan data-hash="9">println!</tspan>("The length of '{}' is {}.", <tspan data-hash="3">s2</tspan>, <tspan data-hash="2">len</tspan>);
}

fn <tspan data-hash="6">calculate_length</tspan>(<tspan data-hash="4">s</tspan>: String) -> (String, usize) {
    let <tspan data-hash="5">length</tspan> = <tspan data-hash="4">s</tspan>.<tspan data-hash="8">len</tspan>(); // len() returns the length of a String

    (<tspan data-hash="4">s</tspan>, <tspan data-hash="5">length</tspan>)
}