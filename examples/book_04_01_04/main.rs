use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceOwner {
        hash: 4,
        name: String::from("s"),
        is_mut: true,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::None
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //
    vd.append_event(&s, Event::Acquire { from: None }, &(2 as usize));
    vd.append_event(&s, Event::GoOutOfScope, &(7 as usize));

    svg_generation::render_svg(&"04_01_04".to_owned(), &"".to_owned(), &vd);
}
