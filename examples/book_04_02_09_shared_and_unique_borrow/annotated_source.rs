fn main(){
    let mut <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="5">String::from</tspan>("hello");

    let <tspan data-hash="2">r1</tspan> = <tspan data-hash="1">&amp;s</tspan>; <tspan fill="#AAA">// no problem</tspan>
    let <tspan data-hash="3">r2</tspan> = <tspan data-hash="1">&amp;s</tspan>; <tspan fill="#AAA">// no problem</tspan>
    println!("{} and {}", <tspan data-hash="2">r1</tspan>, <tspan data-hash="3">r2</tspan>);
    <tspan fill="#AAA">// <tspan data-hash="2">r1</tspan> and <tspan data-hash="3">r2</tspan> are no longer used after this point</tspan>

    let <tspan data-hash="4">r3</tspan> = <tspan data-hash="1">&amp;mut s</tspan>; <tspan fill="#AAA">// no problem</tspan>
    println!("{}", <tspan data-hash="4">r3</tspan>);

}
