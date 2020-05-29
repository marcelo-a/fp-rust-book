use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let no_dangle = ResourceAccessPoint::Function(Function {
        hash: 2,
        name: String::from("no_dangle"),
    });
    let string_ctor = ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("String::from()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s gets resource from String::from
    vd.append_external_event(ExternalEvent::Move{from: Some(string_ctor.clone()), to: Some(s.clone())}, &(2 as usize));
    // move out as return value, does not render because to=None
    vd.append_external_event(ExternalEvent::Move{from: Some(s.clone()), to: None}, &(4 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.clone() },  &(5 as usize));
    
    //rendering image
    svg_generation::render_svg(&"04_02_12".to_owned(), &"no_dangle".to_owned(), &vd);
}
