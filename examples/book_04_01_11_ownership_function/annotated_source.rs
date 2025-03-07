fn main() {
    let <tspan data-hash="1">s</tspan> = <tspan class="fn" data-hash="0" hash="5">String::from</tspan>("hello");  // s comes into scope

    <tspan class="fn" data-hash="0" hash="6">takes_ownership</tspan>(<tspan data-hash="1">s</tspan>);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let <tspan data-hash="2">x</tspan> = 5;                      // x comes into scope

    <tspan class="fn" data-hash="0" hash="7">makes_copy</tspan>(<tspan data-hash="2">x</tspan>);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn <tspan class="fn" data-hash="0" hash="6">takes_ownership</tspan>(<tspan data-hash="3">some_string</tspan>: String) { // some_string comes into scope
    println!("{}", <tspan data-hash="3">some_string</tspan>);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn <tspan class="fn" data-hash="0" hash="7">makes_copy</tspan>(<tspan data-hash="4">some_integer</tspan>: i32) { // some_integer comes into scope
    println!("{}", <tspan data-hash="4">some_integer</tspan>);
} // Here, some_integer goes out of scope. Nothing special happens.