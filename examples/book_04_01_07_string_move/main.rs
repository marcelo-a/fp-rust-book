use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s1 = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s1"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Move
    });
    let s2 = ResourceAccessPoint::Owner(Owner {
        hash: 2,
        name: String::from("s2"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Move
    });
    let from_func = ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("String::from()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    vd.append_external_event(ExternalEvent::Duplicate{from: Some(from_func.clone()), to: Some(s1.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Move{from: Some(s1.clone()), to: Some(s2.clone())}, &(3 as usize));

    svg_generation::render_svg(&"04_01_07".to_owned(), &"string_move".to_owned(), &vd);
}
