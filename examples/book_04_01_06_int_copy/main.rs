use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let x = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy
    });
    let y = ResourceAccessPoint::Owner(Owner {
        hash: 2,
        name: String::from("y"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash x : 3
    //      y : 4
    //

    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: Some(x.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: Some(x.clone()), to: Some(y.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: y.clone() }, &(4 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: x.clone() }, &(4 as usize));

    svg_generation::render_svg(&"04_01_06".to_owned(), &"int_copy".to_owned(), &vd);
}