use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::None
    });
    let from_func = ResourceAccessPoint::Function(Function {
        hash: 2,
        name: String::from("String::from()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //
    vd.append_external_event(ExternalEvent::Move{from: Some(from_func.to_owned()), to: Some(s.clone())}, &(2 as usize));

    svg_generation::render_svg(&"04_01_03".to_owned(), &"string_from".to_owned(), &vd);
}
