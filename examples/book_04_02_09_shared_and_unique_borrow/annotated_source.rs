fn main(){
    let mut <tspan data-hash="1">s</tspan> = <tspan data-hash="5">String::from</tspan>("hello");

    let <tspan data-hash="2">r1</tspan> = <tspan data-hash="1">&amp;s</tspan>; // no problem
    let <tspan data-hash="3">r2</tspan> = <tspan data-hash="1">&amp;s</tspan>; // no problem
    <tspan data-hash="6">println!</tspan>("{} and {}", <tspan data-hash="2">r1</tspan>, <tspan data-hash="3">r2</tspan>);
    // <tspan data-hash="2">r1</tspan> and <tspan data-hash="3">r2</tspan> are no longer used after this point

    let <tspan data-hash="4">r3</tspan> = <tspan data-hash="1">&amp;mut s</tspan>; // no problem
    <tspan data-hash="6">println!</tspan>("{}", <tspan data-hash="4">r3</tspan>);

}
