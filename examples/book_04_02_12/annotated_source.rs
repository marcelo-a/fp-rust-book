fn no_dangle() -> String {
    let <tspan data-hash="1">s</tspan> = String::from("hello");

    <tspan data-hash="1">s</tspan>
}