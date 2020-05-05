fn main() {
    let mut <tspan data-hash="1">s</tspan> = String::from"hello");

    change<tspan data-hash="1">(&amp;mut s</tspan>);
}

fn change(<tspan data-hash="2">some_string</tspan>: &amp;mut String) {
    <tspan data-hash="2">some_string</tspan>.push_str(", world");
}
