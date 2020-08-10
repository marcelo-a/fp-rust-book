fn no_dangle() -> String {
    let <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="2">String::from</tspan>("hello");

    <tspan data-hash="1">s</tspan>
}