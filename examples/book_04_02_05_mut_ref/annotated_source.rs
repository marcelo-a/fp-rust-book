fn main() {
    let mut <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="4">String::from</tspan>("hello");

    <tspan class="fn" data-hash="0" hash="3">change</tspan>(<tspan data-hash="1">&amp;mut s</tspan>);
}

fn <tspan class="fn" data-hash="0" hash="3">change</tspan>(<tspan data-hash="2">some_string</tspan>: &amp;mut String) {
    <tspan data-hash="2">some_string</tspan>.<tspan class="fn" data-hash="0" hash="5">push_str</tspan>(", world");
}
