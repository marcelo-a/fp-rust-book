fn main() {
    {
        let <tspan data-hash="4">s</tspan> = String::from("hello"); // s is valid from this point forward
    
        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}