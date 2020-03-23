use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let x = ResourceOwner {
        hash: 3,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::Copy
    };
    let y = ResourceOwner {
        hash: 4,
        name: String::from("y"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::Copy
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash x : 3
    //      y : 4
    //
    vd.append_event(&x, Event::Acquire { from: None }, &(2 as usize));
    vd.append_event(&x, Event::Move { to: Some(y.clone()) }, &(3 as usize));
    vd.append_event(&y, Event::Acquire { from: Some(x.clone()) }, &(3 as usize));
    vd.append_event(&x, Event::GoOutOfScope, &(4 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(4 as usize));

    svg_generation::render_svg(&"book_04_01_06".to_owned(), &vd);
}