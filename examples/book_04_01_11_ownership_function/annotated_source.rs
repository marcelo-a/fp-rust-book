fn main() {
    let <tspan data-hash="1">s</tspan> = String::from("hello");  // s comes into scope

    takes_ownership(<tspan data-hash="1">s</tspan>);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let <tspan data-hash="2">x</tspan> = 5;                      // x comes into scope

    makes_copy(<tspan data-hash="2">x</tspan>);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(<tspan data-hash="3">some_string</tspan>: String) { // some_string comes into scope
    println!("{}", <tspan data-hash="3">some_string</tspan>);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(<tspan data-hash="4">some_integer</tspan>: i32) { // some_integer comes into scope
    println!("{}", <tspan data-hash="4">some_integer</tspan>);
} // Here, some_integer goes out of scope. Nothing special happens.