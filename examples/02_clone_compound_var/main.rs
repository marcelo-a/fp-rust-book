use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of simple_lifetime/example.rs
fn main() {
    let y = ResourceOwner {
        hash: 1,
        name: String::from("y"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let z = ResourceOwner {
        hash: 2,
        name: String::from("z"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let a = ResourceOwner {
        hash: 3,
        name: String::from("a"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let b = ResourceOwner {
        hash: 4,
        name: String::from("b"),
        is_mut: true,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    //
    // hash y : 1
    //      z : 2
    //      a : 3
    //      b : 4
    // functions: 0
    vd.append_event(&y, Event::Acquire { from: None }, &(2 as usize));
    vd.append_event(&z, Event::Acquire { from: None }, &(3 as usize));
    vd.append_event(&a, Event::Acquire { from: Some(y.clone()) }, &(4 as usize));
    vd.append_event(&b, Event::Acquire { from: Some(z.clone()) }, &(7 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(10 as usize));
    vd.append_event(&z, Event::GoOutOfScope, &(10 as usize));
    vd.append_event(&a, Event::GoOutOfScope, &(10 as usize));
    vd.append_event(&b, Event::GoOutOfScope, &(10 as usize));

    //rendering image
    svg_generation::render_svg(&"02_clone_compound_var".to_owned(), &vd);
}
