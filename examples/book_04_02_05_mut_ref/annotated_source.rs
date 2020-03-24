fn main() {
    let mut <tspan data-hash="1">s</tspan> = <tspan data-hash="2">String::from</tspan>("hello");

    <tspan data-hash="3">change</tspan>(&amp;mut <tspan data-hash="1">s</tspan>);
}

fn change(<tspan data-hash="4">some_string</tspan>: &amp;mut String) {
    <tspan data-hash="4">some_string</tspan>.<tspan data-hash="5">push_str</tspan>(", world");
}
