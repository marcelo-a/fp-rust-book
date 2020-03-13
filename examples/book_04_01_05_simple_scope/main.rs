use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of simple_lifetime/example.rs
fn main() {
    let s = ResourceOwner {
        hash: 4,
        name: String::from("s"),
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
    // hash s : 1
    //
    vd.append_event(&s, Event::Acquire { from: None }, &(3 as usize));
    vd.append_event(&s, Event::GoOutOfScope, &(6 as usize));

    svg_generation::render_svg(&"book_04_01_05_simple_scope".to_owned(), &vd);
}
