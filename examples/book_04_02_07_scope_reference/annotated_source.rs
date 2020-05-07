
let mut <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="4">String::from</tspan>("hello");

{
    let <tspan data-hash="2">r1</tspan> = <tspan data-hash="1">&amp;mut s</tspan>;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let <tspan data-hash="3">r2</tspan> = <tspan data-hash="1">&amp;mut s</tspan>;