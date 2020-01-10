extern crate rust_resource_lifetime;

fn main() {
    let ro = ResourceOwner {
        hash: 14,
        name: String::from("var1")
    };
}
