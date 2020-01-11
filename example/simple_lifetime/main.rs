use data::ResourceOwner;
use data::LifetimeTrait;

fn main() {
    let ro = ResourceOwner {
        hash: 14,
        name: String::from("var1"),
        lifetime_trait: LifetimeTrait::Move,
    };
}
