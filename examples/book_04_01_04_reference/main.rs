use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s"),
        is_mut: true,
        lifetime_trait: LifetimeTrait::None
    });
    let from_func = ResourceAccessPoint::Function(Function {
        hash: 2,
        name: String::from("String::from()"),
    });
    let push_str = ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("push_str()"),
    });
    let println_func = ResourceAccessPoint::Function(Function {
        hash: 4,
        name: String::from("println!()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //
    vd.append_external_event(ExternalEvent::Move{from: Some(from_func.to_owned()), to: Some(s.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::PassByMutableReference{from: Some(s.clone()), to: Some(push_str.to_owned())}, &(4 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(s.clone()), to: Some(println_func.to_owned())}, &(6 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s.clone() }, &(7 as usize));

    svg_generation::render_svg(&"04_01_04".to_owned(), &"reference".to_owned(), &vd);
}
