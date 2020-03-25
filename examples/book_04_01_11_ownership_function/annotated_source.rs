fn main() {
    let <tspan data-hash="1">s</tspan> = <tspan data-hash="2">String::from</tspan>("hello");  // s comes into scope

    <tspan data-hash="3">takes_ownership</tspan>(<tspan data-hash="1">s</tspan>);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let <tspan data-hash="4">x</tspan> = 5;                      // x comes into scope

    <tspan data-hash="5">makes_copy</tspan>(<tspan data-hash="4">x</tspan>);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn <tspan data-hash="3">takes_ownership</tspan>(<tspan data-hash="6">some_string</tspan>: String) { // some_string comes into scope
    <tspan data-hash="8">println!</tspan>("{}", <tspan data-hash="6">some_string</tspan>);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn <tspan data-hash="5">makes_copy</tspan>(<tspan data-hash="7">some_integer</tspan>: i32) { // some_integer comes into scope
    <tspan data-hash="8">println!</tspan>("{}", <tspan data-hash="7">some_integer</tspan>);
} // Here, some_integer goes out of scope. Nothing special happens.