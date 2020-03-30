use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let no_dangle = ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("no_dangle"),
    });
    let string_ctor = ResourceOwner::Function(Function {
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
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : no_dangle.clone() },  &(5 as usize));
    
    //rendering image
    svg_generation::render_svg(&"04_02_12".to_owned(), &"no_dangle".to_owned(), &vd);
}
