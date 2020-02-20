use rrt_lib::data::{Event, ExternalEvent, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of simple_lifetime/example.rs
fn main() {
    let x = ResourceOwner {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
    };
    let y = ResourceOwner {
        hash: 2,
        name: String::from("y"),
        is_mut: false,
    };
    let z = ResourceOwner {
        hash: 3,
        name: String::from("z"),
        is_mut: false,
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    vd.append_external_event(2, ExternalEvent::Acquire{from: None, to: Some(z.clone())});
    //
    // hash x : 1
    //      y : 2
    //      z : 3
    // functions: 0
    vd.append_event(&y, Event::Acquire { from: None }, &(2 as usize));
    vd.append_event(&z, Event::Acquire { from: None }, &(3 as usize));
    vd.append_event(
        &y,
        Event::Move {
            to: Some(x.clone()),
        },
        &(4 as usize),
    );
    vd.append_event(
        &x,
        Event::Acquire {
            from: Some(y.clone()),
        },
        &(4 as usize),
    );
    vd.append_event(&x, Event::GoOutOfScope, &(9 as usize));
    vd.append_event(&z, Event::GoOutOfScope, &(9 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(9 as usize));

    svg_generation::render_svg(&"move_with_no_borrow".to_owned(), &vd);
}
