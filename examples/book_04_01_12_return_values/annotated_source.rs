fn main() {
    let <tspan data-hash="1">s1</tspan> = <tspan class="fn" data-hash="0" hash="6">gives_ownership</tspan>();         // gives_ownership moves its return
                                        // value into s1

    let <tspan data-hash="2">s2</tspan> = <tspan class="fn" data-hash="0" hash="8">String::from</tspan>("hello");     // s2 comes into scope

    let <tspan data-hash="3">s3</tspan> = <tspan class="fn" data-hash="0" hash="7">takes_and_gives_back</tspan>(<tspan data-hash="2">s2</tspan>);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn <tspan class="fn" data-hash="0" hash="6">gives_ownership</tspan>() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let <tspan data-hash="4">some_string</tspan> = <tspan class="fn" data-hash="0" hash="8">String::from</tspan>("hello"); // some_string comes into scope

    <tspan data-hash="4">some_string</tspan>                             // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn <tspan class="fn" data-hash="0" hash="7">takes_and_gives_back</tspan>(<tspan data-hash="5">a_string</tspan>: String) -> String { // a_string comes into
                                                      // scope

    <tspan data-hash="5">a_string</tspan>  // a_string is returned and moves out to the calling function
}
