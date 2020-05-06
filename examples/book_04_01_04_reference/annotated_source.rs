fn main() {
    let mut <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="2">String::from</tspan>("hello");

    <tspan data-hash="1">s</tspan>.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", <tspan data-hash="1">s</tspan>); // This will print `hello, world!`
}
